# Trajectory

Trajectory is a pure-Rust DNS tunnel optimized for public-resolver throughput. It uses a pipelined request/ack protocol over DNS TXT queries, resolver-aware scheduling, and a lightweight downlink path for interactive traffic.

The repo ships two binaries:

- `trajectory-server`: authoritative UDP DNS server that forwards tunnel sessions to a TCP target
- `trajectory-client`: local TCP listener that carries a session over one or more public DNS resolvers

## Build

```bash
cargo build --release --bin trajectory-client --features client
cargo build --release --bin trajectory-server --features server
```

## Local Smoke Test

Start a local TCP sink:

```bash
python3 - <<'PY'
import socket, pathlib
out = pathlib.Path("/tmp/trajectory-smoke.out")
s = socket.socket()
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(("127.0.0.1", 5201))
s.listen(1)
c, _ = s.accept()
with c, out.open("wb") as handle:
    while True:
        chunk = c.recv(65536)
        if not chunk:
            break
        handle.write(chunk)
PY
```

Start the server:

```bash
./target/release/trajectory-server \
  --dns-listen-port 8853 \
  --target-address 127.0.0.1:5201 \
  --domain test.com
```

Start the client:

```bash
./target/release/trajectory-client \
  --tcp-listen-port 7000 \
  --resolver 127.0.0.1:8853 \
  --domain test.com \
  --congestion-control bbr \
  --keep-alive-interval 50
```

Send a payload:

```bash
python3 - <<'PY'
import socket, time
payload = b"hello-fast-path\n" * 64
s = socket.create_connection(("127.0.0.1", 7000), timeout=5)
s.sendall(payload)
time.sleep(1)
s.shutdown(socket.SHUT_WR)
s.close()
PY
```

## Deploy

Install the release binaries and service unit:

```bash
install -d -m 755 /opt/trajectory /opt/trajectory/certs
install -m 755 target/release/trajectory-server /opt/trajectory/trajectory-server
install -m 755 target/release/trajectory-client /opt/trajectory/trajectory-client
install -m 644 deploy/trajectory.service /etc/systemd/system/trajectory.service
systemctl daemon-reload
systemctl enable --now trajectory.service
```

Why this matters: the server binds UDP `:53`, so a host-local resolver such as `systemd-resolved` cannot stay on that port at the same time.

If the host previously used `systemd-resolved`, replace `/etc/resolv.conf` with real upstream resolvers before or after stopping it, for example:

```bash
rm -f /etc/resolv.conf
cat >/etc/resolv.conf <<'EOF'
nameserver 1.1.1.1
nameserver 8.8.8.8
nameserver 9.9.9.9
options edns0
EOF
```

Why this matters: SSH dynamic forwarding with `-D` relies on remote hostname resolution. If `/etc/resolv.conf` still points at `127.0.0.53` after `systemd-resolved` is stopped, browser traffic through the SOCKS proxy will hang on DNS lookups even though raw SSH through the tunnel still works.

## Public Resolver Benchmark

Benchmark the current build against upstream Slipstream:

```bash
python3 scripts/benchmark_public.py --size-bytes 16384 --timeout-seconds 180 --stall-seconds 30
```

Benchmark the saved translated baseline against upstream Slipstream:

```bash
python3 scripts/benchmark_public.py \
  --resolver 1.1.1.1:53 \
  --size-bytes 16384 \
  --timeout-seconds 180 \
  --stall-seconds 30 \
  --trajectory-client-bin /tmp/trajectory-baseline/trajectory-client.translated \
  --trajectory-server-bin /tmp/trajectory-baseline/trajectory-server.translated
```

Learning Notes:
- Public DNS throughput is dominated by query budget, in-flight window sizing, resolver behavior, and retransmit policy more than language choice.
- Recursive resolvers may issue non-TXT zone probes; the server answers those authoritatively instead of timing out so the real tunnel queries keep flowing.

Why This Matters:
- A direct authoritative test can hide resolver behavior that decides real-world throughput.
- Keeping the benchmark harness in-repo makes it easy to compare the current fast path, the old baseline, and upstream Slipstream under the same network conditions.
