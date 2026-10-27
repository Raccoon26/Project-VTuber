import fs from "node:fs";
import assert from "node:assert/strict";
const docs = [
  "PROJECT_STATUS",
  "REQUIREMENTS",
  "ARCHITECTURE",
  "DEV_ROADMAP",
  "TECH_RESEARCH",
  "TECH_SPIKES",
  "MODEL_STRATEGY",
  "MODEL_CONTRACT",
  "COMPATIBILITY_MATRIX",
  "VBRIDGER_REQUIREMENTS",
  "QUALITY_GATES",
  "TEST_STRATEGY",
  "FAILURE_REGISTER",
  "LICENSE_AUDIT",
  "SECURITY",
  "LOCALIZATION",
  "MONETIZATION",
  "DECISIONS",
  "ENVIRONMENT_REPORT",
  "TEST_RUNS",
  "STORAGE_RECOVERY",
  "MIGRATION_STRATEGY",
  "JOB_PERSISTENCE",
];
for (const doc of docs)
  assert.ok(
    fs.readFileSync(`docs/${doc}.md`, "utf8").length > 200,
    `Missing/empty ${doc}`,
  );
assert.ok(fs.readFileSync("README.md", "utf8").includes("desktop"));
const requirements = JSON.parse(
  fs.readFileSync("docs/requirements.json", "utf8"),
);
assert.equal(requirements.length, 78);
assert.equal(new Set(requirements.map((r) => r.id)).size, 78);
assert.deepEqual(
  requirements,
  JSON.parse(fs.readFileSync("requirements.json", "utf8")),
);
for (let n = 1; n <= 78; n++)
  assert.equal(requirements[n - 1].id, `REQ-${String(n).padStart(3, "0")}`);
const coverage = fs.readFileSync("docs/REQUIREMENTS.md", "utf8");
for (const r of requirements)
  assert.ok(
    coverage.includes(
      `| ${r.id} | ${r.description} | ${r.priority} | ${r.dev.join(", ")} | ${r.module} | ${r.verification} | ${r.status} |`,
    ),
    `Markdown out of sync: ${r.id}`,
  );
const allowed = [
  "Planned",
  "Implemented",
  "Verified",
  "Blocked",
  "Partially Implemented",
  "Rejected with Reason",
];
const roadmap = fs.readFileSync("docs/DEV_ROADMAP.md", "utf8");
for (let n = 0; n <= 33; n++)
  assert.ok(
    roadmap.includes(`| ${String(n).padStart(2, "0")} |`),
    `Roadmap DEV-${n}`,
  );
for (const r of requirements) {
  for (const field of [
    "id",
    "description",
    "priority",
    "module",
    "verification",
    "status",
    "risk",
  ])
    assert.ok(
      typeof r[field] === "string" && r[field].length > 0,
      `${r.id}.${field}`,
    );
  assert.ok(allowed.includes(r.status), r.id);
  assert.ok(
    r.dev.length > 0 &&
      r.dev.every((d) => /^DEV-(0[0-9]|[12][0-9]|3[0-3])$/.test(d)),
    r.id,
  );
  assert.ok(
    r.tests.length > 0 && r.tests.every((f) => fs.existsSync(f)),
    `${r.id}: missing evidence path`,
  );
}
const sample = JSON.parse(
  fs.readFileSync("samples/character-project.json", "utf8"),
);
assert.equal(sample.schema_version, 1);
assert.equal(sample.master_status, "not_generated");
console.log(
  `PASS ${docs.length} required documents, ${requirements.length} requirements, mirrors, 34 DEV ownership plans, evidence references; content quality needs human review.`,
);
