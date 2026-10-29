# Backend evidence and limits — DEV-03

| Backend | Current managed environment | Evidence / activation |
|---|---|---|
| ONNX Runtime CPU1.31.1 | Installed; actual calculation verified on this Windows machine | affine/ReLU numeric checks, varied inputs, model load, normal exit, cancellation |
| PyTorch CPU | Not installed in this provider environment | no capability claim; future adapter needs its own lock/probe |
| PyTorch CUDA | Not installed | no CUDA tensor test, no torch install performed |
| ONNX DirectML | Not installed | no GPU tensor test; not silently substituted |
| NVIDIA CUDA system capability | Unverified | GPU name/driver alone do not establish a runnable adapter |
| Offload/quantization/large image→3D | Unsupported by this provider | no automatic download or fallback |

Actual device query2026-10-29 JST: Ryzen7 4800H8cores/16threads; RAM
41,809,104KiB total; RTX2060 total6144MiB/free4332MiB at query; driver591.74;
E NTFS with ~2.77TB free. Free memory is a point-in-time observation, not reserved
capacity. Probe/preflight reads current RAM/disk immediately before work. GPU
resources are reported separately; CPU diagnostic requires0MiB VRAM.

The UI uses managed-environment inventory, not claims about all user Python
installations or system CUDA Toolkits. installed_unverified, not_installed,
unsupported, probe failure, and actual verified results are distinct. Failed GPU
support is never converted to verified CPU fallback unless that exact CPU provider
has passed its real calculation. Here CPU is the explicit baseline, not a hidden
quality reduction; no source image preprocessing occurs.

Official [ORT installation](https://onnxruntime.ai/docs/install/) now describes
DirectML as sustained engineering and suggests WinML for new Windows projects.
It is retained as a researched candidate, not installed merely to detect a GPU.
[ORT Python API](https://onnxruntime.ai/docs/api/python/api_summary.html) explains
execution-provider priority/fallback; this provider explicitly requests only CPU
and disables automatic fallback. An available-provider string is not our test.

GPU inference, full GPU driver matrix, GPU OOM/offload recovery and physical
network-disconnected operation remain unexecuted. Model-specific DEV-04/12/31
probes must add actual operator/output/resource evidence before enabling them.
