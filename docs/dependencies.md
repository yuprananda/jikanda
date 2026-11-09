# Dependency distribution for the Windows beta

The active Windows normal/build graph contains 250 packages. Branch choices,
retained texts, and provenance are in [license-review.md](license-review.md).
The complete vendor collection contains 542 source crates, including inactive
optional/platform packages, inventoried in [source-license-inventory.md](source-license-inventory.md).
Those inactive sources are not all linked into the Windows executable.

- Slint 1.18.1 and its UI/compiler components: GPL-3.0-only option selected.
- winresource 0.1.34: build-only icon/version embedding, MIT. No runtime framework.
- png, serde/serde_json, chrono: local graphics, persistence and timestamps.
- notify-rust: OS notifications; its tauri-winrt-notification dependency is only
  a Windows API binding, not an application framework or web runtime.
- windows-sys/raw-window-handle: Windows window, registry, audio and power APIs.
- zbus on Linux: local D-Bus/logind integration, no cloud service.
- Plus Jakarta Sans: unmodified font with SIL OFL 1.1 retained.
- Original alarm signals and generator: GPL-3.0-only, no third-party samples.

There is no new Rust audio engine. Windows playback uses the OS; Linux optionally
uses paplay/aplay. Original upstream copyright/notice files are retained in the
release and corresponding-source archive. Some inactive historical source crates
omit standalone texts; their declarations and canonical-text provenance are
explicitly recorded, without inventing copyright ownership or years.

See [LICENSING](../LICENSING.md) for distribution and source requirements.
