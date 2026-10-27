import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  CharacterProject,
  ProjectList,
  RecoveryPreview,
  RevisionReport,
} from "./contracts";

export function RecoveryPanel({
  list,
  project,
  t,
  expert,
  disabled,
  recovered,
  report,
}: {
  list: ProjectList;
  project: CharacterProject | null;
  t: (key: string) => string;
  expert: boolean;
  disabled: boolean;
  recovered: (p: CharacterProject) => Promise<void>;
  report: (e: unknown) => void;
}) {
  const [id, setId] = useState("");
  const [history, setHistory] = useState<RevisionReport[]>([]);
  const [preview, setPreview] = useState<RecoveryPreview | null>(null);
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  async function act(fn: () => Promise<void>) {
    setBusy(true);
    try {
      await fn();
    } catch (e) {
      setPreview(null);
      setConfirmed(false);
      report(e);
    } finally {
      setBusy(false);
    }
  }
  async function inspect(value: string) {
    setId(value);
    setHistory([]);
    setPreview(null);
    setConfirmed(false);
    setHistory(await invoke("project_history", { id: value }));
  }
  function close() {
    setId("");
    setHistory([]);
    setPreview(null);
    setConfirmed(false);
  }
  return (
    <section aria-label={t("recovery")}>
      <h2>{t("recovery")}</h2>
      {list.index_warning && (
        <p role="status">
          {t("index_unavailable")}
          {expert && <code>{list.index_warning}</code>}
        </p>
      )}
      {list.issues.map((issue) => (
        <div key={issue.id} className="error">
          <p>{t(issue.status)}</p>
          <small>
            {t("projectId")}: {issue.id}
          </small>
          {expert && <code>{issue.error}</code>}
          <button
            disabled={disabled || busy}
            onClick={() => void act(() => inspect(issue.id))}
          >
            {t("diagnose")}
          </button>
        </div>
      ))}
      {project && (
        <button
          className="secondary"
          disabled={disabled || busy}
          onClick={() => void act(() => inspect(project.id))}
        >
          {t("history")}
        </button>
      )}
      {id && (
        <div>
          <h3>{t("history")}</h3>
          <p>{t("historyHint")}</p>
          {history.length === 0 && <p>{t("unrecoverable")}</p>}
          <ul>
            {history.map((r) => (
              <li key={r.revision}>
                {t("revision")} {r.revision} ·{" "}
                {r.project?.name ?? t(r.error ?? "error")}
                {r.project && (
                  <button
                    disabled={disabled || busy}
                    onClick={() =>
                      void act(async () => {
                        setConfirmed(false);
                        setPreview(
                          await invoke("preview_recovery", {
                            id,
                            revision: r.revision,
                          }),
                        );
                      })
                    }
                  >
                    {t("previewRecovery")} {r.revision}
                  </button>
                )}
              </li>
            ))}
          </ul>
          {preview && (
            <div aria-label={t("previewRecovery")}>
              <h3>{t("previewRecovery")}</h3>
              <p>
                {t("revision")}: {preview.selected.revision} →{" "}
                {preview.latest_revision + 1}
              </p>
              <p>
                {t("name")}: {preview.latest?.name ?? t("latest_unreadable")} →{" "}
                {preview.selected.name}
              </p>
              <p>
                {t("dimension")}: {preview.selected.dimension} · {t("target")}:{" "}
                {preview.selected.target_profile}
              </p>
              <p>
                {t("sourceCount")}: {preview.selected.sources.length}
              </p>
              {preview.selected.sources.map((s) => (
                <p key={s.sha256}>
                  {s.width} × {s.height} · {s.bytes} bytes
                  {expert && <code>{s.sha256}</code>}
                </p>
              ))}
              <p>
                {t("changedFields")}:{" "}
                {preview.changed_fields
                  .map((f) => t(`field_${f}`))
                  .join(", ") || t("noChanges")}
              </p>
              <p>{t("recoveryPreserves")}</p>
              <p>{t("recoveryExcludes")}</p>
              <label>
                <input
                  type="checkbox"
                  checked={confirmed}
                  disabled={busy || disabled}
                  onChange={(e) => setConfirmed(e.target.checked)}
                />
                {t("confirmRecovery")}
              </label>
              <button
                disabled={!confirmed || busy || disabled}
                onClick={() =>
                  void act(async () => {
                    const p = await invoke<CharacterProject>(
                      "recover_project",
                      {
                        id,
                        revision: preview.selected.revision,
                        token: preview.token,
                      },
                    );
                    close();
                    await recovered(p);
                  })
                }
              >
                {t("restore")}
              </button>
            </div>
          )}
          <button className="secondary" disabled={busy} onClick={close}>
            {t("cancelRecovery")}
          </button>
        </div>
      )}
    </section>
  );
}
