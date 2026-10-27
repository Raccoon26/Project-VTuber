import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  profiles,
  type CharacterProject,
  type Dimension,
  type GpuReport,
  type Job,
  type ProjectList,
} from "./contracts";
import { message, type Locale } from "./i18n";
import "./style.css";
import { RecoveryPanel } from "./RecoveryPanel";

function App() {
  const [locale, setLocale] = useState<Locale>("ko");
  const t = (key: string) => message(locale, key);
  const [expert, setExpert] = useState(false);
  const [name, setName] = useState("");
  const [dimension, setDimension] = useState<Dimension>("2d");
  const [target, setTarget] = useState("vbridger-vts");
  const [project, setProject] = useState<CharacterProject | null>(null);
  const [saved, setSaved] = useState<CharacterProject[]>([]);
  const [listing, setListing] = useState<ProjectList>({
    version: 2,
    healthy: [],
    issues: [],
    index_warning: null,
  });
  const [jobs, setJobs] = useState<Job[]>([]);
  const [search, setSearch] = useState("");
  const [job, setJob] = useState<Job | null>(null);
  const [gpu, setGpu] = useState<GpuReport | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const native = isTauri();
  const running = job?.status === "running" || job?.status === "queued";
  function report(e: unknown) {
    setError(
      typeof e === "object" && e !== null && "code" in e
        ? String(e.code)
        : "error",
    );
  }
  async function action(fn: () => Promise<void>) {
    setBusy(true);
    setError("");
    try {
      await fn();
    } catch (e) {
      report(e);
    } finally {
      setBusy(false);
    }
  }
  const refresh = async () => {
    const result = await invoke<ProjectList>("search_projects", {
      query: search,
    });
    setListing(result);
    setSaved(result.healthy);
    setJobs(await invoke<Job[]>("job_history"));
  };
  const select = (p: CharacterProject) => {
    setProject(p);
    setName(p.name);
    setDimension(p.dimension);
    setTarget(p.target_profile);
  };
  useEffect(() => {
    document.documentElement.lang = locale;
    document.documentElement.dir = "ltr";
  }, [locale]);
  useEffect(() => {
    if (native) {
      void refresh().catch(report);
      void invoke<Job | null>("job_status").then(setJob).catch(report);
    }
  }, [native]);
  useEffect(() => {
    if (!running) return;
    let active = true;
    const timer = setInterval(() => {
      void invoke<Job | null>("job_status")
        .then((j) => {
          if (active) setJob(j);
        })
        .catch((e) => {
          if (active) report(e);
        });
    }, 150);
    return () => {
      active = false;
      clearInterval(timer);
    };
  }, [running]);
  useEffect(() => {
    if (native && !running)
      void invoke<Job[]>("job_history").then(setJobs).catch(report);
  }, [native, running]);
  const blocked = !native || busy || !!running;
  return (
    <div className="shell">
      <header>
        <div className="brand">
          <span className="logo" aria-hidden="true">
            V
          </span>
          <div>
            <strong>PROJECT VTUBER AI</strong>
            <small>{t("foundation")}</small>
          </div>
        </div>
        <div className="settings">
          <label>
            {t("mode")}
            <select
              value={expert ? "expert" : "beginner"}
              onChange={(e) => setExpert(e.target.value === "expert")}
            >
              <option value="beginner">{t("beginner")}</option>
              <option value="expert">{t("expert")}</option>
            </select>
          </label>
          <label>
            {t("language")}
            <select
              aria-label={t("language")}
              value={locale}
              onChange={(e) => setLocale(e.target.value as Locale)}
            >
              <option value="ko">한국어</option>
              <option value="ja">日本語</option>
              <option value="en">English</option>
            </select>
          </label>
        </div>
      </header>
      <main>
        <div className="intro">
          <div>
            <p className="eyebrow">LOCAL CREATION STUDIO</p>
            <h1>{t("title")}</h1>
            <p>{t("subtitle")}</p>
          </div>
          <span className="version">00 / FOUNDATION</span>
        </div>
        <div className="notice">{t("notice")}</div>
        {!native && <div className="notice browser">{t("browser")}</div>}
        {error && (
          <div role="alert" className="error">
            {t(error)}
            {expert && <code>{error}</code>}
          </div>
        )}
        <div className="columns">
          <div>
            <section>
              <h2>{t("step1")}</h2>
              <label>
                {t("search")}
                <input
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
              </label>
              <button
                className="secondary"
                disabled={blocked}
                onClick={() => void action(refresh)}
              >
                {t("refresh")}
              </button>
              <label>
                {t("open")}
                <select
                  disabled={blocked}
                  value={project?.id ?? ""}
                  onChange={(e) => {
                    const id = e.target.value;
                    if (id)
                      void action(async () => {
                        select(await invoke("open_project", { id }));
                        setJob(null);
                      });
                  }}
                >
                  <option value="">{t("choose")}</option>
                  {saved
                    .filter((p) =>
                      p.name
                        .toLocaleLowerCase()
                        .includes(search.toLocaleLowerCase()),
                    )
                    .map((p) => (
                      <option key={p.id} value={p.id}>
                        {p.name}
                      </option>
                    ))}
                </select>
              </label>
              <label>
                {t("name")}
                <input
                  value={name}
                  maxLength={120}
                  onChange={(e) => setName(e.target.value)}
                  disabled={busy || running}
                />
              </label>
              <div className="row">
                <label>
                  {t("dimension")}
                  <select
                    value={dimension}
                    disabled={busy || running}
                    onChange={(e) => {
                      const d = e.target.value as Dimension;
                      setDimension(d);
                      setTarget(d === "2d" ? "vbridger-vts" : "vseeface");
                    }}
                  >
                    <option value="2d">2D</option>
                    <option value="3d">3D</option>
                  </select>
                </label>
                <label>
                  {t("target")}
                  <select
                    value={target}
                    disabled={busy || running}
                    onChange={(e) => setTarget(e.target.value)}
                  >
                    {profiles
                      .filter((p) => p.dimension === dimension)
                      .map((p) => (
                        <option key={p.id} value={p.id}>
                          {p.label}
                        </option>
                      ))}
                  </select>
                </label>
              </div>
              <div className="buttons">
                <button
                  disabled={blocked || !name.trim()}
                  onClick={() =>
                    void action(async () => {
                      select(
                        await invoke("create_project", {
                          name,
                          dimension,
                          target,
                        }),
                      );
                      setJob(null);
                      await refresh();
                    })
                  }
                >
                  {t("create")}
                </button>
                <button
                  className="secondary"
                  disabled={blocked || !project || !name.trim()}
                  onClick={() =>
                    void action(async () => {
                      if (project) {
                        select(
                          await invoke("save_project", {
                            id: project.id,
                            revision: project.revision,
                            name,
                          }),
                        );
                        await refresh();
                      }
                    })
                  }
                >
                  {t("save")}
                </button>
              </div>
              {project && (
                <small>
                  {project.name} · {t("revision")} {project.revision} ·{" "}
                  {profiles.find((p) => p.id === project.target_profile)?.label}
                </small>
              )}
            </section>
            <RecoveryPanel
              list={listing}
              project={project}
              t={t}
              expert={expert}
              disabled={blocked}
              report={report}
              recovered={async (p) => {
                select(p);
                await refresh();
              }}
            />
            <section>
              <h2>{t("step2")}</h2>
              <div className="source">
                <span className="source-symbol" aria-hidden="true">
                  ◇
                </span>
                <div>
                  <strong>
                    {t(
                      project?.sources.length
                        ? "sourcePreserved"
                        : "sourceEmpty",
                    )}
                  </strong>
                  {project?.sources.map((s) => (
                    <p key={s.sha256}>
                      {s.width} × {s.height} · {(s.bytes / 1024).toFixed(1)} KiB
                      · PNG{expert && <code>SHA-256 {s.sha256}</code>}
                    </p>
                  ))}
                </div>
              </div>
              <p className="muted">{t("sourceHint")}</p>
              <button
                className="secondary"
                disabled={blocked || !project || !!project.sources.length}
                onClick={() =>
                  void action(async () => {
                    if (project) {
                      const p = await invoke<CharacterProject | null>(
                        "import_source",
                        { id: project.id, revision: project.revision },
                      );
                      if (p) {
                        select(p);
                        await refresh();
                      }
                    }
                  })
                }
              >
                {t("import")}
              </button>
            </section>
            <section>
              <h2>{t("step3")}</h2>
              <p role="status">{t(job?.status ?? "idle")}</p>
              {job?.error && (
                <p role="alert" className="error">
                  {t(job.error)}
                </p>
              )}
              {jobs.length > 0 && (
                <details>
                  <summary>{t("jobHistory")}</summary>
                  <ul>
                    {jobs.map((j) => (
                      <li key={j.id}>
                        {t(j.status)} · {j.progress}%
                        {expert && <code>{j.id}</code>}
                        {[
                          "interrupted",
                          "failed",
                          "cancelled",
                          "recovery_required",
                        ].includes(j.status) && <p>{t("retryHint")}</p>}
                      </li>
                    ))}
                  </ul>
                </details>
              )}
              <progress
                max={100}
                value={job?.progress ?? 0}
                aria-label={t("running")}
              />
              <div className="buttons">
                <button
                  disabled={blocked || !project?.sources.length}
                  onClick={() =>
                    void action(async () => {
                      if (project)
                        setJob(await invoke("start_job", { id: project.id }));
                    })
                  }
                >
                  {t("start")}
                </button>
                <button
                  className="secondary"
                  disabled={!running || busy}
                  onClick={() =>
                    void action(async () => {
                      if (job) await invoke("cancel_job", { id: job.id });
                    })
                  }
                >
                  {t("cancel")}
                </button>
              </div>
            </section>
          </div>
          <aside>
            <section>
              <h2>{t("step4")}</h2>
              <h3>{t("gates")}</h3>
              <ul className="gates">
                {["A", "B", "C", "D", "E", "F"].map((id) => (
                  <li key={id}>
                    <span>
                      <b>{id}</b> {t(id)}
                    </span>
                    <span className="badge">{t("unverified")}</span>
                  </li>
                ))}
              </ul>
              <p className="muted">{t("exportBlocked")}</p>
              <button disabled>{t("export")}</button>
            </section>
            <section>
              <h2>{t("gpu")}</h2>
              {gpu?.devices.map((d) => (
                <div key={d.name}>
                  <strong>{d.name}</strong>
                  <p>
                    {d.vram_mib} MiB · {t("driver")} {d.driver}
                  </p>
                </div>
              ))}
              {gpu?.detection === "unknown" && <p>{t("unknown")}</p>}
              <p className="muted">{t("inference")}</p>
              <button
                className="secondary"
                disabled={!native || busy}
                onClick={() =>
                  void action(async () => setGpu(await invoke("gpu_report")))
                }
              >
                {t("detect")}
              </button>
            </section>
            {expert && project && (
              <section>
                <h2>{t("details")}</h2>
                <pre>{JSON.stringify(project, null, 2)}</pre>
              </section>
            )}
          </aside>
        </div>
        {busy && <p role="status">{t("busy")}</p>}
      </main>
      <footer>{t("offline")}</footer>
    </div>
  );
}
createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
