import { spawnSync } from "node:child_process";
import { mkdir, readdir, rm, copyFile, writeFile, readFile } from "node:fs/promises";
import { constants, existsSync } from "node:fs";
import { accessSync } from "node:fs";
import path from "node:path";

const desktopDir = path.resolve(import.meta.dirname, "..");
const repoRoot = path.resolve(desktopDir, "..", "..");
const tauriDir = path.join(desktopDir, "src-tauri");
const releaseDir = path.join(tauriDir, "target", "release");
const bundleRoot = path.join(releaseDir, "bundle");
const portableRoot = path.join(bundleRoot, "windows", "portable");
const packageJson = JSON.parse(
  await readText(path.join(desktopDir, "package.json")),
);
const version = packageJson.version;
const zipName = `Trajectory_${version}_x64_portable.zip`;
const portableDir = path.join(portableRoot, `Trajectory_${version}_x64_portable`);
const zipPath = path.join(portableRoot, zipName);

if (process.platform !== "win32") {
  throw new Error("Windows portable packaging must run on a Windows host");
}

await rm(portableRoot, { recursive: true, force: true });
await mkdir(portableDir, { recursive: true });

const launcher = await findLauncher();
const stagedSidecar = path.join(tauriDir, "bin", "trajectory-client-x86_64-pc-windows-msvc.exe");
requireFile(launcher, "Tauri release launcher");
requireFile(stagedSidecar, "staged Windows trajectory-client sidecar");

await copyFile(launcher, path.join(portableDir, path.basename(launcher)));
await copyFile(stagedSidecar, path.join(portableDir, "trajectory-client-x86_64-pc-windows-msvc.exe"));
await copyIfExists(path.join(releaseDir, "resources"), path.join(portableDir, "resources"));
if (existsSync(path.join(repoRoot, "LICENSE"))) {
  await copyFile(path.join(repoRoot, "LICENSE"), path.join(portableDir, "LICENSE"));
}
await writeFile(
  path.join(portableDir, "README-portable.txt"),
  [
    "Trajectory Windows portable build",
    "",
    "Run Trajectory.exe or trajectory-desktop.exe from this folder.",
    "Keep the bundled trajectory-client executable beside the app executable.",
    "No installer is required.",
    "",
  ].join("\r\n"),
  "utf8",
);

await rm(zipPath, { force: true });
runChecked("powershell.exe", [
  "-NoProfile",
  "-ExecutionPolicy",
  "Bypass",
  "-Command",
  `Compress-Archive -Path '${escapePowerShell(path.join(portableDir, "*"))}' -DestinationPath '${escapePowerShell(zipPath)}' -Force`,
], "create Windows portable zip");

await rm(portableDir, { recursive: true, force: true });
console.log(`created ${zipPath}`);

async function findLauncher() {
  const entries = await readdir(releaseDir, { withFileTypes: true });
  const launchers = entries
    .filter((entry) => entry.isFile())
    .map((entry) => path.join(releaseDir, entry.name))
    .filter((file) => /(?:Trajectory|trajectory-desktop)\.exe$/i.test(path.basename(file)))
    .filter((file) => !/trajectory-client/i.test(path.basename(file)))
    .sort((left, right) => path.basename(left).localeCompare(path.basename(right)));
  if (launchers.length === 0) {
    throw new Error(`could not find Tauri launcher executable in ${releaseDir}`);
  }
  return launchers[0];
}

async function copyIfExists(source, destination) {
  if (!existsSync(source)) return;
  await rm(destination, { recursive: true, force: true });
  const result = spawnSync("robocopy.exe", [source, destination, "/E"], {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });
  if (result.status === null || result.status >= 8) {
    throw new Error(`copy ${source} to ${destination} failed: ${result.stderr || result.stdout}`);
  }
}

function requireFile(file, label) {
  accessSync(file, constants.R_OK);
}

function runChecked(command, args, label) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: "utf8",
    timeout: 120_000,
    windowsHide: true,
  });
  if (result.status !== 0) {
    throw new Error(`${label} failed with exit ${result.status}: ${result.stderr || result.stdout}`);
  }
}

async function readText(file) {
  return readFile(file, "utf8");
}

function escapePowerShell(value) {
  return value.replaceAll("'", "''");
}
