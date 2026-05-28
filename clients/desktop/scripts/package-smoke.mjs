import { spawn, spawnSync } from "node:child_process";
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

if (process.platform === "linux") {
  await smokeLinux(files);
} else if (process.platform === "darwin") {
  await smokeMac(files);
} else if (process.platform === "win32") {
  await smokeWindows(files);
} else {
  throw new Error(`unsupported desktop package smoke platform: ${process.platform}`);
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
  requireOne(debFiles, /usr[/\\]bin[/\\]trajectory-client$/, "trajectory-client inside .deb");
  requireOne(debFiles, /usr[/\\]bin[/\\]trajectory-desktop$/, "trajectory-desktop inside .deb");

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
    requireOne(rpmFiles, /usr[/\\]bin[/\\]trajectory-client$/, "trajectory-client inside .rpm");
    requireOne(rpmFiles, /usr[/\\]bin[/\\]trajectory-desktop$/, "trajectory-desktop inside .rpm");
  } else if (process.env.CI) {
    throw new Error("rpm2cpio and cpio are required for CI Linux RPM package smoke");
  } else {
    manifest.push("rpm extraction=skipped locally because rpm2cpio/cpio are unavailable");
  }
  manifest.push(`deb=${relative(deb)}`, `rpm=${relative(rpm)}`, `appimage=${relative(appImage)}`);

  const launcher = commandExists("xvfb-run") ? "xvfb-run" : appRun;
  const args = commandExists("xvfb-run") ? ["-a", appRun] : [];
  await assertLaunches(launcher, args, "Linux AppDir launch smoke");
  const appImageLauncher = commandExists("xvfb-run") ? "xvfb-run" : appImage;
  const appImageArgs = commandExists("xvfb-run") ? ["-a", appImage] : [];
  await assertLaunches(appImageLauncher, appImageArgs, "Linux AppImage launch smoke", {
    APPIMAGE_EXTRACT_AND_RUN: "1",
  });
}

async function smokeMac(files) {
  const dmg = findOptional(files, new RegExp(`Trajectory_?${escapeRegex(version)}.*\\.dmg$`), "macOS .dmg");
  const appBundle = path.join(bundleRoot, "macos", "Trajectory.app");
  const macosDir = path.join(appBundle, "Contents", "MacOS");
  const executables = (await listFiles(macosDir)).filter(isExecutable);
  const sidecar = executables.find((file) => path.basename(file).startsWith("trajectory-client"));
  const launcher = executables.find((file) => !path.basename(file).startsWith("trajectory-client"));
  if (!sidecar) throw new Error("missing macOS bundled trajectory-client sidecar");
  if (!launcher) throw new Error("missing macOS app launcher executable");
  runChecked(sidecar, ["--help"], "macOS bundled sidecar --help");
  if (dmg) {
    runChecked("hdiutil", ["verify", dmg], "macOS dmg verification");
    manifest.push(`dmg=${relative(dmg)}`);
  } else {
    manifest.push("dmg=not emitted by this Tauri build");
  }
  manifest.push(`app=${relative(appBundle)}`);
  await assertLaunches(launcher, [], "macOS app bundle launch smoke");
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

  manifest.push(`msi=${relative(msi)}`, `setup=${relative(setup)}`);
  await assertLaunches(msiLauncher, [], "Windows MSI app launch smoke");
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
    timeout: 30_000,
    windowsHide: true,
  });
  writeCommandLog(label, result);
  if (result.status !== 0) {
    throw new Error(`${label} failed with status ${result.status}`);
  }
}

async function assertLaunches(command, args, label, extraEnv = {}) {
  const readyFile = path.join(artifactDir, `${safeName(label)}-frontend-ready.txt`);
  const stateFile = path.join(artifactDir, `${safeName(label)}-state-ready.txt`);
  const child = spawn(command, args, {
    cwd: repoRoot,
    env: {
      ...process.env,
      TRAJECTORY_DESKTOP_SMOKE: "1",
      TRAJECTORY_DESKTOP_SMOKE_READY_FILE: readyFile,
      TRAJECTORY_DESKTOP_SMOKE_STATE_FILE: stateFile,
      NO_AT_BRIDGE: "1",
      WEBKIT_DISABLE_COMPOSITING_MODE: "1",
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

  const ready = await waitForReadyFiles(child, [readyFile, stateFile], 30_000, () => launchError);
  if (!ready.ok) {
    await writeFile(path.join(artifactDir, `${safeName(label)}.log`), stdout + stderr);
    throw new Error(`${label} did not prove packaged frontend/backend readiness: ${ready.reason}`);
  }
  stopProcess(child);
  await writeFile(path.join(artifactDir, `${safeName(label)}.log`), stdout + stderr);
  manifest.push(`${label}=frontend and backend ready`);
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
