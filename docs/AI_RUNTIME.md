# DEV-03 local runtime contract

The runtime is a local authoring component. The only enabled real provider is
`onnx-synthetic-v1`: a 216-byte, untrained affine/ReLU diagnostic. It proves model
loading and tensor execution; it recognizes no character features and produces no
rig, mesh, texture or broadcast-ready master. Dummy protocol v1 remains available.

`runtime.rs` owns the typed registry, manifest-bound consent, environment selection,
requests and artifact admission. `runtime_process.rs` supervises JSONL v2 and OS
limits. `workers/runtime.py` is the registered bootstrap/ONNX adapter. Model-specific
imports occur only in the provider environment, not the global Python installation.
`JobManager::start_task` retains the existing single-job OS lock and append-only
journal. No provider calls ProjectStore or changes CharacterProject v1/v2.

IPC: `runtime_view` returns a consent token, installation inventory and backend
status; `prepare_runtime(approved,token)`, `probe_runtime()` and
`start_inference(project_id)` return a durable Job. `runtime_result(job_id)` requires
a completed journal entry. UI supplies neither executable paths nor scripts, URLs,
model paths, output paths or arbitrary Python arguments. Registry additions require
an audited application update; external plugins/remote code are not enabled.

Protocol v2 request binds protocol/provider/job/action, model environment lock hash,
core-owned runtime/staging roots and (for inference) a validated source hash and
fixed synthetic input. Response fields: protocol, job_id, provider, kind, progress,
data. Sequence is progress* → result → completed → successful process exit.
Maximum line64KiB/events1000, monotonic0–100, exactly one result/terminal, matching
IDs/version/provider. Unknown fields, oversized output, invalid errors and failed
exit reject the job. Only known safe error codes reach the UI; stderr is not shown.
Timeout is60s for diagnostic/probe and600s for installation, not an AI quality SLA.

Results first land in `runtime/staging/<job>/result.json`. Rust checks safe paths,
size, SHA-256, strict schema, expected numeric result, model/source identity,
finite timings, precision and memory limit. The core reopens the project and
rechecks its source before publishing a content-addressed diagnostic artifact and
job report. A diagnostic never changes master_status or any Gate. Cancellation or
journal failure can leave an unreferenced immutable artifact, not a completed job.

Provenance includes source hash, provider, version, weights hash, input,
preprocessing=none, seed=null and uncertainty=unknown. Source bytes are not passed
to this provider. DEV-04 must introduce explicit read-only derived input grants,
full-resolution coordinate/color metadata and observed/inferred feature contracts;
it must not reinterpret this diagnostic as image analysis.

CPU evidence is per-run; installed != verified. The inventory never marks a backend verified from installation alone. Recorded
completed job results can be displayed after restart; every new probe/inference
revalidates the environment and computes again. Historical reports are diagnostics,
not a promise that later device/driver/library versions work.

See MODEL_INSTALLATION, BACKEND_COMPATIBILITY, RESOURCE_BUDGET and PROVIDER_SECURITY.

Reports use a version1 envelope with job ID and canonical data hash. On retrieval the core rechecks envelope and accepted artifact bytes/schema; corrupted records are not successful results. An invalid active environment is blocked rather than reported as absent. These are corruption checks, not cryptographic authentication against a malicious same-user process.
