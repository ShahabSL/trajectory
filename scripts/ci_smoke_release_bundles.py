#!/usr/bin/env python3
"""Smoke-test packaged CLI release bundles by extracting and running them."""

from __future__ import annotations

import argparse
import os
import re
import socket
import subprocess
import tarfile
import tempfile
import threading
import time
import zipfile
from contextlib import closing
from pathlib import Path


EXPECTED_BINARIES = ("trajectory-client", "trajectory-admin", "trajectory-server")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_dir", help="Directory containing packaged release archives")
    return parser.parse_args()


def archive_paths(artifact_dir: Path) -> list[Path]:
    return sorted(
        [
            *artifact_dir.glob("trajectory-v*-cli.tar.gz"),
            *artifact_dir.glob("trajectory-v*-cli.zip"),
        ]
    )


def extract_archive(archive: Path, destination: Path) -> None:
    if archive.suffix == ".zip":
        with zipfile.ZipFile(archive) as bundle:
            bundle.extractall(destination)
        return

    if archive.name.endswith(".tar.gz"):
        with tarfile.open(archive, "r:gz") as bundle:
            try:
                bundle.extractall(destination, filter="data")
            except TypeError:
                bundle.extractall(destination)
        return

    raise SystemExit(f"unsupported archive format: {archive}")


def find_binary(extract_dir: Path, name: str) -> Path:
    candidates = [*extract_dir.rglob(name), *extract_dir.rglob(f"{name}.exe")]
    if len(candidates) != 1:
        raise SystemExit(f"expected one {name} binary in {extract_dir}, found {candidates}")
    return candidates[0]


def smoke_help(binary: Path) -> None:
    result = subprocess.run(
        [str(binary), "--help"],
        check=True,
        capture_output=True,
        text=True,
    )
    if "Usage:" not in result.stdout:
        raise SystemExit(f"{binary} --help did not print usage text")


def free_port() -> int:
    for _ in range(100):
        with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as tcp:
            tcp.bind(("127.0.0.1", 0))
            port = tcp.getsockname()[1]
        with closing(socket.socket(socket.AF_INET, socket.SOCK_DGRAM)) as udp:
            try:
                udp.bind(("127.0.0.1", port))
            except OSError:
                continue
        return port
    raise SystemExit("could not allocate local smoke-test port")


def wait_for_tcp(addr: tuple[str, int], deadline: float) -> None:
    while time.monotonic() < deadline:
        try:
            with socket.create_connection(addr, timeout=0.2):
                return
        except OSError:
            time.sleep(0.05)
    raise RuntimeError(f"timed out waiting for TCP listener {addr[0]}:{addr[1]}")


def start_echo_server(addr: tuple[str, int]) -> tuple[threading.Event, threading.Thread]:
    stop = threading.Event()

    def serve() -> None:
        with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as listener:
            listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            listener.bind(addr)
            listener.listen()
            listener.settimeout(0.2)
            while not stop.is_set():
                try:
                    conn, _ = listener.accept()
                except socket.timeout:
                    continue
                with conn:
                    conn.settimeout(5)
                    while True:
                        data = conn.recv(4096)
                        if not data:
                            break
                        conn.sendall(data)

    thread = threading.Thread(target=serve, daemon=True)
    thread.start()
    return stop, thread


def create_access_key(admin: Path, client_db: Path) -> str:
    result = subprocess.run(
        [
            str(admin),
            "create-client",
            "--client-db",
            str(client_db),
            "--label",
            "bundle-smoke",
        ],
        check=True,
        capture_output=True,
        text=True,
    )
    match = re.search(r"^access_key=(.+)$", result.stdout, re.MULTILINE)
    if not match:
        raise RuntimeError(f"could not parse access key from trajectory-admin output:\n{result.stdout}")
    return match.group(1).strip()


def terminate(process: subprocess.Popen[str] | None) -> None:
    if process is None or process.poll() is not None:
        return
    process.terminate()
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait(timeout=5)


def process_output(process: subprocess.Popen[str]) -> str:
    stdout, stderr = process.communicate(timeout=1) if process.poll() is not None else ("", "")
    return f"stdout:\n{stdout}\nstderr:\n{stderr}\n"


def smoke_loopback(binaries: dict[str, Path], work_dir: Path) -> None:
    client_db = work_dir / "clients.json"
    access_key = create_access_key(binaries["trajectory-admin"], client_db)
    domain = "t.bundle-smoke"
    dns_port = free_port()
    client_port = free_port()
    target_port = free_port()
    echo_stop, echo_thread = start_echo_server(("127.0.0.1", target_port))
    server = None
    client = None
    try:
        server = subprocess.Popen(
            [
                str(binaries["trajectory-server"]),
                "--domain",
                domain,
                "--client-db",
                str(client_db),
                "--bind",
                "127.0.0.1",
                "--dns-listen-port",
                str(dns_port),
                "--target-address",
                f"127.0.0.1:{target_port}",
            ],
            cwd=work_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        deadline = time.monotonic() + 20
        client = subprocess.Popen(
            [
                str(binaries["trajectory-client"]),
                "--listen",
                f"127.0.0.1:{client_port}",
                "--domain",
                domain,
                "--access-key",
                access_key,
                "--resolver",
                f"127.0.0.1:{dns_port}",
                "--resolver-transport",
                "udp",
                "--mode",
                "velocity",
                "--resolver-admission-min",
                "1",
            ],
            cwd=work_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env={**os.environ, "RUST_BACKTRACE": "1"},
        )
        wait_for_tcp(("127.0.0.1", client_port), deadline)
        payload = b"trajectory packaged loopback smoke\n"
        with socket.create_connection(("127.0.0.1", client_port), timeout=10) as stream:
            stream.settimeout(10)
            stream.sendall(payload)
            received = stream.recv(len(payload))
        if received != payload:
            raise RuntimeError(f"loopback echo mismatch: {received!r}")
    except Exception as error:
        details = []
        if server:
            details.append("server " + process_output(server))
        if client:
            details.append("client " + process_output(client))
        raise RuntimeError(f"packaged loopback smoke failed: {error}\n" + "\n".join(details))
    finally:
        terminate(client)
        terminate(server)
        echo_stop.set()
        echo_thread.join(timeout=1)


def main() -> None:
    artifact_dir = Path(parse_args().artifact_dir).resolve()
    archives = archive_paths(artifact_dir)
    if not archives:
        raise SystemExit(f"no CLI release archives found in {artifact_dir}")

    for archive in archives:
        with tempfile.TemporaryDirectory(prefix=f"{archive.stem}-") as temp_dir:
            extract_dir = Path(temp_dir)
            extract_archive(archive, extract_dir)
            binaries = {
                binary_name: find_binary(extract_dir, binary_name)
                for binary_name in EXPECTED_BINARIES
            }
            for binary_name in EXPECTED_BINARIES:
                smoke_help(binaries[binary_name])
            smoke_loopback(binaries, extract_dir)
            print(f"smoked {archive.name}")


if __name__ == "__main__":
    main()
