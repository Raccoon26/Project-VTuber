param([string]$Python = $env:VTUBER_PYTHON)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/env.ps1"
if (-not $Python -or -not [IO.Path]::IsPathRooted($Python)) { throw 'Pass an absolute working Python 3.12 path.' }
$env:VTUBER_PYTHON = $Python
if (-not $env:VTUBER_RUNTIME_DIR) { $env:VTUBER_RUNTIME_DIR = Join-Path $taskTools 'runtime-onnx-v1' }
$taskRuntimeFull = [IO.Path]::GetFullPath($env:VTUBER_RUNTIME_DIR)
if (-not $taskRuntimeFull.StartsWith($taskRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw 'Runtime must be inside this workspace.' }
# Explicit developer/CI preparation authorizes only the pinned ~27MiB CPU wheels.
# The product UI requires its own manifest-bound consent checkbox before this action.
cargo run --manifest-path src-tauri/Cargo.toml --locked --example runtime_check -- install
if ($LASTEXITCODE -ne 0) { throw 'Runtime preparation failed; partial generations are preserved.' }
