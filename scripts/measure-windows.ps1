param([int]$SampleSeconds = 10)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$executable = Join-Path $projectRoot 'target/release/jikanda.exe'
if (!(Test-Path -LiteralPath $executable)) { throw 'Build first: cargo build --release' }
$outputDirectory = Join-Path $projectRoot 'artifacts'
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
$results = @()
foreach ($layout in @('main', 'floating', 'both')) {
    foreach ($state in @('idle', 'running')) {
        $arguments = @('--probe-main')
        if ($layout -eq 'floating') { $arguments += '--probe-floating' }
        if ($layout -eq 'both') { $arguments += '--probe-both' }
        if ($state -eq 'running') { $arguments += '--probe-running' }
        $launch = @{ FilePath = $executable; WorkingDirectory = $projectRoot; PassThru = $true; WindowStyle = 'Hidden' }
        if ($arguments.Count -gt 0) { $launch.ArgumentList = $arguments }
        $watch = [Diagnostics.Stopwatch]::StartNew()
        $appProcess = Start-Process @launch
        try {
            do {
                $appProcess.Refresh()
                if ($appProcess.HasExited) { throw 'Application exited before opening a window' }
                if ($appProcess.MainWindowHandle -ne [IntPtr]::Zero) { break }
                Start-Sleep -Milliseconds 20
            } while ($watch.Elapsed.TotalSeconds -lt 15)
            if ($appProcess.MainWindowHandle -eq [IntPtr]::Zero) { throw 'No native main window within 15 seconds' }
            $watch.Stop()
            Start-Sleep -Seconds 3
            $appProcess.Refresh()
            $cpuStart = $appProcess.TotalProcessorTime.TotalSeconds
            $sampleWatch = [Diagnostics.Stopwatch]::StartNew()
            Start-Sleep -Seconds $SampleSeconds
            $appProcess.Refresh()
            if ($appProcess.HasExited) { throw 'Application exited during sample' }
            $sampleWatch.Stop()
            $results += [pscustomobject]@{
                layout = $layout; state = $state
                nativeWindowStartupMs = [math]::Round($watch.Elapsed.TotalMilliseconds, 1)
                workingSetMiB = [math]::Round($appProcess.WorkingSet64 / 1MB, 2)
                privateBytesMiB = [math]::Round($appProcess.PrivateMemorySize64 / 1MB, 2)
                cpuPercentOneCore = [math]::Round(100 * ($appProcess.TotalProcessorTime.TotalSeconds - $cpuStart) / $sampleWatch.Elapsed.TotalSeconds, 3)
                sampleSeconds = [math]::Round($sampleWatch.Elapsed.TotalSeconds, 2)
            }
        } finally {
            if (!$appProcess.HasExited) {
                $appProcess.CloseMainWindow() | Out-Null
                if (!$appProcess.WaitForExit(2000)) { $appProcess.Kill(); $appProcess.WaitForExit() }
            }
            $appProcess.Dispose()
        }
    }
}
$report = [pscustomobject]@{
    measuredAt = (Get-Date -Format o)
    os = (Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version, OSArchitecture)
    cpu = (Get-CimInstance Win32_Processor | Select-Object Name, NumberOfLogicalProcessors)
    ramBytes = (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory
    profile = 'release; thin LTO; stripped; Winit software renderer; accessibility enabled'
    executableSha256 = (Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash
    method = '3s warm-up, process CPU delta, end-of-sample working set/private bytes; startup polls MainWindowHandle every 20ms, not first paint; single warm run per case'
    results = $results
}
$report | ConvertTo-Json -Depth 5 | Set-Content -Encoding UTF8 -LiteralPath (Join-Path $outputDirectory 'windows-performance.json')
$report | ConvertTo-Json -Depth 5 | Set-Content -Encoding UTF8 -LiteralPath (Join-Path $outputDirectory ("windows-performance-{0}.json" -f (Get-Date -Format 'yyyyMMdd-HHmmss')))
$results | Format-Table -AutoSize
