# Build and verify

Use Rust stable 1.92+ and the committed Cargo.lock. Windows requires MSVC C++ Build
Tools and the Windows SDK, including its resource compiler. The build-only
winresource crate embeds icon/version metadata; it adds no application runtime.

```sh
cargo build --release --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## Offline corresponding source archive

Extract the separate source ZIP. It includes source, assets/generators, Cargo.lock,
vendor crates, notices, and `.cargo/config.toml`. With Rust and system tools installed:

```sh
cargo build --release --locked --offline
```

Do not override its bundled vendor configuration. Rust, MSVC, the Windows SDK and
platform system libraries are not bundled. This does not promise bit-for-bit
reproducibility across different toolchain versions.

## Linux source development

For an Ubuntu-like system:

```sh
sudo apt-get install build-essential pkg-config libfontconfig1-dev libxkbcommon-dev libwayland-dev libx11-dev libxcb1-dev libudev-dev
cargo build --release --locked
```

Audio optionally uses paplay (PulseAudio/PipeWire) or aplay (ALSA). Notifications need
a session D-Bus notification service. Wayland window positioning/topmost depend on
the compositor; opacity currently falls back to opaque. No Linux runtime binary is
claimed tested in this Windows beta.

## Windows integration and asset checks

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify-native.ps1 -TimeoutSeconds 120
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify-windows.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/measure-windows.ps1
```

Diagnostic probes use isolated data in artifacts. Automated tests suppress real
notifications and audio. `jikanda.exe --probe-alarm=0` (or 1/2) explicitly previews
one sound. Regenerate alarms with `python scripts/generate-alarms.py`; regenerate
Windows icons with `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/export-app-icons.ps1`.

## Prepare a Windows beta locally

After checks, export icons, run the license reviews, and vendor all locked crates:

```sh
python scripts/review-licenses.py --fetch-missing
python scripts/review-licenses.py --all-lockfile --fetch-missing
cargo vendor --locked --versioned-dirs artifacts/vendor-0.1.0-beta.1
python scripts/prepare-release.py
```

The preparer creates binary/source ZIPs and SHA256SUMS.txt in artifacts/releases.
It does not upload or publish. Validate an extracted binary ZIP, build the source
archive offline with an empty Cargo home, and require passing GitHub Actions before
publishing the beta. The older package-windows.ps1/package-linux.sh scripts are
historical internal-build helpers; they are not the public beta packager.
