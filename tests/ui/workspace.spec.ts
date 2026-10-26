import { test, expect } from "@playwright/test";
test("browser preview stays honest and supports three locales", async ({
  page,
}) => {
  await page.goto("/");
  await expect(
    page.getByRole("heading", { name: "캐릭터 제작실" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "프로젝트 만들기", exact: true }),
  ).toBeDisabled();
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  await page.getByLabel("언어", { exact: true }).selectOption("ja");
  await expect(
    page.getByRole("heading", { name: "キャラクター工房" }),
  ).toBeVisible();
  await page.getByLabel("言語", { exact: true }).selectOption("en");
  await page.getByLabel("Model type").selectOption("3d");
  await expect(page.getByLabel("Broadcast application")).toHaveValue(
    "vseeface",
  );
  await expect(
    page.getByRole("option", { name: "VBridger → VMC receiver" }),
  ).toBeAttached();
  await expect(page.getByText("Unverified", { exact: true })).toHaveCount(6);
  await page.setViewportSize({ width: 800, height: 650 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await page.screenshot({
    path: "artifacts/browser-preview.png",
    fullPage: true,
  });
});
