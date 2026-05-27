#!/usr/bin/env python3
"""Compare normal-path SOCKS transfers across DNS tunnel implementations.

The harness starts one implementation at a time on the benchmark VPS DNS port
and drives a local SOCKS5 transfer through public recursive resolvers. It uses a
VPS-loopback HTTP payload service so download and upload test the same browser
shape: local app -> local tunnel listener -> DNS tunnel -> remote SOCKS -> HTTP.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import shlex
import signal
import socket
import subprocess
import tempfile
import textwrap
import time
from dataclasses import asdict, dataclass

from benchmark_public import (
    BENCH_SERVICE,
    DEFAULT_NATIVE_CERT_DIR,
    DEFAULT_SERVER_ENV,
    DEFAULT_SLIPSTREAM_DIR,
    REPO_ROOT,
    SINK_SERVICE,
    TRAJECTORY_SERVICE,
    TRAJECTORY_SOCKS_SERVICE,
    SshSession,
    choose_local_listen_port,
    ensure_native_certs,
    ensure_remote_service_active,
    ensure_slipstream_build,
    ensure_trajectory_build,
    generate_bench_access_key,
    load_server_auth,
    remote_is_active,
    run,
    start_output_reader,
    stop_services,
    wait_for_output_pattern,
    wait_for_remote_dns_port_idle,
)
from benchmark_public_go_tunnels import (
    GO_BENCH_SERVICES,
    GO_IMPLEMENTATIONS,
    cleanup_remote_benchmark as cleanup_go_remote,
    ensure_go_build,
    install_remote_files as install_go_remote_files,
    patch_local_listen_port,
    prepare_runtime_files as prepare_go_runtime_files,
)


TRANSFER_STAGE_DIR = "/opt/trajectory-transfer"
TRANSFER_PAYLOAD_SERVICE = "trajectory-transfer-payload.service"
TRANSFER_TRAJECTORY_SERVICE = "trajectory-transfer-trajectory.service"
TRANSFER_SLIPSTREAM_SERVICE = "trajectory-transfer-slipstream.service"
TRANSFER_PAYLOAD_PORT = 19081
REMOTE_SOCKS_TARGET = "127.0.0.1:1080"
DEFAULT_DOMAIN = "t.7-b.cc"
DEFAULT_SIZE_BYTES = 4 * 1024 * 1024
DEFAULT_RESOLVERS = ["1.1.1.1:53", "1.0.0.1:53", "8.8.8.8:53", "8.8.4.4:53"]

TRANSFER_PAYLOAD_SCRIPT = textwrap.dedent(
    """\
    #!/usr/bin/env python3
    import argparse
    import json
    import time
    from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

    parser = argparse.ArgumentParser()
    parser.add_argument("--bind", default="127.0.0.1")
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--size-bytes", type=int, required=True)
    args = parser.parse_args()

    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, fmt, *values):
            return

        def do_GET(self):
            if self.path not in ("/download", "/download.bin", "/payload"):
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header("Content-Type", "application/octet-stream")
            self.send_header("Content-Length", str(args.size_bytes))
            self.send_header("Connection", "close")
            self.end_headers()
            remaining = args.size_bytes
            chunk = b"x" * 65536
            while remaining:
                part = chunk[: min(len(chunk), remaining)]
                self.wfile.write(part)
                remaining -= len(part)

        def do_POST(self):
            self._receive_upload()

        def do_PUT(self):
            self._receive_upload()

        def _receive_upload(self):
            length = int(self.headers.get("Content-Length", "0") or "0")
            start = time.perf_counter()
            remaining = length
            total = 0
            digest = __import__("hashlib").sha256()
            while remaining:
                chunk = self.rfile.read(min(65536, remaining))
                if not chunk:
                    break
                digest.update(chunk)
                total += len(chunk)
                remaining -= len(chunk)
            payload = json.dumps({
                "bytes": total,
                "elapsed": time.perf_counter() - start,
                "complete": total == length,
                "sha256": digest.hexdigest(),
            }).encode("ascii")
            self.send_response(200 if total == length else 400)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(payload)

    ThreadingHTTPServer((args.bind, args.port), Handler).serve_forever()
    """
)


@dataclass
class TransferMeasurement:
    implementation: str
    direction: str
    success: bool
    http_code: int
    curl_exit: int
    bytes_downloaded: int
    bytes_uploaded: int
    elapsed_seconds: float
    speed_bytes_per_second: float
    checksum_ok: bool
    expected_sha256: str
    actual_sha256: str
    error: str


@dataclass
class ImplementationResult:
    implementation: str
    listen_port: int
    startup_seconds: float
    download: TransferMeasurement
    upload: TransferMeasurement
    client_log_tail: list[str]
    trajectory_diag: list[dict[str, object]]
    trajectory_diag_tail: list[dict[str, object]]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server-env", type=pathlib.Path, default=DEFAULT_SERVER_ENV)
    parser.add_argument("--domain", default=DEFAULT_DOMAIN)
    parser.add_argument("--size-bytes", type=int, default=DEFAULT_SIZE_BYTES)
    parser.add_argument("--timeout-seconds", type=int, default=420)
    parser.add_argument("--resolver", action="append", dest="resolvers")
    parser.add_argument("--resolver-file", type=pathlib.Path, default=None)
    parser.add_argument(
        "--implementation",
        action="append",
        choices=("trajectory", "slipstream", "masterdnsvpn", "stormdns"),
        help="Repeat to choose a subset. Defaults to all.",
    )
    parser.add_argument("--trajectory-dns-max-payload", type=int, default=None)
    parser.add_argument(
        "--trajectory-mode",
        choices=("secure", "velocity", "resilient", "frontier"),
        default=None,
    )
    parser.add_argument("--trajectory-resolver-socks-proxy", default=None)
    parser.add_argument(
        "--trajectory-resolver-transport",
        choices=("auto", "udp", "tcp"),
        default=None,
    )
    parser.add_argument("--trajectory-resolver-cohort-size", type=int, default=None)
    parser.add_argument("--trajectory-resolver-admission-min", type=int, default=None)
    parser.add_argument("--trajectory-admission-report", default=None)
    parser.add_argument("--slipstream-dir", type=pathlib.Path, default=DEFAULT_SLIPSTREAM_DIR)
    parser.add_argument("--native-cert-dir", type=pathlib.Path, default=DEFAULT_NATIVE_CERT_DIR)
    parser.add_argument("--report", type=pathlib.Path, default=None)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    resolvers = load_resolvers(args)
    implementations = args.implementation or [
        "trajectory",
        "slipstream",
        "masterdnsvpn",
        "stormdns",
    ]
    report = args.report or (
        REPO_ROOT
        / ".secrets"
        / "reports"
        / f"normal-transfer-4m-{time.strftime('%Y%m%dT%H%M%SZ', time.gmtime())}.jsonl"
    )
    report.parent.mkdir(parents=True, exist_ok=True)

    auth = load_server_auth(args.server_env)
    ssh = SshSession(auth)
    results: list[ImplementationResult] = []
    try:
        assets = prepare_assets(ssh, args, resolvers, implementations)
        install_transfer_payload(ssh, args.size_bytes)
        for implementation in implementations:
            result = run_implementation(
                ssh=ssh,
                implementation=implementation,
                assets=assets,
                domain=args.domain,
                resolvers=resolvers,
                size_bytes=args.size_bytes,
                timeout_seconds=args.timeout_seconds,
                trajectory_dns_max_payload=args.trajectory_dns_max_payload,
                trajectory_mode=args.trajectory_mode,
                trajectory_resolver_socks_proxy=args.trajectory_resolver_socks_proxy,
                trajectory_resolver_transport=args.trajectory_resolver_transport,
                trajectory_resolver_cohort_size=args.trajectory_resolver_cohort_size,
                trajectory_resolver_admission_min=args.trajectory_resolver_admission_min,
                trajectory_admission_report=args.trajectory_admission_report,
            )
            results.append(result)
            append_jsonl(report, {"event": "implementation_result", **result_to_json(result)})
            print_result(result)
        append_jsonl(
            report,
            {
                "event": "comparison_summary",
                "size_bytes": args.size_bytes,
                "resolvers": resolvers,
                "results": [result_to_json(result) for result in results],
            },
        )
        print(f"report={report}")
        return 0
    finally:
        restore_remote(ssh)
        ssh.close()


def load_resolvers(args: argparse.Namespace) -> list[str]:
    resolvers = list(args.resolvers or [])
    if args.resolver_file is not None:
        for raw in args.resolver_file.read_text(encoding="utf-8").splitlines():
            resolver = raw.split("#", 1)[0].strip()
            if not resolver:
                continue
            if ":" not in resolver:
                resolver = f"{resolver}:53"
            resolvers.append(resolver)
    return resolvers or DEFAULT_RESOLVERS


def prepare_assets(
    ssh: SshSession,
    args: argparse.Namespace,
    resolvers: list[str],
    implementations: list[str],
) -> dict[str, object]:
    trajectory_paths = ensure_trajectory_build(client_override=None, server_override=None)
    slipstream_paths = ensure_slipstream_build(args.slipstream_dir)
    native_cert_paths = ensure_native_certs(args.native_cert_dir)
    bench_access = generate_bench_access_key(ssh.temp_path)

    go_builds = []
    if "masterdnsvpn" in implementations:
        go_builds.append(ensure_go_build("masterdnsvpn"))
    if "stormdns" in implementations:
        go_builds.append(ensure_go_build("stormdns"))
    go_runtimes = []
    if go_builds:
        go_tmp = pathlib.Path(tempfile.mkdtemp(prefix="trajectory-transfer-go-"))
        go_runtimes = prepare_go_runtime_files(
            go_tmp,
            go_builds,
            args.domain,
            resolvers,
            "",
        )
        install_go_remote_files(
            ssh,
            go_tmp,
            go_builds,
            go_runtimes,
            size_bytes=1,
            runtime_max_seconds=args.timeout_seconds + 180,
        )

    install_transfer_servers(
        ssh,
        trajectory_paths,
        slipstream_paths,
        native_cert_paths,
        bench_access.registry_path,
        args.domain,
        args.timeout_seconds + 180,
    )
    return {
        "trajectory_paths": trajectory_paths,
        "slipstream_paths": slipstream_paths,
        "bench_access": bench_access,
        "go_runtimes": {runtime.implementation: runtime for runtime in go_runtimes},
    }


def install_transfer_payload(ssh: SshSession, size_bytes: int) -> None:
    payload_script = ssh.temp_path / "transfer_payload.py"
    payload_unit = ssh.temp_path / TRANSFER_PAYLOAD_SERVICE
    payload_script.write_text(TRANSFER_PAYLOAD_SCRIPT, encoding="utf-8")
    payload_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Trajectory transfer comparison payload service
            After=network-online.target
            Wants=network-online.target

            [Service]
            Type=simple
            ExecStart=/usr/bin/python3 {TRANSFER_STAGE_DIR}/transfer_payload.py --bind 127.0.0.1 --port {TRANSFER_PAYLOAD_PORT} --size-bytes {size_bytes}
            Restart=no
            RuntimeMaxSec=1800

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )
    ssh.remote(f"mkdir -p /tmp/trajectory-transfer-upload {TRANSFER_STAGE_DIR}", check=True)
    ssh.copy([payload_script, payload_unit], "/tmp/trajectory-transfer-upload/")
    ssh.remote(
        textwrap.dedent(
            f"""\
            install -m 755 /tmp/trajectory-transfer-upload/{payload_script.name} {TRANSFER_STAGE_DIR}/transfer_payload.py
            install -m 644 /tmp/trajectory-transfer-upload/{payload_unit.name} /etc/systemd/system/{TRANSFER_PAYLOAD_SERVICE}
            systemctl daemon-reload
            systemctl restart {TRAJECTORY_SOCKS_SERVICE}
            systemctl restart {TRANSFER_PAYLOAD_SERVICE}
            """
        ),
        check=True,
    )
    ensure_remote_service_active(ssh, TRANSFER_PAYLOAD_SERVICE, timeout_seconds=10)
    ensure_remote_service_active(ssh, TRAJECTORY_SOCKS_SERVICE, timeout_seconds=10)


def install_transfer_servers(
    ssh: SshSession,
    trajectory_paths: dict[str, pathlib.Path],
    slipstream_paths: dict[str, pathlib.Path],
    native_cert_paths: dict[str, pathlib.Path],
    registry_path: pathlib.Path,
    domain: str,
    runtime_max_seconds: int,
) -> None:
    trajectory_unit = ssh.temp_path / TRANSFER_TRAJECTORY_SERVICE
    slipstream_unit = ssh.temp_path / TRANSFER_SLIPSTREAM_SERVICE
    trajectory_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Trajectory transfer comparison server
            After=network-online.target {TRAJECTORY_SOCKS_SERVICE}
            Wants=network-online.target {TRAJECTORY_SOCKS_SERVICE}

            [Service]
            Type=simple
            WorkingDirectory={TRANSFER_STAGE_DIR}
            ExecStart={TRANSFER_STAGE_DIR}/trajectory-server --dns-listen-port 53 --target-address socks5-direct --domain {domain} --client-db {TRANSFER_STAGE_DIR}/trajectory-transfer-clients.json
            Restart=no
            RuntimeMaxSec={runtime_max_seconds}

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )
    slipstream_unit.write_text(
        textwrap.dedent(
            f"""\
            [Unit]
            Description=Slipstream transfer comparison server
            After=network-online.target {TRAJECTORY_SOCKS_SERVICE}
            Wants=network-online.target {TRAJECTORY_SOCKS_SERVICE}

            [Service]
            Type=simple
            WorkingDirectory={TRANSFER_STAGE_DIR}
            ExecStart={TRANSFER_STAGE_DIR}/slipstream-server --dns-listen-port=53 --target-address={REMOTE_SOCKS_TARGET} --domain {domain} --cert {TRANSFER_STAGE_DIR}/cert.pem --key {TRANSFER_STAGE_DIR}/key.pem
            Restart=no
            RuntimeMaxSec={runtime_max_seconds}

            [Install]
            WantedBy=multi-user.target
            """
        ),
        encoding="utf-8",
    )
    upload_paths = [
        trajectory_paths["client"],
        trajectory_paths["server"],
        slipstream_paths["client"],
        slipstream_paths["server"],
        slipstream_paths["cert"],
        slipstream_paths["key"],
        native_cert_paths["cert"],
        native_cert_paths["key"],
        registry_path,
        trajectory_unit,
        slipstream_unit,
    ]
    ssh.remote(f"mkdir -p /tmp/trajectory-transfer-upload {TRANSFER_STAGE_DIR}", check=True)
    ssh.copy(upload_paths, "/tmp/trajectory-transfer-upload/")
    ssh.remote(
        textwrap.dedent(
            f"""\
            install -m 755 /tmp/trajectory-transfer-upload/{trajectory_paths["client"].name} {TRANSFER_STAGE_DIR}/trajectory-client
            install -m 755 /tmp/trajectory-transfer-upload/{trajectory_paths["server"].name} {TRANSFER_STAGE_DIR}/trajectory-server
            install -m 755 /tmp/trajectory-transfer-upload/{slipstream_paths["client"].name} {TRANSFER_STAGE_DIR}/slipstream-client
            install -m 755 /tmp/trajectory-transfer-upload/{slipstream_paths["server"].name} {TRANSFER_STAGE_DIR}/slipstream-server
            install -m 644 /tmp/trajectory-transfer-upload/{slipstream_paths["cert"].name} {TRANSFER_STAGE_DIR}/cert.pem
            install -m 600 /tmp/trajectory-transfer-upload/{slipstream_paths["key"].name} {TRANSFER_STAGE_DIR}/key.pem
            install -m 644 /tmp/trajectory-transfer-upload/{native_cert_paths["cert"].name} {TRANSFER_STAGE_DIR}/native-cert.pem
            install -m 600 /tmp/trajectory-transfer-upload/{native_cert_paths["key"].name} {TRANSFER_STAGE_DIR}/native-key.pem
            install -m 600 /tmp/trajectory-transfer-upload/{registry_path.name} {TRANSFER_STAGE_DIR}/trajectory-transfer-clients.json
            install -m 644 /tmp/trajectory-transfer-upload/{trajectory_unit.name} /etc/systemd/system/{trajectory_unit.name}
            install -m 644 /tmp/trajectory-transfer-upload/{slipstream_unit.name} /etc/systemd/system/{slipstream_unit.name}
            systemctl daemon-reload
            """
        ),
        check=True,
    )


def run_implementation(
    *,
    ssh: SshSession,
    implementation: str,
    assets: dict[str, object],
    domain: str,
    resolvers: list[str],
    size_bytes: int,
    timeout_seconds: int,
    trajectory_dns_max_payload: int | None,
    trajectory_mode: str | None,
    trajectory_resolver_socks_proxy: str | None,
    trajectory_resolver_transport: str | None,
    trajectory_resolver_cohort_size: int | None,
    trajectory_resolver_admission_min: int | None,
    trajectory_admission_report: str | None,
) -> ImplementationResult:
    resolved_active = remote_is_active(ssh, "systemd-resolved")
    client: subprocess.Popen[str] | None = None
    client_output: list[str] = []
    listen_port = choose_local_listen_port(preferred_port(implementation))
    started_at = time.perf_counter()
    try:
        stop_remote_servers(ssh)
        if resolved_active:
            ssh.remote("systemctl stop systemd-resolved >/dev/null 2>&1 || true", check=False)
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
        ensure_remote_service_active(ssh, TRAJECTORY_SOCKS_SERVICE, timeout_seconds=10)
        ensure_remote_service_active(ssh, TRANSFER_PAYLOAD_SERVICE, timeout_seconds=10)

        client_cmd, cwd, service = client_command_and_service(
            implementation=implementation,
            listen_port=listen_port,
            domain=domain,
            resolvers=resolvers,
            assets=assets,
            trajectory_dns_max_payload=trajectory_dns_max_payload,
            trajectory_mode=trajectory_mode,
            trajectory_resolver_socks_proxy=trajectory_resolver_socks_proxy,
            trajectory_resolver_transport=trajectory_resolver_transport,
            trajectory_resolver_cohort_size=trajectory_resolver_cohort_size,
            trajectory_resolver_admission_min=trajectory_resolver_admission_min,
            trajectory_admission_report=trajectory_admission_report,
        )
        ssh.remote(f"systemctl start {service}", check=True)
        ensure_remote_service_active(ssh, service, timeout_seconds=10)

        env = os.environ.copy()
        if implementation == "trajectory":
            env["TRAJECTORY_DIAG"] = "1"
        client = subprocess.Popen(
            client_cmd,
            cwd=cwd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            env=env,
        )
        start_output_reader(client, client_output)
        wait_for_client_ready(implementation, client, client_output, timeout_seconds)
        startup_seconds = time.perf_counter() - started_at

        download = run_transfer(
            implementation=implementation,
            direction="download",
            listen_port=listen_port,
            size_bytes=size_bytes,
            timeout_seconds=timeout_seconds,
        )
        upload = run_transfer(
            implementation=implementation,
            direction="upload",
            listen_port=listen_port,
            size_bytes=size_bytes,
            timeout_seconds=timeout_seconds,
        )
        return ImplementationResult(
            implementation=implementation,
            listen_port=listen_port,
            startup_seconds=startup_seconds,
            download=download,
            upload=upload,
            client_log_tail=sanitize_log_tail(client_output),
            trajectory_diag=parse_trajectory_diag(client_output),
            trajectory_diag_tail=parse_trajectory_diag_tail(client_output),
        )
    finally:
        if client is not None and client.poll() is None:
            client.send_signal(signal.SIGTERM)
            try:
                client.wait(timeout=5)
            except subprocess.TimeoutExpired:
                client.kill()
        stop_remote_servers(ssh)
        try:
            wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
        except TimeoutError:
            pass
        if resolved_active:
            ssh.remote("systemctl start systemd-resolved >/dev/null 2>&1 || true", check=False)


def client_command_and_service(
    *,
    implementation: str,
    listen_port: int,
    domain: str,
    resolvers: list[str],
    assets: dict[str, object],
    trajectory_dns_max_payload: int | None,
    trajectory_mode: str | None,
    trajectory_resolver_socks_proxy: str | None,
    trajectory_resolver_transport: str | None,
    trajectory_resolver_cohort_size: int | None,
    trajectory_resolver_admission_min: int | None,
    trajectory_admission_report: str | None,
) -> tuple[list[str], pathlib.Path, str]:
    if implementation == "trajectory":
        trajectory_paths = assets["trajectory_paths"]
        bench_access = assets["bench_access"]
        assert isinstance(trajectory_paths, dict)
        cmd = [
            str(trajectory_paths["client"]),
            "--listen",
            "127.0.0.1:0",
            "--socks-listen",
            f"127.0.0.1:{listen_port}",
            "--domain",
            domain,
            "--access-key",
            bench_access.access_key,
        ]
        if trajectory_dns_max_payload is not None:
            cmd.extend(["--dns-max-payload", str(trajectory_dns_max_payload)])
        if trajectory_mode is not None:
            cmd.extend(["--mode", trajectory_mode])
        if trajectory_resolver_socks_proxy is not None:
            cmd.extend(["--resolver-socks-proxy", trajectory_resolver_socks_proxy])
        if trajectory_resolver_transport is not None:
            cmd.extend(["--resolver-transport", trajectory_resolver_transport])
        if trajectory_resolver_cohort_size is not None:
            cmd.extend(["--resolver-cohort-size", str(trajectory_resolver_cohort_size)])
        if trajectory_resolver_admission_min is not None:
            cmd.extend(["--resolver-admission-min", str(trajectory_resolver_admission_min)])
        if trajectory_admission_report is not None:
            cmd.extend(["--admission-report", trajectory_admission_report])
        for resolver in resolvers:
            cmd.extend(["--resolver", resolver])
        return cmd, REPO_ROOT, TRANSFER_TRAJECTORY_SERVICE

    if implementation == "slipstream":
        slipstream_paths = assets["slipstream_paths"]
        assert isinstance(slipstream_paths, dict)
        cmd = [
            str(slipstream_paths["client"]),
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
            cmd.extend(["--resolver", resolver])
        return cmd, slipstream_paths["client"].parent.parent, TRANSFER_SLIPSTREAM_SERVICE

    go_runtimes = assets["go_runtimes"]
    runtime = go_runtimes[implementation]
    local_config = patch_local_listen_port(runtime.client_config, listen_port)
    client_bin = (
        REPO_ROOT
        / "target"
        / "go-bench"
        / implementation
        / f"{implementation}-client"
    )
    cmd = [
        str(client_bin),
        "--config",
        str(local_config),
        "--resolvers",
        str(runtime.resolver_file),
    ]
    return cmd, GO_IMPLEMENTATIONS[implementation], runtime.service_name


def wait_for_client_ready(
    implementation: str,
    client: subprocess.Popen[str],
    output: list[str],
    timeout_seconds: int,
) -> None:
    if implementation == "trajectory":
        wait_for_output_pattern(
            output,
            "trajectory client listening",
            timeout_seconds=timeout_seconds,
            process=client,
        )
        return
    if implementation in GO_IMPLEMENTATIONS:
        wait_for_output_pattern(
            output,
            "SOCKS5 Proxy server is listening",
            timeout_seconds=timeout_seconds,
            process=client,
        )
        return
    time.sleep(2)
    if client.poll() is not None:
        raise RuntimeError("".join(output[-80:]) or "client exited before readiness")


def run_transfer(
    *,
    implementation: str,
    direction: str,
    listen_port: int,
    size_bytes: int,
    timeout_seconds: int,
) -> TransferMeasurement:
    with tempfile.NamedTemporaryFile(prefix="trajectory-upload-", suffix=".bin") as payload:
        if direction == "upload":
            write_payload(payload, size_bytes)
            payload.flush()
            os.fsync(payload.fileno())
            expected_sha256 = file_sha256(pathlib.Path(payload.name))
        else:
            expected_sha256 = repeated_byte_sha256(b"x", size_bytes)
        output = tempfile.NamedTemporaryFile(
            prefix=f"trajectory-{direction}-", suffix=".out", delete=False
        )
        output_path = pathlib.Path(output.name)
        output.close()
        url = f"http://127.0.0.1:{TRANSFER_PAYLOAD_PORT}/download"
        command = [
            "curl",
            "--http1.1",
            "--silent",
            "--show-error",
            "--fail-with-body",
            "--socks5-hostname",
            f"127.0.0.1:{listen_port}",
            "--noproxy",
            "",
            "--connect-timeout",
            "60",
            "--max-time",
            str(timeout_seconds),
            "--output",
            str(output_path),
            "--write-out",
            "%{http_code}\t%{size_download}\t%{size_upload}\t%{time_total}\t%{speed_download}\t%{speed_upload}",
        ]
        if direction == "upload":
            url = f"http://127.0.0.1:{TRANSFER_PAYLOAD_PORT}/upload"
            command.extend(["--header", "Content-Type: application/octet-stream"])
            command.extend(["--data-binary", f"@{payload.name}"])
        command.append(url)

        completed = subprocess.run(command, capture_output=True, text=True, check=False)
        if direction == "download":
            actual_sha256 = file_sha256(output_path) if output_path.exists() else ""
        else:
            actual_sha256 = upload_response_sha256(output_path)
        try:
            output_path.unlink()
        except FileNotFoundError:
            pass
    metrics = parse_curl_metrics(completed.stdout)
    http_code = int(metrics.get("http_code", 0))
    bytes_downloaded = int(metrics.get("size_download", 0))
    bytes_uploaded = int(metrics.get("size_upload", 0))
    elapsed_seconds = float(metrics.get("time_total", 0.0))
    speed_download = float(metrics.get("speed_download", 0.0))
    speed_upload = float(metrics.get("speed_upload", 0.0))
    speed = speed_download if direction == "download" else speed_upload
    expected_bytes = bytes_downloaded if direction == "download" else bytes_uploaded
    checksum_ok = expected_sha256 == actual_sha256
    success = (
        completed.returncode == 0
        and http_code == 200
        and expected_bytes == size_bytes
        and checksum_ok
    )
    errors = []
    if completed.returncode != 0:
        errors.append(f"curl_exit={completed.returncode}")
    if http_code != 200:
        errors.append(f"http_code={http_code}")
    if expected_bytes != size_bytes:
        errors.append(f"bytes={expected_bytes}, expected={size_bytes}")
    if not checksum_ok:
        errors.append(f"sha256={actual_sha256 or 'missing'}, expected={expected_sha256}")
    if completed.stderr.strip():
        errors.append(completed.stderr.strip().replace("\n", " | "))
    return TransferMeasurement(
        implementation=implementation,
        direction=direction,
        success=success,
        http_code=http_code,
        curl_exit=completed.returncode,
        bytes_downloaded=bytes_downloaded,
        bytes_uploaded=bytes_uploaded,
        elapsed_seconds=elapsed_seconds,
        speed_bytes_per_second=speed,
        checksum_ok=checksum_ok,
        expected_sha256=expected_sha256,
        actual_sha256=actual_sha256,
        error="; ".join(errors),
    )


def write_payload(handle, size_bytes: int) -> None:
    chunk = b"trajectory-transfer-upload\n" * 2048
    remaining = size_bytes
    while remaining:
        part = chunk[: min(len(chunk), remaining)]
        handle.write(part)
        remaining -= len(part)


def file_sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def repeated_byte_sha256(byte: bytes, size_bytes: int) -> str:
    digest = hashlib.sha256()
    chunk = byte * 65536
    remaining = size_bytes
    while remaining:
        part = chunk[: min(len(chunk), remaining)]
        digest.update(part)
        remaining -= len(part)
    return digest.hexdigest()


def upload_response_sha256(path: pathlib.Path) -> str:
    if not path.exists():
        return ""
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return ""
    return str(payload.get("sha256", ""))


def parse_curl_metrics(stdout: str) -> dict[str, float]:
    line = stdout.strip().splitlines()[-1] if stdout.strip() else ""
    parts = line.split("\t")
    fields = [
        "http_code",
        "size_download",
        "size_upload",
        "time_total",
        "speed_download",
        "speed_upload",
    ]
    if len(parts) != len(fields):
        return {}
    parsed: dict[str, float] = {}
    for key, value in zip(fields, parts):
        try:
            parsed[key] = float(value)
        except ValueError:
            parsed[key] = 0.0
    return parsed


def preferred_port(implementation: str) -> int:
    return {
        "trajectory": 27200,
        "slipstream": 27201,
        "masterdnsvpn": 27202,
        "stormdns": 27203,
    }[implementation]


def stop_remote_servers(ssh: SshSession) -> None:
    stop_services(
        ssh,
        [
            TRANSFER_TRAJECTORY_SERVICE,
            TRANSFER_SLIPSTREAM_SERVICE,
            *GO_BENCH_SERVICES,
            BENCH_SERVICE,
            SINK_SERVICE,
            TRAJECTORY_SERVICE,
        ],
    )


def restore_remote(ssh: SshSession) -> None:
    stop_remote_servers(ssh)
    try:
        cleanup_go_remote(ssh)
    except Exception:
        pass
    try:
        wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
    except TimeoutError:
        pass
    ssh.remote(
        textwrap.dedent(
            f"""\
            systemctl reset-failed {TRANSFER_TRAJECTORY_SERVICE} {TRANSFER_SLIPSTREAM_SERVICE} {TRANSFER_PAYLOAD_SERVICE} >/dev/null 2>&1 || true
            systemctl restart {TRAJECTORY_SOCKS_SERVICE} {TRANSFER_PAYLOAD_SERVICE} {TRAJECTORY_SERVICE}
            """
        ),
        check=False,
    )
    ensure_remote_service_active(ssh, TRAJECTORY_SERVICE, timeout_seconds=15)


def sanitize_log_tail(lines: list[str]) -> list[str]:
    sensitive = ("KEY", "SECRET", "TOKEN", "PASSWORD", "ENCRYPTION")
    output = []
    for line in lines[-120:]:
        upper = line.upper()
        if any(term in upper for term in sensitive):
            continue
        stripped = line.strip()
        if stripped:
            output.append(stripped)
    return output[-60:]


def parse_trajectory_diag(lines: list[str]) -> list[dict[str, object]]:
    diags = []
    for line in lines:
        stripped = line.strip()
        if "client_transport_diag" not in stripped:
            continue
        try:
            payload = json.loads(stripped)
        except json.JSONDecodeError:
            continue
        diags.append(payload)
    return diags


def parse_trajectory_diag_tail(lines: list[str]) -> list[dict[str, object]]:
    return parse_trajectory_diag(lines)[-8:]


def result_to_json(result: ImplementationResult) -> dict[str, object]:
    return {
        "implementation": result.implementation,
        "listen_port": result.listen_port,
        "startup_seconds": round(result.startup_seconds, 6),
        "download": asdict(result.download),
        "upload": asdict(result.upload),
        "client_log_tail": result.client_log_tail,
        "trajectory_diag": result.trajectory_diag,
        "trajectory_diag_tail": result.trajectory_diag_tail,
    }


def append_jsonl(path: pathlib.Path, payload: dict[str, object]) -> None:
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, sort_keys=True) + "\n")


def print_result(result: ImplementationResult) -> None:
    def fmt(measurement: TransferMeasurement) -> str:
        status = "ok" if measurement.success else "fail"
        return (
            f"{status} {measurement.elapsed_seconds:.3f}s "
            f"{measurement.speed_bytes_per_second:.1f} B/s"
        )

    print(
        json.dumps(
            {
                "implementation": result.implementation,
                "download": fmt(result.download),
                "upload": fmt(result.upload),
                "download_error": result.download.error,
                "upload_error": result.upload.error,
            },
            sort_keys=True,
        ),
        flush=True,
    )


if __name__ == "__main__":
    raise SystemExit(main())
