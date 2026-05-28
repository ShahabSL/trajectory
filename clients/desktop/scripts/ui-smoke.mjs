import { chromium } from "@playwright/test";
import { spawn, spawnSync } from "node:child_process";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import zlib from "node:zlib";

const args = process.argv.slice(2);
const usePreview = args.includes("--preview");
const artifactDir = args.find((arg) => arg !== "--preview") ?? path.join(process.env.RUNNER_TEMP ?? "/tmp", "trajectory-desktop-ui");
const baseUrl = usePreview ? "http://127.0.0.1:4173" : "http://127.0.0.1:1420";

await mkdir(artifactDir, { recursive: true });

const npm = "npm";
const serverCommand = usePreview ? "preview" : "dev";
const serverPort = usePreview ? "4173" : "1420";
const server = spawn(npm, ["run", serverCommand, "--", "--host", "127.0.0.1", "--port", serverPort, "--strictPort"], {
  cwd: path.resolve(import.meta.dirname, ".."),
  detached: process.platform !== "win32",
  shell: process.platform === "win32",
  stdio: ["ignore", "pipe", "pipe"],
  windowsHide: true,
});

let serverLog = "";
let serverExit = null;
server.stdout.on("data", (chunk) => {
  serverLog += chunk.toString();
});
server.stderr.on("data", (chunk) => {
  serverLog += chunk.toString();
});
server.once("exit", (code, signal) => {
  serverExit = { code, signal };
});
server.once("error", (error) => {
  serverLog += `\nfailed to start dev server: ${error.message}\n`;
});

