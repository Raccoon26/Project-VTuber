# DEV-03 resource budget

The diagnostic accepts exactly float32[1,3] and outputs float32[1,2]. Its216-byte
IR8/opset13 ONNX graph computes Relu(XW+B); it does not scale or read images.
Preflight requires512MiB available RAM and512MiB free disk. Only onnx_cpu is
accepted; unsupported CUDA/offload requests fail before model loading. GPU model
requirements exceeding a6GB device cannot be admitted by this CPU-only profile.

One task across windows/processes sharing a data root holds the existing journal
OS lock. No parallel GPU scheduling is claimed. Thread counts are1 for ORT intra/
inter-op and relevant BLAS/OpenMP environment. Windows Job Object bounds total
commit to512MiB, active descendants to8, and kills remaining owned descendants on
handle close. The parent waits/reaps before terminal state. Python venv redirectors
are created only by the already-owned base interpreter after its stdin request.
This trusted base-interpreter path is the verified scope; hostile launchers are not.

First actual sample: input[1,2,3] → [2.25,5.5]; load4.59ms, tensor call0.313ms,
child process peak61,743,104B. These are one small synthetic sample on this PC,
not generic AI speed. Receipt hashing and process startup take additional wall
time; CPU seconds include preparation overhead. Native final evidence supersedes
early samples. GPU peak is not measured because no GPU work was run.

Tests inject no-RAM/no-disk/backend mismatch, and a real800MiB allocation under the
512MiB Windows job limit is refused. This is bounded real OS memory enforcement,
not a test that fills the user's physical RAM/disk. JSONL is64KiB/1000events,
artifact64KiB, download≤64MiB per current artifact, environment inventory≤10000
files/256MiB each, receipt≤4MiB, active history≤128. Exceeding limits is explicit.

No pruning of original data, saved revisions, incomplete environments or prior
model versions occurs. Future retention requires reference tracking and user-safe
recovery rules. Network-volume/power-loss durability and long real-AI checkpoint
resume remain DEV-28. Do not lower source quality to meet this resource profile.

Final active generation file lengths total 100,486,892 bytes, plus28,074,688B cached wheels. This excludes retained abandoned test generations, disk cluster overhead and WebView cache. Final native probe load4.49ms/inference0.305ms/peak60,477,440B; CPU0.53125s is process time rather than utilization percentage. Full final evidence is docs/evidence/dev-03-local.json.
