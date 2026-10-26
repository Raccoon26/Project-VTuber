// Opt-in developer-only check against a REAL running Tauri WebView on loopback.
// Run with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223.
// Seed only the synthetic fixture first; no production endpoint exposes raw IO.
import { chromium, expect } from "@playwright/test";
const browser = await chromium.connectOverCDP("http://127.0.0.1:9223");
try {
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
  const pages = browser.contexts().flatMap((c) => c.pages());
  const page = pages.find((p) => p.url().includes("tauri.localhost"));
  if (!page)
    throw new Error(`Native page missing: ${pages.map((p) => p.url())}`);
  await page.reload();
  await expect(
    page.getByRole("heading", { name: "캐릭터 제작실" }),
  ).toBeVisible();
  await expect(
    page.getByText("브라우저 화면 미리보기입니다.", { exact: false }),
  ).toHaveCount(0);
  await page.getByLabel("프로젝트 이름").fill("Native IPC 한글 日本語");
  await page
    .getByRole("button", { name: "프로젝트 만들기", exact: true })
    .click();
  await expect(
    page.getByText(/Native IPC 한글 日本語 · 저장 버전 1/),
  ).toBeVisible();
  await page.getByLabel("프로젝트 이름").fill("Native IPC saved");
  await page.getByRole("button", { name: "이름 저장", exact: true }).click();
  await expect(page.getByText(/Native IPC saved · 저장 버전 2/)).toBeVisible();
  await page.reload();
  await page
    .getByLabel("저장된 프로젝트 열기")
    .selectOption({ label: "Native IPC saved" });
  await expect(page.getByLabel("프로젝트 이름")).toHaveValue(
    "Native IPC saved",
  );
  await page
    .getByLabel("저장된 프로젝트 열기")
    .selectOption({ label: "DEV-00 synthetic native test" });
  await expect(page.getByText("원본 보존됨", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "더미 작업 실행" }).click();
  await expect(
    page.getByText("더미 작업 완료. 모델은 생성되지 않았습니다.", {
      exact: true,
    }),
  ).toBeVisible({ timeout: 15000 });
  await page.getByRole("button", { name: "더미 작업 실행" }).click();
  await page.getByRole("button", { name: "작업 취소", exact: true }).click();
  await expect(
    page.getByText("작업을 취소했습니다. 원본은 안전하게 보존됩니다.", {
      exact: true,
    }),
  ).toBeVisible();
  await page.getByRole("button", { name: "GPU 확인" }).click();
  await expect(
    page.getByText("NVIDIA GeForce RTX 2060", { exact: true }),
  ).toBeVisible({ timeout: 10000 });
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  await page.getByLabel("언어", { exact: true }).selectOption("ja");
  await expect(
    page.getByRole("heading", { name: "キャラクター工房" }),
  ).toBeVisible();
  await page.screenshot({ path: "artifacts/native-app.png", fullPage: true });
  console.log(
    "PASS real Tauri IPC: create/save/reload/open, fixture source, Python completion/cancel, GPU, export disabled, Japanese UI",
  );
} finally {
  await browser.close();
}
