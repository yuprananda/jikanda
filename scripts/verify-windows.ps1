$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Add-Type @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public static class JikandaWindows {
    public delegate bool EnumCallback(IntPtr hwnd, IntPtr data);
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumCallback callback, IntPtr data);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hwnd, out uint pid);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr hwnd, StringBuilder text, int count);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hwnd);
    [DllImport("user32.dll", EntryPoint="GetWindowLongPtrW")] public static extern IntPtr GetWindowLongPtr(IntPtr hwnd, int index);
    [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr hwnd, uint message, IntPtr wparam, IntPtr lparam);
    public static IntPtr[] Windows(uint processId) {
        var handles = new List<IntPtr>();
        EnumWindows((hwnd, data) => { uint pid; GetWindowThreadProcessId(hwnd, out pid); if (pid == processId) handles.Add(hwnd); return true; }, IntPtr.Zero);
        return handles.ToArray();
    }
    public static string Title(IntPtr hwnd) { var text = new StringBuilder(256); GetWindowText(hwnd, text, text.Capacity); return text.ToString(); }
}
'@
$executable = Join-Path $projectRoot 'target/release/jikanda.exe'
$appProcess = Start-Process -FilePath $executable -ArgumentList '--probe-both' -WorkingDirectory $projectRoot -WindowStyle Hidden -PassThru
try {
    $watch = [Diagnostics.Stopwatch]::StartNew()
    do {
        $appProcess.Refresh()
        if ($appProcess.HasExited) { throw 'Application exited before opening a window' }
        if ($appProcess.MainWindowHandle -ne [IntPtr]::Zero) { break }
        Start-Sleep -Milliseconds 20
    } while ($watch.Elapsed.TotalSeconds -lt 15)
    if ($appProcess.MainWindowHandle -eq [IntPtr]::Zero) { throw 'Application did not open a native window' }
    Start-Sleep -Seconds 2
    $handles = [JikandaWindows]::Windows($appProcess.Id)
    $main = $handles | Where-Object { [JikandaWindows]::Title($_) -eq 'Jikanda' } | Select-Object -First 1
    $floating = $handles | Where-Object { [JikandaWindows]::Title($_) -eq 'Jikanda timer' } | Select-Object -First 1
    if (!$main -or !$floating) { throw 'Expected both native windows' }
    $topmost = ([JikandaWindows]::GetWindowLongPtr($floating, -20).ToInt64() -band 8) -ne 0
    if (!$topmost) { throw 'WS_EX_TOPMOST was not applied' }
    [JikandaWindows]::PostMessage($floating, 0x10, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    Start-Sleep -Milliseconds 500
    if (![JikandaWindows]::IsWindowVisible($main)) { throw 'Main window was not restored' }
    if ([JikandaWindows]::IsWindowVisible($floating)) { throw 'Floating window did not hide' }
    [JikandaWindows]::PostMessage($main, 0x10, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    if (!$appProcess.WaitForExit(5000)) { throw 'Closing main did not quit' }
    if ($appProcess.ExitCode -ne 0) { throw "Application exited with $($appProcess.ExitCode)" }
    Write-Output 'PASS: two native windows, WS_EX_TOPMOST, floating close recovery, main close quits.'
} finally {
    if (!$appProcess.HasExited) { $appProcess.Kill(); $appProcess.WaitForExit() }
    $appProcess.Dispose()
}
