# Jikanda 0.1.0-beta.1 release readiness

Status: **Windows beta** prepared for the maintainer-requested GitHub release.
The checks below describe this beta; they do not claim stable support across all
Windows/Linux desktops. Publication requires the linked CI checks to pass.

## Completed local checks

- Windows 11 Home Single Language 10.0.26200 x64, Intel i5-13420H, 16 GB RAM,
  desktop scaling 125%. Rust stable 1.95; MSRV declaration 1.92.
- Formatting, Clippy with warnings denied, 38 unit tests, and release build passed.
- Native integration: all three modes, pause/resume, finish/save/discard, corrected
  history, session name/task snapshots, settings persistence, two real-data PNG
  exports, language changes, modal guards, and persisted quit passed.
- Main/mini native windows, WS_EX_TOPMOST, mini-close recovery, and main-close exit passed.
- Responsive render checks cover 1000x680 / 1280x800 logical px, both languages,
  all pages/preferences, long titles/tasks and 1234:56:59 time displays.
- Windows notification banner was confirmed visible by the maintainer on 9 November.
  Three alarm playback probes returned success; automatic tests are silent.
- Existing maintainer-supplied mascot exported at 16/24/32/48/64/128/256 px.
  The beta exe contains icon/group-icon/version PE resources and Jikanda publisher
  metadata. Native windows and optional Start menu shortcut use this icon.
- Binary is x64, GUI subsystem, **unsigned**, and imports VCRUNTIME140.dll.
  Installation instructions link Microsoft's official x64 runtime if needed.
- 250 actual Windows normal/build dependency packages have recorded license branch
  selections and retained notices. All 542 vendored crate sources have a separate
  inventory, including provenance for upstream omissions in inactive source crates.
- Original font license, brand provenance, alarm generator/license, bilingual
  installation, screenshots, bug-report template and beta notes are prepared.

## Measured Windows sample

See [raw report](windows-performance-beta1.json), bound to its executable SHA-256.
Release, thin LTO, software renderer, accessibility enabled; isolated probe data.
One warm run per case, 3-second warm-up, 5-second sample, process CPU delta and
end-of-sample memory. This is not a cold-start/first-paint or large-history benchmark.

| Window layout | Idle working set | Idle private bytes | Idle CPU (one core) |
| --- | --- | --- | --- |
| Main | 29.46 MiB | 10.17 MiB | 0% sampled |
| Mini | 23.11 MiB | 4.79 MiB | 0% sampled |
| Both | 29.05 MiB | 10.39 MiB | 0% sampled |

Native-window-handle startup was 61.6-144.2 ms in these warm probes. Running main
sampled 0.312% of one core; other running cases rounded to 0% at the sampling
resolution. These are measurements on this machine, not guaranteed cross-device targets.

## Final preparation checks

- Offline source build with an empty Cargo home: **passed**, Rust 1.95 release build, 5m 57s.
  Only the vendored archive supplied Cargo crates; no registry source cache was present.
- Extracted Windows package integration: **passed**, including isolated data,
  session/task/settings persistence and PNG exports. Optional shortcut script also passed.
  Release ZIPs are CRC-checked and bound to SHA256SUMS.txt during final packaging.
- GitHub Actions Windows/Linux build checks: [code baseline ed1b356](https://github.com/yuprananda/jikanda/actions/runs/42561825342).
  A new Clippy lint in Rust 1.99 was corrected in an audio unit test; Windows release
  runtime behavior was unchanged. Final release notes record the completed CI outcome.
- Release assets: Windows x64 ZIP, complete vendored source ZIP, SHA256SUMS.txt,
  plus public source/notes at the matching GitHub release tag. No user data is packaged.

## Beta limitations to disclose

- Windows 11 x64 is the tested desktop. Windows 10/ARM64 are not verified.
- No Linux binary in this beta. Build-only CI is not X11/Wayland runtime validation.
  Topmost/positioning depend on the compositor. Linux opacity is unavailable;
  audio optionally requires paplay/aplay.
- Physical sleep/hibernate, more hardware/scaling/multi-monitor combinations,
  assistive technology and long-term real usage still need community/device tests.
  Programmatic snapshots do not substitute for those checks.
- Active sessions are not restored/credited after crashes or restart; users should
  back up state.json before beta testing.
- Clicking a Windows toast does not activate the main window yet.
- Brand uses approved-for-beta existing raster assets; vector masters and further
  micro-icon refinement remain future work. No trademark clearance is asserted.
- Preliminary exact-name web checks on 9 November found unrelated uses of Jikanda
  and no established clearance. Indexed searches of PDKI/WIPO are not exhaustive
  registry searches or a legal opinion. Preserve the maintainer's chosen name;
  further trademark review remains appropriate before a commercial/stable launch.

## Reproducible preparation

See scripts/review-licenses.py, scripts/prepare-release.py, scripts/export-app-icons.ps1,
scripts/inspect-windows-artifact.py, the committed lockfile, and docs/building.md.
The GitHub source excludes target/artifacts, real history, credentials, and test data.
Screenshots use only diagnostic example sessions/tasks.

The earlier internal review record is retained in
[history](history/release-readiness-before-beta.md). Its old sizes, palette,
packaging status and source-audit gaps are superseded by this beta record.
