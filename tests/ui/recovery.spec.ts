import { test, expect } from "@playwright/test";
import sample from "../../samples/character-project.json" with { type: "json" };

test("simulated IPC recovery requires explicit review, supports cancel and three locales", async ({
  page,
}) => {
  await page.addInitScript((fixture) => {
    const w = window as unknown as {
      isTauri: boolean;
      __TAURI_INTERNALS__: unknown;
      restores: number;
    };
    w.isTauri = true;
    Object.assign(w, {
      __TAURI_EVENT_PLUGIN_INTERNALS__: { unregisterListener: () => {} },
    });
    w.restores = 0;
    const p = { ...fixture, name: "Preserved fixture", revision: 1 };
    w.__TAURI_INTERNALS__ = {
      metadata: {
        currentWindow: { label: "main" },
        currentWebview: { label: "main" },
      },
      transformCallback: () => 1,
      invoke: async (cmd: string) => {
        if (cmd.startsWith("plugin:event|")) return 1;
        if (cmd === "runtime_view") return null;
        if (cmd === "job_status") return null;
        if (cmd === "job_history") return [];
        if (cmd === "search_projects" || cmd === "list_projects_v2")
          return {
            version: 2,
            healthy: w.restores ? [{ ...p, revision: 3 }] : [],
            issues: w.restores
              ? []
              : [
                  {
                    id: p.id,
                    status: "recoverable",
                    error: "invalid_project",
                    recoverable: [1],
                  },
                ],
            index_warning: null,
          };
        if (cmd === "project_history")
          return [
            { revision: 1, project: p, error: null },
            { revision: 2, project: null, error: "invalid_project" },
          ];
        if (cmd === "preview_recovery")
          return {
            selected: p,
            latest: null,
            latest_revision: 2,
            token: "test-token",
            changed_fields: ["latest_unreadable"],
          };
        if (cmd === "recover_project") {
          w.restores++;
          return { ...p, revision: 3 };
        }
        throw { code: "error" };
      },
    };
  }, sample);
  await page.goto("/");
  await page.getByRole("button", { name: "진단 및 저장 이력 확인" }).click();
  await page.getByRole("button", { name: "복구 미리보기 1" }).click();
  await expect(
    page.getByRole("button", { name: "새 복구 버전 만들기" }),
  ).toBeDisabled();
  await page.getByRole("button", { name: "복구 취소" }).click();
  expect(
    await page.evaluate(
      () => (window as unknown as { restores: number }).restores,
    ),
  ).toBe(0);
  await page.getByLabel("언어", { exact: true }).selectOption("ja");
  await page.getByRole("button", { name: "保存履歴を確認" }).click();
  await page.getByRole("button", { name: "復旧内容を確認 1" }).click();
  await expect(
    page.getByRole("button", { name: "復旧版を作成", exact: true }),
  ).toBeDisabled();
  await page.getByLabel("言語", { exact: true }).selectOption("en");
  await page
    .getByRole("checkbox", {
      name: "I reviewed the changes and want to create a restored version.",
    })
    .focus();
  await page.keyboard.press("Space");
  await page
    .getByRole("button", { name: "Create restored version", exact: true })
    .click();
  await expect(page.getByLabel("Project name", { exact: true })).toHaveValue(
    "Preserved fixture",
  );
  expect(
    await page.evaluate(
      () => (window as unknown as { restores: number }).restores,
    ),
  ).toBe(1);
  await expect(
    page.getByRole("button", { name: "Export model" }),
  ).toBeDisabled();
});
