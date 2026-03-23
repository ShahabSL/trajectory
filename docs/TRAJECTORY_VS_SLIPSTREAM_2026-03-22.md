# Trajectory vs Slipstream on Public Resolvers (2026-03-22)

This note records a real sequential comparison between Trajectory and upstream Slipstream on the same machine, the same VPS, the same domain, and the same public recursive resolver set.

## Methodology

- Local machine: this workspace host
- Remote server: the existing benchmark VPS from `.secrets/server.env`
- Domain: `t.7-b.cc`
- DNS port: `53`
- Resolver set for multipath runs:
  - `1.1.1.1:53`
  - `1.0.0.1:53`
  - `8.8.8.8:53`
  - `8.8.4.4:53`
- Single-resolver control:
  - `1.1.1.1:53`
- Congestion control: `bbr`
- Keep-alive interval: `100`
- Comparison rule:
  - run implementations sequentially, never at the same time
  - score paired blocks by completion first, elapsed time second
  - treat invalid control cases as invalid, not as wins

The comparison used the upstream Slipstream client and server built from `https://github.com/EndPositive/slipstream`, and the current Trajectory client and server from this repository.

The synthetic comparison used the existing harness in [scripts/benchmark_public.py](../scripts/benchmark_public.py). The browser-like comparison reused the same VPS and domain, but pointed both servers at the VPS-local SOCKS upstream on `127.0.0.1:1080` so the local clients could be exercised as browser proxies.

## Direct Control

Direct, non-tunnel control to the public payload endpoint for `1 MiB`:

- run 1: `1317705 B/s`
- run 2: `1489198 B/s`
- median: `1403451.5 B/s`

The public `trajectory-4m.bin` control endpoint returned `404` during this run window, so any browser block that depended on that URL is not a valid transport comparison.

## Synthetic Results

All synthetic cold-start comparison blocks failed for both implementations on this route.

### `1.1.1.1` only, `1 MiB`

- Slipstream: timeout, `0` bytes delivered
- Trajectory: timeout, `0` bytes delivered

### `1 MiB`, four resolvers, three paired blocks

- Slipstream: `0/3` completions
- Trajectory: `0/3` completions

### `4 MiB`, four resolvers, two paired blocks

- Slipstream: `0/2` completions
- Trajectory: `0/2` completions

### `10 MiB`, four resolvers, one paired block

- Slipstream: timeout, `0` bytes delivered
- Trajectory: timeout, `0` bytes delivered

Conclusion for the synthetic matrix: **no winner**. On this March 22, 2026 public-recursive path, both stacks collapsed under the large cold-start synthetic cases.

## Browser-Like Results

### `HEAD https://example.com`

- Slipstream: timeout at `20.007731s`
- Trajectory: timeout at `20.007470s`

No winner. Both failed.

### `GET /trajectory-1m.bin` through SOCKS

- Trajectory:
  - `1048576` bytes
  - `118821 B/s`
  - `8.824773s`
  - completed cleanly
- Slipstream:
  - `1047509` bytes
  - `77629 B/s`
  - `13.493764s`
  - `curl` exit `18` (transfer closed short)

Winner: **Trajectory**

### `GET /trajectory-4m.bin` through SOCKS

- Slipstream: `404` in `0.610998s`
- Trajectory: `404` in `135.711194s`

This block is **invalid for transport winner scoring** because the control object was missing. It is still worth noting that Trajectory handled the missing object path badly in this one run window.

### `HEAD https://www.youtube.com`

- Trajectory: `200` in `1.136109s`
- Slipstream: `200` in `1.453402s`

Winner: **Trajectory**

### `GET https://www.youtube.com`

- Slipstream: timeout at `59.999400s`
- Trajectory: timeout at `60.000472s`

No winner. Both failed.

### `4 x 1 MiB` concurrent through SOCKS

Trajectory:

- completion count: `3/4`
- successful runs:
  - `91274 B/s`
  - `81422 B/s`
  - `85300 B/s`
- one timeout at `120s`

Slipstream:

- completion count: `2/4`
- successful runs:
  - `45520 B/s`
  - `45489 B/s`
- two timeouts at `120s`

Winner: **Trajectory**

## Final Call

On this route, with this machine, this VPS, this domain, and this resolver set:

- **Synthetic large cold-start comparison:** no clear winner, because both implementations failed.
- **Meaningful browser/file comparison:** **Trajectory wins**.

Why the winner call is Trajectory:

- it completed the only valid `1 MiB` browser-file transfer cleanly, while Slipstream delivered a short response
- it was faster on the valid YouTube `HEAD` check
- it completed more of the concurrent `4 x 1 MiB` block (`3/4` vs `2/4`)

What this result does **not** mean:

- it does not prove Trajectory is universally faster than Slipstream
- it does not prove either tunnel is healthy for large cold-start transfers on every public-recursive path
- it does not validate the missing-object `4 MiB` block as a fair transport benchmark

## Reproduction Notes

Re-run the synthetic public harness with:

```bash
python3 scripts/benchmark_public.py \
  --size-bytes 1048576 \
  --timeout-seconds 240 \
  --trajectory-client-bin target/release/trajectory-client \
  --trajectory-server-bin target/release/trajectory-server \
  --trajectory-access-key 'traj1_...'
```

The full March 22, 2026 comparison also used a private local script that reused the same VPS/domain path for:

- direct non-tunnel control
- synthetic `1 MiB`, `4 MiB`, and `10 MiB` pairs
- `example.com` and `youtube.com` browser checks
- browser-style file fetches through SOCKS
- concurrent `4 x 1 MiB` transfer pressure

If you repeat this comparison, keep the two implementations sequential, not concurrent, and compare paired blocks under the same time-local conditions.
