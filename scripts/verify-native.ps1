param([int]$TimeoutSeconds = 90, [string]$ExecutablePath = '', [string]$WorkingDirectory = '')
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$runDirectory = if ($WorkingDirectory) { (Resolve-Path -LiteralPath $WorkingDirectory).Path } else { $projectRoot }
$executable = if ($ExecutablePath) { (Resolve-Path -LiteralPath $ExecutablePath).Path } else { Join-Path $projectRoot 'target/release/jikanda.exe' }
$process = Start-Process -FilePath $executable -ArgumentList '--verify-native' -WorkingDirectory $runDirectory -PassThru -WindowStyle Hidden
try {
    if (!$process.WaitForExit($TimeoutSeconds * 1000)) { throw 'Native verification timed out' }
    if ($process.ExitCode -ne 0) { throw "Native verification process failed: $($process.ExitCode)" }
    $snapshots = Get-ChildItem -LiteralPath (Join-Path $runDirectory 'artifacts') -Directory -Filter "native-$($process.Id)-*" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (!$snapshots -or !(Test-Path -LiteralPath (Join-Path $snapshots.FullName 'PASSED.txt'))) { throw 'Native assertions did not finish' }
    $state = Get-ChildItem -LiteralPath (Join-Path $runDirectory 'artifacts') -Directory -Filter "state-$($process.Id)-*" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    $data = Get-Content -Raw -LiteralPath (Join-Path $state.FullName 'state.json') | ConvertFrom-Json
    if ($data.records.Count -ne 3) { throw 'Unexpected persisted history count' }
    if ($data.settings.alarm_enabled -ne $false -or $data.settings.alarm_sound -ne 'BrightTone') { throw 'Alarm mute or sound selection was not persisted' }
    if ($data.settings.display_name -ne 'Nama pilihan') { throw 'Display name was not persisted' }
    if ($data.settings.language -ne 'English') { throw 'Language selection was not persisted' }
    if ($IsWindows -or $env:OS -eq 'Windows_NT') {
        if ($data.settings.floating_opacity -ne 80) { throw 'Mini timer opacity was not persisted' }
    }
    if ($data.tasks.entries.Count -ne 1 -or $data.session_name -ne 'Write a blog') { throw 'Named session draft was not persisted' }
    $firstSession = $data.records | Where-Object mode -eq 'DeepFocus'
    $flowSession = $data.records | Where-Object mode -eq 'Flow'
    if ($firstSession.session_name -ne 'Learn Rust revised' -or $firstSession.tasks.entries.Count -ne 2) { throw 'Historical session context was lost' }
    if (!$firstSession.tasks.entries[0].completed -or $firstSession.tasks.entries[0].title -ne 'Draft a paragraph') { throw 'Main/mini task synchronization or snapshot failed' }
    if ($flowSession.session_name -ne 'Write a blog' -or $flowSession.tasks.entries.Count -ne 1) { throw 'New session tasks were mixed with the previous session' }
    if (($data.records | Where-Object mode -eq 'Interval').focus_ms -ne 2000) { throw 'Interval focus duration includes break or was lost' }
    if (($data.records | Where-Object mode -eq 'DeepFocus').focus_ms -ne 8000) { throw 'History correction was not persisted' }
    if (($data.records | Where-Object mode -eq 'Flow').outcome -ne 'Completed') { throw 'Explicit Flow completion was not persisted' }
    $pngs = @(Get-ChildItem -LiteralPath (Join-Path $state.FullName 'exports') -Filter '*.png' -File)
    if ($pngs.Count -ne 2) { throw 'Both real-data templates were not exported' }
    Add-Type -AssemblyName System.Drawing
    $dimensions = @()
    foreach ($png in $pngs) {
        $image = [Drawing.Image]::FromFile($png.FullName)
        try { $dimensions += "$($image.Width)x$($image.Height)" }
        finally { $image.Dispose() }
    }
    if ($dimensions -notcontains '1080x1920' -or $dimensions -notcontains '1080x1350') { throw 'Incorrect Story/Feed PNG dimensions' }
    Write-Output "PASS: three modes, pause/resume, stop save/discard, history correction/delete, language, session tasks, settings, two real-data PNGs, persisted quit. Snapshots: $($snapshots.FullName)"
} finally {
    if (!$process.HasExited) { $process.Kill(); $process.WaitForExit() }
    $process.Dispose()
}
