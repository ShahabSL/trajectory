# GitHub Actions Live E2E

Trajectory CI has two layers:

- normal CI runs on pushes and pull requests without secrets
- live e2e runs only from trusted events and uses a protected GitHub environment

Do not use `pull_request_target` for live e2e. Pull request code is untrusted, and the live tunnel access key must not be exposed to forked code or logs.

## Protected Environment

Create a GitHub environment named:

```text
trajectory-live-e2e
```

Required protection:

- required reviewers enabled
- deployment branch policy limited to `main`
- no secrets stored at repository level for live tunnel access

## Repository Variable

Set this repository variable to control automatic main-branch live tests:

```text
TRAJECTORY_E2E_ENABLED=false
```

Keep it `false` unless the live server is stable enough for every `main` push to run a network test.

## Environment Secrets

Store these as environment secrets on `trajectory-live-e2e`:

```text
TRAJECTORY_E2E_DOMAIN
TRAJECTORY_E2E_ACCESS_KEY
TRAJECTORY_E2E_RESOLVERS
```

Optional:

```text
TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY
TRAJECTORY_E2E_FETCH_URL
TRAJECTORY_E2E_DNS_MAX_PAYLOAD
TRAJECTORY_E2E_HTTP_LISTEN_PORT
```

`TRAJECTORY_E2E_RESOLVERS` can be newline, comma, or space separated. Example:

```text
1.1.1.1:53
1.0.0.1:53
8.8.8.8:53
8.8.4.4:53
```

## Setup With GitHub CLI

```bash
gh api --method PUT repos/OWNER/REPO/environments/trajectory-live-e2e

gh variable set TRAJECTORY_E2E_ENABLED --body false

gh secret set TRAJECTORY_E2E_DOMAIN --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_ACCESS_KEY --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_RESOLVERS --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_FETCH_URL --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_DNS_MAX_PAYLOAD --env trajectory-live-e2e
gh secret set TRAJECTORY_E2E_HTTP_LISTEN_PORT --env trajectory-live-e2e
```

## Manual Live Run

```bash
gh workflow run ci.yml --ref main -f live_e2e=true
```

## Safety Rules

- never print access keys in workflow logs
- pass the access key through `TRAJECTORY_ACCESS_KEY`, not a CLI argument
- do not upload client logs, admission reports, or generated resolver files as artifacts
- do not run live e2e on `pull_request`
- do not use `pull_request_target` with checked-out PR code

The live e2e script writes all sensitive runtime files under `$RUNNER_TEMP` with `umask 077` and deletes them on exit.
It validates both SOCKS5 access through `--listen` and HTTP proxy access through
`--http-listen`, so the protected server should use `socks5-direct` or another
SOCKS5-capable egress target.

## Android VPN CI Boundary

Normal CI also builds the Android APK, runs JVM unit tests, and checks that the
APK contains both `arm64-v8a` and `x86_64` native libraries for
`trajectory-client` and `trajectory_vpn_bridge`.

GitHub-hosted runners can prove build packaging and static Android VPN wiring.
They cannot prove user consent, OEM background behavior, always-on lockdown,
DNS leaks, sleep/wake, or arbitrary app capture without an emulator/device lab.
Those tests must run on trusted device runners with secrets scoped to a protected
environment.
