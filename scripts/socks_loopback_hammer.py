#!/usr/bin/env python3
"""Hammer an already-running SOCKS proxy against VPS loopback HTTP endpoints.

The expected setup is:

* a local SOCKS5 listener, usually a Trajectory client, on 127.0.0.1:11092
* a VPS-local HTTP payload service reachable from the remote SOCKS upstream

This script intentionally does not start or reconfigure either service. It is a
thin harness around curl so transfer semantics match the operator smoke tests.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import pathlib
import shutil
import socket
import subprocess
import sys
import tempfile
import time
from dataclasses import asdict, dataclass


DEFAULT_SOCKS = "127.0.0.1:11092"
DEFAULT_DOWNLOAD_URL = "http://127.0.0.1:18081/trajectory-1m.bin"
DEFAULT_UPLOAD_URL = "http://127.0.0.1:18082/upload"
DEFAULT_DOWNLOAD_BYTES = 1_048_576
WRITE_OUT_FIELDS = (
    "http_code",
    "size_download",
    "size_upload",
    "time_total",
    "speed_download",
    "speed_upload",
    "time_connect",
    "time_starttransfer",
)
REQUIRED_CURL_FLAGS = (
    "--connect-timeout",
    "--data-binary",
    "--fail-with-body",
    "--header",
    "--http1.1",
    "--max-time",
    "--noproxy",
    "--output",
    "--socks5-hostname",
    "--upload-file",
    "--write-out",
)


@dataclass
class HammerResult:
    phase: str
    run: int
    worker: int
    kind: str
    url: str
    success: bool
    slo_pass: bool
    http_code: int
    curl_exit: int
    bytes_downloaded: int
    bytes_uploaded: int
    elapsed_seconds: float
    wall_seconds: float
    bytes_per_second: float
    error: str

    @property
    def ok(self) -> bool:
        return self.success and self.slo_pass


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--socks", default=DEFAULT_SOCKS, help="SOCKS5 host:port to hammer.")
    parser.add_argument("--download-url", default=DEFAULT_DOWNLOAD_URL)
    parser.add_argument("--upload-url", default=DEFAULT_UPLOAD_URL)
    parser.add_argument(
        "--phase",
        action="append",
        choices=["download", "upload", "concurrent"],
        help="Phase to run. Repeatable; defaults to all phases.",
    )
    parser.add_argument("--runs", type=positive_int, default=3, help="Sequential runs per phase.")
    parser.add_argument(
        "--concurrency",
        type=positive_int,
        default=4,
        help="Workers per concurrent batch.",
    )
    parser.add_argument(
        "--concurrent-batches",
        type=positive_int,
        default=1,
        help="Number of concurrent batches to run.",
    )
    parser.add_argument(
        "--concurrent-kind",
        choices=["download", "upload", "mixed"],
        default="download",
        help="Workload used by the concurrent phase.",
    )
    parser.add_argument("--upload-bytes", type=positive_int, default=DEFAULT_DOWNLOAD_BYTES)
    parser.add_argument(
        "--upload-method",
        choices=["post", "put"],
        default="post",
        help="POST uses curl --data-binary; PUT uses curl --upload-file.",
    )
    parser.add_argument(
        "--expect-download-bytes",
        type=nonnegative_int,
        default=DEFAULT_DOWNLOAD_BYTES,
        help="Expected download size. Use 0 to disable the byte-count assertion.",
    )
    parser.add_argument(
        "--download-http-codes",
        default="200",
        help="Comma-separated acceptable HTTP codes for downloads.",
    )
    parser.add_argument(
        "--upload-http-codes",
        default="200,201,204",
        help="Comma-separated acceptable HTTP codes for uploads.",
    )
    parser.add_argument("--timeout-seconds", type=positive_float, default=120.0)
    parser.add_argument("--connect-timeout-seconds", type=positive_float, default=10.0)
    parser.add_argument(
        "--max-elapsed-seconds",
        type=nonnegative_float,
        default=0.0,
        help="Optional elapsed-time SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--min-download-bps",
        type=nonnegative_float,
        default=0.0,
        help="Minimum per-run download throughput SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--min-upload-bps",
        type=nonnegative_float,
        default=0.0,
        help="Minimum per-run upload throughput SLO. Use 0 to disable.",
    )
    parser.add_argument(
        "--delay-seconds",
        type=nonnegative_float,
        default=0.0,
        help="Delay between sequential runs and concurrent batches.",
    )
    parser.add_argument(
        "--jsonl",
        action="store_true",
        help="Print each result as JSON instead of human-readable lines.",
    )
    parser.add_argument(
        "--skip-proxy-check",
        action="store_true",
        help="Skip the local TCP connect check against the SOCKS listener.",
    )
    parser.add_argument("--curl-bin", default="curl")
    return parser.parse_args()


def positive_int(value: str) -> int:
    parsed = int(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("must be greater than zero")
    return parsed


def nonnegative_int(value: str) -> int:
    parsed = int(value)
    if parsed < 0:
        raise argparse.ArgumentTypeError("must be greater than or equal to zero")
    return parsed


def positive_float(value: str) -> float:
    parsed = float(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("must be greater than zero")
    return parsed


def nonnegative_float(value: str) -> float:
    parsed = float(value)
    if parsed < 0:
        raise argparse.ArgumentTypeError("must be greater than or equal to zero")
    return parsed


def parse_http_codes(value: str) -> set[int]:
    codes = set()
    for part in value.split(","):
        stripped = part.strip()
        if not stripped:
            continue
        code = int(stripped)
        if code < 100 or code > 599:
            raise argparse.ArgumentTypeError(f"invalid HTTP status code: {code}")
        codes.add(code)
    if not codes:
        raise argparse.ArgumentTypeError("at least one HTTP status code is required")
    return codes


def split_host_port(value: str) -> tuple[str, int]:
    if value.startswith("["):
        host, separator, port_text = value[1:].partition("]:")
        if not separator:
            raise ValueError(f"expected [host]:port, got {value!r}")
        return host, int(port_text)

    host, separator, port_text = value.rpartition(":")
    if not separator:
        raise ValueError(f"expected host:port, got {value!r}")
    return host, int(port_text)


def check_curl(curl_bin: str) -> str:
    resolved = shutil.which(curl_bin)
    if resolved is None:
        raise RuntimeError(f"curl binary not found: {curl_bin}")

    completed = subprocess.run(
        [resolved, "--help", "all"],
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"failed to inspect curl help: {completed.stderr.strip()}")

    missing = [flag for flag in REQUIRED_CURL_FLAGS if flag not in completed.stdout]
    if missing:
        raise RuntimeError(f"curl is missing required flags: {', '.join(missing)}")
    return resolved


def check_proxy(socks: str, timeout_seconds: float) -> None:
    host, port = split_host_port(socks)
    with socket.create_connection((host, port), timeout=timeout_seconds):
        return


def make_upload_payload(size_bytes: int):
    payload = tempfile.NamedTemporaryFile(prefix="trajectory-upload-", suffix=".bin")
    chunk = b"trajectory-hammer\n" * 4096
    remaining = size_bytes
    while remaining > 0:
        part = chunk[: min(len(chunk), remaining)]
        payload.write(part)
        remaining -= len(part)
    payload.flush()
    os.fsync(payload.fileno())
    return payload


def curl_write_out_format() -> str:
    return "\t".join(f"%{{{field}}}" for field in WRITE_OUT_FIELDS)


def build_curl_command(
    *,
    curl_bin: str,
    socks: str,
    timeout_seconds: float,
    connect_timeout_seconds: float,
    kind: str,
    url: str,
    upload_method: str,
    upload_payload_path: pathlib.Path | None,
) -> list[str]:
    command = [
        curl_bin,
        "--http1.1",
        "--silent",
        "--show-error",
        "--fail-with-body",
        "--socks5-hostname",
        socks,
        # Loopback targets often match NO_PROXY; this keeps them on the tunnel path.
        "--noproxy",
        "",
        "--connect-timeout",
        str(connect_timeout_seconds),
        "--max-time",
        str(timeout_seconds),
        "--output",
        os.devnull,
        "--write-out",
        curl_write_out_format(),
    ]

    if kind == "upload":
        if upload_payload_path is None:
            raise RuntimeError("upload payload path is required for upload tests")
        command.extend(["--header", "Content-Type: application/octet-stream"])
        if upload_method == "post":
            command.extend(["--data-binary", f"@{upload_payload_path}"])
        elif upload_method == "put":
            command.extend(["--upload-file", str(upload_payload_path)])
        else:
            raise RuntimeError(f"unsupported upload method: {upload_method}")

    command.append(url)
    return command


def run_curl(
    *,
    curl_bin: str,
    socks: str,
    timeout_seconds: float,
    connect_timeout_seconds: float,
    phase: str,
    run_index: int,
    worker_index: int,
    kind: str,
    url: str,
    upload_method: str,
    upload_payload_path: pathlib.Path | None,
    expected_download_bytes: int,
    upload_bytes: int,
    expected_http_codes: set[int],
    min_download_bps: float,
    min_upload_bps: float,
    max_elapsed_seconds: float,
) -> HammerResult:
    command = build_curl_command(
        curl_bin=curl_bin,
        socks=socks,
        timeout_seconds=timeout_seconds,
        connect_timeout_seconds=connect_timeout_seconds,
        kind=kind,
        url=url,
        upload_method=upload_method,
        upload_payload_path=upload_payload_path,
    )

    start = time.perf_counter()
    completed = subprocess.run(command, check=False, capture_output=True, text=True)
    wall_seconds = time.perf_counter() - start

    metrics = parse_curl_metrics(completed.stdout)
    http_code = int(metrics.get("http_code", 0))
    bytes_downloaded = int(metrics.get("size_download", 0))
    bytes_uploaded = int(metrics.get("size_upload", 0))
    elapsed_seconds = float(metrics.get("time_total", wall_seconds))
    speed_download = float(metrics.get("speed_download", 0.0))
    speed_upload = float(metrics.get("speed_upload", 0.0))

    if kind == "download":
        bytes_per_second = speed_download
    elif kind == "upload":
        bytes_per_second = speed_upload
    else:
        bytes_per_second = 0.0

    errors = []
    success = True
    slo_pass = True

    if completed.returncode != 0:
        success = False
        errors.append(f"curl_exit={completed.returncode}")
    if http_code not in expected_http_codes:
        success = False
        errors.append(f"http_code={http_code}")
    if (
        kind == "download"
        and expected_download_bytes
        and bytes_downloaded != expected_download_bytes
    ):
        success = False
        errors.append(f"bytes_downloaded={bytes_downloaded}, expected={expected_download_bytes}")
    if kind == "upload" and bytes_uploaded != upload_bytes:
        success = False
        errors.append(f"bytes_uploaded={bytes_uploaded}, expected={upload_bytes}")

    if max_elapsed_seconds and elapsed_seconds > max_elapsed_seconds:
        slo_pass = False
        errors.append(f"elapsed_slo={elapsed_seconds:.3f}>{max_elapsed_seconds:.3f}")
    if kind == "download" and min_download_bps and speed_download < min_download_bps:
        slo_pass = False
        errors.append(f"download_bps_slo={speed_download:.1f}<{min_download_bps:.1f}")
    if kind == "upload" and min_upload_bps and speed_upload < min_upload_bps:
        slo_pass = False
        errors.append(f"upload_bps_slo={speed_upload:.1f}<{min_upload_bps:.1f}")

    stderr = completed.stderr.strip()
    if stderr:
        errors.append(stderr.replace("\n", " | "))

    return HammerResult(
        phase=phase,
        run=run_index,
        worker=worker_index,
        kind=kind,
        url=url,
        success=success,
        slo_pass=slo_pass,
        http_code=http_code,
        curl_exit=completed.returncode,
        bytes_downloaded=bytes_downloaded,
        bytes_uploaded=bytes_uploaded,
        elapsed_seconds=elapsed_seconds,
        wall_seconds=wall_seconds,
        bytes_per_second=bytes_per_second,
        error="; ".join(errors),
    )


def parse_curl_metrics(stdout: str) -> dict[str, float]:
    line = stdout.strip().splitlines()[-1] if stdout.strip() else ""
    parts = line.split("\t")
    if len(parts) != len(WRITE_OUT_FIELDS):
        return {}

    parsed: dict[str, float] = {}
    for field, value in zip(WRITE_OUT_FIELDS, parts):
        try:
            parsed[field] = float(value)
        except ValueError:
            parsed[field] = 0.0
    return parsed


def print_result(result: HammerResult, *, jsonl: bool) -> None:
    payload = asdict(result)
    payload["schema_version"] = 1
    payload["event"] = "hammer_measurement"
    payload["ok"] = result.ok
    payload["elapsed_seconds"] = round(result.elapsed_seconds, 6)
    payload["wall_seconds"] = round(result.wall_seconds, 6)
    payload["bytes_per_second"] = round(result.bytes_per_second, 1)
    payload["failure_class"] = failure_class(result)
    payload["acceptance"] = {
        "ok": result.ok,
        "gates": [
            {
                "name": "transfer_success",
                "ok": result.success,
                "value": "ok" if result.success else result.error,
                "threshold": "curl_exit=0,http_code_allowed,byte_count_match",
            },
            {
                "name": "slo",
                "ok": result.slo_pass,
                "value": "ok" if result.slo_pass else result.error,
                "threshold": "configured elapsed/throughput SLOs",
            },
        ],
    }

    if jsonl:
        print(json.dumps(payload, sort_keys=True))
        return

    status = "ok" if result.ok else "fail"
    print(
        " ".join(
            [
                f"{status}",
                f"phase={result.phase}",
                f"run={result.run}",
                f"worker={result.worker}",
                f"kind={result.kind}",
                f"http={result.http_code}",
                f"curl={result.curl_exit}",
                f"down={result.bytes_downloaded}",
                f"up={result.bytes_uploaded}",
                f"time={result.elapsed_seconds:.3f}s",
                f"speed={result.bytes_per_second:.1f}B/s",
                f"error={result.error or '-'}",
            ]
        )
    )


def failure_class(result: HammerResult) -> str | None:
    if result.ok:
        return None
    if result.curl_exit != 0:
        return f"curl_exit:{result.curl_exit}"
    if result.http_code < 200 or result.http_code >= 400:
        return f"http_status:{result.http_code}"
    if "bytes_downloaded=" in result.error:
        return "download_size_mismatch"
    if "bytes_uploaded=" in result.error:
        return "upload_size_mismatch"
    if "elapsed_slo=" in result.error:
        return "elapsed_slo"
    if "download_bps_slo=" in result.error:
        return "download_throughput_slo"
    if "upload_bps_slo=" in result.error:
        return "upload_throughput_slo"
    return "unknown"


def result_kwargs(
    *,
    args: argparse.Namespace,
    curl_bin: str,
    upload_payload_path: pathlib.Path,
    download_http_codes: set[int],
    upload_http_codes: set[int],
    phase: str,
    run_index: int,
    worker_index: int,
    kind: str,
) -> dict[str, object]:
    if kind == "download":
        url = args.download_url
        expected_codes = download_http_codes
    elif kind == "upload":
        url = args.upload_url
        expected_codes = upload_http_codes
    else:
        raise RuntimeError(f"unsupported run kind: {kind}")

    return {
        "curl_bin": curl_bin,
        "socks": args.socks,
        "timeout_seconds": args.timeout_seconds,
        "connect_timeout_seconds": args.connect_timeout_seconds,
        "phase": phase,
        "run_index": run_index,
        "worker_index": worker_index,
        "kind": kind,
        "url": url,
        "upload_method": args.upload_method,
        "upload_payload_path": upload_payload_path,
        "expected_download_bytes": args.expect_download_bytes,
        "upload_bytes": args.upload_bytes,
        "expected_http_codes": expected_codes,
        "min_download_bps": args.min_download_bps,
        "min_upload_bps": args.min_upload_bps,
        "max_elapsed_seconds": args.max_elapsed_seconds,
    }


def concurrent_kind_for_worker(pattern: str, worker_index: int) -> str:
    if pattern == "mixed":
        return "download" if worker_index % 2 else "upload"
    return pattern


def run_hammer(args: argparse.Namespace) -> list[HammerResult]:
    curl_bin = check_curl(args.curl_bin)
    if not args.skip_proxy_check:
        check_proxy(args.socks, args.connect_timeout_seconds)

    phases = args.phase or ["download", "upload", "concurrent"]
    download_http_codes = parse_http_codes(args.download_http_codes)
    upload_http_codes = parse_http_codes(args.upload_http_codes)

    results: list[HammerResult] = []
    with make_upload_payload(args.upload_bytes) as upload_payload:
        upload_payload_path = pathlib.Path(upload_payload.name)

        for phase in phases:
            if phase in {"download", "upload"}:
                for run_index in range(1, args.runs + 1):
                    result = run_curl(
                        **result_kwargs(
                            args=args,
                            curl_bin=curl_bin,
                            upload_payload_path=upload_payload_path,
                            download_http_codes=download_http_codes,
                            upload_http_codes=upload_http_codes,
                            phase=phase,
                            run_index=run_index,
                            worker_index=1,
                            kind=phase,
                        )
                    )
                    results.append(result)
                    print_result(result, jsonl=args.jsonl)
                    if args.delay_seconds and run_index < args.runs:
                        time.sleep(args.delay_seconds)
                continue

            for batch_index in range(1, args.concurrent_batches + 1):
                with concurrent.futures.ThreadPoolExecutor(
                    max_workers=args.concurrency
                ) as executor:
                    futures = []
                    for worker_index in range(1, args.concurrency + 1):
                        kind = concurrent_kind_for_worker(args.concurrent_kind, worker_index)
                        futures.append(
                            executor.submit(
                                run_curl,
                                **result_kwargs(
                                    args=args,
                                    curl_bin=curl_bin,
                                    upload_payload_path=upload_payload_path,
                                    download_http_codes=download_http_codes,
                                    upload_http_codes=upload_http_codes,
                                    phase=phase,
                                    run_index=batch_index,
                                    worker_index=worker_index,
                                    kind=kind,
                                ),
                            )
                        )

                    for future in concurrent.futures.as_completed(futures):
                        result = future.result()
                        results.append(result)
                        print_result(result, jsonl=args.jsonl)

                if args.delay_seconds and batch_index < args.concurrent_batches:
                    time.sleep(args.delay_seconds)

    return results


def print_summary(results: list[HammerResult], *, jsonl: bool) -> None:
    failures = [result for result in results if not result.success]
    slo_misses = [result for result in results if result.success and not result.slo_pass]
    ok = len(results) - len(failures) - len(slo_misses)

    summary = {
        "total": len(results),
        "ok": ok,
        "failures": len(failures),
        "slo_misses": len(slo_misses),
        "failure_classes": counted(
            failure_class(result) for result in results if failure_class(result)
        ),
        "by_phase_kind": summarize_hammer_groups(results),
    }

    if jsonl:
        print(
            json.dumps(
                {"schema_version": 1, "event": "hammer_summary", "summary": summary},
                sort_keys=True,
            )
        )
        return

    print(
        f"summary total={summary['total']} ok={summary['ok']} "
        f"failures={summary['failures']} slo_misses={summary['slo_misses']}"
    )
    for key, group in summary["by_phase_kind"].items():
        print(
            " ".join(
                [
                    f"group={key}",
                    f"total={group['total']}",
                    f"ok={group['ok']}",
                    f"failures={group['failures']}",
                    f"elapsed_p50={group['elapsed_seconds']['p50']}",
                    f"elapsed_p95={group['elapsed_seconds']['p95']}",
                    f"bps_p50={group['bytes_per_second']['p50']}",
                    f"bps_p95={group['bytes_per_second']['p95']}",
                ]
            )
        )


def summarize_hammer_groups(results: list[HammerResult]) -> dict[str, dict[str, object]]:
    grouped: dict[str, list[HammerResult]] = {}
    for result in results:
        grouped.setdefault(f"{result.phase}:{result.kind}", []).append(result)

    return {
        key: {
            "total": len(group),
            "ok": sum(1 for result in group if result.ok),
            "failures": sum(1 for result in group if not result.success),
            "slo_misses": sum(1 for result in group if result.success and not result.slo_pass),
            "elapsed_seconds": numeric_summary([result.elapsed_seconds for result in group]),
            "wall_seconds": numeric_summary([result.wall_seconds for result in group]),
            "bytes_per_second": numeric_summary([result.bytes_per_second for result in group]),
            "bytes_downloaded": numeric_summary([result.bytes_downloaded for result in group]),
            "bytes_uploaded": numeric_summary([result.bytes_uploaded for result in group]),
            "http_codes": counted(str(result.http_code) for result in group),
            "curl_exits": counted(str(result.curl_exit) for result in group),
            "failure_classes": counted(
                failure_class(result) for result in group if failure_class(result)
            ),
        }
        for key, group in sorted(grouped.items())
    }


def counted(values) -> dict[str, int]:
    counts: dict[str, int] = {}
    for value in values:
        if value is None:
            continue
        key = str(value)
        counts[key] = counts.get(key, 0) + 1
    return dict(sorted(counts.items()))


def numeric_summary(values: list[float | int]) -> dict[str, float | int | None]:
    if not values:
        return {"count": 0, "min": None, "p50": None, "p95": None, "max": None}
    ordered = sorted(float(value) for value in values)
    return {
        "count": len(ordered),
        "min": round(ordered[0], 6),
        "p50": round(percentile(ordered, 50), 6),
        "p95": round(percentile(ordered, 95), 6),
        "max": round(ordered[-1], 6),
    }


def percentile(ordered_values: list[float], percentile_value: float) -> float:
    if len(ordered_values) == 1:
        return ordered_values[0]
    rank = (len(ordered_values) - 1) * (percentile_value / 100.0)
    lower = int(rank)
    upper = min(lower + 1, len(ordered_values) - 1)
    weight = rank - lower
    return ordered_values[lower] * (1.0 - weight) + ordered_values[upper] * weight


def main() -> int:
    args = parse_args()
    try:
        results = run_hammer(args)
    except (
        OSError,
        RuntimeError,
        ValueError,
        argparse.ArgumentTypeError,
        subprocess.SubprocessError,
    ) as exc:
        print(f"hammer setup failed: {exc}", file=sys.stderr)
        return 2

    print_summary(results, jsonl=args.jsonl)
    return 0 if all(result.ok for result in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
