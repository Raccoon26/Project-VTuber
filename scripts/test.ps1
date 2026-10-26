param([string]$Python = $env:VTUBER_PYTHON, [switch]$SkipUI)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/env.ps1"
Set-Location $taskRoot
if (-not $Python) { $Python = (Get-Command python -ErrorAction Stop).Source }
$env:VTUBER_PYTHON = $Python
New-Item -ItemType Directory -Force artifacts | Out-Null
function Invoke-Check([string]$Name, [scriptblock]$Command) {
    Write-Output "Running $Name"
    & $Command 2>&1 | Tee-Object -FilePath "artifacts/$Name.log"
    if ($LASTEXITCODE -ne 0) { throw "$Name failed ($LASTEXITCODE)" }
}
Invoke-Check 'docs-audit' { node scripts/audit-docs.mjs }
Invoke-Check 'frontend-format' { npm run format:check }
Invoke-Check 'frontend-build' { npm run build }
Invoke-Check 'contracts-test' { npm test }
Invoke-Check 'python-test' { & $Python -m unittest discover -s workers -v }
Invoke-Check 'rust-format' { cargo fmt --manifest-path src-tauri/Cargo.toml --check }
Invoke-Check 'rust-test' { cargo test --manifest-path src-tauri/Cargo.toml --locked }
Invoke-Check 'clippy' { cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings }
if (-not $SkipUI) { Invoke-Check 'ui-test' { npm run test:ui } }
Write-Output 'All requested checks passed. This does not certify generated models or external broadcast apps.'
