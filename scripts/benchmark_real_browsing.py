#!/usr/bin/env python3
"""Benchmark real browsing through an already-running SOCKS proxy.

The script emits JSON Lines so runs can be piped into jq, tee, or later
comparison tooling without scraping terminal prose.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import shutil
import socket
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any
from urllib.parse import urlparse


DEFAULT_PLAYWRIGHT_DIR = pathlib.Path("/tmp/trajectory-pwbench")
DEFAULT_TIMEOUT_SECONDS = 30.0
USER_AGENT = (
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
)


@dataclass(frozen=True)
class Target:
    name: str
    url: str
    allowed_host_suffixes: tuple[str, ...]
    markers: tuple[str, ...]
    min_bytes: int


CURL_TARGETS = (
    Target(
        name="google",
        url="https://www.google.com/?hl=en",
        allowed_host_suffixes=(".google.com", "google.com"),
        markers=("google",),
        min_bytes=1024,
    ),
    Target(
        name="wikipedia",
        url="https://www.wikipedia.org/",
        allowed_host_suffixes=(".wikipedia.org", "wikipedia.org"),
        markers=("wikipedia", "free encyclopedia"),
        min_bytes=8192,
    ),
    Target(
        name="digikala",
        url="https://www.digikala.com/",
        allowed_host_suffixes=(".digikala.com", "digikala.com"),
        markers=("digikala", "dkstatics", "digikala.com"),
        min_bytes=8192,
    ),
)

CURL_TARGET_MAP = {target.name: target for target in CURL_TARGETS}
PLAYWRIGHT_TARGET = Target(
    name="google",
    url="https://www.google.com/?hl=en",
    allowed_host_suffixes=(".google.com", "google.com"),
    markers=("google",),
    min_bytes=1024,
)
PLAYWRIGHT_TARGETS = {target.name: target for target in CURL_TARGETS}
DEFAULT_PLAYWRIGHT_TARGETS = ("google",)


class BenchmarkError(RuntimeError):
    pass


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--socks",
        required=True,
        help="Already-running SOCKS proxy as host:port, for example 127.0.0.1:11092.",
    )
    parser.add_argument(
        "--timeout-seconds",
        type=float,
        default=DEFAULT_TIMEOUT_SECONDS,
        help=f"Per-request timeout. Defaults to {DEFAULT_TIMEOUT_SECONDS:.0f}s.",
    )
    parser.add_argument(
        "--connect-timeout-seconds",
        type=float,
        default=10.0,
        help="TCP/TLS connect timeout for curl and the initial proxy probe.",
    )
    parser.add_argument(
        "--repeat",
        type=int,
        default=1,
        help="Number of attempts per benchmark target. Defaults to 1.",
    )
    parser.add_argument(
        "--playwright-dir",
        type=pathlib.Path,
        default=DEFAULT_PLAYWRIGHT_DIR,
        help="Directory containing a Node Playwright install.",
    )
    parser.add_argument(
        "--no-playwright",
        action="store_true",
        help="Skip the optional headless Chromium/Chrome navigation benchmark.",
    )
    parser.add_argument(
        "--curl-target",
        action="append",
        choices=tuple(CURL_TARGET_MAP) + ("all",),
        help="Curl target to request. Repeatable; defaults to all.",
    )
    parser.add_argument(
        "--playwright-target",
        action="append",
        choices=tuple(PLAYWRIGHT_TARGETS) + ("all",),
        help=(
            "Browser target to navigate. Repeatable; defaults to google. "
            "Use 'all' for every built-in browser target."
        ),
    )
    parser.add_argument(
        "--accept-max-browser-request-failures",
        type=int,
        default=0,
        help="Maximum failed browser subrequests allowed per page. Defaults to 0.",
    )
    parser.add_argument(
        "--accept-max-response-start-ms",
        type=float,
        default=0.0,
        help="Optional browser responseStart SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--accept-max-load-event-ms",
        type=float,
        default=0.0,
        help="Optional browser loadEventEnd SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--accept-max-curl-total-ms",
        type=float,
        default=0.0,
        help="Optional curl total-time SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--trajectory-diag-jsonl",
        action="append",
        type=pathlib.Path,
        help=(
            "Read a client stderr/log file containing TRAJECTORY_DIAG JSON lines "
            "and include a runtime diagnostic summary in benchmark_end."
        ),
    )
    parser.add_argument(
        "--curl-bin",
        default="curl",
        help="curl executable to use. Defaults to curl from PATH.",
    )
    parser.add_argument(
        "--node-bin",
        default="node",
        help="node executable to use for Playwright. Defaults to node from PATH.",
    )
    parser.add_argument(
        "--insecure",
        action="store_true",
        help="Pass -k to curl and ignore HTTPS errors in Playwright.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.repeat < 1:
        raise BenchmarkError("--repeat must be >= 1")
    if args.timeout_seconds <= 0:
        raise BenchmarkError("--timeout-seconds must be > 0")
    if args.connect_timeout_seconds <= 0:
        raise BenchmarkError("--connect-timeout-seconds must be > 0")
    if args.accept_max_browser_request_failures < 0:
        raise BenchmarkError("--accept-max-browser-request-failures must be >= 0")
    if args.accept_max_response_start_ms < 0:
        raise BenchmarkError("--accept-max-response-start-ms must be >= 0")
    if args.accept_max_load_event_ms < 0:
        raise BenchmarkError("--accept-max-load-event-ms must be >= 0")
    if args.accept_max_curl_total_ms < 0:
        raise BenchmarkError("--accept-max-curl-total-ms must be >= 0")

    socks_host, socks_port = parse_host_port(args.socks)
    curl_bin = shutil.which(args.curl_bin)
    if not curl_bin:
        raise BenchmarkError(f"curl executable not found: {args.curl_bin}")
    acceptance = acceptance_config(args)
    curl_targets = select_targets(args.curl_target, CURL_TARGET_MAP, ("all",))
    playwright_targets = select_playwright_targets(args.playwright_target)

    emit(
        {
            "schema_version": 1,
            "event": "benchmark_start",
            "socks": f"{socks_host}:{socks_port}",
            "repeat": args.repeat,
            "timeout_seconds": args.timeout_seconds,
            "connect_timeout_seconds": args.connect_timeout_seconds,
            "playwright_dir": str(args.playwright_dir),
            "curl_targets": [target.name for target in curl_targets],
            "playwright_targets": [target.name for target in playwright_targets],
            "acceptance": acceptance,
            "diag_files": [str(path) for path in args.trajectory_diag_jsonl or ()],
            "started_at": now_iso8601(),
        }
    )

    proxy_ok = check_proxy_socks5(socks_host, socks_port, args.connect_timeout_seconds)
    emit(proxy_ok)
    if not proxy_ok["ok"]:
        return 2

    exit_code = 0
    results: list[dict[str, Any]] = []
    for attempt in range(1, args.repeat + 1):
        for target in curl_targets:
            result = run_curl_benchmark(
                curl_bin=curl_bin,
                socks=f"{socks_host}:{socks_port}",
                target=target,
                timeout_seconds=args.timeout_seconds,
                connect_timeout_seconds=args.connect_timeout_seconds,
                attempt=attempt,
                insecure=args.insecure,
                max_total_ms=args.accept_max_curl_total_ms,
            )
            results.append(result)
            emit(result)
            if not result["ok"]:
                exit_code = 1

    if not args.no_playwright:
        for attempt in range(1, args.repeat + 1):
            for target in playwright_targets:
                result = run_playwright_benchmark(
                    node_bin=args.node_bin,
                    playwright_dir=args.playwright_dir,
                    socks=f"{socks_host}:{socks_port}",
                    target=target,
                    timeout_seconds=args.timeout_seconds,
                    attempt=attempt,
                    insecure=args.insecure,
                    acceptance=acceptance,
                )
                results.append(result)
                emit(result)
                if result.get("status") != "skipped" and not result["ok"]:
                    exit_code = 1

    summary = summarize_results(results)
    diag_summary = parse_trajectory_diag_files(args.trajectory_diag_jsonl or [])
    if diag_summary is not None:
        summary["runtime_diag"] = diag_summary

    emit(
        {
            "schema_version": 1,
            "event": "benchmark_end",
            "ok": exit_code == 0,
            "summary": summary,
            "ended_at": now_iso8601(),
        }
    )
    return exit_code


def select_playwright_targets(names: list[str] | None) -> list[Target]:
    return select_targets(names, PLAYWRIGHT_TARGETS, DEFAULT_PLAYWRIGHT_TARGETS)


def select_targets(
    names: list[str] | None,
    target_map: dict[str, Target],
    default_names: tuple[str, ...],
) -> list[Target]:
    selected = names or list(default_names)
    if "all" in selected:
        return list(target_map.values())
    return [target_map[name] for name in selected]


def acceptance_config(args: argparse.Namespace) -> dict[str, Any]:
    return {
        "curl": {
            "max_total_ms": args.accept_max_curl_total_ms,
        },
        "browser": {
            "max_request_failures": args.accept_max_browser_request_failures,
            "max_response_start_ms": args.accept_max_response_start_ms,
            "max_load_event_ms": args.accept_max_load_event_ms,
        },
    }


def run_curl_benchmark(
    *,
    curl_bin: str,
    socks: str,
    target: Target,
    timeout_seconds: float,
    connect_timeout_seconds: float,
    attempt: int,
    insecure: bool,
    max_total_ms: float,
) -> dict[str, Any]:
    with tempfile.NamedTemporaryFile(prefix="trajectory-curl-body-", delete=True) as body:
        write_out = "\n".join(
            [
                "http_code=%{http_code}",
                "http_version=%{http_version}",
                "remote_ip=%{remote_ip}",
                "time_namelookup=%{time_namelookup}",
                "time_connect=%{time_connect}",
                "time_appconnect=%{time_appconnect}",
                "time_pretransfer=%{time_pretransfer}",
                "time_starttransfer=%{time_starttransfer}",
                "time_total=%{time_total}",
                "size_download=%{size_download}",
                "speed_download=%{speed_download}",
                "num_redirects=%{num_redirects}",
                "url_effective=%{url_effective}",
                "ssl_verify_result=%{ssl_verify_result}",
                "",
            ]
        )
        command = [
            curl_bin,
            "--silent",
            "--show-error",
            "--location",
            "--request",
            "GET",
            "--socks5-hostname",
            socks,
            "--connect-timeout",
            format_seconds(connect_timeout_seconds),
            "--max-time",
            format_seconds(timeout_seconds),
            "--compressed",
            "--user-agent",
            USER_AGENT,
            "--header",
            "Accept-Language: en-US,en;q=0.8",
            "--output",
            body.name,
            "--write-out",
            write_out,
            target.url,
        ]
        if insecure:
            command.insert(1, "--insecure")

        started = time.perf_counter()
        try:
            completed = subprocess.run(
                command,
                check=False,
                capture_output=True,
                text=True,
                timeout=timeout_seconds + 5,
            )
            wall_ms = elapsed_ms(started)
        except subprocess.TimeoutExpired as exc:
            return base_result(
                tool="curl",
                target=target,
                attempt=attempt,
                ok=False,
                status="timeout",
                wall_ms=elapsed_ms(started),
                error=f"curl subprocess timeout after {exc.timeout}s",
            )

        metrics = parse_curl_write_out(completed.stdout)
        body.seek(0)
        body_bytes = body.read()
        validation = validate_http_result(
            target=target,
            final_url=str(metrics.get("url_effective") or ""),
            status_code=to_int(metrics.get("http_code")),
            body=body_bytes,
        )

    ok = completed.returncode == 0 and validation["ok"]
    result = base_result(
        tool="curl",
        target=target,
        attempt=attempt,
        ok=ok,
        status="ok" if ok else "failed",
        wall_ms=wall_ms,
    )
    result.update(
        {
            "curl_returncode": completed.returncode,
            "stderr": trim(completed.stderr),
            "validation": validation,
            "timings_ms": curl_timings_ms(metrics),
            "http": {
                "status_code": to_int(metrics.get("http_code")),
                "http_version": metrics.get("http_version"),
                "remote_ip": empty_to_none(metrics.get("remote_ip")),
                "final_url": metrics.get("url_effective"),
                "ssl_verify_result": to_int(metrics.get("ssl_verify_result")),
                "redirects": to_int(metrics.get("num_redirects")),
                "size_download_bytes": to_float(metrics.get("size_download")),
                "speed_download_bytes_per_second": to_float(metrics.get("speed_download")),
            },
        }
    )
    result["acceptance"] = evaluate_curl_acceptance(result, max_total_ms)
    result["ok"] = bool(result["acceptance"]["ok"])
    result["status"] = "ok" if result["ok"] else "failed"
    if not ok and not result["stderr"]:
        result["error"] = validation["reason"]
    return result


def run_playwright_benchmark(
    *,
    node_bin: str,
    playwright_dir: pathlib.Path,
    socks: str,
    target: Target,
    timeout_seconds: float,
    attempt: int,
    insecure: bool,
    acceptance: dict[str, Any],
) -> dict[str, Any]:
    node_path = shutil.which(node_bin)
    if not node_path:
        return skipped_playwright(target, attempt, f"node executable not found: {node_bin}")

    package_json = playwright_dir / "node_modules" / "playwright" / "package.json"
    if not package_json.exists():
        return skipped_playwright(target, attempt, f"Playwright package not found at {package_json}")

    payload = {
        "socks": socks,
        "url": target.url,
        "allowedHostSuffixes": target.allowed_host_suffixes,
        "markers": target.markers,
        "minBytes": target.min_bytes,
        "timeoutMs": int(timeout_seconds * 1000),
        "userAgent": USER_AGENT,
        "ignoreHTTPSErrors": insecure,
        "acceptance": acceptance["browser"],
    }
    command = [node_path, "-e", PLAYWRIGHT_JS, json.dumps(payload, ensure_ascii=True)]
    started = time.perf_counter()
    try:
        completed = subprocess.run(
            command,
            cwd=playwright_dir,
            check=False,
            capture_output=True,
            text=True,
            timeout=timeout_seconds + 15,
        )
        wall_ms = elapsed_ms(started)
    except subprocess.TimeoutExpired as exc:
        return base_result(
            tool="playwright",
            target=target,
            attempt=attempt,
            ok=False,
            status="timeout",
            wall_ms=elapsed_ms(started),
            error=f"Playwright subprocess timeout after {exc.timeout}s",
        )

    parsed = parse_playwright_stdout(completed.stdout)
    if parsed is None:
        result = base_result(
            tool="playwright",
            target=target,
            attempt=attempt,
            ok=False,
            status="failed",
            wall_ms=wall_ms,
            error="Playwright did not emit parseable JSON",
        )
    else:
        result = base_result(
            tool="playwright",
            target=target,
            attempt=attempt,
            ok=completed.returncode == 0 and bool(parsed.get("ok")),
            status="ok" if completed.returncode == 0 and bool(parsed.get("ok")) else "failed",
            wall_ms=wall_ms,
        )
        result.update(parsed)

    result["playwright_returncode"] = completed.returncode
    result["stderr"] = trim(completed.stderr)
    if completed.returncode != 0 and not result.get("error"):
        failed_gates = [
            str(gate.get("name"))
            for gate in (result.get("acceptance") or {}).get("gates", [])
            if not gate.get("ok")
        ]
        if failed_gates:
            result["error"] = "acceptance_failed:" + ",".join(failed_gates)
        else:
            result["error"] = result["stderr"] or "Playwright process failed"
    return result


def check_proxy_socks5(host: str, port: int, timeout_seconds: float) -> dict[str, Any]:
    started = time.perf_counter()
    try:
        with socket.create_connection((host, port), timeout=timeout_seconds) as sock:
            sock.settimeout(timeout_seconds)
            sock.sendall(b"\x05\x01\x00")
            response = sock.recv(2)
    except OSError as exc:
        return {
            "schema_version": 1,
            "event": "proxy_check",
            "ok": False,
            "socks": f"{host}:{port}",
            "protocol": "socks5",
            "wall_ms": elapsed_ms(started),
            "error": str(exc),
        }
    if response != b"\x05\x00":
        return {
            "schema_version": 1,
            "event": "proxy_check",
            "ok": False,
            "socks": f"{host}:{port}",
            "protocol": "socks5",
            "wall_ms": elapsed_ms(started),
            "error": f"SOCKS5 no-auth handshake failed: {response.hex() or '<empty>'}",
        }
    return {
        "schema_version": 1,
        "event": "proxy_check",
        "ok": True,
        "socks": f"{host}:{port}",
        "protocol": "socks5",
        "wall_ms": elapsed_ms(started),
    }


def validate_http_result(
    *,
    target: Target,
    final_url: str,
    status_code: int | None,
    body: bytes,
) -> dict[str, Any]:
    final_host = (urlparse(final_url).hostname or "").lower()
    body_text = body[:2_000_000].decode("utf-8", errors="ignore").lower()
    reasons = []

    if status_code is None or not (200 <= status_code < 400):
        reasons.append(f"unexpected_status:{status_code}")
    if not host_matches(final_host, target.allowed_host_suffixes):
        reasons.append(f"unexpected_host:{final_host or '<empty>'}")
    if len(body) < target.min_bytes:
        reasons.append(f"body_too_small:{len(body)}<{target.min_bytes}")
    if not any(marker in body_text for marker in target.markers):
        reasons.append("missing_content_marker")

    return {
        "ok": not reasons,
        "reason": ",".join(reasons) if reasons else "ok",
        "final_host": final_host or None,
        "body_bytes": len(body),
        "matched_marker": next((marker for marker in target.markers if marker in body_text), None),
    }


def evaluate_curl_acceptance(result: dict[str, Any], max_total_ms: float) -> dict[str, Any]:
    validation = result.get("validation") or {}
    timings = result.get("timings_ms") or {}
    total_ms = timings.get("total_ms")
    gates = [
        acceptance_gate(
            "content_validation",
            bool(validation.get("ok")),
            validation.get("reason"),
            "ok",
        )
    ]
    if max_total_ms > 0:
        gates.append(
            acceptance_gate(
                "curl_total_ms",
                total_ms is not None and total_ms <= max_total_ms,
                total_ms,
                max_total_ms,
            )
        )
    return {
        "ok": all(gate["ok"] for gate in gates),
        "gates": gates,
    }


def acceptance_gate(
    name: str,
    ok: bool,
    value: object,
    threshold: object,
) -> dict[str, object]:
    return {
        "name": name,
        "ok": ok,
        "value": value,
        "threshold": threshold,
    }


def parse_curl_write_out(stdout: str) -> dict[str, str]:
    metrics: dict[str, str] = {}
    for line in stdout.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        metrics[key.strip()] = value.strip()
    return metrics


def curl_timings_ms(metrics: dict[str, str]) -> dict[str, float | None]:
    fields = (
        "time_namelookup",
        "time_connect",
        "time_appconnect",
        "time_pretransfer",
        "time_starttransfer",
        "time_total",
    )
    return {field.removeprefix("time_") + "_ms": seconds_text_to_ms(metrics.get(field)) for field in fields}


def parse_playwright_stdout(stdout: str) -> dict[str, Any] | None:
    for line in reversed(stdout.splitlines()):
        line = line.strip()
        if not line:
            continue
        try:
            parsed = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(parsed, dict):
            return parsed
    return None


def base_result(
    *,
    tool: str,
    target: Target,
    attempt: int,
    ok: bool,
    status: str,
    wall_ms: float,
    error: str | None = None,
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "schema_version": 1,
        "event": "measurement",
        "tool": tool,
        "target": target.name,
        "url": target.url,
        "attempt": attempt,
        "ok": ok,
        "status": status,
        "wall_ms": wall_ms,
        "ended_at": now_iso8601(),
    }
    if error:
        result["error"] = error
    return result


def skipped_playwright(target: Target, attempt: int, reason: str) -> dict[str, Any]:
    result = base_result(
        tool="playwright",
        target=target,
        attempt=attempt,
        ok=False,
        status="skipped",
        wall_ms=0.0,
    )
    result["reason"] = reason
    return result


def summarize_results(results: list[dict[str, Any]]) -> dict[str, Any]:
    summary: dict[str, Any] = {
        "measurements": len(results),
        "ok": sum(1 for result in results if result.get("ok") is True),
        "failed": sum(
            1
            for result in results
            if result.get("ok") is not True and result.get("status") != "skipped"
        ),
        "skipped": sum(1 for result in results if result.get("status") == "skipped"),
        "by_tool_target": {},
        "acceptance": {
            "ok": True,
            "failed_gates": {},
        },
        "failure_samples": [],
    }
    groups: dict[str, list[dict[str, Any]]] = {}
    failed_gates: dict[str, int] = {}

    for result in results:
        key = f"{result.get('tool')}:{result.get('target')}"
        groups.setdefault(key, []).append(result)
        if result.get("ok") is not True and result.get("status") != "skipped":
            if len(summary["failure_samples"]) < 12:
                summary["failure_samples"].append(failure_sample(result))
        for gate in (result.get("acceptance") or {}).get("gates", []):
            if not gate.get("ok"):
                name = str(gate.get("name"))
                failed_gates[name] = failed_gates.get(name, 0) + 1

    summary["acceptance"]["failed_gates"] = failed_gates
    summary["acceptance"]["ok"] = not failed_gates and summary["failed"] == 0
    for key, group in groups.items():
        summary["by_tool_target"][key] = summarize_group(group)
    return summary


def summarize_group(group: list[dict[str, Any]]) -> dict[str, Any]:
    wall_ms = [value for result in group if (value := to_float(result.get("wall_ms"))) is not None]
    curl_total = [
        value
        for result in group
        if (value := nested_float(result, "timings_ms", "total_ms")) is not None
    ]
    response_start = [
        value
        for result in group
        if (value := nested_float(result, "timings_ms", "response_start_ms")) is not None
    ]
    load_event = [
        value
        for result in group
        if (value := nested_float(result, "timings_ms", "load_event_ms")) is not None
    ]
    browser_failures = [
        value
        for result in group
        if (value := nested_int(result, "network", "failed_count")) is not None
    ]
    return {
        "total": len(group),
        "ok": sum(1 for result in group if result.get("ok") is True),
        "failed": sum(
            1
            for result in group
            if result.get("ok") is not True and result.get("status") != "skipped"
        ),
        "skipped": sum(1 for result in group if result.get("status") == "skipped"),
        "wall_ms": percentile_summary(wall_ms),
        "curl_total_ms": percentile_summary(curl_total),
        "response_start_ms": percentile_summary(response_start),
        "load_event_ms": percentile_summary(load_event),
        "browser_request_failures": percentile_summary(browser_failures),
    }


def failure_sample(result: dict[str, Any]) -> dict[str, Any]:
    sample = {
        "tool": result.get("tool"),
        "target": result.get("target"),
        "attempt": result.get("attempt"),
        "status": result.get("status"),
        "error": result.get("error") or (result.get("validation") or {}).get("reason"),
    }
    acceptance = result.get("acceptance") or {}
    failed = [gate for gate in acceptance.get("gates", []) if not gate.get("ok")]
    if failed:
        sample["failed_gates"] = failed[:5]
    network = result.get("network") or {}
    if network.get("failed_count"):
        sample["network_failed_count"] = network.get("failed_count")
        sample["network_failed"] = network.get("failed", [])[:3]
    return sample


def percentile_summary(values: list[float]) -> dict[str, float | int | None]:
    if not values:
        return {"count": 0, "min": None, "p50": None, "p95": None, "max": None}
    ordered = sorted(values)
    return {
        "count": len(ordered),
        "min": round(ordered[0], 3),
        "p50": round(percentile(ordered, 50), 3),
        "p95": round(percentile(ordered, 95), 3),
        "max": round(ordered[-1], 3),
    }


def percentile(ordered_values: list[float], percentile_value: float) -> float:
    if len(ordered_values) == 1:
        return ordered_values[0]
    rank = (len(ordered_values) - 1) * (percentile_value / 100.0)
    lower = int(rank)
    upper = min(lower + 1, len(ordered_values) - 1)
    weight = rank - lower
    return ordered_values[lower] * (1.0 - weight) + ordered_values[upper] * weight


def nested_float(result: dict[str, Any], section: str, key: str) -> float | None:
    nested = result.get(section)
    if not isinstance(nested, dict):
        return None
    return to_float(nested.get(key))


def nested_int(result: dict[str, Any], section: str, key: str) -> int | None:
    nested = result.get(section)
    if not isinstance(nested, dict):
        return None
    return to_int(nested.get(key))


def parse_trajectory_diag_files(paths: list[pathlib.Path]) -> dict[str, Any] | None:
    if not paths:
        return None

    snapshots = 0
    transport_snapshots = 0
    parse_errors = 0
    latest_by_stream: dict[str, dict[str, Any]] = {}
    latest_global: dict[str, Any] | None = None
    file_summaries = []

    for path in paths:
        file_snapshots = 0
        file_errors = 0
        try:
            lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        except OSError as exc:
            file_summaries.append({"path": str(path), "error": str(exc)})
            parse_errors += 1
            continue

        for line in lines:
            payload = parse_json_object_from_line(line)
            if not payload:
                continue
            kind = payload.get("kind")
            if kind == "client_transport_diag":
                snapshots += 1
                transport_snapshots += 1
                file_snapshots += 1
                if latest_global is None or diag_snapshot_score(payload) >= diag_snapshot_score(latest_global):
                    latest_global = payload
                continue
            if kind != "client_diag":
                continue
            snapshots += 1
            file_snapshots += 1
            stream_id = payload.get("stream_id")
            if stream_id is not None:
                stream_key = str(stream_id)
                previous = latest_by_stream.get(stream_key)
                if previous is None or (to_int(payload.get("elapsed_ms")) or 0) >= (
                    to_int(previous.get("elapsed_ms")) or 0
                ):
                    latest_by_stream[stream_key] = payload
            if latest_global is None or diag_snapshot_score(payload) >= diag_snapshot_score(latest_global):
                latest_global = payload

        file_summaries.append(
            {
                "path": str(path),
                "snapshots": file_snapshots,
                "parse_errors": file_errors,
            }
        )
        parse_errors += file_errors

    streams = sorted(
        latest_by_stream.values(),
        key=lambda item: (
            to_int(item.get("outstanding")) or 0,
            to_int(item.get("downlink_pending")) or 0,
            to_int(item.get("queries_failed")) or 0,
        ),
        reverse=True,
    )
    latest = compact_diag_snapshot(latest_global) if latest_global else None
    return {
        "files": file_summaries,
        "snapshots": snapshots,
        "transport_snapshots": transport_snapshots,
        "parse_errors": parse_errors,
        "stream_count": len(latest_by_stream),
        "latest_global": latest,
        "streams_with_backlog": [
            compact_diag_snapshot(stream)
            for stream in streams
            if (to_int(stream.get("outstanding")) or 0) > 0
            or (to_int(stream.get("downlink_pending")) or 0) > 0
        ][:12],
    }


def parse_json_object_from_line(line: str) -> dict[str, Any] | None:
    start = line.find("{")
    if start < 0:
        return None
    try:
        parsed = json.loads(line[start:])
    except json.JSONDecodeError:
        return None
    return parsed if isinstance(parsed, dict) else None


def diag_snapshot_score(snapshot: dict[str, Any]) -> int:
    return sum(
        to_int(snapshot.get(key)) or 0
        for key in ("queries_sent", "queries_ok", "queries_failed", "elapsed_ms")
    )


def compact_diag_snapshot(snapshot: dict[str, Any]) -> dict[str, Any]:
    queries_sent = to_int(snapshot.get("queries_sent")) or 0
    queries_failed = to_int(snapshot.get("queries_failed")) or 0
    response_wire_bytes = to_int(snapshot.get("response_wire_bytes")) or 0
    data_bytes_received = to_int(snapshot.get("data_bytes_received")) or 0
    useful_ratio = (
        round(data_bytes_received / response_wire_bytes, 6)
        if response_wire_bytes > 0
        else None
    )
    failure_rate = round(queries_failed / queries_sent, 6) if queries_sent > 0 else None
    keys = (
        "kind",
        "conn_id",
        "stream_id",
        "streams",
        "elapsed_ms",
        "outstanding",
        "downlink_next_offset",
        "downlink_pending",
        "downlink_first_pending",
        "queries_sent",
        "queries_ok",
        "queries_failed",
        "query_wire_bytes",
        "response_wire_bytes",
        "data_bytes_received",
        "data_frames_received",
        "open_packets_sent",
        "data_packets_sent",
        "ping_packets_sent",
        "qname_too_long_splits",
        "tcp_fallbacks",
    )
    compact = {key: snapshot.get(key) for key in keys if key in snapshot}
    compact["query_failure_rate"] = failure_rate
    compact["useful_response_byte_ratio"] = useful_ratio
    return compact


def parse_host_port(value: str) -> tuple[str, int]:
    if value.startswith("["):
        host, separator, rest = value[1:].partition("]")
        if separator != "]" or not rest.startswith(":"):
            raise BenchmarkError(f"invalid --socks address: {value}")
        port_text = rest[1:]
    else:
        host, separator, port_text = value.rpartition(":")
        if separator != ":":
            raise BenchmarkError(f"invalid --socks address: {value}")
    if not host:
        raise BenchmarkError(f"invalid --socks host: {value}")
    try:
        port = int(port_text)
    except ValueError as exc:
        raise BenchmarkError(f"invalid --socks port: {value}") from exc
    if not 1 <= port <= 65535:
        raise BenchmarkError(f"invalid --socks port: {value}")
    return host, port


def host_matches(host: str, suffixes: tuple[str, ...]) -> bool:
    return any(host == suffix.lstrip(".") or host.endswith(suffix) for suffix in suffixes)


def emit(payload: dict[str, Any]) -> None:
    print(json.dumps(payload, sort_keys=True, ensure_ascii=True), flush=True)


def now_iso8601() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def elapsed_ms(started: float) -> float:
    return round((time.perf_counter() - started) * 1000.0, 3)


def format_seconds(value: float) -> str:
    return f"{value:.3f}".rstrip("0").rstrip(".")


def seconds_text_to_ms(value: str | None) -> float | None:
    parsed = to_float(value)
    if parsed is None:
        return None
    return round(parsed * 1000.0, 3)


def to_float(value: object) -> float | None:
    if value is None or value == "":
        return None
    try:
        return float(value)
    except (TypeError, ValueError):
        return None


def to_int(value: object) -> int | None:
    if value is None or value == "":
        return None
    try:
        return int(float(str(value)))
    except ValueError:
        return None


def empty_to_none(value: object) -> object | None:
    return None if value == "" else value


def trim(value: str, limit: int = 2000) -> str:
    stripped = value.strip()
    if len(stripped) <= limit:
        return stripped
    return stripped[: limit - 3] + "..."


PLAYWRIGHT_JS = r"""
const payload = JSON.parse(process.argv[1]);
const { chromium } = require("playwright");

