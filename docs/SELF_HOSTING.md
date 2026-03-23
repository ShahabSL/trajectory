# Self-Hosting Trajectory

This guide is for operators who want to run a Trajectory server and hand credentials to CLI, desktop, or Android users.

## Requirements

- a DNS domain delegated to the server host
- a Linux server with root access
- a local SOCKS5 upstream on the server, or a `hev-socks5-server` binary you want the installer to manage
- Rust locally if you are building binaries from source

## One-command server install

The fastest operator path is a single command on the server:

```bash
curl -fsSL https://raw.githubusercontent.com/ShahabSL/trajectory/main/deploy/bootstrap_server.sh | \
  sudo bash -s -- \
  --domain your.domain.example \
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

This bootstrap script:

- installs the build dependencies it needs
- installs Rust if the host does not already have it
- clones the repo
- builds `trajectory-server` and `trajectory-admin`
- runs `deploy/install_server.sh`
- prints an initial client access key

## Build the operator binaries manually

```bash
cargo build --release -p trajectory-cli --bins
```

This produces:

- `target/release/trajectory-server`
- `target/release/trajectory-admin`
- `target/release/trajectory-client`

## Install the server manually

If the host already has a local SOCKS5 upstream on `127.0.0.1:1080`:

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

If you want the installer to manage a local `hev-socks5-server` binary too:

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --hev-binary /path/to/hev-socks5-server \
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

The installer:

- copies binaries into `/opt/trajectory`
- writes `/etc/trajectory/server.env`
- creates `/opt/trajectory/trajectory-clients.json`
- optionally prints an initial client access key
- installs and starts `trajectory.service`
- optionally installs and starts `trajectory-socks.service`

## Manage client credentials

Create another client:

```bash
/opt/trajectory/trajectory-admin create-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --label laptop
```

List clients:

```bash
/opt/trajectory/trajectory-admin list-clients \
  --client-db /opt/trajectory/trajectory-clients.json
```

Disable a client:

```bash
/opt/trajectory/trajectory-admin disable-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --id 7cad96fb
```

## Client setup

Clients need only:

- the domain, for example `your.domain.example`
- their `traj1_...` access key

Resolvers can be left blank. Current clients fall back to the built-in public resolver set and automatically probe/select active paths at runtime.

### CLI client

```bash
./trajectory-client \
  --tcp-listen-port 7000 \
  --domain your.domain.example \
  --access-key traj1_0123abcd_BASE32SECRET \
  --keep-alive-interval 50
```

### Desktop client

Open the desktop app and paste:

- access key
- domain

Then point applications at the local SOCKS listener.

### Android client

Build and install the APK:

```bash
scripts/build_android_release.sh
scripts/install_android_release.sh
```

The build helper isolates Gradle state automatically by default. Set `GRADLE_USER_HOME` yourself if you want persistent local Gradle caches across repeated builds.

Then open the app and paste:

- access key
- domain

The Android app supports both proxy mode and VPN mode.

## Validation

Server:

```bash
systemctl status trajectory.service
```

Client:

```bash
curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 https://example.com
```

## Operational notes

- `trajectory.service` binds UDP/53 by default, so the host must not already have another DNS server on that port.
- If the machine still runs `systemd-resolved` on port 53, rerun the installer with `--disable-systemd-resolved` or free the port yourself.
- Recursive public DNS paths still vary by resolver family and by authoritative server placement.
