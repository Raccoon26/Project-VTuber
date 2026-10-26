# Dot-source before development commands. All task-owned writes stay in the checkout.
$taskRoot = Split-Path -Parent $PSScriptRoot
$taskTools = Join-Path $taskRoot '.tools'
foreach ($taskFolder in @('tmp','npm-cache','cargo','app-data','webview','playwright','python-cache')) {
    New-Item -ItemType Directory -Force (Join-Path $taskTools $taskFolder) | Out-Null
}
$env:TEMP = Join-Path $taskTools 'tmp'
$env:TMP = $env:TEMP
$env:npm_config_cache = Join-Path $taskTools 'npm-cache'
$env:CARGO_HOME = Join-Path $taskTools 'cargo'
$env:CARGO_TARGET_DIR = Join-Path $taskRoot 'src-tauri/target'
$env:PLAYWRIGHT_BROWSERS_PATH = Join-Path $taskTools 'playwright'
$env:VTUBER_DATA_DIR = Join-Path $taskTools 'app-data'
$env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $taskTools 'webview'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:PYTHONPYCACHEPREFIX = Join-Path $taskTools 'python-cache'
# Use installed compilers read-only; bypass rustup shims that update a user-profile cache.
$taskCompiler = Join-Path $env:USERPROFILE '.rustup/toolchains/1.99.1-x86_64-pc-windows-msvc/bin'
if (Test-Path (Join-Path $taskCompiler 'cargo.exe')) { $env:PATH = "$taskCompiler;$env:PATH" }
# Select a complete installed Windows SDK. Multiple VS installations can otherwise
# select an SDK missing dbghelp.lib even when another complete SDK is installed.
if ($IsWindows -or $env:OS -eq 'Windows_NT') {
    $taskSdkRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits/10'
    $taskSdk = Get-ChildItem (Join-Path $taskSdkRoot 'Lib') -Directory -ErrorAction SilentlyContinue |
        Where-Object { (Test-Path (Join-Path $_.FullName 'um/x64/dbghelp.lib')) -and (Test-Path (Join-Path $_.FullName 'ucrt/x64/ucrt.lib')) } |
        Sort-Object { [version]$_.Name } -Descending | Select-Object -First 1
    $taskVswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
    if ((Test-Path $taskVswhere) -and $taskSdk) {
        $taskVs = & $taskVswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        $taskMsvc = Get-ChildItem (Join-Path $taskVs 'VC/Tools/MSVC') -Directory | Sort-Object Name -Descending | Select-Object -First 1
        $env:LIB = "$($taskMsvc.FullName)/lib/x64;$($taskSdk.FullName)/um/x64;$($taskSdk.FullName)/ucrt/x64;$env:LIB"
        $env:INCLUDE = "$($taskMsvc.FullName)/include;$taskSdkRoot/Include/$($taskSdk.Name)/ucrt;$taskSdkRoot/Include/$($taskSdk.Name)/shared;$taskSdkRoot/Include/$($taskSdk.Name)/um;$env:INCLUDE"
        $env:PATH = "$($taskMsvc.FullName)/bin/Hostx64/x64;$env:PATH"
    }
}
