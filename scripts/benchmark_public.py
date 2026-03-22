#!/usr/bin/env python3
"""Benchmark Trajectory against Slipstream through a public resolver.

This script:
1. Builds the local Trajectory binary.
2. Clones/builds upstream Slipstream locally if needed.
3. SSHes into the benchmark VPS from `.secrets/server.env`.
4. Installs a one-shot remote HTTP payload service plus a server unit for either implementation.
5. Runs the matching client locally through a public resolver.
6. Fetches the payload through the tunnel and compares delivered bytes and throughput.
"""

from __future__ import annotations

import argparse
import base64
import json
import os
import pathlib
import shlex
import secrets
import socket
import subprocess
import sys
import tempfile
import textwrap
import time
from dataclasses import dataclass


REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_SERVER_ENV = REPO_ROOT / ".secrets" / "server.env"
DEFAULT_SLIPSTREAM_DIR = pathlib.Path("/tmp/trajectory-slipstream-upstream")
DEFAULT_NATIVE_CERT_DIR = REPO_ROOT / "target" / "native-certs"
REMOTE_STAGE_DIR = "/opt/trajectory-bench"
REMOTE_STATUS_PATH = "/var/tmp/trajectory-bench-status.json"
REMOTE_SINK_PORT = 19000
BENCH_SERVICE = "trajectory-bench-impl.service"
SINK_SERVICE = "trajectory-bench-sink.service"
TRAJECTORY_SERVICE = "trajectory.service"
TRAJECTORY_SOCKS_SERVICE = "trajectory-socks.service"
BENCH_VARIANT_SERVICES = [
    BENCH_SERVICE,
    SINK_SERVICE,
    "trajectory-bench-trajectory.service",
    "trajectory-bench-slipstream.service",
]
BENCH_CLEANUP_SERVICES = [*BENCH_VARIANT_SERVICES, TRAJECTORY_SERVICE]

REMOTE_SINK_SCRIPT = textwrap.dedent(
    """\
    #!/usr/bin/env python3
    import argparse
    import json
    import os
    import socket
    import time

    parser = argparse.ArgumentParser()
    parser.add_argument("--bind", default="127.0.0.1")
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--status", required=True)
    parser.add_argument("--size-bytes", type=int, required=True)
    args = parser.parse_args()

    def flush_status(payload):
        tmp = args.status + ".tmp"
        with open(tmp, "w", encoding="utf-8") as handle:
            json.dump(payload, handle)
        os.replace(tmp, args.status)

    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind((args.bind, args.port))
    server.listen(1)
    flush_status({"ready": True, "complete": False, "bytes": 0, "elapsed": 0.0})

    conn, _ = server.accept()
    conn.settimeout(2.0)
    total = 0
    header = b""
    body_chunk = b"x" * 65536

    with conn:
        while b"\\r\\n\\r\\n" not in header:
            try:
                chunk = conn.recv(4096)
            except socket.timeout:
                continue
            if not chunk:
                flush_status({"ready": True, "complete": False, "bytes": 0, "elapsed": 0.0})
                raise SystemExit(1)
            header += chunk

        response_header = (
            f"HTTP/1.1 200 OK\\r\\n"
            f"Content-Length: {args.size_bytes}\\r\\n"
            f"Content-Type: application/octet-stream\\r\\n"
            f"Connection: close\\r\\n"
            f"\\r\\n"
        ).encode("ascii")
        conn.sendall(response_header)

        start = time.perf_counter()
        last_flush = start
        while total < args.size_bytes:
            part = body_chunk[: min(len(body_chunk), args.size_bytes - total)]
            now = time.perf_counter()
            try:
                conn.sendall(part)
            except (BrokenPipeError, ConnectionResetError):
                flush_status(
                    {
                        "ready": True,
                        "complete": False,
                        "bytes": total,
                        "elapsed": now - start,
                    }
                )
                raise SystemExit(1)
            total += len(part)
            if now - last_flush >= 0.2:
                flush_status(
                    {
                        "ready": True,
                        "complete": False,
                        "bytes": total,
                        "elapsed": now - start,
                    }
                )
                last_flush = now

    elapsed = time.perf_counter() - start
    flush_status({"ready": True, "complete": True, "bytes": total, "elapsed": elapsed})
    """
)


