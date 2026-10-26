param([string]$Python = $env:VTUBER_PYTHON)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/env.ps1"
Set-Location $taskRoot
if (-not $Python) { $Python=(Get-Command python -ErrorAction Stop).Source }
$env:VTUBER_PYTHON=$Python
& $Python --version
if ($LASTEXITCODE -ne 0) {throw 'A working Python 3.12 executable is required'}
npm run desktop:dev
exit $LASTEXITCODE