try {
  await waitForHttp(baseUrl);

  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 } });
  const messages = [];
  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      messages.push(`${message.type()}: ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => {
    messages.push(`pageerror: ${error.message}`);
  });

  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.getByRole("heading", { name: "Local proxy" }).waitFor();
  await expectVisibleText(page, "Disconnected");
  await expectVisibleText(page, "Browser preview does not control a real trajectory-client process.");
  const statusShot = path.join(artifactDir, "desktop-status.png");
  await page.screenshot({ path: statusShot, fullPage: true });
  await assertPngVisual(statusShot, "desktop status");

  await page.getByRole("button", { name: "Settings" }).click();
  await expectVisibleText(page, "Transport Knobs");
  await expectVisibleText(page, "Frontier");
  await expectVisibleText(page, "Experimental");
  await page.locator(".mode-card", { hasText: "Frontier" }).click();
  await page.locator('input[name="transport-mode"][value="frontier"]').waitFor({ state: "attached" });
  if (!(await page.locator('input[name="transport-mode"][value="frontier"]').isChecked())) {
    throw new Error("Frontier mode did not become selected");
  }
  const frontierShot = path.join(artifactDir, "desktop-frontier-selected.png");
  await page.screenshot({ path: frontierShot, fullPage: true });
  await assertPngVisual(frontierShot, "desktop frontier selected");

  await page.getByRole("button", { name: "Profiles" }).click();
  await page.getByLabel("Tunnel domain").fill("t.preview-smoke.local");
  await page.getByLabel("Access key").fill("preview-smoke-access-key");
  await page.getByRole("button", { name: "Save" }).click();
  await page.getByRole("button", { name: "Status" }).click();
  const connectButton = page.getByRole("button", { name: "Connect" });
  await connectButton.waitFor({ state: "visible" });
  if (await connectButton.isDisabled()) {
    throw new Error("Connect button stayed disabled after profile smoke data was saved");
  }
  await connectButton.click();
  await expectVisibleText(page, "Starting");
  const connectedShot = path.join(artifactDir, "desktop-preview-starting.png");
  await page.screenshot({ path: connectedShot, fullPage: true });
  await assertPngVisual(connectedShot, "desktop preview starting");
  await page.getByRole("button", { name: "Disconnect" }).click();
  await expectVisibleText(page, "Disconnected");

  await page.setViewportSize({ width: 390, height: 844 });
  await page.getByRole("button", { name: "Status" }).click();
  await expectVisibleText(page, "Connection Readiness");
  const mobileShot = path.join(artifactDir, "desktop-mobile-status.png");
  await page.screenshot({ path: mobileShot, fullPage: true });
  await assertPngVisual(mobileShot, "desktop mobile status");

  await writeFile(path.join(artifactDir, "dom.txt"), await page.locator("body").innerText());
  await writeFile(path.join(artifactDir, "console.txt"), messages.join("\n"));
  await browser.close();

  const relevantErrors = messages.filter((line) => !line.includes("favicon"));
  if (relevantErrors.length > 0) {
    throw new Error(`desktop UI console errors:\n${relevantErrors.join("\n")}`);
  }
} finally {
  stopServer();
  await writeFile(path.join(artifactDir, "vite.log"), serverLog);
}

async function waitForHttp(url) {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    if (serverExit) {
      throw new Error(`desktop dev server exited before ${url} was ready: ${JSON.stringify(serverExit)}`);
    }
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 250));
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

function stopServer() {
  if (!server.pid) return;
  if (process.platform === "win32") {
    spawnSync("taskkill", ["/pid", String(server.pid), "/T", "/F"], { stdio: "ignore" });
    return;
  }
  try {
    process.kill(-server.pid, "SIGTERM");
  } catch {
    server.kill("SIGTERM");
  }
}

async function expectVisibleText(page, text) {
  await page.getByText(text, { exact: false }).first().waitFor({ state: "visible" });
}

async function assertPngVisual(file, label) {
  const image = decodePng(await readFile(file));
  const pixels = image.width * image.height;
  const step = Math.max(1, Math.floor(pixels / 80_000));
  let sampled = 0;
  let nonBackground = 0;
  let ink = 0;
  let edges = 0;
  let minLuma = 255;
  let maxLuma = 0;
  let previousLuma = null;
  const buckets = new Set();
  for (let index = 0; index < pixels; index += step) {
    const y = Math.floor(index / image.width);
    if (y < image.height * 0.02 || y > image.height * 0.98) continue;
    const offset = index * image.channels;
    const red = image.pixels[offset];
    const green = image.pixels[offset + 1];
    const blue = image.pixels[offset + 2];
    const luma = Math.trunc((red * 299 + green * 587 + blue * 114) / 1000);
    const chroma = Math.max(red, green, blue) - Math.min(red, green, blue);
    sampled += 1;
    minLuma = Math.min(minLuma, luma);
    maxLuma = Math.max(maxLuma, luma);
    if (luma < 242 || chroma > 8) nonBackground += 1;
    if (luma < 120) ink += 1;
    if (previousLuma !== null && Math.abs(luma - previousLuma) > 20) edges += 1;
    previousLuma = luma;
    buckets.add(`${red >> 5}:${green >> 5}:${blue >> 5}`);
  }
  const contrast = maxLuma - minLuma;
  const nonBackgroundRatio = nonBackground / Math.max(1, sampled);
  const inkRatio = ink / Math.max(1, sampled);
  const edgeRatio = edges / Math.max(1, sampled - 1);
  const report = [
    `label=${label}`,
    `width=${image.width}`,
    `height=${image.height}`,
    `sampled=${sampled}`,
    `non_background_ratio=${nonBackgroundRatio.toFixed(5)}`,
    `ink_ratio=${inkRatio.toFixed(5)}`,
    `edge_ratio=${edgeRatio.toFixed(5)}`,
    `contrast=${contrast}`,
    `color_buckets=${buckets.size}`,
  ].join("\n");
  await writeFile(file.replace(/\.png$/, ".visual.txt"), `${report}\n`);
  const failures = [];
  if (sampled < 1000) failures.push(`not enough sampled pixels (${sampled})`);
  if (contrast < 35) failures.push(`contrast too low (${contrast})`);
  if (nonBackgroundRatio < 0.02) failures.push(`non-background pixel ratio too low (${nonBackgroundRatio.toFixed(5)})`);
  if (inkRatio < 0.002) failures.push(`ink pixel ratio too low (${inkRatio.toFixed(5)})`);
  if (edgeRatio < 0.0008) failures.push(`edge ratio too low (${edgeRatio.toFixed(5)})`);
  if (buckets.size < 6) failures.push(`too few color buckets (${buckets.size})`);
  if (failures.length > 0) {
    throw new Error(`${label} screenshot failed visual smoke: ${failures.join("; ")}`);
  }
}

function decodePng(data) {
  const signature = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
  if (!data.subarray(0, 8).equals(signature)) {
    throw new Error("screenshot is not a PNG");
  }
  let offset = 8;
  let width = 0;
  let height = 0;
  let bitDepth = 0;
  let colorType = 0;
  let interlace = 0;
  const idat = [];
  while (offset + 12 <= data.length) {
    const length = data.readUInt32BE(offset);
    const kind = data.subarray(offset + 4, offset + 8).toString("ascii");
    const payload = data.subarray(offset + 8, offset + 8 + length);
    offset += 12 + length;
    if (kind === "IHDR") {
      width = payload.readUInt32BE(0);
      height = payload.readUInt32BE(4);
      bitDepth = payload[8];
      colorType = payload[9];
      interlace = payload[12];
    } else if (kind === "IDAT") {
      idat.push(payload);
    } else if (kind === "IEND") {
      break;
    }
  }
  if (!width || !height || bitDepth !== 8 || ![2, 6].includes(colorType) || interlace !== 0) {
    throw new Error(`unsupported PNG screenshot format: ${width}x${height} depth=${bitDepth} color=${colorType} interlace=${interlace}`);
  }
  const channels = colorType === 6 ? 4 : 3;
  const rowBytes = width * channels;
  const inflated = zlib.inflateSync(Buffer.concat(idat));
  const pixels = Buffer.alloc(width * height * channels);
  const previous = Buffer.alloc(rowBytes);
  let source = 0;
  let target = 0;
  for (let y = 0; y < height; y += 1) {
    const filter = inflated[source];
    source += 1;
    for (let x = 0; x < rowBytes; x += 1) {
      const value = inflated[source + x];
      const left = x >= channels ? pixels[target + x - channels] : 0;
      const up = previous[x];
      const upLeft = x >= channels ? previous[x - channels] : 0;
      let decoded = value;
      if (filter === 1) decoded += left;
      else if (filter === 2) decoded += up;
      else if (filter === 3) decoded += Math.floor((left + up) / 2);
      else if (filter === 4) decoded += paeth(left, up, upLeft);
      else if (filter !== 0) throw new Error(`unsupported PNG filter ${filter}`);
      pixels[target + x] = decoded & 0xff;
    }
    pixels.copy(previous, 0, target, target + rowBytes);
    source += rowBytes;
    target += rowBytes;
  }
  return { width, height, channels, pixels };
}

function paeth(left, up, upLeft) {
  const estimate = left + up - upLeft;
  const leftDistance = Math.abs(estimate - left);
  const upDistance = Math.abs(estimate - up);
  const upLeftDistance = Math.abs(estimate - upLeft);
  if (leftDistance <= upDistance && leftDistance <= upLeftDistance) return left;
  return upDistance <= upLeftDistance ? up : upLeft;
}
