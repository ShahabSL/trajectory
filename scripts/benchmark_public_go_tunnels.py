#!/usr/bin/env python3
"""Benchmark Go DNS tunnel competitors through public recursive resolvers.

This is a companion to `benchmark_public.py`. It keeps the same remote VPS sink
and public-resolver path, but adapts the Go TOML-configured TCP tunnel shape used
by StormDNS and MasterDnsVPN.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import secrets
import shlex
import socket
import subprocess
import tempfile
import threading
import textwrap
import time
from dataclasses import dataclass

from benchmark_public import (
    BENCH_SERVICE,
    DEFAULT_SERVER_ENV,
    REMOTE_SINK_PORT,
    REMOTE_SINK_SCRIPT,
    REMOTE_STATUS_PATH,
    REPO_ROOT,
    SINK_SERVICE,
    TRAJECTORY_SERVICE,
    TRAJECTORY_SOCKS_SERVICE,
    SshSession,
    choose_local_listen_port,
    ensure_remote_service_active,
    fetch_remote_status,
    load_server_auth,
    remote_is_active,
    run,
    stop_services,
    wait_for_remote_completion,
    wait_for_remote_dns_port_idle,
    wait_for_remote_status_ready,
)


REMOTE_STAGE_DIR = "/opt/trajectory-bench-go"
LIVE_PAYLOAD_SERVICE = "trajectory-live-payload.service"
DEFAULT_RESOLVER_FILE = REPO_ROOT / ".secrets" / "dnses.txt"
DEFAULT_RESOLVER_SOCKS_PROXY = "127.0.0.1:11092"
REMOTE_SOCKS_TARGET = "127.0.0.1:1080"
RESERVED_LOCAL_PORTS = {11092}
GO_IMPLEMENTATIONS = {
    "stormdns": REPO_ROOT / ".secrets" / "repos" / "StormDNS",
    "masterdnsvpn": REPO_ROOT / ".secrets" / "repos" / "MasterDnsVPN",
}
GO_BENCH_SERVICES = [
    BENCH_SERVICE,
    SINK_SERVICE,
    "trajectory-bench-stormdns.service",
    "trajectory-bench-masterdnsvpn.service",
    "trajectory-bench-trajectory.service",
    "trajectory-bench-slipstream.service",
]


@dataclass
class GoTunnelBuild:
    implementation: str
    repo_dir: pathlib.Path
    client: pathlib.Path
    server: pathlib.Path


@dataclass
class GoTunnelRuntime:
    implementation: str
    service_name: str
    client_config: pathlib.Path
    resolver_file: pathlib.Path


@dataclass
class GoTunnelResult:
    implementation: str
    bytes_sent: int
    bytes_delivered: int
    elapsed_seconds: float
    complete: bool
    timed_out: bool
    diagnostic: str = ""

    @property
    def bytes_per_second(self) -> float:
        if self.elapsed_seconds <= 0:
            return 0.0
        return self.bytes_delivered / self.elapsed_seconds


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server-env", type=pathlib.Path, default=DEFAULT_SERVER_ENV)
    parser.add_argument("--domain", default="t.7-b.cc")
    parser.add_argument("--size-bytes", type=int, default=65536)
    parser.add_argument("--timeout-seconds", type=int, default=180)
    parser.add_argument("--stall-seconds", type=int, default=12)
    parser.add_argument("--listen-port", type=int, default=27110)
    parser.add_argument("--resolver-file", type=pathlib.Path, default=DEFAULT_RESOLVER_FILE)
    parser.add_argument("--resolver-socks-proxy", default=DEFAULT_RESOLVER_SOCKS_PROXY)
    parser.add_argument("--fetch-url", default=f"http://127.0.0.1:{REMOTE_SINK_PORT}/payload")
    parser.add_argument(
        "--resolver",
        action="append",
        dest="resolvers",
        help="Public resolver to use. Repeat for multipath.",
    )
    parser.add_argument(
        "--implementation",
        action="append",
        choices=sorted(GO_IMPLEMENTATIONS),
        help="Implementation to benchmark. Repeatable; defaults to both.",
    )
    parser.add_argument("--keep-artifacts", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.resolvers = load_resolvers(args.resolvers, args.resolver_file)
    implementations = args.implementation or sorted(GO_IMPLEMENTATIONS)

    auth = load_server_auth(args.server_env)
    ssh = SshSession(auth)
    with tempfile.TemporaryDirectory(prefix="trajectory-go-bench-") as tmp_name:
        tmp = pathlib.Path(tmp_name)
        try:
            builds = [ensure_go_build(name) for name in implementations]
            runtimes = prepare_runtime_files(
                tmp,
                builds,
                args.domain,
                args.resolvers,
                args.resolver_socks_proxy,
            )
            install_remote_files(ssh, tmp, builds, runtimes, args.size_bytes, args.timeout_seconds + 120)
            stop_services(ssh, [*GO_BENCH_SERVICES, TRAJECTORY_SERVICE])
            wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)

            results = []
            for index, runtime in enumerate(runtimes):
                result = benchmark_once(
                    ssh=ssh,
                    runtime=runtime,
                    size_bytes=args.size_bytes,
                    timeout_seconds=args.timeout_seconds,
                    stall_seconds=args.stall_seconds,
                    preferred_listen_port=args.listen_port + index,
                    fetch_url=args.fetch_url,
                )
                results.append(result)
                print_result(result)
            print_comparison(results)
            return 0
        finally:
            cleanup_remote_benchmark(ssh)
            if not args.keep_artifacts:
                ssh.remote(
                    f"rm -f {shlex.quote(REMOTE_STATUS_PATH)}; systemctl daemon-reload >/dev/null 2>&1 || true",
                    check=False,
                )
            ssh.close()


def ensure_go_build(implementation: str) -> GoTunnelBuild:
    repo_dir = GO_IMPLEMENTATIONS[implementation]
    out_dir = REPO_ROOT / "target" / "go-bench" / implementation
    out_dir.mkdir(parents=True, exist_ok=True)
    client = out_dir / f"{implementation}-client"
    server = out_dir / f"{implementation}-server"
    run(["go", "build", "-o", str(client), "./cmd/client"], cwd=repo_dir, capture_output=False)
    run(["go", "build", "-o", str(server), "./cmd/server"], cwd=repo_dir, capture_output=False)
    return GoTunnelBuild(implementation=implementation, repo_dir=repo_dir, client=client, server=server)


def load_resolvers(resolver_args: list[str] | None, resolver_file: pathlib.Path) -> list[str]:
    if resolver_args:
        return resolver_args
    resolvers = []
    for line in resolver_file.read_text(encoding="utf-8").splitlines():
        entry = line.split("#", 1)[0].strip()
        if entry:
            resolvers.append(entry)
    if not resolvers:
        raise ValueError(f"resolver file is empty: {resolver_file}")
    return resolvers


def prepare_runtime_files(
    temp_dir: pathlib.Path,
    builds: list[GoTunnelBuild],
    domain: str,
    resolvers: list[str],
    resolver_socks_proxy: str,
) -> list[GoTunnelRuntime]:
    sink_script = temp_dir / "bench_sink.py"
    sink_script.write_text(REMOTE_SINK_SCRIPT, encoding="utf-8")

    runtimes = []
    for build in builds:
        key = secrets.token_hex(16)
        local_port = choose_local_listen_port(0)
        service_name = f"trajectory-bench-{build.implementation}.service"
        server_config = temp_dir / f"{build.implementation}-server_config.toml"
        client_config = temp_dir / f"{build.implementation}-client_config.toml"
        resolver_file = temp_dir / f"{build.implementation}-client_resolvers.txt"
        key_file = temp_dir / f"{build.implementation}-encrypt_key.txt"
        unit = temp_dir / service_name

        key_file.write_text(f"{key}\n", encoding="utf-8")
        resolver_file.write_text("\n".join(resolvers) + "\n", encoding="utf-8")
        server_config.write_text(
            render_server_config(
                implementation=build.implementation,
                domain=domain,
                key_file=key_file.name,
            ),
            encoding="utf-8",
        )
        client_config.write_text(
            render_client_config(
                implementation=build.implementation,
                domain=domain,
                key=key,
                listen_port=local_port,
                resolver_socks_proxy=resolver_socks_proxy,
            ),
            encoding="utf-8",
        )
        unit.write_text(
            textwrap.dedent(
                f"""\
                [Unit]
                Description={build.implementation} public DNS tunnel benchmark server
                After=network-online.target {SINK_SERVICE}
                Wants=network-online.target

                [Service]
                Type=simple
                WorkingDirectory={REMOTE_STAGE_DIR}
                ExecStart={REMOTE_STAGE_DIR}/{build.implementation}-server --config {REMOTE_STAGE_DIR}/{server_config.name}
                Restart=no
                RuntimeMaxSec=420

                [Install]
                WantedBy=multi-user.target
                """
            ),
            encoding="utf-8",
        )
        runtimes.append(
            GoTunnelRuntime(
                implementation=build.implementation,
                service_name=service_name,
                client_config=client_config,
                resolver_file=resolver_file,
            )
        )

    return runtimes


def render_server_config(*, implementation: str, domain: str, key_file: str) -> str:
    compatibility = ""
    if implementation == "stormdns":
        compatibility = textwrap.dedent(
            """\
            SESSION_ORPHAN_QUEUE_INITIAL_CAPACITY = 128
            STREAM_QUEUE_INITIAL_CAPACITY = 256
            DNS_FRAGMENT_STORE_CAPACITY = 512
            SOCKS5_FRAGMENT_STORE_CAPACITY = 1024
            MAX_STREAMS_PER_SESSION = 4096
            MAX_DNS_RESPONSE_BYTES = 32768
            """
        )

    return textwrap.dedent(
        f"""\
        PROTOCOL_TYPE = "SOCKS5"
        UDP_HOST = "0.0.0.0"
        UDP_PORT = 53
        DOMAIN = ["{domain}"]
        MIN_VPN_LABEL_LENGTH = 1
        DATA_ENCRYPTION_METHOD = 1
        ENCRYPTION_KEY_FILE = "{key_file}"
        USE_EXTERNAL_SOCKS5 = false
        ALLOW_PRIVATE_SOCKS_TARGETS = true
        FORWARD_IP = "{REMOTE_SOCKS_TARGET.split(':', 1)[0]}"
        FORWARD_PORT = {REMOTE_SOCKS_TARGET.rsplit(':', 1)[1]}
        LOG_LEVEL = "INFO"
        MAX_PACKETS_PER_BATCH = 5
        ARQ_WINDOW_SIZE = 4096
        ARQ_INITIAL_RTO_SECONDS = 0.35
        ARQ_MAX_RTO_SECONDS = 2.0
        UDP_READERS = 8
        DNS_REQUEST_WORKERS = 16
        DEFERRED_SESSION_WORKERS = 8
        MAX_CONCURRENT_REQUESTS = 32768
        SUPPORTED_UPLOAD_COMPRESSION_TYPES = [0, 1, 2, 3]
        SUPPORTED_DOWNLOAD_COMPRESSION_TYPES = [0, 1, 2, 3]
        SOCKET_BUFFER_SIZE = 8388608
        MAX_PACKET_SIZE = 65535
        DEFERRED_SESSION_QUEUE_LIMIT = 8192
        PACKET_BLOCK_CONTROL_DUPLICATION = 1
        STREAM_SETUP_ACK_TTL_SECONDS = 400.0
        STREAM_RESULT_PACKET_TTL_SECONDS = 300.0
        STREAM_FAILURE_PACKET_TTL_SECONDS = 120.0
        ARQ_CONTROL_INITIAL_RTO_SECONDS = 0.35
        ARQ_CONTROL_MAX_RTO_SECONDS = 2.0
        ARQ_MAX_CONTROL_RETRIES = 300
        ARQ_INACTIVITY_TIMEOUT_SECONDS = 1800.0
        ARQ_DATA_PACKET_TTL_SECONDS = 2400.0
        ARQ_CONTROL_PACKET_TTL_SECONDS = 1200.0
        ARQ_MAX_DATA_RETRIES = 1200
        ARQ_DATA_NACK_MAX_GAP = 128
        ARQ_DATA_NACK_INITIAL_DELAY_SECONDS = 0.35
        ARQ_DATA_NACK_REPEAT_SECONDS = 0.8
        ARQ_TERMINAL_DRAIN_TIMEOUT_SECONDS = 120.0
        ARQ_TERMINAL_ACK_WAIT_TIMEOUT_SECONDS = 90.0
        {compatibility}
        """
    )


def render_client_config(
    *,
    implementation: str,
    domain: str,
    key: str,
    listen_port: int,
    resolver_socks_proxy: str,
) -> str:
    if implementation == "stormdns":
        implementation_knobs = textwrap.dedent(
            """\
            STARTUP_MODE = "resolvers"
            UPLOAD_PACKET_DUPLICATION_COUNT = 1
            DOWNLOAD_PACKET_DUPLICATION_COUNT = 1
            UPLOAD_SETUP_PACKET_DUPLICATION_COUNT = 2
            DOWNLOAD_SETUP_PACKET_DUPLICATION_COUNT = 2
            MTU_TEST_RETRIES_RESOLVERS = 1
            MTU_TEST_TIMEOUT_RESOLVERS = 8.0
            MTU_TEST_PARALLELISM_RESOLVERS = 48
            MTU_TEST_RETRIES_LOGS = 0
            MTU_TEST_TIMEOUT_LOGS = 1.0
            MTU_TEST_PARALLELISM_LOGS = 1
            TX_CHANNEL_SIZE = 8192
            RESOLVER_UDP_CONNECTION_POOL_SIZE = 128
            STREAM_QUEUE_INITIAL_CAPACITY = 512
            ORPHAN_QUEUE_INITIAL_CAPACITY = 256
            """
        )
    else:
        implementation_knobs = textwrap.dedent(
            """\
            PACKET_DUPLICATION_COUNT = 1
            SETUP_PACKET_DUPLICATION_COUNT = 2
            MTU_TEST_RETRIES = 1
            MTU_TEST_TIMEOUT = 8.0
            MTU_TEST_PARALLELISM = 48
            SAVE_MTU_SERVERS_TO_FILE = false
            AUTO_REMOVE_LOW_MTU_SERVERS = true
            """
        )

    return textwrap.dedent(
        f"""\
        PROTOCOL_TYPE = "SOCKS5"
        LISTEN_IP = "127.0.0.1"
        LISTEN_PORT = {listen_port}
        DOMAINS = ["{domain}"]
        ENCRYPTION_KEY = "{key}"
        RESOLVER_SOCKS_PROXY = "{resolver_socks_proxy}"
        RESOLVER_BALANCING_STRATEGY = 3
        DATA_ENCRYPTION_METHOD = 1
        MIN_UPLOAD_MTU = 38
        MIN_DOWNLOAD_MTU = 100
        MAX_UPLOAD_MTU = 150
        MAX_DOWNLOAD_MTU = 1200
        TUNNEL_READER_WORKERS = 16
        TUNNEL_WRITER_WORKERS = 16
        TUNNEL_PROCESS_WORKERS = 16
        RX_CHANNEL_SIZE = 8192
        ARQ_WINDOW_SIZE = 2048
        ARQ_INITIAL_RTO_SECONDS = 0.35
        ARQ_MAX_RTO_SECONDS = 2.0
        DISPATCHER_IDLE_POLL_INTERVAL_SECONDS = 0.002
        LOG_LEVEL = "INFO"
        PING_AGGRESSIVE_INTERVAL_SECONDS = 0.050
        PING_LAZY_INTERVAL_SECONDS = 0.150
        PING_COOLDOWN_INTERVAL_SECONDS = 1.0
        PING_COLD_INTERVAL_SECONDS = 10.0
        PING_WARM_THRESHOLD_SECONDS = 10.0
        PING_COOL_THRESHOLD_SECONDS = 15.0
        PING_COLD_THRESHOLD_SECONDS = 30.0
        ARQ_CONTROL_INITIAL_RTO_SECONDS = 0.35
        ARQ_CONTROL_MAX_RTO_SECONDS = 2.0
        ARQ_INACTIVITY_TIMEOUT_SECONDS = 1800.0
        ARQ_DATA_PACKET_TTL_SECONDS = 2400.0
        ARQ_CONTROL_PACKET_TTL_SECONDS = 1200.0
        ARQ_MAX_DATA_RETRIES = 1200
        ARQ_DATA_NACK_MAX_GAP = 128
        STREAM_RESOLVER_FAILOVER_RESEND_THRESHOLD = 2
        STREAM_RESOLVER_FAILOVER_COOLDOWN = 1.0
        RECHECK_INACTIVE_SERVERS_ENABLED = true
        AUTO_DISABLE_TIMEOUT_SERVERS = true
        UPLOAD_COMPRESSION_TYPE = 0
        DOWNLOAD_COMPRESSION_TYPE = 0
        COMPRESSION_MIN_SIZE = 120
        TUNNEL_PACKET_TIMEOUT_SECONDS = 10.0
        MAX_PACKETS_PER_BATCH = 1
        ARQ_MAX_CONTROL_RETRIES = 300
        ARQ_DATA_NACK_INITIAL_DELAY_SECONDS = 0.35
        ARQ_DATA_NACK_REPEAT_SECONDS = 0.8
        {implementation_knobs}
        """
    )


def install_remote_files(
    ssh: SshSession,
    temp_dir: pathlib.Path,
    builds: list[GoTunnelBuild],
    runtimes: list[GoTunnelRuntime],
    size_bytes: int,
    runtime_max_seconds: int,
) -> None:
    sink_unit = temp_dir / SINK_SERVICE
    sink_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Go DNS tunnel benchmark HTTP payload service
            After=network-online.target
            Wants=network-online.target

            [Service]
            Type=simple
            ExecStart=/usr/bin/python3 {REMOTE_STAGE_DIR}/bench_sink.py --bind 127.0.0.1 --port {REMOTE_SINK_PORT} --status {REMOTE_STATUS_PATH} --size-bytes {size_bytes}
            Restart=no
            RuntimeMaxSec={runtime_max_seconds}

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )

    upload_paths = [temp_dir / "bench_sink.py", sink_unit]
    for build in builds:
        upload_paths.extend([build.client, build.server])
    for runtime in runtimes:
        upload_paths.extend(
            [
                temp_dir / runtime.service_name,
                runtime.client_config,
                runtime.resolver_file,
                temp_dir / f"{runtime.implementation}-server_config.toml",
                temp_dir / f"{runtime.implementation}-encrypt_key.txt",
            ]
        )

    ssh.remote(
        f"mkdir -p /tmp/trajectory-bench-go-upload {shlex.quote(REMOTE_STAGE_DIR)} "
        f"&& rm -f {shlex.quote(REMOTE_STATUS_PATH)}",
        check=True,
    )
    ssh.copy(upload_paths, "/tmp/trajectory-bench-go-upload/")

    commands = [
        f"install -m 755 /tmp/trajectory-bench-go-upload/bench_sink.py {REMOTE_STAGE_DIR}/bench_sink.py",
        f"install -m 644 /tmp/trajectory-bench-go-upload/{SINK_SERVICE} /etc/systemd/system/{SINK_SERVICE}",
    ]
    for build in builds:
        commands.append(
            f"install -m 755 /tmp/trajectory-bench-go-upload/{build.client.name} {REMOTE_STAGE_DIR}/{build.implementation}-client"
        )
        commands.append(
            f"install -m 755 /tmp/trajectory-bench-go-upload/{build.server.name} {REMOTE_STAGE_DIR}/{build.implementation}-server"
        )
    for runtime in runtimes:
        commands.extend(
            [
                f"install -m 644 /tmp/trajectory-bench-go-upload/{runtime.service_name} /etc/systemd/system/{runtime.service_name}",
                f"install -m 644 /tmp/trajectory-bench-go-upload/{runtime.implementation}-server_config.toml {REMOTE_STAGE_DIR}/{runtime.implementation}-server_config.toml",
                f"install -m 600 /tmp/trajectory-bench-go-upload/{runtime.implementation}-encrypt_key.txt {REMOTE_STAGE_DIR}/{runtime.implementation}-encrypt_key.txt",
            ]
        )
    commands.append("systemctl daemon-reload")
    ssh.remote("\n".join(commands), check=True)


def benchmark_once(
    *,
    ssh: SshSession,
    runtime: GoTunnelRuntime,
    size_bytes: int,
    timeout_seconds: int,
    stall_seconds: int,
    preferred_listen_port: int,
    fetch_url: str,
) -> GoTunnelResult:
    stop_services(ssh, [*GO_BENCH_SERVICES, TRAJECTORY_SERVICE])
    wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
    resolved_active = remote_is_active(ssh, "systemd-resolved")
    if resolved_active:
        ssh.remote("systemctl stop systemd-resolved >/dev/null 2>&1 || true", check=False)

    local_config = patch_local_listen_port(runtime.client_config, preferred_listen_port)
    listen_port = int(read_config_value(local_config, "LISTEN_PORT"))
    build = GO_IMPLEMENTATIONS[runtime.implementation]
    client_bin = (
        REPO_ROOT
        / "target"
        / "go-bench"
        / runtime.implementation
        / f"{runtime.implementation}-client"
    )
    client_cmd = [
        str(client_bin),
        "--config",
        str(local_config),
        "--resolvers",
        str(runtime.resolver_file),
    ]

    ssh.remote(
        textwrap.dedent(
            f"""\
            rm -f {shlex.quote(REMOTE_STATUS_PATH)}
            systemctl start {SINK_SERVICE}
            systemctl start {runtime.service_name}
            """
        ),
        check=True,
    )
    wait_for_remote_status_ready(ssh, timeout_seconds=15)
    ensure_remote_service_active(ssh, runtime.service_name, timeout_seconds=8)

    client = subprocess.Popen(
        client_cmd,
        cwd=build,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    client_output: list[str] = []
    start_output_reader(client, client_output)
    try:
        wait_for_output_pattern(
            client_output,
            "SOCKS5 Proxy server is listening",
            timeout_seconds=max(180, timeout_seconds),
            process=client,
        )
        fetch = start_socks_fetch(listen_port, timeout_seconds, fetch_url)
        status = wait_for_remote_completion(
            ssh,
            service=runtime.service_name,
            client_fetch=fetch,
            timeout_seconds=timeout_seconds,
            stall_seconds=stall_seconds,
        )
        client_fetch = finish_socks_fetch(fetch, abort=bool(status.get("timed_out", False)))
        delivered_bytes = min(int(status.get("bytes", 0)), client_fetch["bytes"])
        elapsed_seconds = max(float(status.get("elapsed", 0.0)), client_fetch["elapsed"])
        complete = (
            bool(status.get("complete", False))
            and not bool(status.get("timed_out", False))
            and client_fetch["http_code"] == 200
            and client_fetch["bytes"] == size_bytes
            and delivered_bytes == size_bytes
        )
        diagnostic = ""
        if not complete:
            diagnostic = build_failure_diagnostic(status, client_fetch, client_output)
        return GoTunnelResult(
            implementation=runtime.implementation,
            bytes_sent=size_bytes,
            bytes_delivered=delivered_bytes,
            elapsed_seconds=elapsed_seconds,
            complete=complete,
            timed_out=bool(status.get("timed_out", False)) or client_fetch["http_code"] != 200,
            diagnostic=diagnostic,
        )
    finally:
        client.terminate()
        try:
            client.wait(timeout=5)
        except subprocess.TimeoutExpired:
            client.kill()
        stop_services(ssh, GO_BENCH_SERVICES)
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
        if resolved_active:
            ssh.remote("systemctl start systemd-resolved >/dev/null 2>&1 || true", check=False)


def patch_local_listen_port(config_path: pathlib.Path, preferred_port: int) -> pathlib.Path:
    port = choose_candidate_port(preferred_port)
    text = config_path.read_text(encoding="utf-8")
    lines = []
    for line in text.splitlines():
        if line.strip().startswith("LISTEN_PORT"):
            lines.append(f"LISTEN_PORT = {port}")
        else:
            lines.append(line)
    patched = config_path.with_name(config_path.stem + "-active.toml")
    patched.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return patched


def choose_candidate_port(preferred_port: int) -> int:
    candidates = [preferred_port, 0, 0, 0, 0]
    for candidate in candidates:
        if candidate in RESERVED_LOCAL_PORTS:
            continue
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            sock.bind(("127.0.0.1", candidate))
            selected = int(sock.getsockname()[1])
            if selected in RESERVED_LOCAL_PORTS:
                continue
            return selected
        except OSError:
            continue
        finally:
            sock.close()
    return choose_local_listen_port(preferred_port)


def start_socks_fetch(port: int, timeout_seconds: int, url: str) -> subprocess.Popen[str]:
    return subprocess.Popen(
        [
            "curl",
            "--http1.1",
            "--silent",
            "--show-error",
            "--fail-with-body",
            "--socks5-hostname",
            f"127.0.0.1:{port}",
            "--noproxy",
            "",
            "--output",
            "/dev/null",
            "--write-out",
            "%{http_code} %{size_download} %{time_total}",
            "--connect-timeout",
            "60",
            "--max-time",
            str(timeout_seconds),
            url,
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )


def finish_socks_fetch(process: subprocess.Popen[str], *, abort: bool) -> dict[str, object]:
    started = time.perf_counter()
    if abort and process.poll() is None:
        process.terminate()
    try:
        if abort:
            stdout, stderr = process.communicate(timeout=5)
        else:
            stdout, stderr = process.communicate()
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, stderr = process.communicate()
    elapsed = time.perf_counter() - started
    parts = stdout.strip().split()
    if len(parts) == 3:
        http_code, size_download, time_total = parts
        return {
            "http_code": int(http_code),
            "bytes": int(float(size_download)),
            "elapsed": float(time_total),
            "returncode": process.returncode,
            "stderr": stderr.strip(),
            "stdout": stdout.strip(),
        }
    if process.returncode != 0:
        return {
            "http_code": 0,
            "bytes": 0,
            "elapsed": elapsed,
            "returncode": process.returncode,
            "stderr": stderr.strip(),
            "stdout": stdout.strip(),
        }
    else:
        raise RuntimeError(f"unexpected curl output: {stdout!r}")


def build_failure_diagnostic(
    status: dict[str, object],
    client_fetch: dict[str, object],
    client_output: list[str],
) -> str:
    interesting_terms = (
        "SOCKS",
        "CONNECT",
        "Stream",
        "Session",
        "error",
        "failed",
        "timeout",
        "closed",
        "MTU",
    )
    sensitive_terms = ("KEY", "ENCRYPTION", "SECRET", "TOKEN", "PASSWORD")
    lines = []
    for line in client_output[-120:]:
        upper = line.upper()
        if any(term in upper for term in sensitive_terms):
            continue
        if any(term.upper() in upper for term in interesting_terms):
            lines.append(line.strip())
    tail = os.linesep.join(lines[-30:])
    payload = {
        "remote_status": status,
        "curl_returncode": client_fetch.get("returncode"),
        "curl_stderr": client_fetch.get("stderr"),
        "curl_stdout": client_fetch.get("stdout"),
        "client_log_tail": tail,
    }
    return json.dumps(payload, sort_keys=True)


def read_config_value(config_path: pathlib.Path, key: str) -> str:
    prefix = f"{key} = "
    for line in config_path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped.startswith(prefix):
            return stripped.removeprefix(prefix).strip().strip('"')
    raise KeyError(key)


def start_output_reader(process: subprocess.Popen[str], output: list[str]) -> None:
    def read() -> None:
        if process.stdout is None:
            return
        for line in process.stdout:
            output.append(line)

    thread = threading.Thread(target=read, daemon=True)
    thread.start()


def wait_for_output_pattern(
    output: list[str],
    pattern: str,
    *,
    timeout_seconds: int,
    process: subprocess.Popen[str],
) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        if any(pattern in line for line in output):
            return
        if process.poll() is not None:
            raise RuntimeError(f"client exited before ready:\n{''.join(output)[-4000:]}")
        time.sleep(0.2)
    raise TimeoutError(f"client did not report readiness:\n{''.join(output)[-4000:]}")


def cleanup_remote_benchmark(ssh: SshSession) -> None:
    stop_services(ssh, GO_BENCH_SERVICES)
    try:
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
    except TimeoutError:
        pass
    names = " ".join(shlex.quote(service) for service in GO_BENCH_SERVICES)
    ssh.remote(f"systemctl reset-failed {names} >/dev/null 2>&1 || true", check=False)
    ssh.remote(f"rm -f {shlex.quote(REMOTE_STATUS_PATH)} >/dev/null 2>&1 || true", check=False)
    ssh.remote(
        f"systemctl restart {TRAJECTORY_SOCKS_SERVICE} {LIVE_PAYLOAD_SERVICE} {TRAJECTORY_SERVICE}",
        check=False,
    )
    ensure_remote_service_active(ssh, TRAJECTORY_SERVICE, timeout_seconds=15)


def print_result(result: GoTunnelResult) -> None:
    print(
        json.dumps(
            {
                "implementation": result.implementation,
                "bytes_sent": result.bytes_sent,
                "bytes_delivered": result.bytes_delivered,
                "elapsed_seconds": round(result.elapsed_seconds, 3),
                "bytes_per_second": round(result.bytes_per_second, 1),
                "complete": result.complete,
                "timed_out": result.timed_out,
                "diagnostic": result.diagnostic,
            },
            sort_keys=True,
        )
    )


def print_comparison(results: list[GoTunnelResult]) -> None:
    summary = {
        result.implementation: {
            "bps": round(result.bytes_per_second, 1),
            "complete": result.complete,
            "timed_out": result.timed_out,
        }
        for result in results
    }
    print(json.dumps({"comparison": summary}, sort_keys=True))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        raise SystemExit(130)