function nowMs(start) {
  return Number(process.hrtime.bigint() - start) / 1e6;
}

function hostMatches(host, suffixes) {
  return suffixes.some((suffix) => host === suffix.replace(/^\./, "") || host.endsWith(suffix));
}

function safeHost(url) {
  try {
    return new URL(url).hostname.toLowerCase();
  } catch {
    return "";
  }
}

function increment(map, key) {
  const safeKey = key || "<empty>";
  map[safeKey] = (map[safeKey] || 0) + 1;
}

function topObject(map, limit) {
  return Object.fromEntries(
    Object.entries(map)
      .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
      .slice(0, limit)
  );
}

function summarizeNetwork(networkStats, requestRecords, start) {
  if (!networkStats || !requestRecords) {
    return null;
  }
  const now = nowMs(start);
  const records = Array.from(requestRecords.values()).map((record) => {
    const endMs = record.end_ms === null ? now : record.end_ms;
    return {
      ...record,
      duration_ms: Number((endMs - record.start_ms).toFixed(3)),
    };
  });
  const pendingRecords = records.filter((record) => record.end_ms === null);
  const pending = pendingRecords
    .sort((a, b) => b.duration_ms - a.duration_ms)
    .slice(0, 20);
  const slowRequests = records
    .sort((a, b) => b.duration_ms - a.duration_ms)
    .slice(0, 20);
  return {
    requests: networkStats.requests,
    responses: networkStats.responses,
    finished: networkStats.finished,
    failed_count: networkStats.failedCount,
    pending_count: pendingRecords.length,
    by_type: networkStats.byType,
    by_host: topObject(networkStats.byHost, 20),
    statuses: networkStats.statuses,
    status_by_host: topObject(networkStats.statusByHost, 20),
    failed_by_error: networkStats.failedByError,
    failed_by_type: networkStats.failedByType,
    failed_by_host: topObject(networkStats.failedByHost, 20),
    failed: networkStats.failed,
    slow_requests: slowRequests,
    pending,
  };
}

