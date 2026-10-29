# Provider trust boundary

Only application-shipped, compile-time-hash-checked runtime.py and the pinned
synthetic ONNX bytes are registered. UI cannot choose an executable/script/model
URL/path. Model input is a fixed synthetic vector with source hash context; the
worker receives no original/project directory. Original file bytes, image metadata,
secrets and user paths are not sent to a download service. No pickle, remote code,
custom ORT operator library, source distribution or third-party plugin is loaded.

The bootstrap GETs only pinned public wheel URLs after consent. Redirects repeat
HTTPS/host validation. Inference uses verified local files, explicitly CPU-only
ORT and disabled ORT telemetry. The tensor child denies Python socket events
(except local gethostname) and subprocess launches. Trusted stdlib OS discovery is
completed before this hook; Windows can use its local `ver` command. Tests perform
real inference under the hook and verify an attempted socket connection is denied.
This establishes the application's offline path, not a physical unplugged test or
OS firewall audit of every native library. No network interfaces or firewall rules
on the user's PC were changed. Native-code network denial remains a stronger
DEV-29 boundary; it is not claimed from -I, a venv or Python hooks.

Windows OS Job Objects provide tested process/memory lifecycle limits for the
trusted provider. They are not an AppContainer filesystem/network sandbox. A
malicious same-user process could bypass path checks, alter the app/receipts or
access files with the user's account rights. Sources are absent from the supported
worker input contract, not protected from all malicious code by an ACL guarantee.
Untrusted providers remain disabled until actual OS restrictions are designed and
tested. The dummy v1 path retains its historical direct-child cleanup scope.

Runtime paths inspect ancestors/reparse points and bound sizes; hash checks happen
before load and artifact admission. Same-user TOCTOU/hardlink attacks, exhaustive
ONNX parser fuzzing, machine-wide compromise, all GPU native faults and every
process-tree pattern are outside current evidence. Installed packages are pinned
and receipts detect changes; source hashes alone do not establish safe model code
or preserved character identity. See SECURITY and CHARACTER_IDENTITY_POLICY.

Failures are machine codes with3-language recovery explanations. No raw stderr,
API token, arbitrary exception text or private input pathname enters UI diagnostics.
Job failure/journal failure never promotes incomplete output to a character master.
