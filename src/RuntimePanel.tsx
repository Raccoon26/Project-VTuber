import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { CharacterProject, Job } from "./contracts";

interface RuntimeView {
  version: 1;
  consent_token: string;
  download_bytes: number;
  disk_mib: number;
  install_state: string;
  manifest: { version: string; sha256: string; weights_license: string };
  backends: { id: string; status: string }[];
}
export function RuntimePanel({
  native,
  expert,
  project,
  job,
  started,
  report,
  t,
}: {
  native: boolean;
  expert: boolean;
  project: CharacterProject | null;
  job: Job | null;
  started: (job: Job) => void;
  report: (error: unknown) => void;
  t: (key: string) => string;
}) {
  const [view, setView] = useState<RuntimeView | null>(null);
  const [approved, setApproved] = useState(false);
  const [working, setWorking] = useState(false);
  const [result, setResult] = useState<Record<string, unknown> | null>(null);
  const output = Array.isArray(result?.output)
    ? result.output
    : result?.probe &&
        typeof result.probe === "object" &&
        "output" in result.probe
      ? result.probe.output
      : null;
  const running = job?.status === "running" || job?.status === "queued";
  const refresh = async () => {
    const v = await invoke<RuntimeView>("runtime_view");
    if (v?.version === 1) setView(v);
  };
  useEffect(() => {
    if (native) void refresh().catch(report);
  }, [native]);
  useEffect(() => {
    if (!native || running || !job || job.provider === "dummy") return;
    void refresh().catch(report);
    if (job.status === "completed") {
      void invoke<Record<string, unknown>>("runtime_result", { id: job.id })
        .then(setResult)
        .catch(report);
    }
  }, [native, job?.id, job?.status]);
  async function begin(command: string, args?: Record<string, unknown>) {
    setWorking(true);
    setResult(null);
    try {
      started(await invoke<Job>(command, args));
    } catch (e) {
      report(e);
    } finally {
      setWorking(false);
    }
  }
  const blocked = !native || working || !!running;
  return (
    <section aria-label={t("runtimeTitle")}>
      <h2>{t("runtimeTitle")}</h2>
      <p role="status">
        {t(
          result
            ? "runtimeVerified"
            : view?.install_state === "blocked"
              ? "runtimeBlocked"
              : view?.install_state === "installed_unverified"
                ? "runtimeInstalled"
                : "runtimeNeedsSetup",
        )}
      </p>
      <p>{t("runtimeDiagnosticOnly")}</p>
      {view && (
        <>
          <details open={view.install_state !== "installed_unverified"}>
            <summary>{t("runtimeSetupDetails")}</summary>
            <p>{t("runtimeSetupPurpose")}</p>
            <p>
              {(view.download_bytes / 1048576).toFixed(1)} MiB ·{" "}
              {t("runtimeDisk")}: {view.disk_mib} MiB · CPU / RAM 512 MiB
            </p>
            <p>{t("runtimeLicenseNotice")}</p>
            <p>{t("runtimeDownloadOrigin")}</p>
            <label>
              <input
                type="checkbox"
                checked={approved}
                disabled={blocked}
                onChange={(e) => setApproved(e.target.checked)}
              />
              {t("runtimeConsent")}
            </label>
            <button
              disabled={blocked || !approved}
              onClick={() =>
                void begin("prepare_runtime", {
                  approved,
                  token: view.consent_token,
                })
              }
            >
              {t("runtimePrepare")}
            </button>
          </details>
          <div className="buttons">
            {running && job?.provider !== "dummy" && (
              <button
                className="secondary"
                onClick={() =>
                  void invoke("cancel_job", { id: job.id }).catch(report)
                }
              >
                {t("cancel")}
              </button>
            )}
            <button
              className="secondary"
              disabled={
                blocked || view.install_state !== "installed_unverified"
              }
              onClick={() => void begin("probe_runtime")}
            >
              {t("runtimeProbe")}
            </button>
            <button
              disabled={
                blocked ||
                view.install_state !== "installed_unverified" ||
                !project?.sources.length
              }
              onClick={() => void begin("start_inference", { id: project?.id })}
            >
              {t("runtimeRun")}
            </button>
          </div>
          {result && (
            <p role="status">
              {t("runtimeResult")}: {JSON.stringify(output)} · CPU
            </p>
          )}
          {expert && (
            <details>
              <summary>{t("runtimeExpert")}</summary>
              <pre>
                {JSON.stringify(
                  { manifest: view.manifest, backends: view.backends, result },
                  null,
                  2,
                )}
              </pre>
            </details>
          )}
        </>
      )}
    </section>
  );
}