function browserGate(name, ok, value, threshold) {
  return { name, ok, value, threshold };
}

function evaluateBrowserAcceptance(reasons, networkSummary, navTiming, acceptance) {
  const gates = [
    browserGate(
      "content_validation",
      reasons.length === 0,
      reasons.length === 0 ? "ok" : reasons.join(","),
      "ok"
    ),
  ];
  const failedCount = networkSummary ? networkSummary.failed_count : null;
  gates.push(
    browserGate(
      "browser_request_failures",
      failedCount !== null && failedCount <= acceptance.max_request_failures,
      failedCount,
      acceptance.max_request_failures
    )
  );
  if (acceptance.max_response_start_ms > 0) {
    const responseStart = navTiming ? Number(navTiming.responseStart.toFixed(3)) : null;
    gates.push(
      browserGate(
        "response_start_ms",
        responseStart !== null && responseStart <= acceptance.max_response_start_ms,
        responseStart,
        acceptance.max_response_start_ms
      )
    );
  }
  if (acceptance.max_load_event_ms > 0) {
    const loadEvent = navTiming ? Number(navTiming.loadEventEnd.toFixed(3)) : null;
    gates.push(
      browserGate(
        "load_event_ms",
        loadEvent !== null && loadEvent <= acceptance.max_load_event_ms,
        loadEvent,
        acceptance.max_load_event_ms
      )
    );
  }
  return {
    ok: gates.every((gate) => gate.ok),
    gates,
  };
}

