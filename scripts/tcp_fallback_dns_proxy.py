#!/usr/bin/env python3
"""Expose a local DNS endpoint that drops UDP and forwards TCP queries upstream."""

from __future__ import annotations

import argparse
import socket
import socketserver
import threading


def parse_addr(value: str) -> tuple[str, int]:
    host, port = value.rsplit(":", 1)
    return host, int(port)


def recv_exact(sock: socket.socket, length: int) -> bytes:
    chunks = bytearray()
    while len(chunks) < length:
        chunk = sock.recv(length - len(chunks))
        if not chunk:
            raise ConnectionError("unexpected EOF")
        chunks.extend(chunk)
    return bytes(chunks)


class TcpDnsForwardHandler(socketserver.BaseRequestHandler):
    upstream: tuple[str, int]
    timeout_seconds: float
    upstream_network: str

    def handle(self) -> None:
        self.request.settimeout(self.timeout_seconds)
        length = int.from_bytes(recv_exact(self.request, 2), "big")
        query = recv_exact(self.request, length)
        if self.upstream_network == "udp":
            with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as upstream:
                upstream.settimeout(self.timeout_seconds)
                upstream.sendto(query, self.upstream)
                response, _ = upstream.recvfrom(4096)
            response_len = len(response)
        else:
            with socket.create_connection(self.upstream, timeout=self.timeout_seconds) as upstream:
                upstream.sendall(length.to_bytes(2, "big") + query)
                response_len = int.from_bytes(recv_exact(upstream, 2), "big")
                response = recv_exact(upstream, response_len)
        self.request.sendall(response_len.to_bytes(2, "big") + response)


class ThreadedTcpServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    allow_reuse_address = True
    daemon_threads = True


def udp_blackhole(bind: tuple[str, int], stop_event: threading.Event) -> None:
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        sock.bind(bind)
        sock.settimeout(0.5)
        while not stop_event.is_set():
            try:
                sock.recvfrom(4096)
            except socket.timeout:
                continue


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--listen", default="127.0.0.1:15354")
    parser.add_argument("--upstream", default="1.1.1.1:53")
    parser.add_argument("--upstream-network", choices=("udp", "tcp"), default="udp")
    parser.add_argument("--timeout", type=float, default=3.0)
    args = parser.parse_args()

    listen = parse_addr(args.listen)
    upstream = parse_addr(args.upstream)
    stop_event = threading.Event()
    udp_thread = threading.Thread(target=udp_blackhole, args=(listen, stop_event), daemon=True)
    udp_thread.start()

    TcpDnsForwardHandler.upstream = upstream
    TcpDnsForwardHandler.timeout_seconds = args.timeout
    TcpDnsForwardHandler.upstream_network = args.upstream_network
    with ThreadedTcpServer(listen, TcpDnsForwardHandler) as server:
        try:
            server.serve_forever()
        finally:
            stop_event.set()
            udp_thread.join(timeout=1.0)


if __name__ == "__main__":
    main()
