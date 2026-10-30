// Real Windows Tauri + isolated synthetic data. No mock IPC or user artwork.
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import net from "node:net";
import { spawn, execFileSync } from "node:child_process";
import { chromium, expect } from "@playwright/test";
const root = process.cwd();
const data = fs.mkdtempSync(path.join(root, ".tools", "dev03-native-"));
const env = {
  ...process.env,
  VTUBER_DATA_DIR: data,
  WEBVIEW2_USER_DATA_FOLDER: path.join(data, "webview"),
};
const seeds = ["groups-merged.png", "groups.psd"].map((file) =>
  JSON.parse(
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
        path.join(root, "tests/fixtures/source", file),
      ],
      { env, encoding: "utf8", windowsHide: true },
    ),
  ),
);
const originals = seeds.map((seed) => ({
  manifest: fs.readFileSync(
    path.join(data, "projects", seed.id, "00000002.json"),
  ),
  blob: fs.readFileSync(
    path.join(
      data,
      "projects",
      seed.id,
      "sources",
      `${seed.sources[0].sha256}.${seed.sources[0].format}`,
    ),
  ),
}));
// Use previously downloaded hash-pinned wheels. The real UI still requires consent
// and performs its own hash, install, quarantined load and publication steps.
const lock = JSON.parse(fs.readFileSync("workers/runtime-lock.json", "utf8"));
fs.mkdirSync(path.join(data, "runtime/downloads"), { recursive: true });
for (const file of lock.files)
  fs.copyFileSync(
    path.join(process.env.VTUBER_RUNTIME_DIR, "downloads", file.filename),
    path.join(data, "runtime/downloads", file.filename),
  );
