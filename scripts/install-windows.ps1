# Run explicitly after extracting an internal package into its permanent folder.
# Creates one per-user Start menu shortcut with the AppUserModelID used by toasts.
param([string]$Executable = (Join-Path $PSScriptRoot 'jikanda.exe'), [string]$ShortcutPath = '')
$ErrorActionPreference = 'Stop'
$resolvedExecutable = (Resolve-Path -LiteralPath $Executable).Path
Add-Type @'
using System;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;
using System.Text;
public static class JikandaShortcut {
    [ComImport, Guid("00021401-0000-0000-C000-000000000046")] class ShellLink { }
    [ComImport, InterfaceType(ComInterfaceType.InterfaceIsIUnknown), Guid("000214F9-0000-0000-C000-000000000046")]
    interface IShellLinkW {
        void GetPath(StringBuilder file, int count, IntPtr data, int flags);
        void GetIDList(out IntPtr list); void SetIDList(IntPtr list);
        void GetDescription(StringBuilder name, int count); void SetDescription([MarshalAs(UnmanagedType.LPWStr)] string text);
        void GetWorkingDirectory(StringBuilder directory, int count); void SetWorkingDirectory([MarshalAs(UnmanagedType.LPWStr)] string directory);
        void GetArguments(StringBuilder arguments, int count); void SetArguments([MarshalAs(UnmanagedType.LPWStr)] string arguments);
        void GetHotkey(out short key); void SetHotkey(short key);
        void GetShowCmd(out int command); void SetShowCmd(int command);
        void GetIconLocation(StringBuilder icon, int count, out int index); void SetIconLocation([MarshalAs(UnmanagedType.LPWStr)] string icon, int index);
        void SetRelativePath([MarshalAs(UnmanagedType.LPWStr)] string path, int reserved);
        void Resolve(IntPtr window, int flags); void SetPath([MarshalAs(UnmanagedType.LPWStr)] string path);
    }
    [StructLayout(LayoutKind.Sequential)] struct PropertyKey { public Guid format; public uint id; }
    [StructLayout(LayoutKind.Explicit, Size=24)] struct PropVariant { [FieldOffset(0)] public ushort type; [FieldOffset(8)] public IntPtr value; }
    [ComImport, InterfaceType(ComInterfaceType.InterfaceIsIUnknown), Guid("886D8EEB-8CF2-4446-8D02-CDBA1DBDCF99")]
    interface IPropertyStore {
        void GetCount(out uint count); void GetAt(uint index, out PropertyKey key);
        void GetValue(ref PropertyKey key, out PropVariant value);
        void SetValue(ref PropertyKey key, ref PropVariant value); void Commit();
    }
    public static void Create(string executable, string shortcut) {
        object instance = new ShellLink();
        try {
            var link = (IShellLinkW)instance;
            link.SetPath(executable); link.SetWorkingDirectory(System.IO.Path.GetDirectoryName(executable));
            link.SetDescription("Jikanda - Make time for what matters."); link.SetShowCmd(1);
            link.SetIconLocation(executable, 0);
            var key = new PropertyKey { format = new Guid("9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3"), id = 5 };
            var value = new PropVariant { type = 31, value = Marshal.StringToCoTaskMemUni("YuPrananda.Jikanda") };
            try { var properties = (IPropertyStore)instance; properties.SetValue(ref key, ref value); properties.Commit(); }
            finally { Marshal.FreeCoTaskMem(value.value); }
            ((IPersistFile)instance).Save(shortcut, true);
        } finally { Marshal.FinalReleaseComObject(instance); }
    }
}
'@
$programs = [Environment]::GetFolderPath('Programs')
$shortcutPath = if ($ShortcutPath) { [IO.Path]::GetFullPath($ShortcutPath) } else { Join-Path $programs 'Jikanda.lnk' }
if (Test-Path -LiteralPath $shortcutPath) { throw 'Jikanda.lnk already exists. Remove it explicitly if you want to replace it.' }
[JikandaShortcut]::Create($resolvedExecutable, $shortcutPath)
Write-Output "Created $shortcutPath. Start menu identity: YuPrananda.Jikanda. OS notification settings may still suppress toasts."
