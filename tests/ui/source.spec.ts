import { test, expect } from "@playwright/test";
import sample from "../../samples/character-project.json" with { type: "json" };
test("simulated source input automatically creates a project and separates beginner preview from layer details", async ({
  page,
}) => {
  await page.addInitScript((fixture) => {
    const w = window as unknown as {
      isTauri: boolean;
      __TAURI_INTERNALS__: unknown;
    };
    w.isTauri = true;
    Object.assign(w, {
      __TAURI_EVENT_PLUGIN_INTERNALS__: { unregisterListener: () => {} },
    });
    let p = { ...fixture, sources: [], name: "Character" } as Record<
      string,
      unknown
    >;
    w.__TAURI_INTERNALS__ = {
      metadata: {
        currentWindow: { label: "main" },
        currentWebview: { label: "main" },
      },
      transformCallback: () => 1,
      invoke: async (cmd: string) => {
        if (cmd.startsWith("plugin:event|")) return 1;
        if (cmd === "search_projects")
          return { version: 2, healthy: [], issues: [], index_warning: null };
        if (cmd === "job_history") return [];
        if (cmd === "runtime_view") return null;
        if (cmd === "job_status") return null;
        if (cmd === "create_project") return p;
        if (cmd === "import_source") {
          p = {
            ...p,
            schema_version: 2,
            revision: 2,
            sources: [
              {
                sha256: "a".repeat(64),
                bytes: 100,
                width: 8,
                height: 8,
                format: "psd",
                provenance: "original",
              },
            ],
          };
          return p;
        }
        if (cmd === "source_preview")
          return {
            source_hash: "a".repeat(64),
            format: "psd",
            width: 8,
            height: 8,
            bit_depth: 16,
            color_mode: "RGB",
            color_status: "srgb_assumed",
            icc_hash: null,
            preview_width: 8,
            preview_height: 8,
            preview_url:
              "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVQIHWP4z8DwHwAFgAI/ScLbtAAAAABJRU5ErkJggg==",
            warnings: ["psd_cached_composite_only"],
            layers: [
              {
                index: 0,
                name: "目・눈",
                bounds: [0, 0, 8, 8],
                visible: false,
                opacity: 128,
                blend: "mul ",
                clipping: true,
                mask: true,
                group: 0,
                additional: [],
              },
            ],
          };
        throw { code: "error" };
      },
    };
  }, sample);
  await page.goto("/");
  await page.getByRole("button", { name: "PNG / PSD 일러스트 선택" }).click();
  await expect(
    page.getByRole("img", { name: "원본 이미지의 표시용 사본" }),
  ).toBeVisible();
  await expect(
    page.getByText("원본 및 레이어 정보", { exact: true }),
  ).toHaveCount(0);
  await page.getByRole("combobox").first().selectOption("expert");
  await page.getByText("원본 및 레이어 정보", { exact: true }).click();
  await expect(page.getByText(/目・눈 · 숨김/)).toBeVisible();
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  for (const locale of ["ja", "en", "ko"]) {
    await page.getByRole("combobox").nth(1).selectOption(locale);
    await expect(page.locator("img")).toBeVisible();
  }
  await expect(page.getByRole("alert")).toHaveCount(0);
});