let app, browser, page;
async function launch() {
  const server = net.createServer();
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const port = server.address().port;
  await new Promise((resolve) => server.close(resolve));
  env.WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = `--remote-debugging-port=${port}`;
  app = spawn(
    path.join(root, "src-tauri/target/release/vtuber-desktop.exe"),
    [],
    { env, cwd: root, windowsHide: true, stdio: "ignore" },
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
      { timeout: 20000 },
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
  if (browser) {
    await browser.close().catch(() => {});
    browser = null;
  }
  if (app && app.exitCode === null) {
    execFileSync("taskkill", ["/PID", String(app.pid), "/F"], {
      windowsHide: true,
      stdio: "ignore",
    });
    await new Promise((resolve) => app.once("exit", resolve));
  }
}
async function invoke(command, args = {}) {
  const result = await page.evaluate(
    async ([cmd, a]) => {
      try {
        return {
          ok: true,
          value: await window.__TAURI_INTERNALS__.invoke(cmd, a),
        };
      } catch (error) {
        return { ok: false, error };
      }
    },
    [command, args],
  );
  if (!result.ok) throw result.error;
  return result.value;
}
async function terminal(id) {
  let job;
  await expect
    .poll(
      async () => {
        job = await invoke("job_status");
        return job?.id === id && !["running", "queued"].includes(job.status);
      },
      { timeout: 180000, intervals: [100, 200, 500] },
    )
    .toBe(true);
  return job;
}
function unchanged() {
  for (let i = 0; i < seeds.length; i++) {
    const p = seeds[i];
    expect(
      fs.readFileSync(path.join(data, "projects", p.id, "00000002.json")),
    ).toEqual(originals[i].manifest);
    const bytes = fs.readFileSync(
      path.join(
        data,
        "projects",
        p.id,
        "sources",
        `${p.sources[0].sha256}.${p.sources[0].format}`,
      ),
    );
    expect(bytes).toEqual(originals[i].blob);
    expect(crypto.createHash("sha256").update(bytes).digest("hex")).toBe(
      p.sources[0].sha256,
    );
  }
}
const evidence = {
  method: "actual Windows Tauri IPC and rendered UI",
  gpu: null,
  probe: null,
  results: [],
  checks: [],
  not_run: [
    "GPU tensor execution",
    "OS-disconnected network / complete native traffic audit",
    "trained character analysis",
  ],
};
try {
  await launch();
  const view = await invoke("runtime_view");
  expect(view.install_state).toBe("approval_required");
  await expect(
    page.getByRole("button", { name: "로컬 AI 준비하기", exact: true }),
  ).toBeDisabled();
  await expect(
    invoke("prepare_runtime", { approved: false, token: view.consent_token }),
  ).rejects.toMatchObject({ code: "download_consent" });
  await page
    .getByRole("checkbox", { name: "이 다운로드와 설치에 동의합니다." })
    .check();
  await page
    .getByRole("button", { name: "로컬 AI 준비하기", exact: true })
    .click();
  let job = await invoke("job_status");
  expect((await terminal(job.id)).status).toBe("completed");
  evidence.checks.push(
    "native consent and real installation from verified local wheel cache",
  );
  evidence.gpu = await invoke("gpu_report");
  await expect(
    page.getByRole("button", { name: "CPU 연산 검사", exact: true }),
  ).toBeEnabled();
  await page
    .getByRole("button", { name: "CPU 연산 검사", exact: true })
    .click();
  job = await invoke("job_status");
  expect((await terminal(job.id)).status).toBe("completed");
  evidence.probe = await invoke("runtime_result", { id: job.id });
  for (const seed of seeds) {
    await page.getByLabel("저장된 프로젝트 열기").selectOption(seed.id);
    await page
      .getByRole("button", { name: "로컬 시험 연산 실행", exact: true })
      .click();
    job = await invoke("job_status");
    expect((await terminal(job.id)).status).toBe("completed");
    const result = await invoke("runtime_result", { id: job.id });
    expect(result.output).toEqual([2.25, 5.5]);
    expect(result.source_hash).toBe(seed.sources[0].sha256);
    evidence.results.push(result);
    unchanged();
  }
  const cancelled = await invoke("start_inference", { id: seeds[0].id });
  await invoke("cancel_job", { id: cancelled.id });
  expect((await terminal(cancelled.id)).status).toBe("cancelled");
  await expect(
    invoke("runtime_result", { id: cancelled.id }),
  ).rejects.toMatchObject({ code: "artifact_unverified" });
  evidence.checks.push(
    "actual process cancellation; no completed artifact exposed",
  );
  const interrupted = await invoke("start_inference", { id: seeds[1].id });
  await kill();
  await launch();
  const history = await invoke("job_history");
  expect(history.find((j) => j.id === interrupted.id).status).toBe(
    "interrupted",
  );
  expect((await invoke("runtime_view")).install_state).toBe(
    "installed_unverified",
  );
  unchanged();
  evidence.checks.push(
    "app kill/restart restores interrupted job without auto-resume",
  );
  const activeDir = path.join(data, "runtime/active");
  const activeFile = fs
    .readdirSync(activeDir)
    .filter((x) => x.endsWith(".json"))
    .sort()
    .at(-1);
  const active = JSON.parse(
    fs.readFileSync(path.join(activeDir, activeFile), "utf8"),
  );
  const modelFile = path.join(
    data,
    "runtime/environments",
    active.generation,
    "model.onnx",
  );
  const modelBytes = fs.readFileSync(modelFile);
  fs.writeFileSync(modelFile, Buffer.from("corrupted synthetic test model"));
  try {
    job = await invoke("start_inference", { id: seeds[0].id });
    const failed = await terminal(job.id);
    expect(failed.status).toBe("failed");
    expect(failed.error).toBe("model_hash");
  } finally {
    fs.writeFileSync(modelFile, modelBytes);
  }
  job = await invoke("start_inference", { id: seeds[0].id });
  expect((await terminal(job.id)).status).toBe("completed");
  unchanged();
  evidence.checks.push(
    "model tampering blocks real inference; restored fixture can run again",
  );
  const beforeActive = fs.readFileSync(path.join(activeDir, activeFile));
  const token = (await invoke("runtime_view")).consent_token;
  const cancelledInstall = await invoke("prepare_runtime", {
    approved: true,
    token,
  });
  await expect
    .poll(
      async () => {
        const j = await invoke("job_status");
        return (
          j.id === cancelledInstall.id &&
          j.status === "running" &&
          j.progress >= 65
        );
      },
      { timeout: 60000, intervals: [50, 100] },
    )
    .toBe(true);
  await invoke("cancel_job", { id: cancelledInstall.id });
  expect((await terminal(cancelledInstall.id)).status).toBe("cancelled");
  expect(fs.readFileSync(path.join(activeDir, activeFile))).toEqual(
    beforeActive,
  );
  expect(fs.readdirSync(activeDir).filter((x) => x.endsWith(".json"))).toEqual([
    activeFile,
  ]);
  const killedInstall = await invoke("prepare_runtime", {
    approved: true,
    token,
  });
  await expect
    .poll(
      async () => {
        const j = await invoke("job_status");
        return (
          j.id === killedInstall.id &&
          j.status === "running" &&
          j.progress >= 65
        );
      },
      { timeout: 60000, intervals: [50, 100] },
    )
    .toBe(true);
  await kill();
  await launch();
  expect(
    (await invoke("job_history")).find((j) => j.id === killedInstall.id).status,
  ).toBe("interrupted");
  expect(fs.readFileSync(path.join(activeDir, activeFile))).toEqual(
    beforeActive,
  );
  expect(fs.readdirSync(activeDir).filter((x) => x.endsWith(".json"))).toEqual([
    activeFile,
  ]);
  job = await invoke("start_inference", { id: seeds[0].id });
  expect((await terminal(job.id)).status).toBe("completed");
  unchanged();
  evidence.checks.push(
    "cancel and app kill during installation retain old active generation; incomplete generations are not active; retry calculation succeeds",
  );
  for (const locale of ["ja", "en", "ko"]) {
    await page.getByRole("combobox").nth(1).selectOption(locale);
    expect(await page.locator("html").getAttribute("lang")).toBe(locale);
  }
  await page.getByLabel("저장된 프로젝트 열기").selectOption(seeds[0].id);
  await page
    .getByRole("button", { name: "로컬 시험 연산 실행", exact: true })
    .click();
  job = await invoke("job_status");
  expect((await terminal(job.id)).status).toBe("completed");
  await expect(
    page.getByText("이번 실행에서 CPU 연산을 확인했습니다."),
  ).toBeVisible();
  await page.getByRole("combobox").first().selectOption("expert");
  await page.getByText("백엔드 및 모델 진단", { exact: true }).click();
  await expect(
    page.getByRole("button", { name: "모델 내보내기" }),
  ).toBeDisabled();
  await page.screenshot({
    path: "artifacts/dev-03-native-runtime.png",
    fullPage: true,
  });
  evidence.checks.push(
    "three locales; diagnostic-only result; export remains blocked; PNG/PSD source and revision hashes unchanged",
  );
  fs.writeFileSync(
    "artifacts/dev-03-native-runtime.json",
    JSON.stringify(evidence, null, 2),
  );
  console.log(
    "PASS native local CPU runtime, consent, install, probe, PNG/PSD provenance, cancel, restart, tamper/retry, 3 locales and original preservation. Fixture: " +
      data,
  );
} finally {
  await kill();
}
