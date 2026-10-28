import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { CharacterProject } from "./contracts";
interface Inspection {
  source_hash: string;
  format: string;
  width: number;
  height: number;
  bit_depth: number;
  color_mode: string;
  color_status: string;
  icc_hash: string | null;
  preview_width: number;
  preview_height: number;
  preview_url: string;
  warnings: string[];
  layers: {
    index: number;
    name: string;
    bounds: number[];
    visible: boolean;
    opacity: number;
    blend: string;
    clipping: boolean;
    mask: boolean;
    group: number;
    additional: string[];
  }[];
}
export function SourcePreview({
  project,
  expert,
  t,
  report,
}: {
  project: CharacterProject | null;
  expert: boolean;
  t: (key: string) => string;
  report: (e: unknown) => void;
}) {
  const [info, setInfo] = useState<Inspection | null>(null);
  const [loading, setLoading] = useState(false);
  useEffect(() => {
    let active = true;
    setInfo(null);
    setLoading(false);
    if (!project?.sources.length) return;
    setLoading(true);
    void invoke<Inspection>("source_preview", { id: project.id })
      .then((value) => {
        if (active) setInfo(value);
      })
      .catch((e) => {
        if (active) report(e);
      })
      .finally(() => {
        if (active) setLoading(false);
      });
    return () => {
      active = false;
    };
  }, [project?.id, project?.revision]);
  if (loading) return <p role="status">{t("previewLoading")}</p>;
  if (!info) return null;
  return (
    <div className="source-preview">
      <img
        src={info.preview_url}
        alt={t("originalPreview")}
        style={{
          width: "100%",
          maxWidth: "100%",
          maxHeight: 420,
          objectFit: "contain",
          background:
            "repeating-conic-gradient(#eee 0% 25%, #aaa 0% 50%) 0 / 20px 20px",
        }}
      />
      <p>
        {t("previewCopy")} · {info.preview_width} × {info.preview_height} /{" "}
        {t("originalSize")} {info.width} × {info.height}
      </p>
      {info.warnings.map((w) => (
        <p className="notice" key={w}>
          {t(w)}
        </p>
      ))}
      {expert && (
        <details>
          <summary>{t("sourceDetails")}</summary>
          <p>
            {info.format.toUpperCase()} · {info.bit_depth}-bit ·{" "}
            {info.color_mode} · {t(info.color_status)}
          </p>
          <code>
            SHA-256 {info.source_hash}
            <br />
            ICC {info.icc_hash ?? "—"}
          </code>
          <p>{t("layerInspectionOnly")}</p>
          <ol>
            {info.layers.map((l) => (
              <li key={l.index}>
                {l.name} · {l.visible ? t("layerVisible") : t("layerHidden")} ·{" "}
                {l.opacity}/255 · {l.blend} · [{l.bounds.join(", ")}]{" "}
                {l.mask ? "Mask" : ""} {l.clipping ? "Clipping" : ""}{" "}
                {l.group ? `Group ${l.group}` : ""}
              </li>
            ))}
          </ol>
        </details>
      )}
    </div>
  );
}
