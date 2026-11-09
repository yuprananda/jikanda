# Windows installer validation - beta.1

Environment: Windows 11 x64, local maintainer workstation, 9 November 2026.
Build tool: official unmodified Inno Setup 7.1.1 x64, verified SHA-256 and valid
upstream Authenticode signature; provenance and license in `installer/notices/`.

`scripts/verify-installer.ps1` passed for the final setup:

- Current-user installation, Start menu shortcut, and uninstall registration.
- Installed application SHA-256 matches the originally published portable binary:
  `274f99896af809703240090ad337f1ca4a42f3563e89883f3cd78567b492caa6`.
- Setup rejects an executable held by an exclusive-write lock, without closing it.
- Reinstallation/update succeeds after releasing that lock.
- Installed application passes native integration checks: three modes,
  pause/resume, save/discard, history changes, bilingual settings, session tasks,
  alarm preferences, PNG generation, and persistence.
- Uninstall removes owned executable, shortcut, and uninstall registration.
- Unrelated installation-folder files and a local-data sentinel survive uninstall.

The test uses an artifact installation folder and isolated application test data.
Logs/snapshots are retained locally in `artifacts/installer-smoke-20261109153557/`.
The original portable ZIP and full application source ZIP were not rebuilt.

Not validated: Windows 10/ARM64, a clean machine lacking the Microsoft runtime,
interactive installer accessibility, or the optional desktop shortcut. Setup is
unsigned; upstream compiler signing does not sign the generated Jikanda installer.
Rust source is unchanged; existing successful Windows/Linux Cargo checks apply to
the identical app payload. This task adds packaging, scripts, and documentation.
