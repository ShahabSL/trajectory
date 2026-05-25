# Benchmark Observability

Trajectory benchmark output is JSON Lines so runs can be archived, compared with
`jq`, and promoted into release gates without scraping prose.

## Browser Benchmark

Use `scripts/benchmark_real_browsing.py` against an already-running SOCKS5
listener:

```bash
python3 scripts/benchmark_real_browsing.py \
  --socks 127.0.0.1:7000 \
  --repeat 3 \
  --playwright-target all \
  --accept-max-browser-request-failures 0 \
  --accept-max-response-start-ms 5000 \
  --accept-max-load-event-ms 15000 \
  --trajectory-diag-jsonl /tmp/trajectory-client.stderr.jsonl
```

Every `measurement` event includes:

- `validation`: HTTP status, final host, byte count, and content marker checks.
- `acceptance`: gate results that decide whether the run is acceptable.
- `network.failed_count`: failed browser subrequests.
- `network.failed_by_error`, `failed_by_type`, `failed_by_host`: failure buckets.
- `network.slow_requests`: slowest Playwright request lifecycles.
- `network.pending`: requests still open when the page finished or failed.
- `network.slow_resources`: browser Performance API resource timings.

The final `benchmark_end` event includes grouped p50/p95 summaries by
`tool:target`, failed acceptance gates, failure samples, and optional
`runtime_diag` extracted from `TRAJECTORY_DIAG` JSON lines.

## Runtime Diagnostics

Set `TRAJECTORY_DIAG=1` on the client and redirect stderr to a file:

```bash
TRAJECTORY_DIAG=1 trajectory-client ... 2>/tmp/trajectory-client.stderr.jsonl
```

Current runtime diagnostics are connection-level client counters emitted by the
shared transport scheduler. The benchmark parser reports the latest global
snapshot, query failure rate, useful response byte ratio, and transport backlog.
These metrics are enough to separate browser waterfall failures from transport
backlog, but they are not yet a full per-resolver model.

Missing runtime-side metrics that should be added when core runtime edits are in
scope:

- per-resolver RTT, timeout, cwnd, inflight, loss, goodput, and quarantine state
- per-packet class, resolver, timeout reason, response size, and retry migration
- per-stream queued/sent/acked/repaired bytes
- DNS/TCP lane ID, HOL timeout count, reconnect count, and lane quarantine
- response packing efficiency: useful stream bytes vs DNS wire bytes

## Loopback Hammer

Use `scripts/socks_loopback_hammer.py` for transfer and upload pressure:

```bash
python3 scripts/socks_loopback_hammer.py \
  --socks 127.0.0.1:7000 \
  --runs 3 \
  --concurrency 4 \
  --concurrent-kind mixed \
  --jsonl
```

Each `hammer_measurement` includes `failure_class` and `acceptance` gates. The
final `hammer_summary` includes grouped p50/p95 elapsed time, throughput,
download/upload bytes, HTTP codes, curl exits, and failure classes.

## Acceptance Criteria

For a release candidate, use the same criteria for Trajectory, MasterDNS,
StormDNS, direct SOCKS, and the hostile/twoman baseline:

- zero proxy check failures
- zero browser navigation/content validation failures
- browser request failures no worse than direct baseline, ideally zero
- p50 and p95 `response_start_ms`, `load_event_ms`, and curl total time beat
  competing tunnels for the same resolver set
- no transfer byte mismatches
- upload succeeds while browsing is active
- runtime query failure rate below 1 percent on admitted resolvers
- useful response byte ratio improves or stays stable when tuning

Only compare hostile-network pages that the direct hostile SOCKS path can load.
If direct twoman cannot open a page, that page is not a fair Trajectory failure
target for that path.
