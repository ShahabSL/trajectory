# DNS Path Discovery

Trajectory's production baseline is DNS-native TXT/QNAME carriage with signed
resolver admission and per-resolver transport state. TXT is the best first
carrier because it is standard, variable length, and natural for opaque response
data. It is not guaranteed to be available everywhere.

## Can TXT Be Limited?

Yes. Networks, recursive resolvers, captive portals, and DNS security products
can rate-limit, truncate, block, rewrite, or classify TXT-heavy traffic. That is
not a standards violation by itself; DNS operators routinely apply policy to
query types and suspicious tunneling shapes.

The practical result is that Trajectory must not assume "TXT works" only because
plain A/AAAA DNS works. TXT must be admitted with a signed tunnel challenge and a
small checksum transfer before it enters the scheduler.

## Carrier Candidates

Carrier support should be modeled as adapters under the same selective-ARQ
stream protocol. The server can accept multiple query types, while the client
discovers which candidates work on the current network.

Current carrier:

- `TXT`: primary carrier for secure, velocity, resilient, and frontier modes.

Possible future restricted-network carriers:

- `A`: very small response payload through IPv4 addresses; widely permitted but
  expensive in query count.
- `AAAA`: larger than A through IPv6 addresses; also widely understood.
- `CNAME`: useful for response indirection, but recursive behavior and caching
  make it awkward for high-rate opaque payloads.
- `MX`, `NS`, `SRV`, `CAA`: possible low-rate signaling carriers, usually poor
  bulk carriers because structure consumes bytes and policy may be stricter.
- `NULL`: historically convenient for arbitrary data, but not a safe public
  Internet baseline because support is inconsistent and policy-sensitive.

These are not replacements for the transport. They are path adapters for the
same stream, ACK, credit, and repair machinery.

## Discovery Model

The expert consensus is an ICE-like path discovery system, but DNS-native:

1. Build candidates from resolver, network transport, query type, EDNS size, and
   qname shape.
2. Run plain DNS viability probes to avoid wasting signed tunnel work.
3. Run signed tunnel admission for each viable candidate.
4. Run a 32-128 KiB checksum micro-transfer before scheduler admission.
5. Rank candidates by correctness first, then RTT, loss, stable goodput,
   timeout behavior, cache risk, and policy risk.
6. Admit candidates into active, standby, or quarantined cohorts.
7. Keep observing candidates; promote or demote them when the network changes.

The client should expose this as "Best path" or "Auto" behavior. User-facing
modes such as secure, velocity, resilient, and frontier should select policy
weights and safety limits, not hard-code one carrier forever.

## Implementation Rule

Do not add speculative carriers as silent fallbacks. A carrier is production
eligible only after it passes:

- standards-compliant DNS packet parsing and response generation
- signed admission
- checksum micro-transfer
- resolver cohort telemetry
- Android VPN, desktop proxy, and Linux CLI e2e tests

Until that exists, TXT remains the production carrier and the next meaningful
work is a first-class carrier-candidate admission report rather than more
unobservable fallback logic.

## References

- RFC 1035: DNS message format and core resource records.
- RFC 3596: AAAA records.
- RFC 6891: EDNS(0) payload sizing.
- RFC 7766: DNS over TCP requirements and operational guidance.
- DNS Flag Day 2020: operational limits around fragmented DNS and EDNS buffer
  sizing.
- PowerDNS Defender tunneling guidance: practical DNS tunneling detection.
- Cisco Umbrella DNS request type troubleshooting: real-world resolver behavior
  by query type.
- net2share/vaydns: example of a DNS tunnel exposing multiple record-type
  carriers.
