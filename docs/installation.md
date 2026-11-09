# Install Jikanda beta

[Panduan Bahasa Indonesia](installation-id.md)

## Windows 11 x64

Download the Windows ZIP from [official releases](https://github.com/yuprananda/jikanda/releases).
Extract **all files** to a permanent folder, such as `%LOCALAPPDATA%/Programs/Jikanda`,
then run `jikanda.exe`. Do not run inside the ZIP. No account, administrator access,
background service, or application internet connection is required.
Windows 10 and ARM64 are not validated for this beta.

Optional per-user Start menu shortcut, from PowerShell in the extracted folder:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File ./install-windows.ps1
```

The shortcut uses the executable's mascot icon. The script does not enable autostart
or overwrite an existing shortcut. If you move the app, remove its old shortcut and
run the script again from the new folder.

The beta is **unsigned**; Windows may display a reputation warning. Verify the
source and compare the ZIP's SHA-256 with `SHA256SUMS.txt` from the same release:

```powershell
Get-FileHash ./Jikanda-0.1.0-beta.1-windows-x64.zip -Algorithm SHA256
```

If an MSVC runtime is reported missing, install Microsoft's official
[Visual C++ Redistributable x64](https://aka.ms/vs/17/release/vc_redist.x64.exe).
Do not download individual DLLs from third-party sites.

## First session

English is the default; change language in Preferences > General. Name a session
and optionally add tasks. Pomodoro is 25/5 minutes, four focus segments; Deep Focus
is 50 minutes; Flow counts up until you finish and save. Each Pomodoro segment
starts when ready. Breaks do not count as focus time.

Open the mini timer from the sidebar; its close control restores the main window.
Opacity is in Preferences > Mini timer. Alarm, mute, and desktop notifications are
separate settings. **Save preferences** applies alarm/notification changes. Preview
sound/Test notification explicitly play/send a test. OS volume, permissions, and
Do Not Disturb can affect what is audible or visible.

Create one-session PNGs from History > Create card. Display name and completed
tasks are opt-in; private legacy notes are excluded.

## Update and uninstall

Close Jikanda before replacing files; Windows locks a running executable. Keep the
installation folder separate from `%LOCALAPPDATA%/Jikanda`, where settings, history,
and default exports live. Back up `state.json` before testing a beta.

To uninstall, close Jikanda and remove the app folder and its Start menu shortcut.
History remains until you deliberately remove the data folder. The app's notification
identity is under `HKCU/Software/Classes/AppUserModelId/YuPrananda.Jikanda`; this
app-specific registry key can also be removed on uninstall. No startup entries or
services are created.

## Linux and source builds

No Linux binary is included in this Windows beta. See [building](building.md) and
[release readiness](release-readiness.md) for development libraries and X11/Wayland
limitations. Linux desktop behavior remains unverified. Report bugs with version,
OS, scaling, and steps, without personal session data.
