# Model and runtime installation

ModelManifest v1 is `models/synthetic-affine-v1.json`; the Rust type rejects unknown
fields and future schemas. It separates code/weights/dependency rights, commercial
use, redistribution, territory, origin, exact version/content revision, byte/hash,
resource requirements, input/output, preprocessing and evidence. The synthetic
weights have no third-party dataset and are CC0; the application code's eventual
distribution license remains the owner's decision. Unreviewed or region-ineligible
registrations are blocked. A model version never overwrites previous model bytes.

The first approved environment uses official PyPI Windows x64 CPython3.12 wheels:
onnxruntime1.31.1, numpy2.5.4, flatbuffers25.12.19, packaging26.3, protobuf7.36.2.
`workers/runtime-lock.json` pins filenames, exact versions, HTTPS URLs, sizes and
SHA-256. Total28,074,688 bytes (~26.8MiB); reserve512MiB free disk. No paid account,
cloud inference, CUDA Toolkit, PyTorch or large weights are required. The model is
bundled; the network downloads are runtime wheels. The bounded downloader also
accepts approved ONNX/safetensors payload names, but no unreviewed external model
is exposed in this release.

UI shows purpose/size/disk/RAM/source/license/offline scope before approval. The
token hashes both registry and dependency lock; stale/false consent starts nothing.
Installation uses only HTTPS files.pythonhosted.org with certificate verification,
no proxy credentials, no arbitrary query/userinfo/port, and redirect revalidation.
Length is checked while streaming, reads/time are bounded, and the final digest
must match. No archive supplied by an arbitrary user is installed. pip is isolated,
no-index/no-deps/require-hashes/only-binary/no-compile; no source build script runs.

Storage under runtime/: downloads/*.partial or approved wheel names;
environments/<random>/venv and model.onnx; active/<sequence>.json. Each install gets
a new environment. Failed/abandoned generations and partials are retained and
never treated as active. Retrying starts a new partial rather than trusting an
unverified HTTP Range prefix. A corrupt wheel cache is retained as *.rejected only
under renewed installation approval. Existing active environments are preserved.

After isolated real operator smoke, a receipt hashes all installed environment
files. The receipt and model are validated before an immutable active record is
published via fsync + no-overwrite hard link. Every inference verifies the model
and full environment inventory again. Unknown/changed files fail closed; this
detects corruption, not an adversary able to replace the app and all receipts.
Missing active record means not installed; partial generation never implies ready.

State meanings: discovered/legal_review/blocked are registry policy results;
approval_required precedes downloads; install progress5–55 downloads,65–80 installs
and verifies,80–90 quarantines real model load, then publishes installed state.
`installed_unverified` inventory requires a fresh probe or successful inference to
be `runnable` for that run. Failures remain failed/recovery_required in Job Journal.
There is no automatic checkpoint resume or silent fallback to a bad earlier env.

Normal Windows process-crash durability is tested separately from power loss.
Filesystem+installation+Job Journal are not one transaction: publication can succeed
before a terminal journal write fails. Reinspection/probe resolves installed state;
it must never fabricate a completed job. Automatic deletion/garbage collection is
deferred until reference/backup rules are specified in DEV-28.
