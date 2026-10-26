export type GateStatus = "pass" | "fail" | "unverified" | "not_applicable";
export type Dimension = "2d" | "3d";
export interface SourceAsset {
  sha256: string;
  bytes: number;
  width: number;
  height: number;
  format: "png";
  provenance: "original";
}
export interface CharacterProject {
  schema_version: 1;
  id: string;
  revision: number;
  name: string;
  dimension: Dimension;
  target_profile: string;
  sources: SourceAsset[];
  gates: { id: string; status: GateStatus; evidence: string[] }[];
  master_status: "not_generated";
}
export interface Job {
  id: string;
  project_id: string;
  status: "running" | "completed" | "cancelled" | "failed";
  progress: number;
  provider: "dummy";
  error: string | null;
}
export interface GpuReport {
  detection: "detected" | "unknown";
  devices: { name: string; vram_mib: number; driver: string }[];
  inference: "unverified";
  dummy_cpu_available: boolean;
}
export interface ExportProfile {
  id: string;
  label: string;
  dimension: Dimension;
  format: string;
  implemented: false;
  runtimeValidation: "unverified";
  losses: string[];
  companionRequired: string[];
}
// Capability declarations are NOT implementations. Every exporter is disabled in DEV-00.
export const profiles: ExportProfile[] = [
  {
    id: "vtube-studio",
    label: "VTube Studio",
    dimension: "2d",
    format: "moc3 bundle",
    losses: ["native-editing"],
    companionRequired: ["Cubism export route"],
  },
  {
    id: "vbridger-vts",
    label: "VTube Studio + VBridger",
    dimension: "2d",
    format: "moc3 bundle + mappings",
    losses: ["custom-mapping"],
    companionRequired: ["VBridger", "VTube Studio"],
  },
  {
    id: "inochi-session",
    label: "Inochi Session",
    dimension: "2d",
    format: "INP",
    losses: ["cubism-features"],
    companionRequired: ["Inochi Session"],
  },
  {
    id: "prprlive",
    label: "PrprLive",
    dimension: "2d",
    format: "moc3 bundle (version unverified)",
    losses: ["advanced-parameters"],
    companionRequired: ["PrprLive"],
  },
  {
    id: "vseeface",
    label: "VSeeFace",
    dimension: "3d",
    format: "VRM 0.x / VSFAvatar",
    losses: ["vrm1", "custom-shaders"],
    companionRequired: ["VSeeFace"],
  },
  {
    id: "warudo",
    label: "Warudo",
    dimension: "3d",
    format: "VRM / .warudo",
    losses: ["custom-shaders"],
    companionRequired: ["Warudo"],
  },
  {
    id: "vnyan",
    label: "VNyan",
    dimension: "3d",
    format: "VRM 0.x / 1.0 / VSFAvatar",
    losses: ["custom-shaders"],
    companionRequired: ["VNyan"],
  },
  {
    id: "vrchat",
    label: "VRChat",
    dimension: "3d",
    format: "Unity SDK avatar",
    losses: ["platform-limits"],
    companionRequired: ["Unity", "VRChat SDK"],
  },
  {
    id: "vbridger-vmc",
    label: "VBridger → VMC receiver",
    dimension: "3d",
    format: "ARKit VRM + mapping",
    losses: ["receiver-mapping"],
    companionRequired: ["VBridger", "VMC receiver"],
  },
  {
    id: "glb",
    label: "GLB",
    dimension: "3d",
    format: "glTF 2.0",
    losses: ["vrm-expressions", "spring-bones"],
    companionRequired: [],
  },
].map((p) => ({
  ...p,
  dimension: p.dimension as Dimension,
  implemented: false,
  runtimeValidation: "unverified",
}));

export interface SemanticExpression {
  id: string;
  side: "left" | "right" | "center";
  range: [number, number];
  neutral: number;
  combinations: string[];
  bindings: { target: string; parameter: string; curve: [number, number][] }[];
}
export interface ArtifactRef {
  sha256: string;
  provenance: "original" | "generated" | "manual";
  parentHashes: string[];
  uncertainty: "observed" | "inferred" | "unknown";
}
export interface ModelManifest {
  id: string;
  version: string;
  sha256: string;
  origin: string;
  codeLicense: string;
  weightsLicense: string;
  redistribution: "approved" | "blocked" | "unreviewed";
  vramMiB: number | null;
  backends: string[];
}
export interface AIProvider {
  id: string;
  capabilities: string[];
  preflight(
    model: ModelManifest,
    gpu: GpuReport,
  ): Promise<{ supported: boolean; reasons: string[] }>;
  start(project: CharacterProject): Promise<Job>;
  cancel(jobId: string): Promise<void>;
}
export interface AdvertisementAdapter {
  readonly id: string;
  // No project data, device identifiers, filesystem APIs or model handles are passed.
  banner(locale: string): Promise<{ text: string; sponsor: string } | null>;
  dispose(): void;
}
export class OfflineAdvertisementAdapter implements AdvertisementAdapter {
  readonly id = "disabled";
  async banner(_locale: string) {
    return null;
  }
  dispose() {}
}
export function canExport(
  profile: ExportProfile,
  gates: CharacterProject["gates"],
): boolean {
  return (
    profile.implemented &&
    profile.runtimeValidation !== "unverified" &&
    ["A", "B", "C", "D", "E", "F"].every((id) =>
      gates.some((g) => g.id === id && g.status === "pass"),
    )
  );
}
export interface ParameterBinding {
  semantic: string;
  target: string;
  min: number;
  neutral: number;
  max: number;
}
// Static mapping check only. Names come from the selected creator preset, never guessed globally.
export function validateBindings(
  bindings: ParameterBinding[],
  requiredSemantics: string[],
): string[] {
  const errors: string[] = [];
  const semantics = new Set<string>();
  const targets = new Set<string>();
  for (const b of bindings) {
    if (semantics.has(b.semantic) || targets.has(b.target))
      errors.push("duplicate_binding");
    semantics.add(b.semantic);
    targets.add(b.target);
    if (
      !b.target ||
      ![b.min, b.neutral, b.max].every(Number.isFinite) ||
      b.min >= b.max ||
      b.neutral < b.min ||
      b.neutral > b.max
    )
      errors.push("invalid_range");
  }
  for (const id of requiredSemantics)
    if (!semantics.has(id)) errors.push(`missing:${id}`);
  return errors;
}
