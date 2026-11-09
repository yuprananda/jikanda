# Windows installer source - 0.1.0-beta.1

This supplements the original release's complete application source archive. It
does not replace or modify that archive or the previously published portable ZIP.
The installed application executable must be byte-identical to that portable ZIP.

## Rebuild

1. Obtain the unmodified official Inno Setup 7.1.1 x64 compiler from
   https://github.com/jrsoftware/issrc/releases/tag/is-7_1_1 . Verify its upstream
   SHA-256/Authenticode before use; see notices/provenance.json.
2. Place the original portable ZIP under artifacts/releases/v0.1.0-beta.1 in this
   project/extracted installer-source folder. The builder verifies its pinned hash.
3. Run Python 3.11+:

```powershell
python scripts/build-installer.py --compiler "C:/path/to/Inno Setup 7/ISCC.exe"
```

Output: setup EXE, installer-source ZIP, and an updated SHA256SUMS.txt in that same
release directory. No application code is recompiled. The compiler and runtime
engine remain under their upstream licenses; original copyright/website notices
are retained. Inno Setup is a build tool, not a Jikanda runtime dependency.

## Behavior

- Windows 11 x64, current-user installation without elevation.
- Default location: %LOCALAPPDATA%/Programs/Jikanda.
- Start menu shortcut with mascot icon/AppUserModelID; optional desktop shortcut.
- Registered uninstaller in Windows Settings > Apps; upgrades reuse the same AppId.
- Close Jikanda before updating. Setup never force-closes the running timer.
- Only installer-owned files/shortcuts are removed. Data in %LOCALAPPDATA%/Jikanda,
  exported PNGs, and unrelated files are preserved. No recursive data-delete rule.
- The Microsoft Visual C++ x64 runtime must exist; missing-runtime instructions
  point to Microsoft's official download. It is not silently installed.
- Unsigned beta: no code-signing certificate is claimed.

App and installer script/build code: GPL-3.0-only. Upstream tool/engine: its original
Inno Setup license, provided with copyright notices and provenance in notices/.

## Verification

On an isolated Windows user profile with no installed Jikanda or existing Start
menu shortcut, run `scripts/verify-installer.ps1 -Installer <setup.exe>` from
PowerShell. It checks install, payload hash, locked-update rejection, reinstall,
native application scenarios, uninstall, and preservation of unrelated/local files.
Its logs and native snapshots remain under `artifacts/installer-smoke-*`.
