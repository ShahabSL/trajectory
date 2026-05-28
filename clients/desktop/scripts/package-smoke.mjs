import { spawn, spawnSync } from "node:child_process";
import dgram from "node:dgram";
import http from "node:http";
import { mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { constants, existsSync, writeFileSync } from "node:fs";
import { accessSync, chmodSync } from "node:fs";
import path from "node:path";

const artifactDir =
  process.argv[2] ?? path.join(process.env.RUNNER_TEMP ?? "/tmp", "trajectory-desktop-package");
const desktopDir = path.resolve(import.meta.dirname, "..");
const repoRoot = path.resolve(desktopDir, "..", "..");
const tauriDir = path.join(desktopDir, "src-tauri");
const bundleRoot = path.join(tauriDir, "target", "release", "bundle");
const packageJson = JSON.parse(
  await readText(path.join(desktopDir, "package.json")),
);
const version = packageJson.version;
const releaseTag = process.env.RELEASE_TAG;
if (releaseTag && releaseTag !== `v${version}`) {
  throw new Error(`desktop package version ${version} does not match ${releaseTag}`);
}

await mkdir(artifactDir, { recursive: true });
await writeFile(
  path.join(artifactDir, "package-smoke-started.txt"),
  `started desktop package smoke on ${process.platform} for ${version}\n`,
);

const files = await listFiles(bundleRoot);
const manifest = [`desktop package smoke`, `platform=${process.platform}`, `version=${version}`];
let desktopLiveSmokeSkipNoted = false;
let desktopLocalServer = null;
let desktopLocalOrigin = null;

process.once("exit", () => {
  if (desktopLocalServer?.process && !desktopLocalServer.process.killed) {
    desktopLocalServer.process.kill();
  }
  desktopLocalOrigin?.server?.close();
});

try {
  if (process.platform === "linux") {
    await smokeLinux(files);
  } else if (process.platform === "darwin") {
    await smokeMac(files);
  } else if (process.platform === "win32") {
    await smokeWindows(files);
  } else {
    throw new Error(`unsupported desktop package smoke platform: ${process.platform}`);
  }
  } finally {
    await stopDesktopLocalServer();
    await stopDesktopLocalOrigin();
  }

await writeFile(path.join(artifactDir, "package-smoke.txt"), `${manifest.join("\n")}\n`);

async function smokeLinux(files) {
  const deb = requireOne(files, new RegExp(`Trajectory_${escapeRegex(version)}_.*\\.deb$`), "Linux .deb");
  const rpm = requireOne(files, new RegExp(`Trajectory-${escapeRegex(version)}-.*\\.rpm$`), "Linux .rpm");
  const appImage = requireOne(files, new RegExp(`Trajectory_${escapeRegex(version)}_.*\\.AppImage$`), "Linux AppImage");
  const appDir = path.join(bundleRoot, "appimage", "Trajectory.AppDir");
  const appRun = path.join(appDir, "AppRun");
  const sidecar = path.join(appDir, "usr", "bin", "trajectory-client");
  requireExecutable(appRun, "Linux AppDir AppRun");
  requireExecutable(sidecar, "Linux bundled trajectory-client sidecar");
  requireExecutable(appImage, "Linux AppImage executable");
  runChecked(sidecar, ["--help"], "Linux bundled sidecar --help");
  const debExtract = path.join(artifactDir, "deb-extract");
  runChecked("dpkg-deb", ["-x", deb, debExtract], "extract Linux .deb");
  const debFiles = await listFiles(debExtract);
  const debSidecar = requireOne(debFiles, /usr[/\\]bin[/\\]trajectory-client$/, "trajectory-client inside .deb");
  requireOne(debFiles, /usr[/\\]bin[/\\]trajectory-desktop$/, "trajectory-desktop inside .deb");
  runChecked(debSidecar, ["--help"], "Linux deb payload sidecar --help");

  const rpmExtract = path.join(artifactDir, "rpm-extract");
  if (commandExists("rpm2cpio") && commandExists("cpio")) {
    await mkdir(rpmExtract, { recursive: true });
    runChecked(
      "bash",
      ["-lc", `rpm2cpio ${shellQuote(rpm)} | cpio -idm --quiet`],
      "extract Linux .rpm",
      { cwd: rpmExtract },
    );
    const rpmFiles = await listFiles(rpmExtract);
    const rpmSidecar = requireOne(rpmFiles, /usr[/\\]bin[/\\]trajectory-client$/, "trajectory-client inside .rpm");
    const rpmLauncher = requireOne(rpmFiles, /usr[/\\]bin[/\\]trajectory-desktop$/, "trajectory-desktop inside .rpm");
    runChecked(rpmSidecar, ["--help"], "Linux rpm payload sidecar --help");
    const rpmLaunchCommand = commandExists("xvfb-run") ? "xvfb-run" : rpmLauncher;
    const rpmLaunchArgs = commandExists("xvfb-run") ? ["-a", rpmLauncher] : [];
    await assertLaunches(rpmLaunchCommand, rpmLaunchArgs, "Linux rpm extracted app launch smoke");
  } else if (process.env.CI) {
    throw new Error("rpm2cpio and cpio are required for CI Linux RPM package smoke");
  } else {
    manifest.push("rpm extraction=skipped locally because rpm2cpio/cpio are unavailable");
  }
  manifest.push(`deb=${relative(deb)}`, `rpm=${relative(rpm)}`, `appimage=${relative(appImage)}`);

  const launcher = commandExists("xvfb-run") ? "xvfb-run" : appRun;
  const args = commandExists("xvfb-run") ? ["-a", appRun] : [];
  await assertLaunches(launcher, args, "Linux AppDir launch smoke");
  if (process.env.CI && commandExists("sudo")) {
    const debPackageName = packageField(deb, "Package") || "trajectory-desktop";
    runChecked("sudo", ["dpkg", "-i", deb], "install Linux .deb");
    try {
      const installedLauncher = commandExists("xvfb-run") ? "xvfb-run" : "/usr/bin/trajectory-desktop";
      const installedArgs = commandExists("xvfb-run") ? ["-a", "/usr/bin/trajectory-desktop"] : [];
      await assertLaunches(installedLauncher, installedArgs, "Linux deb installed app launch smoke");
    } finally {
      runChecked("sudo", ["dpkg", "-r", debPackageName], "uninstall Linux .deb");
    }
  } else {
    manifest.push("Linux deb install launch=skipped outside CI or without sudo");
  }
  manifest.push("Linux rpm launch=covered by extracted payload sidecar checks on this runner");
  const appImageLauncher = commandExists("xvfb-run") ? "xvfb-run" : appImage;
  const appImageArgs = commandExists("xvfb-run") ? ["-a", appImage] : [];
  await assertLaunches(appImageLauncher, appImageArgs, "Linux AppImage launch smoke", {
    APPIMAGE_EXTRACT_AND_RUN: "1",
  });
}

async function smokeMac(files) {
  const dmg = findOptional(files, new RegExp(`Trajectory_?${escapeRegex(version)}.*\\.dmg$`), "macOS .dmg");
  let appArchive = findOptional(files, /\.app\.tar\.gz$/, "macOS app tarball");
  const appBundle = path.join(bundleRoot, "macos", "Trajectory.app");
  const { sidecar, launcher } = await inspectMacApp(appBundle, "macOS bundled app");
  runChecked(sidecar, ["--help"], "macOS bundled sidecar --help");
  if (dmg) {
    runChecked("hdiutil", ["verify", dmg], "macOS dmg verification");
    await smokeMountedDmg(dmg);
    manifest.push(`dmg=${relative(dmg)}`);
  } else {
    manifest.push("dmg=not emitted by this Tauri build");
  }
  manifest.push(`app=${relative(appBundle)}`);
  await assertLaunches(launcher, [], "macOS app bundle launch smoke");

  if (!appArchive) {
    if (process.env.CI) {
      throw new Error("macOS app tarball is required for CI desktop package smoke");
    }
    appArchive = path.join(artifactDir, `Trajectory_${version}_local.app.tar.gz`);
    runChecked("tar", ["-C", path.dirname(appBundle), "-czf", appArchive, path.basename(appBundle)], "create local macOS app tarball");
  }
  const appArchiveExtract = path.join(artifactDir, "macos-app-tar-extract");
  await mkdir(appArchiveExtract, { recursive: true });
  runChecked("tar", ["-xzf", appArchive, "-C", appArchiveExtract], "extract macOS app tarball");
  const extractedApp = path.join(appArchiveExtract, "Trajectory.app");
  const { sidecar: extractedSidecar, launcher: extractedLauncher } = await inspectMacApp(
    extractedApp,
    "macOS app tarball",
  );
  manifest.push(`app_tar=${relative(appArchive)}`);
  runChecked(extractedSidecar, ["--help"], "macOS app tarball sidecar --help");
  await assertLaunches(extractedLauncher, [], "macOS app tarball launch smoke");
}

async function inspectMacApp(appBundle, label) {
  const macosDir = path.join(appBundle, "Contents", "MacOS");
  const executables = (await listFiles(macosDir)).filter(isExecutable);
  const sidecar = executables.find((file) => path.basename(file).startsWith("trajectory-client"));
  const launcher = executables.find((file) => !path.basename(file).startsWith("trajectory-client"));
  if (!sidecar) throw new Error(`missing trajectory-client sidecar in ${label}`);
  if (!launcher) throw new Error(`missing app launcher executable in ${label}`);
  return { sidecar, launcher };
}

async function smokeMountedDmg(dmg) {
  const mountPoint = path.join(artifactDir, "dmg-mount");
  await mkdir(mountPoint, { recursive: true });
  runChecked(
    "hdiutil",
    ["attach", "-nobrowse", "-readonly", "-mountpoint", mountPoint, dmg],
    "macOS dmg attach",
  );
  try {
    const mountedApp = path.join(mountPoint, "Trajectory.app");
    const { sidecar, launcher } = await inspectMacApp(mountedApp, "mounted macOS dmg");
    runChecked(sidecar, ["--help"], "macOS mounted dmg sidecar --help");
    await assertLaunches(launcher, [], "macOS mounted dmg launch smoke");
    const copiedAppRoot = path.join(artifactDir, "dmg-installed-copy");
    await mkdir(copiedAppRoot, { recursive: true });
    const copiedApp = path.join(copiedAppRoot, "Trajectory.app");
    runChecked("ditto", [mountedApp, copiedApp], "copy macOS dmg app to installed location");
    const { launcher: copiedLauncher } = await inspectMacApp(copiedApp, "copied macOS dmg app");
    await assertLaunches(copiedLauncher, [], "macOS copied dmg app launch smoke");
  } finally {
    runChecked("hdiutil", ["detach", mountPoint], "macOS dmg detach");
  }
}

async function smokeWindows(files) {
  const msi = requireOne(files, new RegExp(`Trajectory_?${escapeRegex(version)}.*\\.msi$`), "Windows .msi");
  const setup = requireOne(files, new RegExp(`Trajectory_?${escapeRegex(version)}.*\\.exe$`), "Windows setup .exe");
  const targetDir = path.join(artifactDir, "msi-extract");
  runChecked("msiexec.exe", ["/a", msi, "/qn", `TARGETDIR=${targetDir}`], "extract Windows .msi");
  const msiFiles = await listFiles(targetDir);
  const msiSidecar = requireOne(msiFiles, /trajectory-client.*\.exe$/i, "trajectory-client inside .msi");
  const msiLauncher = requireOne(msiFiles, /(?:Trajectory|trajectory-desktop)\.exe$/i, "Trajectory app inside .msi");
  runChecked(msiSidecar, ["--help"], "Windows MSI sidecar --help");

  const sidecar = path.join(tauriDir, "bin", "trajectory-client-x86_64-pc-windows-msvc.exe");
  requireExecutable(sidecar, "Windows staged trajectory-client sidecar");
  runChecked(sidecar, ["--help"], "Windows staged sidecar --help");
  if (commandExists("7z")) {
    runChecked("7z", ["t", setup], "test Windows setup archive");
    const setupExtract = path.join(artifactDir, "setup-extract");
    await mkdir(setupExtract, { recursive: true });
    runChecked("7z", ["x", "-y", `-o${setupExtract}`, setup], "extract Windows setup archive");
    const setupFiles = await expandWindowsSetupPayload(setupExtract);
    const setupSidecar = requireOne(setupFiles, /trajectory-client.*\.exe$/i, "trajectory-client inside Windows setup");
    const setupLauncher = requireOne(setupFiles, /(?:Trajectory|trajectory-desktop)\.exe$/i, "Trajectory app inside Windows setup");
    runChecked(setupSidecar, ["--help"], "Windows setup sidecar --help");
    await assertLaunches(setupLauncher, [], "Windows setup payload launch smoke");
  } else if (process.env.CI) {
    throw new Error("7z is required for CI Windows setup package smoke");
  } else {
    manifest.push("Windows setup archive test=skipped locally because 7z is unavailable");
  }

  manifest.push(`msi=${relative(msi)}`, `setup=${relative(setup)}`);
  await assertLaunches(msiLauncher, [], "Windows MSI app launch smoke");
}

async function expandWindowsSetupPayload(setupExtract) {
  const setupFiles = await listFiles(setupExtract);
  if (
    setupFiles.some((file) => /trajectory-client.*\.exe$/i.test(file)) &&
    setupFiles.some((file) => /(?:Trajectory|trajectory-desktop)\.exe$/i.test(file))
  ) {
    return setupFiles;
  }

  const nestedArchives = setupFiles.filter((file) => file.toLowerCase().endsWith(".7z"));
  if (nestedArchives.length === 0) {
    return setupFiles;
  }

  const payloadExtract = path.join(artifactDir, "setup-payload-extract");
  await mkdir(payloadExtract, { recursive: true });
  for (const archive of nestedArchives) {
    runChecked("7z", ["x", "-y", `-o${payloadExtract}`, archive], `extract nested Windows setup payload ${path.basename(archive)}`);
  }
  return [...setupFiles, ...(await listFiles(payloadExtract))];
}

async function listFiles(root) {
  const output = [];
  async function visit(current) {
    const entries = await readdir(current, { withFileTypes: true });
    for (const entry of entries) {
      const child = path.join(current, entry.name);
      if (entry.isDirectory()) {
        await visit(child);
      } else if (entry.isFile()) {
        output.push(child);
      }
    }
  }
  await visit(root);
  return output;
}

function requireOne(files, pattern, label) {
  const matches = files.filter((file) => pattern.test(file));
  if (matches.length !== 1) {
    throw new Error(`${label}: expected one match for ${pattern}, found ${matches.length}`);
  }
  requireReadable(matches[0], label);
  return matches[0];
}

function findOptional(files, pattern, label) {
  const matches = files.filter((file) => pattern.test(file));
  if (matches.length > 1) {
    throw new Error(`${label}: expected zero or one match for ${pattern}, found ${matches.length}`);
  }
  if (matches.length === 0) {
    return null;
  }
  requireReadable(matches[0], label);
  return matches[0];
}

function requireReadable(file, label) {
  accessSync(file, constants.R_OK);
  manifest.push(`${label}=${relative(file)}`);
}

function requireExecutable(file, label) {
  accessSync(file, constants.R_OK);
  try {
    chmodSync(file, 0o755);
  } catch {
    // Windows ACLs and mounted package files may ignore chmod.
  }
  manifest.push(`${label}=${relative(file)}`);
}

function runChecked(command, args, label, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd ?? repoRoot,
    encoding: "utf8",
    timeout: options.timeoutMs ?? 30_000,
    windowsHide: true,
  });
  writeCommandLog(label, result);
  if (result.status !== 0) {
    throw new Error(`${label} failed with status ${result.status}`);
  }
}

