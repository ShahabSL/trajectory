#!/usr/bin/env python3
"""Run MasterDnsVPN or StormDNS directly with a local SOCKS listener.

This is intentionally not a benchmark harness. It prepares the matching remote
server, starts it on the VPS DNS port, then runs the selected local client in
the foreground so a browser or curl can use its SOCKS5 listener.
"""

from __future__ import annotations

import argparse
import pathlib
import signal
import subprocess
import sys
import tempfile
import textwrap

from benchmark_public import (
    DEFAULT_SERVER_ENV,
    TRAJECTORY_SERVICE,
    TRAJECTORY_SOCKS_SERVICE,
    ensure_remote_service_active,
    load_server_auth,
    remote_is_active,
    stop_services,
    wait_for_remote_dns_port_idle,
)
from benchmark_public_go_tunnels import (
    GO_BENCH_SERVICES,
    GO_IMPLEMENTATIONS,
    REMOTE_STAGE_DIR,
    cleanup_remote_benchmark,
    ensure_go_build,
    install_remote_files,
    load_resolvers,
    patch_local_listen_port,
    prepare_runtime_files,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--implementation",
        required=True,
        choices=sorted(GO_IMPLEMENTATIONS),
        help="Competitor to run: masterdnsvpn or stormdns.",
    )
    parser.add_argument("--server-env", type=pathlib.Path, default=DEFAULT_SERVER_ENV)
    parser.add_argument("--domain", default="t.7-b.cc")
    parser.add_argument("--listen-port", type=int, default=27110)
    parser.add_argument("--resolver", action="append", dest="resolvers")
    parser.add_argument(
        "--resolver-file",
        type=pathlib.Path,
        default=None,
        help="Resolver list. Defaults to three public resolvers.",
    )
    parser.add_argument(
        "--resolver-socks-proxy",
        default="",
        help="Optional SOCKS5 proxy for DNS-over-TCP resolvers, e.g. 127.0.0.1:11092.",
    )
    parser.add_argument(
        "--keep-remote-running",
        action="store_true",
        help="Do not restore the Trajectory server on exit.",
    )
    return parser.parse_args()


def default_resolvers() -> list[str]:
    return ["1.1.1.1:53", "1.0.0.1:53", "8.8.8.8:53"]


def main() -> int:
    args = parse_args()
    if args.resolvers:
        resolvers = args.resolvers
    elif args.resolver_file:
        resolvers = load_resolvers(None, args.resolver_file)
    else:
        resolvers = default_resolvers()

    auth = load_server_auth(args.server_env)
    build = ensure_go_build(args.implementation)
    client: subprocess.Popen[str] | None = None

    from benchmark_public import SshSession

    ssh = SshSession(auth)
    try:
        with tempfile.TemporaryDirectory(prefix=f"{args.implementation}-direct-") as tmp_name:
            tmp = pathlib.Path(tmp_name)
            runtimes = prepare_runtime_files(
                tmp,
                [build],
                args.domain,
                resolvers,
                args.resolver_socks_proxy,
            )
            runtime = runtimes[0]
            install_remote_files(
                ssh,
                tmp,
                [build],
                runtimes,
                size_bytes=1,
                runtime_max_seconds=24 * 60 * 60,
            )

            stop_services(ssh, [*GO_BENCH_SERVICES, TRAJECTORY_SERVICE])
            wait_for_remote_dns_port_idle(ssh, timeout_seconds=30)
            if remote_is_active(ssh, "systemd-resolved"):
                ssh.remote("systemctl stop systemd-resolved >/dev/null 2>&1 || true", check=False)
            ssh.remote(f"systemctl start {runtime.service_name}", check=True)
            ensure_remote_service_active(ssh, runtime.service_name, timeout_seconds=10)

            local_config = patch_local_listen_port(runtime.client_config, args.listen_port)
            client_cmd = [
                str(build.client),
                "--config",
                str(local_config),
                "--resolvers",
                str(runtime.resolver_file),
            ]

            print(
                textwrap.dedent(
                    f"""\
                    {args.implementation} server is running on the VPS from {REMOTE_STAGE_DIR}.
                    Local SOCKS target: 127.0.0.1:{args.listen_port}
                    Test with:
                      curl --socks5-hostname 127.0.0.1:{args.listen_port} -L https://www.google.com/?hl=en -o /tmp/{args.implementation}-google.html -w '%{{http_code}} %{{time_total}}\\n'

                    Starting client. Press Ctrl-C to stop.
                    """
                ),
                flush=True,
            )

            def stop_client(*_: object) -> None:
                if client and client.poll() is None:
                    client.terminate()

            signal.signal(signal.SIGTERM, stop_client)
            signal.signal(signal.SIGINT, stop_client)
            try:
                client = subprocess.Popen(client_cmd, cwd=build.repo_dir)
                return client.wait()
            finally:
                stop_client()
                if args.keep_remote_running:
                    print("Remote competitor server left running.", flush=True)
                else:
                    cleanup_remote_benchmark(ssh)
    finally:
        ssh.close()


if __name__ == "__main__":
    raise SystemExit(main())
