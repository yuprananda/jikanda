# SPDX-License-Identifier: GPL-3.0-only
param([Parameter(Mandatory=$true)][string]$Installer)
$ErrorActionPreference = 'Stop'
$installerPath = (Resolve-Path -LiteralPath $Installer).Path
$registry = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\YuPrananda.Jikanda_is1'
if (Test-Path -LiteralPath $registry) { throw 'An existing Jikanda installation must not be modified by this smoke test.' }
$root = Join-Path (Split-Path -Parent $PSScriptRoot) ('artifacts\installer-smoke-' + (Get-Date -Format yyyyMMddHHmmss))
$app = Join-Path $root 'app'
New-Item -ItemType Directory -Path $root -Force | Out-Null
$group = 'Jikanda'
$shortcut = Join-Path ([Environment]::GetFolderPath('Programs')) ($group + '\Jikanda.lnk')
if (Test-Path -LiteralPath $shortcut) { throw 'Existing Start menu shortcut must not be modified.' }
function Run-Setup([string]$File, [string[]]$Arguments) {
    $process = Start-Process -FilePath $File -ArgumentList $Arguments -WindowStyle Hidden -PassThru
    if (!$process.WaitForExit(60000)) { throw 'Installer test timed out.' }
    if ($process.ExitCode -ne 0) { throw "Installer failed: $($process.ExitCode)" }
}
$arguments = @('/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', '/TASKS=""', ('/DIR="' + $app + '"'), ('/GROUP="' + $group + '"'))
Run-Setup $installerPath ($arguments + ('/LOG="' + $root + '\install.log"'))
if (!(Test-Path -LiteralPath $shortcut) -or !(Test-Path -LiteralPath $registry)) { throw 'Shortcut or per-user uninstall entry missing.' }
$payload = Join-Path (Split-Path -Parent $PSScriptRoot) 'artifacts\installer-staging\0.1.0-beta.1\Jikanda-0.1.0-beta.1-windows-x64\jikanda.exe'
if ((Get-FileHash -LiteralPath $payload).Hash -ne (Get-FileHash -LiteralPath (Join-Path $app 'jikanda.exe')).Hash) { throw 'Installed binary differs from released payload.' }
$keep = Join-Path $app 'unrelated-user-file.txt'
Set-Content -LiteralPath $keep -Value 'preserve me'
$data = Join-Path $env:LOCALAPPDATA 'Jikanda'
New-Item -ItemType Directory -Path $data -Force | Out-Null
$sentinel = Join-Path $data ('installer-smoke-' + [Guid]::NewGuid().ToString() + '.txt')
Set-Content -LiteralPath $sentinel -Value 'preserve local data'
try {
    $lock = [IO.File]::Open((Join-Path $app 'jikanda.exe'), [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        $blocked = Start-Process -FilePath $installerPath -ArgumentList ($arguments + ('/LOG="' + $root + '\locked-update.log"')) -WindowStyle Hidden -PassThru
        if (!$blocked.WaitForExit(30000)) { throw 'Locked-update test timed out.' }
        if ($blocked.ExitCode -eq 0) { throw 'Setup should reject a locked application executable.' }
    } finally { $lock.Dispose() }
    Run-Setup $installerPath ($arguments + ('/LOG="' + $root + '\update.log"'))
    & (Join-Path $PSScriptRoot 'verify-native.ps1') -ExecutablePath (Join-Path $app 'jikanda.exe') -WorkingDirectory $root
    Run-Setup (Join-Path $app 'unins000.exe') @('/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', ('/LOG="' + $root + '\uninstall.log"'))
    if (Test-Path -LiteralPath (Join-Path $app 'jikanda.exe')) { throw 'Installed executable remains after uninstall.' }
    if ((Test-Path -LiteralPath $registry) -or (Test-Path -LiteralPath $shortcut)) { throw 'Uninstall entry or shortcut remains.' }
    if (!(Test-Path -LiteralPath $keep) -or !(Test-Path -LiteralPath $sentinel)) { throw 'Uninstaller removed unrelated files or local data.' }
    'PASSED: install, identical binary, shortcut, locked-update protection, upgrade, native app checks, uninstall, data preservation.' | Set-Content -LiteralPath (Join-Path $root 'PASSED.txt')
    Write-Output $root
} finally {
    Remove-Item -LiteralPath $sentinel -ErrorAction SilentlyContinue
}