function packageField(deb, field) {
  const result = spawnSync("dpkg-deb", ["-f", deb, field], {
    cwd: repoRoot,
    encoding: "utf8",
    timeout: 10_000,
    windowsHide: true,
  });
  writeCommandLog(`read Linux deb ${field}`, result);
  return result.status === 0 ? result.stdout.trim() : "";
}

async function assertLaunches(command, args, label, extraEnv = {}) {
  const backendFile = path.join(artifactDir, `${safeName(label)}-backend-ready.txt`);
  const pageFile = path.join(artifactDir, `${safeName(label)}-page-ready.txt`);
  const frontendFile = path.join(artifactDir, `${safeName(label)}-frontend-ready.txt`);
  const stateFile = path.join(artifactDir, `${safeName(label)}-state-ready.txt`);
  const liveEnv = await prepareDesktopLiveSmoke(label);
  const liveFile = liveEnv.TRAJECTORY_DESKTOP_SMOKE_LIVE_FILE;
  const readyFiles = [backendFile, pageFile, frontendFile, stateFile];
  if (liveFile) readyFiles.push(liveFile);
  const child = spawn(command, args, {
    cwd: artifactDir,
    env: {
      ...process.env,
      TRAJECTORY_DESKTOP_SMOKE: "1",
      TRAJECTORY_DESKTOP_SMOKE_BACKEND_FILE: backendFile,
      TRAJECTORY_DESKTOP_SMOKE_PAGE_FILE: pageFile,
      TRAJECTORY_DESKTOP_SMOKE_READY_FILE: frontendFile,
      TRAJECTORY_DESKTOP_SMOKE_STATE_FILE: stateFile,
      NO_AT_BRIDGE: "1",
      WEBKIT_DISABLE_COMPOSITING_MODE: "1",
      ...liveEnv,
      ...extraEnv,
    },
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
    detached: process.platform !== "win32",
  });
  let stdout = "";
  let stderr = "";
  let launchError = null;
  child.stdout.on("data", (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on("data", (chunk) => {
    stderr += chunk.toString();
  });
  child.once("error", (error) => {
    launchError = error.message;
  });

  const ready = await waitForReadyFiles(
    child,
    readyFiles,
    liveFile ? 90_000 : 45_000,
    () => launchError,
  );
  if (!ready.ok) {
    await writeFile(path.join(artifactDir, `${safeName(label)}.log`), stdout + stderr);
    throw new Error(`${label} did not prove packaged app/frontend/backend readiness: ${ready.reason}`);
  }
  stopProcess(child);
  await writeFile(path.join(artifactDir, `${safeName(label)}.log`), stdout + stderr);
  manifest.push(`${label}=backend, page, frontend IPC, and state ready`);
  if (liveFile) {
    manifest.push(`${label}=live HTTP/SOCKS proxy smoke and shutdown ready`);
  }
}

async function prepareDesktopLiveSmoke(label) {
  let domain = process.env.TRAJECTORY_DESKTOP_SMOKE_DOMAIN?.trim();
  let accessKey = process.env.TRAJECTORY_DESKTOP_SMOKE_ACCESS_KEY?.trim();
  let resolverOverride = null;
  if (process.env.TRAJECTORY_DESKTOP_SMOKE_LOCAL_SERVER === "1" || !domain || !accessKey) {
    const local = await ensureDesktopLocalServer();
    domain = local.domain;
    accessKey = local.accessKey;
    resolverOverride = local.resolver;
  }
  if (!domain || !accessKey) {
    if (process.env.TRAJECTORY_DESKTOP_SMOKE_REQUIRE_LIVE === "1") {
      throw new Error(
        "desktop live proxy smoke requires domain/access key secrets or TRAJECTORY_DESKTOP_SMOKE_LOCAL_SERVER=1",
      );
    }
    if (!desktopLiveSmokeSkipNoted) {
      manifest.push("desktop live proxy smoke=skipped because domain/access key secrets are absent");
      desktopLiveSmokeSkipNoted = true;
    }
    return {};
  }

  const configDir = path.join(artifactDir, `${safeName(label)}-live-config`);
  await mkdir(configDir, { recursive: true });
  const profile = desktopLiveProfile(domain, resolverOverride);
  const origin = process.env.TRAJECTORY_DESKTOP_SMOKE_LOCAL_SERVER === "1"
    ? await ensureDesktopLocalOrigin()
    : null;
  await writeFile(
    path.join(configDir, "profiles.json"),
    `${JSON.stringify({ selectedProfileId: profile.id, profiles: [profile] }, null, 2)}\n`,
  );
  manifest.push(`${label}=desktop live proxy smoke configured`);
  return {
    TRAJECTORY_DESKTOP_CONFIG_DIR: configDir,
    TRAJECTORY_DESKTOP_SMOKE_ACCESS_KEY: accessKey,
    TRAJECTORY_DESKTOP_SMOKE_LIVE_FILE: path.join(artifactDir, `${safeName(label)}-live-proxy.txt`),
    TRAJECTORY_DESKTOP_SMOKE_FETCH_URL: origin?.url ?? process.env.TRAJECTORY_DESKTOP_SMOKE_FETCH_URL?.trim() ?? "http://example.com/",
    ...(origin ? { TRAJECTORY_DESKTOP_SMOKE_EXPECT_BODY: origin.marker } : {}),
  };
}

async function ensureDesktopLocalOrigin() {
  if (desktopLocalOrigin) return desktopLocalOrigin;
  const port = Number(process.env.TRAJECTORY_DESKTOP_SMOKE_LOCAL_ORIGIN_PORT || await pickTcpPort());
  const marker = `trajectory-desktop-smoke-${process.pid}-${Date.now()}`;
  const server = http.createServer((request, response) => {
    response.writeHead(200, {
      "content-type": "text/plain",
      "cache-control": "no-store",
    });
    response.end(`${marker}\n${request.url ?? ""}\n`);
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", resolve);
  });
  desktopLocalOrigin = {
    server,
    marker,
    url: `http://127.0.0.1:${port}/trajectory-smoke.txt`,
  };
  await writeFile(
    path.join(artifactDir, "desktop-local-origin-server.txt"),
    `desktop local origin ready on ${desktopLocalOrigin.url}\nmarker=${marker}\n`,
  );
  return desktopLocalOrigin;
}

async function ensureDesktopLocalServer() {
  if (desktopLocalServer) return desktopLocalServer;
  const domain = process.env.TRAJECTORY_DESKTOP_SMOKE_LOCAL_DOMAIN?.trim() || "t.desktop-smoke";
  const dnsPort = Number(process.env.TRAJECTORY_DESKTOP_SMOKE_LOCAL_DNS_PORT || await pickDualPort());
  const workDir = path.join(process.env.RUNNER_TEMP ?? artifactDir, `trajectory-desktop-live-${process.pid}`);
  await mkdir(workDir, { recursive: true });

  runChecked(
    "cargo",
    ["build", "--release", "-p", "trajectory-cli", "--bin", "trajectory-server", "--bin", "trajectory-admin"],
    "build desktop local live smoke server",
    { timeoutMs: 300_000 },
  );

  const clientDb = path.join(workDir, "clients.json");
  const admin = path.join(repoRoot, "target", "release", binaryName("trajectory-admin"));
  const server = path.join(repoRoot, "target", "release", binaryName("trajectory-server"));
  const keyResult = spawnSync(
    admin,
    ["create-client", "--client-db", clientDb, "--label", "desktop-smoke", "--format", "key"],
    { cwd: repoRoot, encoding: "utf8", timeout: 30_000, windowsHide: true },
  );
  writeCommandLog("create desktop local live smoke client", keyResult);
  if (keyResult.status !== 0) {
    throw new Error(`create desktop local live smoke client failed with status ${keyResult.status}`);
  }
  const accessKey = keyResult.stdout.trim();
  if (!accessKey.startsWith("traj1_")) {
    throw new Error("desktop local live smoke client key was not generated");
  }

  let stdout = "";
  let stderr = "";
  const child = spawn(
    server,
    [
      "--domain",
      domain,
      "--client-db",
      clientDb,
      "--bind",
      "127.0.0.1",
      "--dns-listen-port",
      String(dnsPort),
      "--target-address",
      "socks5-direct",
    ],
    { cwd: repoRoot, stdio: ["ignore", "pipe", "pipe"], windowsHide: true },
  );
  child.stdout.on("data", (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on("data", (chunk) => {
    stderr += chunk.toString();
  });
  await sleep(1000);
  if (child.exitCode !== null) {
    await writeFile(path.join(artifactDir, "desktop-local-live-server.log"), stdout + stderr);
    throw new Error(`desktop local live smoke server exited with status ${child.exitCode}`);
  }

  desktopLocalServer = {
    process: child,
    domain,
    accessKey,
    resolver: `127.0.0.1:${dnsPort}`,
  };
  await writeFile(
    path.join(artifactDir, "desktop-local-live-server.txt"),
    `desktop local live smoke server ready on 127.0.0.1:${dnsPort}\n`,
  );
  return desktopLocalServer;
}

async function stopDesktopLocalServer() {
  const child = desktopLocalServer?.process;
  desktopLocalServer = null;
  if (!child || child.exitCode !== null || child.signalCode !== null) {
    return;
  }
  child.kill();
  await Promise.race([
    new Promise((resolve) => child.once("exit", resolve)),
    sleep(2000),
  ]);
}

async function stopDesktopLocalOrigin() {
  const server = desktopLocalOrigin?.server;
  desktopLocalOrigin = null;
  if (!server) return;
  await new Promise((resolve) => server.close(resolve));
}

function desktopLiveProfile(domain, resolverOverride = null) {
  const resolvers = (resolverOverride || process.env.TRAJECTORY_DESKTOP_SMOKE_RESOLVERS || "1.1.1.1:53,1.0.0.1:53,8.8.8.8:53,8.8.4.4:53")
    .split(/[\s,]+/)
    .map((item) => item.trim())
    .filter(Boolean);
  const resolverCohortSize = numberEnv("TRAJECTORY_DESKTOP_SMOKE_RESOLVER_COHORT_SIZE");
  return {
    id: "smoke-live",
    name: "Live package smoke",
    domain,
    resolvers,
    resolverFile: null,
    resolverSocksProxy: resolverOverride ? null : process.env.TRAJECTORY_DESKTOP_SMOKE_RESOLVER_SOCKS_PROXY?.trim() || null,
    resolverTransport: process.env.TRAJECTORY_DESKTOP_SMOKE_RESOLVER_TRANSPORT?.trim() || "auto",
    transportMode: process.env.TRAJECTORY_DESKTOP_SMOKE_MODE?.trim() || "velocity",
    socks: {
      host: "127.0.0.1",
      port: numberEnv("TRAJECTORY_DESKTOP_SMOKE_SOCKS_PORT") ?? 7000,
      enabled: true,
    },
    http: {
      host: "127.0.0.1",
      port: numberEnv("TRAJECTORY_DESKTOP_SMOKE_HTTP_PORT") ?? 7001,
      enabled: true,
    },
    dnsMaxPayload: numberEnv("TRAJECTORY_DESKTOP_SMOKE_DNS_MAX_PAYLOAD") ?? 1232,
    resolverCohortSize,
    resolverAdmissionMin: numberEnv("TRAJECTORY_DESKTOP_SMOKE_RESOLVER_ADMISSION_MIN") ?? 1,
    pollIntervalMs: numberEnv("TRAJECTORY_DESKTOP_SMOKE_POLL_INTERVAL_MS") ?? 25,
    allowLanWithoutAuth: false,
    admissionReport: resolverOverride ? false : true,
  };
}

function numberEnv(name) {
  const value = process.env[name]?.trim();
  if (!value) return undefined;
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return parsed;
}

function binaryName(base) {
  return process.platform === "win32" ? `${base}.exe` : base;
}

async function pickUdpPort() {
  return await new Promise((resolve, reject) => {
    const socket = dgram.createSocket("udp4");
    socket.once("error", reject);
    socket.bind(0, "127.0.0.1", () => {
      const address = socket.address();
      socket.close(() => resolve(address.port));
    });
  });
}

async function pickDualPort() {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const candidate = await reserveTcpPort();
    if (await udpPortAvailable(candidate.port)) {
      await candidate.close();
      return candidate.port;
    }
    await candidate.close();
  }
  throw new Error("could not find a free TCP/UDP port");
}

async function reserveTcpPort() {
  return await new Promise((resolve, reject) => {
    const server = http.createServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      resolve({
        port: address.port,
        close: () => new Promise((closeResolve) => server.close(closeResolve)),
      });
    });
  });
}

async function udpPortAvailable(port) {
  return await new Promise((resolve) => {
    const socket = dgram.createSocket("udp4");
    socket.once("error", () => {
      socket.close();
      resolve(false);
    });
    socket.bind(port, "127.0.0.1", () => {
      socket.close(() => resolve(true));
    });
  });
}

async function pickTcpPort() {
  return await new Promise((resolve, reject) => {
    const server = http.createServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => resolve(address.port));
    });
  });
}

