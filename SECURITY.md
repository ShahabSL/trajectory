# Security Policy

Trajectory is a restricted-license, source-available network transport. Treat access keys, resolver lists, signing keys, private domains, and live server endpoints as secrets.

## Reporting A Vulnerability

Use GitHub private vulnerability reporting when available:

https://github.com/ShahabSL/trajectory/security/advisories/new

If private reporting is unavailable, contact Shahab Lavasani through the repository owner profile at https://github.com/ShahabSL before publishing details. If a public issue is the only available path, write only "security contact requested" and do not include exploit steps, credentials, private DNS names, resolver lists, or live infrastructure details.

## Supported Surfaces

Security-sensitive reports are accepted for:

- `trajectory-client`, `trajectory-server`, and `trajectory-admin`
- the Rust transport core
- Android proxy and VPN modes
- desktop client packaging and local process launch
- server install scripts and systemd units
- release workflows and signing/packaging scripts

## Boundaries

Trajectory is transport software. It does not claim anonymity, censorship resistance, endpoint compromise protection, or resolver trust. Operators are responsible for lawful use, server hardening, DNS delegation, credential rotation, and monitoring.
