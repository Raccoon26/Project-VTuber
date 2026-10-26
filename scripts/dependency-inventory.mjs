import fs from "node:fs";
import { execFileSync } from "node:child_process";
const lock = JSON.parse(fs.readFileSync("package-lock.json", "utf8"));
const npm = Object.entries(lock.packages)
  .filter(([path]) => path)
  .map(([path, p]) => ({
    path,
    version: p.version,
    license: p.license ?? "UNREVIEWED",
    integrity: p.integrity ?? null,
    source: p.resolved ?? null,
    dev: !!p.dev,
  }));
const cargo = JSON.parse(
  execFileSync(
    "cargo",
    [
      "metadata",
      "--manifest-path",
      "src-tauri/Cargo.toml",
      "--locked",
      "--filter-platform",
      "x86_64-pc-windows-msvc",
      "--format-version",
      "1",
      "--offline",
    ],
    { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 },
  ),
);
fs.mkdirSync("artifacts", { recursive: true });
fs.writeFileSync(
  "artifacts/dependency-inventory.json",
  JSON.stringify(
    {
      generatedAt: new Date().toISOString(),
      notice: "Inventory only; not redistribution clearance",
      npm,
      rust: cargo.packages.map((p) => ({
        name: p.name,
        version: p.version,
        license: p.license ?? "UNREVIEWED",
        source: p.source,
        repository: p.repository,
      })),
    },
    null,
    2,
  ),
);
console.log(
  `Inventory: ${npm.length} npm packages, ${cargo.packages.length} Rust packages`,
);
