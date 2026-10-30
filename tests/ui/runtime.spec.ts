import { test, expect } from "@playwright/test";

test("simulated runtime UI requires manifest consent, separates verified CPU and supports three languages", async ({
  page,
}) => {
  await page.addInitScript(() => {
    const w = window as unknown as Record<string, unknown>;
    w.isTauri = true;
    w.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener: () => {} };
    let installed = false;
    let job: Record<string, unknown> | null = null;
    w.installs = 0;
    w.__TAURI_INTERNALS__ = {
      metadata: {
        currentWindow: { label: "main" },
        currentWebview: { label: "main" },
      },
      transformCallback: () => 1,
      invoke: async (cmd: string, args: Record<string, unknown>) => {
        if (cmd.startsWith("plugin:event|")) return 1;
        if (cmd === "search_projects")
          return { version: 2, healthy: [], issues: [], index_warning: null };
        if (cmd === "job_history") return job ? [job] : [];
        if (cmd === "runtime_view")
          return {
            version: 1,
            consent_token: "pinned",
            download_bytes: 28000000,
            disk_mib: 512,
            install_state: installed
              ? "installed_unverified"
              : "approval_required",
            manifest: {
              version: "1.0.0",
              weights_license: "CC0-1.0",
              sha256: "a".repeat(64),
            },
            backends: [{ id: "onnx_cpu", status: "installed_unverified" }],
          };
        if (cmd === "prepare_runtime") {
          if (args.approved !== true || args.token !== "pinned")
            throw { code: "download_consent" };
          w.installs = Number(w.installs) + 1;
          job = {
            id: "test-job",
            project_id: "",
            provider: "runtime-install-v1",
            status: "running",
            progress: 25,
            error: null,
          };
          return { ...job };
        }
        if (cmd === "job_status") {
          if (job) {
            installed = true;
            job = { ...job, status: "completed", progress: 100 };
          }
          return job;
        }
        if (cmd === "runtime_result")
          return { backend: "onnx_cpu", output: [2.25, 5.5] };
        throw { code: "error" };
      },
    };
  });
  await page.goto("/");
  const prepare = page.getByRole("button", {
    name: "로컬 AI 준비하기",
    exact: true,
  });
  await expect(prepare).toBeDisabled();
  expect(
    await page.evaluate(
      () => (window as unknown as { installs: number }).installs,
    ),
  ).toBe(0);
  await page
    .getByRole("checkbox", { name: "이 다운로드와 설치에 동의합니다." })
    .check();
  await prepare.click();
  await expect(
    page.getByText("이번 실행에서 CPU 연산을 확인했습니다."),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "CPU 연산 검사", exact: true }),
  ).toBeEnabled();
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  for (const locale of ["ja", "en", "ko"]) {
    await page.getByRole("combobox").nth(1).selectOption(locale);
    await expect(page.getByText(/\[2\.25,\s*5\.5\]/)).toBeVisible();
  }
  await expect(page.getByRole("alert")).toHaveCount(0);
});
