// Actual native UI responsiveness; only the synthetic benchmark PNG is imported by a Rust helper.
import fs from "node:fs";
import path from "node:path";
import net from "node:net";
import { spawn, execFileSync } from "node:child_process";
import { chromium, expect } from "@playwright/test";
const root = process.cwd();
const data = fs.mkdtempSync(path.join(root, ".tools", "dev02-large-"));
const env = {
  ...process.env,
  VTUBER_DATA_DIR: data,
  WEBVIEW2_USER_DATA_FOLDER: path.join(data, "webview"),
};
const project = JSON.parse(
  execFileSync(
    "cargo",
    [
      "run",
      "--manifest-path",
      "src-tauri/Cargo.toml",
      "--locked",
      "--example",
      "seed_image",
      "--",
      path.join(root, "artifacts/dev-02-performance/8000x5000-8.png"),
    ],
    { env, encoding: "utf8", windowsHide: true },
  ),
);
const socket = net.createServer();
await new Promise((r) => socket.listen(0, "127.0.0.1", r));
const port = socket.address().port;
await new Promise((r) => socket.close(r));
env.WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = `--remote-debugging-port=${port}`;
const app = spawn(
  path.join(root, "src-tauri/target/release/vtuber-desktop.exe"),
  [],
  { env, windowsHide: true, stdio: "ignore" },
);
let browser;
try {
  await expect
    .poll(
      async () => {
        try {
          browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`, {
            timeout: 1000,
          });
          return true;
        } catch {
          return false;
        }
      },
      { timeout: 30000 },
    )
    .toBe(true);
  let page;
  await expect
    .poll(
      () => {
        page = browser
          .contexts()
          .flatMap((c) => c.pages())
          .find((p) => p.url().includes("tauri.localhost"));
        return !!page;
      },
      { timeout: 20000 },
    )
    .toBe(true);
  await page.evaluate(() => {
    window.__dev02Frames = [];
    let prior = performance.now();
    window.__dev02Sampling = true;
    function tick(now) {
      window.__dev02Frames.push(now - prior);
      prior = now;
      if (window.__dev02Sampling) requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);
  });
  const started = Date.now();
  await page.getByLabel("저장된 프로젝트 열기").selectOption(project.id);
  await expect(
    page.getByRole("img", { name: "원본 이미지의 표시용 사본" }),
  ).toBeVisible();
  const elapsed = Date.now() - started;
  const measurement = await page.evaluate(async (id) => {
    window.__dev02Sampling = false;
    const frames = window.__dev02Frames;
    const start = performance.now();
    const preview = await window.__TAURI_INTERNALS__.invoke("source_preview", {
      id,
    });
    return {
      frame_count: frames.length,
      max_frame_gap_ms: Math.max(0, ...frames),
      cached_preview_ms: performance.now() - start,
      original: [preview.width, preview.height],
      preview: [preview.preview_width, preview.preview_height],
      source_hash: preview.source_hash,
    };
  }, project.id);
  expect(measurement.original).toEqual([8000, 5000]);
  expect(measurement.preview[0]).toBeLessThanOrEqual(1024);
  expect(measurement.source_hash).toBe(project.sources[0].sha256);
  expect(measurement.frame_count).toBeGreaterThan(0);
  await page.screenshot({
    path: "artifacts/dev-02-native-large.png",
    fullPage: true,
  });
  fs.writeFileSync(
    "artifacts/dev-02-native-large.json",
    JSON.stringify(
      {
        scope:
          "actual release Windows/WebView/rAF while40M synthetic PNG preview, developer-only Rust input helper (not OS file-picker gesture)",
        elapsed_ms: elapsed,
        ...measurement,
      },
      null,
      2,
    ),
  );
  console.log("PASS actual native40M preview; " + JSON.stringify(measurement));
} finally {
  if (browser) await browser.close().catch(() => {});
  app.kill();
}