@dataclass
class ServerAuth:
    host: str
    password: str
    user: str = "root"


@dataclass
class BenchResult:
    implementation: str
    bytes_sent: int
    bytes_delivered: int
    elapsed_seconds: float
    complete: bool
    timed_out: bool

    @property
    def bytes_per_second(self) -> float:
        if self.elapsed_seconds <= 0:
            return 0.0
        return self.bytes_delivered / self.elapsed_seconds


@dataclass
class BenchAccessKey:
    access_key: str
    registry_path: pathlib.Path


class SshSession:
    def __init__(self, auth: ServerAuth):
        self.auth = auth
        self.tempdir = tempfile.TemporaryDirectory(prefix="trajectory-bench-")
        self.temp_path = pathlib.Path(self.tempdir.name)
        self.askpass = self.temp_path / "askpass.sh"
        self.known_hosts = self.temp_path / "known_hosts"
        self.askpass.write_text("#!/bin/sh\nprintf '%s\\n' \"$BENCH_PASSWORD\"\n", encoding="utf-8")
        self.askpass.chmod(0o700)
        self._seed_known_hosts()

    def close(self) -> None:
        self.tempdir.cleanup()

    def base_env(self) -> dict[str, str]:
        env = os.environ.copy()
        env.update(
            {
                "DISPLAY": ":0",
                "SSH_ASKPASS": str(self.askpass),
                "SSH_ASKPASS_REQUIRE": "force",
                "BENCH_PASSWORD": self.auth.password,
            }
        )
        return env

    def _seed_known_hosts(self) -> None:
        scan = run(
            ["ssh-keyscan", "-H", self.auth.host],
            capture_output=True,
            check=True,
        ).stdout
        self.known_hosts.write_text(scan, encoding="utf-8")

    def ssh_args(self) -> list[str]:
        return [
            "ssh",
            "-o",
            "PreferredAuthentications=password",
            "-o",
            "PubkeyAuthentication=no",
            "-o",
            "NumberOfPasswordPrompts=1",
            "-o",
            "ConnectTimeout=10",
            "-o",
            "ServerAliveInterval=5",
            "-o",
            "ServerAliveCountMax=3",
            "-o",
            f"UserKnownHostsFile={self.known_hosts}",
            "-o",
            "StrictHostKeyChecking=yes",
            f"{self.auth.user}@{self.auth.host}",
        ]

    def scp_args(self) -> list[str]:
        return [
            "scp",
            "-o",
            "PreferredAuthentications=password",
            "-o",
            "PubkeyAuthentication=no",
            "-o",
            "NumberOfPasswordPrompts=1",
            "-o",
            "ConnectTimeout=10",
            "-o",
            "ServerAliveInterval=5",
            "-o",
            "ServerAliveCountMax=3",
            "-o",
            f"UserKnownHostsFile={self.known_hosts}",
            "-o",
            "StrictHostKeyChecking=yes",
        ]

    def remote(
        self,
        command: str,
        check: bool = True,
        timeout_seconds: int = 45,
    ) -> subprocess.CompletedProcess[str]:
        return run(
            ["setsid", "-w", *self.ssh_args(), command],
            env=self.base_env(),
            capture_output=True,
            check=check,
            timeout_seconds=timeout_seconds,
        )

    def copy(self, paths: list[pathlib.Path], remote_dir: str, timeout_seconds: int = 120) -> None:
        run(
            ["setsid", "-w", *self.scp_args(), *map(str, paths), f"{self.auth.user}@{self.auth.host}:{remote_dir}"],
            env=self.base_env(),
            capture_output=True,
            check=True,
            timeout_seconds=timeout_seconds,
        )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server-env", type=pathlib.Path, default=DEFAULT_SERVER_ENV)
    parser.add_argument(
        "--resolver",
        action="append",
        dest="resolvers",
        help="Public resolver to use. Repeat to benchmark multipath over several resolvers.",
    )
    parser.add_argument("--domain", default="test.example.com")
    parser.add_argument("--size-bytes", type=int, default=65536)
    parser.add_argument("--slipstream-dir", type=pathlib.Path, default=DEFAULT_SLIPSTREAM_DIR)
    parser.add_argument("--native-cert-dir", type=pathlib.Path, default=DEFAULT_NATIVE_CERT_DIR)
    parser.add_argument("--keep-artifacts", action="store_true")
    parser.add_argument("--timeout-seconds", type=int, default=180)
    parser.add_argument("--stall-seconds", type=int, default=8)
    parser.add_argument("--trajectory-listen-port", type=int, default=27010)
    parser.add_argument("--slipstream-listen-port", type=int, default=27011)
    parser.add_argument("--trajectory-keep-alive-interval", type=int, default=0)
    parser.add_argument("--trajectory-client-bin", type=pathlib.Path, default=None)
    parser.add_argument("--trajectory-server-bin", type=pathlib.Path, default=None)
    parser.add_argument(
        "--trajectory-client-db",
        default="/opt/trajectory/trajectory-clients.json",
        help="Remote client registry path for the authenticated Trajectory server.",
    )
    parser.add_argument(
        "--trajectory-access-key",
        default=None,
        help="Client access key to use when benchmarking the authenticated Trajectory client.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if not args.resolvers:
        args.resolvers = [
            "1.1.1.1:53",
            "1.0.0.1:53",
            "8.8.8.8:53",
            "8.8.4.4:53",
            "9.9.9.9:53",
        ]
    auth = load_server_auth(args.server_env)
    ssh = SshSession(auth)
    try:
        trajectory_paths = ensure_trajectory_build(
            client_override=args.trajectory_client_bin,
            server_override=args.trajectory_server_bin,
        )
        slipstream_paths = ensure_slipstream_build(args.slipstream_dir)
        native_cert_paths = ensure_native_certs(args.native_cert_dir)
        bench_access = generate_bench_access_key(ssh.temp_path)

        stop_services(ssh, [*BENCH_VARIANT_SERVICES, TRAJECTORY_SERVICE])
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
        install_remote_files(
            ssh,
            trajectory_paths,
            slipstream_paths,
            native_cert_paths,
            args.domain,
            bench_access.registry_path,
            args.timeout_seconds + 120,
            args.size_bytes,
        )

        results = []
        for implementation in ("trajectory", "slipstream"):
            result = benchmark_once(
                ssh=ssh,
                implementation=implementation,
                domain=args.domain,
                resolvers=args.resolvers,
                size_bytes=args.size_bytes,
                timeout_seconds=args.timeout_seconds,
                stall_seconds=args.stall_seconds,
                trajectory_keep_alive_interval=args.trajectory_keep_alive_interval,
                trajectory_paths=trajectory_paths,
                slipstream_client=slipstream_paths["client"],
                trajectory_listen_port=args.trajectory_listen_port,
                slipstream_listen_port=args.slipstream_listen_port,
                trajectory_access_key=bench_access.access_key,
                resolved_active=remote_is_active(ssh, "systemd-resolved"),
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


def load_server_auth(path: pathlib.Path) -> ServerAuth:
    values: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    return ServerAuth(host=values["ip"], password=values["password"])


def run(
    args: list[str],
    *,
    cwd: pathlib.Path | None = None,
    env: dict[str, str] | None = None,
    capture_output: bool = True,
    check: bool = True,
    timeout_seconds: int | None = None,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        args,
        cwd=cwd,
        env=env,
        text=True,
        capture_output=capture_output,
        check=check,
        timeout=timeout_seconds,
    )


def ensure_trajectory_build(
    *,
    client_override: pathlib.Path | None = None,
    server_override: pathlib.Path | None = None,
) -> dict[str, pathlib.Path]:
    if client_override is not None or server_override is not None:
        if client_override is None or server_override is None:
            raise ValueError("both trajectory override binaries are required")
        return {"client": client_override, "server": server_override}

    run(
        ["cargo", "build", "--release", "-p", "trajectory-cli", "--bin", "trajectory-client"],
        cwd=REPO_ROOT,
        capture_output=False,
    )
    run(
        ["cargo", "build", "--release", "-p", "trajectory-cli", "--bin", "trajectory-server"],
        cwd=REPO_ROOT,
        capture_output=False,
    )
    return {
        "client": REPO_ROOT / "target" / "release" / "trajectory-client",
        "server": REPO_ROOT / "target" / "release" / "trajectory-server",
    }


def ensure_slipstream_build(target_dir: pathlib.Path) -> dict[str, pathlib.Path]:
    if not (target_dir / ".git").exists():
        run(
            [
                "git",
                "clone",
                "--depth",
                "1",
                "--recurse-submodules",
                "https://github.com/EndPositive/slipstream",
                str(target_dir),
            ],
            capture_output=False,
        )

    if not (target_dir / "build" / "slipstream-server").exists():
        if not (target_dir / "build" / "build.ninja").exists():
            run(
                ["meson", "setup", "build", "--buildtype=release"],
                cwd=target_dir,
                capture_output=False,
            )
        run(["meson", "compile", "-C", "build"], cwd=target_dir, capture_output=False)

    return {
        "server": target_dir / "build" / "slipstream-server",
        "client": target_dir / "build" / "slipstream-client",
        "cert": target_dir / "certs" / "cert.pem",
        "key": target_dir / "certs" / "key.pem",
    }


def ensure_native_certs(target_dir: pathlib.Path) -> dict[str, pathlib.Path]:
    cert = target_dir / "cert.pem"
    key = target_dir / "key.pem"
    if cert.exists() and key.exists():
        return {"cert": cert, "key": key}

    target_dir.mkdir(parents=True, exist_ok=True)
    run(
        [
            "openssl",
            "req",
            "-x509",
            "-nodes",
            "-newkey",
            "rsa:2048",
            "-sha256",
            "-days",
            "365",
            "-keyout",
            str(key),
            "-out",
            str(cert),
            "-subj",
            "/CN=test.example.com",
            "-addext",
            "basicConstraints=critical,CA:FALSE",
            "-addext",
            "keyUsage=critical,digitalSignature,keyEncipherment",
            "-addext",
            "extendedKeyUsage=serverAuth",
            "-addext",
            "subjectAltName=DNS:test.example.com,DNS:localhost",
        ],
        capture_output=False,
    )
    return {"cert": cert, "key": key}


def remote_is_active(ssh: SshSession, service: str) -> bool:
    result = ssh.remote(f"systemctl is-active {service}", check=False)
    return result.returncode == 0 and result.stdout.strip() == "active"


def stop_services(ssh: SshSession, services: list[str]) -> None:
    names = " ".join(shlex.quote(service) for service in services)
    try:
        ssh.remote(
            f"systemctl stop {names} >/dev/null 2>&1 || true",
            check=False,
            timeout_seconds=15,
        )
    except subprocess.TimeoutExpired:
        pass


def wait_for_remote_dns_port_idle(ssh: SshSession, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        try:
            result = ssh.remote(
                "ss -ltnup | grep -E ':53[[:space:]]|:53$' >/dev/null 2>&1; echo $?",
                check=False,
                timeout_seconds=15,
            )
        except subprocess.TimeoutExpired:
            time.sleep(0.5)
            continue
        if result.stdout.strip() == "1":
            return
        time.sleep(0.5)
    raise TimeoutError("remote dns port 53 did not become idle")


def cleanup_remote_benchmark(ssh: SshSession) -> None:
    stop_services(ssh, BENCH_VARIANT_SERVICES)
    try:
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
    except TimeoutError:
        pass
    ssh.remote(
        " ".join(
            [
                "systemctl reset-failed",
                *[shlex.quote(service) for service in BENCH_VARIANT_SERVICES],
                ">/dev/null 2>&1 || true",
            ]
        ),
        check=False,
    )
    ssh.remote(f"rm -f {shlex.quote(REMOTE_STATUS_PATH)} >/dev/null 2>&1 || true", check=False)
    ssh.remote(
        f"systemctl restart {TRAJECTORY_SOCKS_SERVICE} {TRAJECTORY_SERVICE}",
        check=False,
    )
    ensure_remote_service_active(ssh, TRAJECTORY_SERVICE, timeout_seconds=15)


def install_remote_files(
    ssh: SshSession,
    trajectory_paths: dict[str, pathlib.Path],
    slipstream_paths: dict[str, pathlib.Path],
    native_cert_paths: dict[str, pathlib.Path],
    domain: str,
    bench_registry_path: pathlib.Path,
    runtime_max_seconds: int,
    size_bytes: int,
) -> None:
    temp = ssh.temp_path
    sink_script = temp / "bench_sink.py"
    sink_script.write_text(REMOTE_SINK_SCRIPT, encoding="utf-8")

    sink_unit = temp / SINK_SERVICE
    sink_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Trajectory benchmark HTTP payload service
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

    trajectory_unit = temp / "trajectory-bench-trajectory.service"
    trajectory_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Trajectory benchmark server
            After=network-online.target {SINK_SERVICE}
            Wants=network-online.target

            [Service]
            Type=simple
            WorkingDirectory={REMOTE_STAGE_DIR}
            ExecStart={REMOTE_STAGE_DIR}/trajectory-server --dns-listen-port 53 --target-address 127.0.0.1:{REMOTE_SINK_PORT} --domain {domain} --client-db {REMOTE_STAGE_DIR}/trajectory-bench-clients.json --cert {REMOTE_STAGE_DIR}/cert.pem --key {REMOTE_STAGE_DIR}/key.pem
            Restart=no
            RuntimeMaxSec={runtime_max_seconds}

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )

    slipstream_unit = temp / "trajectory-bench-slipstream.service"
    slipstream_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Slipstream benchmark server
            After=network-online.target {SINK_SERVICE}
            Wants=network-online.target

            [Service]
            Type=simple
            WorkingDirectory={REMOTE_STAGE_DIR}
            ExecStart={REMOTE_STAGE_DIR}/slipstream-server --dns-listen-port=53 --target-address=127.0.0.1:{REMOTE_SINK_PORT} --domain {domain} --cert {REMOTE_STAGE_DIR}/cert.pem --key {REMOTE_STAGE_DIR}/key.pem
            Restart=no
            RuntimeMaxSec={runtime_max_seconds}

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )

    ssh.remote(
        "mkdir -p /tmp/trajectory-bench-upload "
        f"{shlex.quote(REMOTE_STAGE_DIR)} "
        f"&& rm -f {shlex.quote(REMOTE_STATUS_PATH)}",
        check=True,
    )
    ssh.copy(
        [
            trajectory_paths["client"],
            trajectory_paths["server"],
            slipstream_paths["client"],
            slipstream_paths["server"],
            slipstream_paths["cert"],
            slipstream_paths["key"],
            native_cert_paths["cert"],
            native_cert_paths["key"],
            bench_registry_path,
            sink_script,
            sink_unit,
            trajectory_unit,
            slipstream_unit,
        ],
        "/tmp/trajectory-bench-upload/",
    )
    ssh.remote(
        textwrap.dedent(
            f"""\
            install -m 755 /tmp/trajectory-bench-upload/{trajectory_paths["client"].name} {REMOTE_STAGE_DIR}/trajectory-client
            install -m 755 /tmp/trajectory-bench-upload/{trajectory_paths["server"].name} {REMOTE_STAGE_DIR}/trajectory-server
            install -d -m 755 {REMOTE_STAGE_DIR}/slipstream/build {REMOTE_STAGE_DIR}/slipstream/certs
            install -m 755 /tmp/trajectory-bench-upload/{slipstream_paths["client"].name} {REMOTE_STAGE_DIR}/slipstream/build/slipstream-client
            install -m 755 /tmp/trajectory-bench-upload/{slipstream_paths["server"].name} {REMOTE_STAGE_DIR}/slipstream/build/slipstream-server
            install -m 644 /tmp/trajectory-bench-upload/{slipstream_paths["cert"].name} {REMOTE_STAGE_DIR}/slipstream/certs/cert.pem
            install -m 600 /tmp/trajectory-bench-upload/{slipstream_paths["key"].name} {REMOTE_STAGE_DIR}/slipstream/certs/key.pem
            install -m 755 /tmp/trajectory-bench-upload/{slipstream_paths["server"].name} {REMOTE_STAGE_DIR}/slipstream-server
            install -m 644 /tmp/trajectory-bench-upload/{slipstream_paths["cert"].name} {REMOTE_STAGE_DIR}/cert.pem
            install -m 600 /tmp/trajectory-bench-upload/{slipstream_paths["key"].name} {REMOTE_STAGE_DIR}/key.pem
            install -m 644 /tmp/trajectory-bench-upload/{native_cert_paths["cert"].name} {REMOTE_STAGE_DIR}/native-cert.pem
            install -m 600 /tmp/trajectory-bench-upload/{native_cert_paths["key"].name} {REMOTE_STAGE_DIR}/native-key.pem
            install -m 600 /tmp/trajectory-bench-upload/{bench_registry_path.name} {REMOTE_STAGE_DIR}/trajectory-bench-clients.json
            install -m 755 /tmp/trajectory-bench-upload/{sink_script.name} {REMOTE_STAGE_DIR}/bench_sink.py
            install -m 644 /tmp/trajectory-bench-upload/{sink_unit.name} /etc/systemd/system/{SINK_SERVICE}
            install -m 644 /tmp/trajectory-bench-upload/{trajectory_unit.name} /etc/systemd/system/{trajectory_unit.name}
            install -m 644 /tmp/trajectory-bench-upload/{slipstream_unit.name} /etc/systemd/system/{slipstream_unit.name}
            systemctl daemon-reload
            """
        ),
        check=True,
    )


def benchmark_once(
    *,
    ssh: SshSession,
    implementation: str,
    domain: str,
    resolvers: list[str],
    size_bytes: int,
    timeout_seconds: int,
    stall_seconds: int,
    trajectory_keep_alive_interval: int,
    trajectory_paths: dict[str, pathlib.Path],
    slipstream_client: pathlib.Path,
    trajectory_listen_port: int,
    slipstream_listen_port: int,
    trajectory_access_key: str | None,
    resolved_active: bool,
) -> BenchResult:
    needs_resolver_port = True

    if implementation == "trajectory":
        service_source = "trajectory-bench-trajectory.service"
        listen_port = choose_local_listen_port(trajectory_listen_port)
        client_cmd = [
            str(trajectory_paths["client"]),
            "--tcp-listen-port",
            str(listen_port),
            "--domain",
            domain,
            "--keep-alive-interval",
            str(trajectory_keep_alive_interval),
            "--congestion-control",
            "bbr",
        ]
        if trajectory_access_key is None:
            raise ValueError("trajectory benchmark requires --trajectory-access-key")
        client_cmd.extend(["--access-key", trajectory_access_key])
        for resolver in resolvers:
            client_cmd.extend(["--resolver", resolver])
    else:
        service_source = "trajectory-bench-slipstream.service"
        listen_port = choose_local_listen_port(slipstream_listen_port)
        client_cmd = [
            str(slipstream_client),
            "--tcp-listen-port",
            str(listen_port),
            "--domain",
            domain,
            "--congestion-control",
            "bbr",
            "--keep-alive-interval",
            "100",
        ]
        for resolver in resolvers:
            client_cmd.extend(["--resolver", resolver])

    if needs_resolver_port and resolved_active:
        ssh.remote("systemctl stop systemd-resolved >/dev/null 2>&1 || true", check=False)

    stop_services(ssh, [*BENCH_VARIANT_SERVICES, TRAJECTORY_SERVICE])
    wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
    ssh.remote(
        textwrap.dedent(
            f"""\
            rm -f {shlex.quote(REMOTE_STATUS_PATH)}
            cp /etc/systemd/system/{service_source} /etc/systemd/system/{BENCH_SERVICE}
            systemctl daemon-reload
            systemctl start {SINK_SERVICE}
            systemctl start {BENCH_SERVICE}
            """
        ),
        check=True,
    )
    wait_for_remote_status_ready(ssh, timeout_seconds=15)
    ensure_remote_service_active(ssh, BENCH_SERVICE, timeout_seconds=5)

    client = subprocess.Popen(
        client_cmd,
        cwd=REPO_ROOT if implementation == "trajectory" else slipstream_client.parent.parent,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    try:
        time.sleep(2)
        if client.poll() is not None:
            raise RuntimeError(client.stdout.read() if client.stdout else "client exited early")

        fetch = start_http_fetch(listen_port, timeout_seconds)
        status = wait_for_remote_completion(
            ssh,
            service=BENCH_SERVICE,
            client_fetch=fetch,
            timeout_seconds=timeout_seconds,
            stall_seconds=stall_seconds,
        )
        client_fetch = finish_http_fetch(fetch, abort=bool(status.get("timed_out", False)))
        delivered_bytes = min(int(status.get("bytes", 0)), client_fetch["bytes"])
        elapsed_seconds = max(float(status.get("elapsed", 0.0)), client_fetch["elapsed"])
        complete = (
            bool(status.get("complete", False))
            and not bool(status.get("timed_out", False))
            and client_fetch["http_code"] == 200
            and client_fetch["bytes"] == size_bytes
            and delivered_bytes == size_bytes
        )
        return BenchResult(
            implementation=implementation,
            bytes_sent=size_bytes,
            bytes_delivered=delivered_bytes,
            elapsed_seconds=elapsed_seconds,
            complete=complete,
            timed_out=bool(status.get("timed_out", False)) or client_fetch["http_code"] != 200,
        )
    finally:
        client.terminate()
        try:
            client.wait(timeout=5)
        except subprocess.TimeoutExpired:
            client.kill()
        stop_services(ssh, BENCH_VARIANT_SERVICES)
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
        if needs_resolver_port and resolved_active:
            ssh.remote("systemctl start systemd-resolved >/dev/null 2>&1 || true", check=False)


def wait_for_remote_status_ready(ssh: SshSession, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        status = fetch_remote_status(ssh)
        if status.get("ready"):
            return
        time.sleep(0.5)
    raise TimeoutError("remote sink did not become ready")


def choose_local_listen_port(preferred_port: int) -> int:
    candidates = [preferred_port, 0, 0, 0]
    for candidate in candidates:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            sock.bind(("127.0.0.1", candidate))
            return int(sock.getsockname()[1])
        except OSError:
            continue
        finally:
            sock.close()
    raise RuntimeError("failed to find a free local benchmark listen port")


def ensure_remote_service_active(ssh: SshSession, service: str, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        if remote_is_active(ssh, service):
            return
        time.sleep(0.5)

    journal = ssh.remote(
        f"journalctl -u {shlex.quote(service)} -n 50 --no-pager --output=cat",
        check=False,
    ).stdout
    raise RuntimeError(f"{service} failed to stay active:\\n{journal}")


def fetch_remote_status(ssh: SshSession) -> dict[str, object]:
    result = ssh.remote(f"cat {shlex.quote(REMOTE_STATUS_PATH)} 2>/dev/null || true", check=False)
    if not result.stdout.strip():
        return {}
    return json.loads(result.stdout)


def wait_for_remote_completion(
    ssh: SshSession,
    *,
    service: str,
    client_fetch: subprocess.Popen[str],
    timeout_seconds: int,
    stall_seconds: int,
) -> dict[str, object]:
    deadline = time.time() + timeout_seconds
    last_bytes = -1
    last_progress = time.time()

    while time.time() < deadline:
        status = fetch_remote_status(ssh)
        current_bytes = int(status.get("bytes", 0))
        if current_bytes != last_bytes:
            last_bytes = current_bytes
            last_progress = time.time()
        if status.get("complete"):
            return status
        if client_fetch.poll() is not None and current_bytes == 0 and not status.get("complete"):
            status["timed_out"] = True
            status["service_failed"] = not remote_is_active(ssh, service)
            return status
        if current_bytes == 0 and not remote_is_active(ssh, service):
            status["timed_out"] = True
            status["service_failed"] = True
            return status
        if time.time() - last_progress >= stall_seconds:
            status["timed_out"] = True
            return status
        time.sleep(1.0)

    status = fetch_remote_status(ssh)
    status["timed_out"] = True
    return status


def generate_bench_access_key(temp_dir: pathlib.Path) -> BenchAccessKey:
    client_id = secrets.randbits(32)
    secret = secrets.token_bytes(32)
    secret_base32 = base64.b32encode(secret).decode("ascii").rstrip("=")
    access_key = f"traj1_{client_id:08x}_{secret_base32}"
    registry_path = temp_dir / "trajectory-bench-clients.json"
    registry_path.write_text(
        json.dumps(
            {
                "keys": [
                    {
                        "id": client_id,
                        "label": "Benchmark",
                        "secret_base32": secret_base32,
                        "created_unix": int(time.time()),
                        "enabled": True,
                    }
                ]
            }
        ),
        encoding="utf-8",
    )
    return BenchAccessKey(access_key=access_key, registry_path=registry_path)


def start_http_fetch(port: int, timeout_seconds: int) -> subprocess.Popen[str]:
    return subprocess.Popen(
        [
            "curl",
            "--http1.1",
            "--silent",
            "--show-error",
            "--output",
            "/dev/null",
            "--write-out",
            "%{http_code} %{size_download} %{time_total}",
            "--max-time",
            str(timeout_seconds),
            f"http://127.0.0.1:{port}/payload",
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )


def finish_http_fetch(process: subprocess.Popen[str], *, abort: bool) -> dict[str, object]:
    started = time.perf_counter()
    if abort and process.poll() is None:
        process.terminate()
    try:
        stdout, _stderr = process.communicate(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, _stderr = process.communicate()
    elapsed = time.perf_counter() - started
    if process.returncode != 0:
        return {"http_code": 0, "bytes": 0, "elapsed": elapsed}
    parts = stdout.strip().split()
    if len(parts) != 3:
        raise RuntimeError(f"unexpected curl output: {stdout!r}")
    http_code, size_download, time_total = parts
    return {
        "http_code": int(http_code),
        "bytes": int(float(size_download)),
        "elapsed": float(time_total),
    }


def print_result(result: BenchResult) -> None:
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
            },
            sort_keys=True,
        )
    )


def print_comparison(results: list[BenchResult]) -> None:
    by_name = {result.implementation: result for result in results}
    if "trajectory" not in by_name or "slipstream" not in by_name:
        return

    trajectory = by_name["trajectory"]
    slipstream = by_name["slipstream"]
    ratio = 0.0
    if trajectory.bytes_per_second > 0:
        ratio = slipstream.bytes_per_second / trajectory.bytes_per_second

    summary = {
        "trajectory_bps": round(trajectory.bytes_per_second, 1),
        "slipstream_bps": round(slipstream.bytes_per_second, 1),
        "speedup_ratio": round(ratio, 2),
        "trajectory_complete": trajectory.complete,
        "slipstream_complete": slipstream.complete,
    }
    print(json.dumps({"comparison": summary}, sort_keys=True))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        raise SystemExit(130)
