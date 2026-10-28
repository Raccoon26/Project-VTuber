// Actual release app and native file dialog. Select only tests/fixtures/source/groups.psd.
import fs from "node:fs";
import path from "node:path";
import net from "node:net";
import crypto from "node:crypto";
import { spawn } from "node:child_process";
import { chromium, expect } from "@playwright/test";
const root = process.cwd();
const data = fs.mkdtempSync(path.join(root, ".tools", "dev02-native-"));
const server = net.createServer();
await new Promise((r) => server.listen(0, "127.0.0.1", r));
const port = server.address().port;
await new Promise((r) => server.close(r));
const app = spawn(
  path.join(root, "src-tauri/target/release/vtuber-desktop.exe"),
  [],
  {
    env: {
      ...process.env,
      VTUBER_DATA_DIR: data,
      WEBVIEW2_USER_DATA_FOLDER: path.join(data, "webview"),
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}`,
    },
    windowsHide: false,
    stdio: "ignore",
  },
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
  await page.getByRole("button", { name: /PNG.*선택/ }).click();
  fs.writeFileSync(
    "artifacts/dev-02-native-ready.json",
    JSON.stringify({ port, data, pid: app.pid }),
  );
  console.log(
    "READY: native dialog; select E:/Project-VTuber/tests/fixtures/source/groups.psd",
  );
  await expect(
    page.getByRole("img", { name: "원본 이미지의 표시용 사본" }),
  ).toBeVisible({ timeout: 180000 });
  const project = await page.evaluate(() =>
    window.__TAURI_INTERNALS__.invoke("list_projects_v2"),
  );
  const saved = project.healthy.find((p) => p.sources[0]?.format === "psd");
  if (!saved) throw Error("PSD was not imported");
  const original = fs.readFileSync("tests/fixtures/source/groups.psd");
  const blob = path.join(
    data,
    "projects",
    saved.id,
    "sources",
    `${saved.sources[0].sha256}.psd`,
  );
  expect(fs.readFileSync(blob)).toEqual(original);
  expect(crypto.createHash("sha256").update(original).digest("hex")).toBe(
    saved.sources[0].sha256,
  );
  await page.getByRole("combobox").first().selectOption("expert");
  await page.getByText("원본 및 레이어 정보", { exact: true }).click();
  await expect(page.getByText(/Eye ·/)).toBeVisible();
  for (const locale of ["en", "ja", "ko"]) {
    await page.getByRole("combobox").nth(1).selectOption(locale);
    await expect(page.locator("img")).toBeVisible();
  }
  const cached = await page.evaluate(async (id) => {
    const start = performance.now();
    await window.__TAURI_INTERNALS__.invoke("source_preview", { id });
    return performance.now() - start;
  }, saved.id);
  await page.screenshot({
    path: "artifacts/dev-02-native-input.png",
    fullPage: true,
  });
  await page.reload();
  await page.getByLabel("저장된 프로젝트 열기").selectOption(saved.id);
  await expect(page.locator("img")).toBeVisible();
  fs.writeFileSync(
    "artifacts/dev-02-native-input.json",
    JSON.stringify(
      {
        scope:
          "actual Windows release/native dialog/IPC/preview/reload/3locales/original hash",
        project: saved,
        cached_preview_ms: cached,
        original_preserved: true,
      },
      null,
      2,
    ),
  );
  console.log(
    "PASS native PSD dialog input, preview, layers, 3 locales, reopen and source hash",
  );
} finally {
  if (browser) await browser.close().catch(() => {});
  app.kill();
}
