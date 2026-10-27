// Real Windows Tauri IPC + OS process kill. Only freshly-created synthetic data.
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import net from "node:net";
import { spawn, execFileSync } from "node:child_process";
import { chromium, expect } from "@playwright/test";
const root = process.cwd();
const data = fs.mkdtempSync(path.join(root, ".tools", "dev01-native-"));
if (
  !data
    .toLowerCase()
    .startsWith(path.join(root, ".tools").toLowerCase() + path.sep)
)
  throw Error("Unsafe fixture root");
const env = {
  ...process.env,
  VTUBER_DATA_DIR: data,
  WEBVIEW2_USER_DATA_FOLDER: path.join(data, "webview"),
  WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: "--remote-debugging-port=9223",
  VTUBER_TEST_CDP_PORT: "9223",
};
const seed = JSON.parse(
  execFileSync(
    "cargo",
    [
      "run",
      "--manifest-path",
      "src-tauri/Cargo.toml",
      "--locked",
      "--example",
      "seed_fixture",
    ],
    { env, encoding: "utf8", windowsHide: true },
  ),
);
const dir = path.join(data, "projects", seed.id);
const originalManifest = fs.readFileSync(path.join(dir, "00000002.json"));
const blob = path.join(dir, "sources", `${seed.sources[0].sha256}.png`);
const original = fs.readFileSync(blob);
let app, browser, page;
async function launch() {
  const listener = net.createServer();
  await new Promise((resolve, reject) => {
    listener.once("error", reject);
    listener.listen(0, "127.0.0.1", resolve);
  });
  const port = listener.address().port;
  await new Promise((resolve, reject) =>
    listener.close((error) => (error ? reject(error) : resolve())),
  );
  env.WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = `--remote-debugging-port=${port}`;
  env.VTUBER_TEST_CDP_PORT = String(port);
  app = spawn(
    path.join(root, "src-tauri", "target", "release", "vtuber-desktop.exe"),
    [],
    {
      env,
      cwd: path.join(root, "src-tauri", "target", "release"),
      windowsHide: true,
      stdio: "ignore",
    },
  );
  await expect
    .poll(
      async () => {
        try {
          browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`, {
            timeout: 2000,
          });
          return true;
        } catch {
          return false;
        }
      },
      { timeout: 20000 },
    )
    .toBe(true);
  await expect
    .poll(
      () =>
        browser
          .contexts()
          .flatMap((c) => c.pages())
          .some((p) => p.url().includes("tauri.localhost")),
      { timeout: 15000 },
    )
    .toBe(true);
  page = browser
    .contexts()
    .flatMap((c) => c.pages())
    .find((p) => p.url().includes("tauri.localhost"));
  await expect(
    page.getByRole("heading", { name: "캐릭터 제작실" }),
  ).toBeVisible();
}
async function kill() {
  const child = app;
  if (child && child.exitCode === null) {
    const exited = new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(Error("App did not exit")), 10000);
      child.once("exit", () => {
        clearTimeout(timer);
        resolve();
      });
    });
    child.kill();
    await exited;
  }
  if (browser) await browser.close().catch(() => {});
  app = null;
  browser = null;
}
try {
  await launch();
  execFileSync(process.execPath, ["scripts/native-smoke.mjs"], {
    env,
    encoding: "utf8",
    stdio: "pipe",
    windowsHide: true,
  });
  await page.reload();
  await page
    .getByLabel("저장된 프로젝트 열기")
    .selectOption({ label: "DEV-00 synthetic native test" });
  await page
    .getByRole("button", { name: "더미 작업 실행", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "작업 취소", exact: true }),
  ).toBeEnabled();
  await kill();
  fs.writeFileSync(path.join(dir, "00000003.json"), "{broken fixture");
  fs.writeFileSync(
    path.join(data, "projects", "index.sqlite"),
    "corrupt index fixture",
  );
  await launch();
  await expect(
    page
      .getByText(
        "이전 작업이 비정상적으로 중단되었습니다. 완료가 확인되지 않았습니다.",
        { exact: true },
      )
      .first(),
  ).toBeVisible();
  await page
    .getByLabel("저장된 프로젝트 열기")
    .selectOption({ label: "Native IPC saved" });
  await expect(page.getByLabel("프로젝트 이름")).toHaveValue(
    "Native IPC saved",
  );
  await page.getByRole("button", { name: "진단 및 저장 이력 확인" }).click();
  await page
    .getByRole("button", { name: "복구 미리보기 2", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "새 복구 버전 만들기", exact: true }),
  ).toBeDisabled();
  await page.getByRole("button", { name: "복구 취소", exact: true }).click();
  expect(fs.existsSync(path.join(dir, "00000004.json"))).toBe(false);
  await page.getByRole("button", { name: "진단 및 저장 이력 확인" }).click();
  await page
    .getByRole("button", { name: "복구 미리보기 2", exact: true })
    .click();
  await page
    .getByRole("checkbox", {
      name: "변경 내용을 확인했으며 새 복구 버전을 만들겠습니다.",
    })
    .check();
  await page
    .getByRole("button", { name: "새 복구 버전 만들기", exact: true })
    .click();
  await expect(
    page.getByText(/DEV-00 synthetic native test · 저장 버전 4/),
  ).toBeVisible();
  expect(fs.readFileSync(path.join(dir, "00000002.json"))).toEqual(
    originalManifest,
  );
  expect(fs.readFileSync(blob)).toEqual(original);
  expect(
    crypto.createHash("sha256").update(fs.readFileSync(blob)).digest("hex"),
  ).toBe(seed.sources[0].sha256);
  expect(fs.readFileSync(path.join(dir, "00000003.json"), "utf8")).toBe(
    "{broken fixture",
  );
  await page.reload();
  await page
    .getByLabel("저장된 프로젝트 열기")
    .selectOption({ label: "DEV-00 synthetic native test" });
  await expect(page.getByText(/저장 버전 4/)).toBeVisible();
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  await page.screenshot({
    path: "artifacts/native-dev01-recovery.png",
    fullPage: true,
  });
  console.log(
    "PASS actual native DEV-00 regression, process kill/relaunch, interrupted job, corrupt project isolation, cancel/confirm recovery, preserved original/history, rebuild index, reopen revision 4. Fixture: " +
      data,
  );
} finally {
  await kill();
}