async function sleep(ms) {
  await new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForReadyFiles(child, readyFiles, ms, launchError) {
  const deadline = Date.now() + ms;
  while (Date.now() < deadline) {
    const missing = readyFiles.filter((file) => !existsSync(file));
    if (missing.length === 0) {
      return { ok: true };
    }
    const error = launchError();
    if (error) {
      return { ok: false, reason: `launch error: ${error}` };
    }
    if (child.exitCode !== null || child.signalCode !== null) {
      return {
        ok: false,
        reason: `exited early with code=${child.exitCode ?? ""} signal=${child.signalCode ?? ""}`,
      };
    }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  const missing = readyFiles.filter((file) => !existsSync(file));
  return { ok: false, reason: `timed out waiting for ${missing.join(", ")}` };
}

function stopProcess(child) {
  if (!child.pid) return;
  if (process.platform === "win32") {
    spawnSync("taskkill", ["/pid", String(child.pid), "/T", "/F"], { stdio: "ignore" });
    return;
  }
  try {
    process.kill(-child.pid, "SIGTERM");
  } catch {
    child.kill("SIGTERM");
  }
  spawnSync("pkill", ["-TERM", "-P", String(child.pid)], { stdio: "ignore" });
}

function commandExists(command) {
  const checker = process.platform === "win32" ? "where" : "command";
  const args = process.platform === "win32" ? [command] : ["-v", command];
  const result = spawnSync(checker, args, {
    shell: process.platform !== "win32",
    stdio: "ignore",
  });
  return result.status === 0;
}

function writeCommandLog(label, result) {
  const body = [
    `command=${label}`,
    `status=${result.status}`,
    `signal=${result.signal ?? ""}`,
    result.stdout ?? "",
    result.stderr ?? "",
  ].join("\n");
  writeFileSync(path.join(artifactDir, `${safeName(label)}.log`), body);
  manifest.push(`${label}=status ${result.status}`);
}

function shellQuote(value) {
  return `'${String(value).replaceAll("'", "'\\''")}'`;
}

function isExecutable(file) {
  try {
    accessSync(file, constants.X_OK);
    return true;
  } catch {
    return process.platform === "win32" && file.toLowerCase().endsWith(".exe");
  }
}

async function readText(file) {
  return readFile(file, "utf8");
}

function relative(file) {
  return path.relative(repoRoot, file);
}

function safeName(value) {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
