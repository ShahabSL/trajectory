import { chromium } from "@playwright/test";
import { spawn } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

const artifactDir = process.argv[2] ?? path.join(process.env.RUNNER_TEMP ?? "/tmp", "trajectory-desktop-ui");
const baseUrl = "http://127.0.0.1:1420";

await mkdir(artifactDir, { recursive: true });

const npm = process.platform === "win32" ? "npm.cmd" : "npm";
const server = spawn(npm, ["run", "dev", "--", "--host", "127.0.0.1", "--port", "1420", "--strictPort"], {
  cwd: path.resolve(import.meta.dirname, ".."),
  detached: process.platform !== "win32",
  stdio: ["ignore", "pipe", "pipe"],
});

let serverLog = "";
server.stdout.on("data", (chunk) => {
  serverLog += chunk.toString();
});
server.stderr.on("data", (chunk) => {
  serverLog += chunk.toString();
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
  await page.screenshot({ path: path.join(artifactDir, "desktop-status.png"), fullPage: true });

  await page.getByRole("button", { name: "Settings" }).click();
  await expectVisibleText(page, "Transport Knobs");
  await expectVisibleText(page, "Frontier");
  await expectVisibleText(page, "Experimental");
  await page.locator(".mode-card", { hasText: "Frontier" }).click();
  await page.locator('input[name="transport-mode"][value="frontier"]').waitFor({ state: "attached" });
  if (!(await page.locator('input[name="transport-mode"][value="frontier"]').isChecked())) {
    throw new Error("Frontier mode did not become selected");
  }
  await page.screenshot({ path: path.join(artifactDir, "desktop-frontier-selected.png"), fullPage: true });

  await page.setViewportSize({ width: 390, height: 844 });
  await page.getByRole("button", { name: "Status" }).click();
  await expectVisibleText(page, "Connection Readiness");
  await page.screenshot({ path: path.join(artifactDir, "desktop-mobile-status.png"), fullPage: true });

  await writeFile(path.join(artifactDir, "dom.txt"), await page.locator("body").innerText());
  await writeFile(path.join(artifactDir, "console.txt"), messages.join("\n"));
  await browser.close();

  const relevantErrors = messages.filter((line) => !line.includes("favicon"));
  if (relevantErrors.length > 0) {
    throw new Error(`desktop UI console errors:\n${relevantErrors.join("\n")}`);
  }
} finally {
  if (server.pid) {
    if (process.platform === "win32") {
      server.kill("SIGTERM");
    } else {
      try {
        process.kill(-server.pid, "SIGTERM");
      } catch {
        server.kill("SIGTERM");
      }
    }
  }
  await writeFile(path.join(artifactDir, "vite.log"), serverLog);
}

async function waitForHttp(url) {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 250));
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function expectVisibleText(page, text) {
  await page.getByText(text, { exact: false }).first().waitFor({ state: "visible" });
}