async function main() {
  const totalStart = process.hrtime.bigint();
  let browser;
  let launchMs = null;
  let navigationWallMs = null;
  let networkStats = null;
  let requestRecords = null;
  try {
    const socksHost = payload.socks.replace(/:\d+$/, "");
    const launchStart = process.hrtime.bigint();
    browser = await chromium.launch({
      headless: true,
      proxy: { server: `socks5://${payload.socks}` },
      args: [
        "--disable-background-networking",
        "--disable-quic",
        `--host-resolver-rules=MAP * ~NOTFOUND, EXCLUDE ${socksHost}`,
      ],
      timeout: payload.timeoutMs,
    });
    launchMs = nowMs(launchStart);

    const context = await browser.newContext({
      ignoreHTTPSErrors: payload.ignoreHTTPSErrors,
      userAgent: payload.userAgent,
      viewport: { width: 1280, height: 720 },
      locale: "en-US",
      serviceWorkers: "block",
    });
    const page = await context.newPage();
    networkStats = {
      requests: 0,
      responses: 0,
      finished: 0,
      failedCount: 0,
      failed: [],
      byType: {},
      byHost: {},
      statuses: {},
      statusByHost: {},
      failedByError: {},
      failedByType: {},
      failedByHost: {},
    };
    requestRecords = new Map();
    page.on("request", (request) => {
      networkStats.requests += 1;
      const type = request.resourceType();
      const url = request.url();
      const host = safeHost(url);
      increment(networkStats.byType, type);
      increment(networkStats.byHost, host);
      requestRecords.set(request, {
        url: url.slice(0, 240),
        host,
        method: request.method(),
        type,
        start_ms: Number(nowMs(totalStart).toFixed(3)),
        response_ms: null,
        end_ms: null,
        status: null,
        failure: null,
        finished: false,
      });
    });
    page.on("response", (response) => {
      networkStats.responses += 1;
      const status = String(response.status());
      increment(networkStats.statuses, status);
      const request = response.request();
      const record = requestRecords.get(request);
      const host = record ? record.host : safeHost(response.url());
      increment(networkStats.statusByHost, `${host}:${status}`);
      if (record) {
        record.status = response.status();
        record.response_ms = Number(nowMs(totalStart).toFixed(3));
      }
    });
    page.on("requestfinished", (request) => {
      networkStats.finished += 1;
      const record = requestRecords.get(request);
      if (record) {
        record.end_ms = Number(nowMs(totalStart).toFixed(3));
        record.finished = true;
      }
    });
    page.on("requestfailed", (request) => {
      networkStats.failedCount += 1;
      const failure = request.failure()?.errorText || null;
      const type = request.resourceType();
      const host = safeHost(request.url());
      increment(networkStats.failedByError, failure || "<unknown>");
      increment(networkStats.failedByType, type);
      increment(networkStats.failedByHost, host);
      const record = requestRecords.get(request);
      if (record) {
        record.end_ms = Number(nowMs(totalStart).toFixed(3));
        record.failure = failure;
      }
      if (networkStats.failed.length < 20) {
        networkStats.failed.push({
          url: request.url().slice(0, 200),
          host,
          type,
          error: failure,
        });
      }
    });

    const navStart = process.hrtime.bigint();
    const response = await page.goto(payload.url, {
      waitUntil: "load",
      timeout: payload.timeoutMs,
    });
    navigationWallMs = nowMs(navStart);

    const title = await page.title();
    const finalUrl = page.url();
    const bodyText = await page.locator("body").innerText({ timeout: 5000 }).catch(() => "");
    const html = await page.content();
    const searchableText = `${title}\n${bodyText}\n${html.slice(0, 2000000)}`.toLowerCase();
    const finalHost = new URL(finalUrl).hostname.toLowerCase();
    const statusCode = response ? response.status() : null;
    const navTiming = await page.evaluate(() => {
      const entry = performance.getEntriesByType("navigation")[0];
      return entry ? entry.toJSON() : null;
    });
    const slowResources = await page.evaluate(() => {
      return performance
        .getEntriesByType("resource")
        .map((entry) => ({
          name: entry.name.slice(0, 200),
          initiator_type: entry.initiatorType,
          start_ms: Number(entry.startTime.toFixed(3)),
          duration_ms: Number(entry.duration.toFixed(3)),
          response_end_ms: Number(entry.responseEnd.toFixed(3)),
          transfer_size: entry.transferSize || 0,
          encoded_body_size: entry.encodedBodySize || 0,
        }))
        .sort((a, b) => b.duration_ms - a.duration_ms)
        .slice(0, 20);
    });

    const reasons = [];
    if (statusCode === null || statusCode < 200 || statusCode >= 400) {
      reasons.push(`unexpected_status:${statusCode}`);
    }
    if (!hostMatches(finalHost, payload.allowedHostSuffixes)) {
      reasons.push(`unexpected_host:${finalHost || "<empty>"}`);
    }
    if (html.length < payload.minBytes) {
      reasons.push(`body_too_small:${html.length}<${payload.minBytes}`);
    }
    const matchedMarker = payload.markers.find((marker) => searchableText.includes(marker)) || null;
    if (!matchedMarker) {
      reasons.push("missing_content_marker");
    }
    const networkSummary = summarizeNetwork(networkStats, requestRecords, totalStart);
    const acceptance = evaluateBrowserAcceptance(
      reasons,
      networkSummary,
      navTiming,
      payload.acceptance
    );

    await context.close();
    await browser.close();
    browser = null;

    console.log(JSON.stringify({
      ok: acceptance.ok,
      status: acceptance.ok ? "ok" : "failed",
      browser: "chromium",
      timings_ms: {
        launch_ms: Number(launchMs.toFixed(3)),
        navigation_wall_ms: Number(navigationWallMs.toFixed(3)),
        total_ms: Number(nowMs(totalStart).toFixed(3)),
        response_start_ms: navTiming ? Number(navTiming.responseStart.toFixed(3)) : null,
        dom_content_loaded_ms: navTiming ? Number(navTiming.domContentLoadedEventEnd.toFixed(3)) : null,
        load_event_ms: navTiming ? Number(navTiming.loadEventEnd.toFixed(3)) : null,
      },
      http: {
        status_code: statusCode,
        final_url: finalUrl,
        final_host: finalHost,
      },
      validation: {
        ok: reasons.length === 0,
        reason: reasons.length === 0 ? "ok" : reasons.join(","),
        html_chars: html.length,
        body_text_chars: bodyText.length,
        title,
        matched_marker: matchedMarker,
      },
      acceptance,
      network: networkSummary
        ? {
            ...networkSummary,
            slow_resources: slowResources,
          }
        : { slow_resources: slowResources },
    }));
    process.exit(acceptance.ok ? 0 : 1);
  } catch (error) {
    if (browser) {
      await browser.close().catch(() => {});
    }
    console.log(JSON.stringify({
      ok: false,
      status: "failed",
      browser: "chromium",
      timings_ms: {
        launch_ms: launchMs === null ? null : Number(launchMs.toFixed(3)),
        navigation_wall_ms: navigationWallMs === null ? null : Number(navigationWallMs.toFixed(3)),
        total_ms: Number(nowMs(totalStart).toFixed(3)),
      },
      network: summarizeNetwork(networkStats, requestRecords, totalStart),
      error: error && error.message ? error.message : String(error),
    }));
    process.exit(1);
  }
}

main();
"""


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BenchmarkError as exc:
        print(f"error: {exc}", file=sys.stderr)
        raise SystemExit(2)
