#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ "${1:-}" != "--internal" ]]; then
  printf '%s\n' 'Public release is not approved. Pass --internal for a local review build.' >&2
  exit 1
fi
cargo build --release --locked --manifest-path "$root/Cargo.toml"
name="Jikanda-0.1.0-linux-$(uname -m)-internal-$(date +%Y%m%d-%H%M%S)"
stage="$root/artifacts/packages/$name"
mkdir -p -- "$root/artifacts/packages"
mkdir -- "$stage"
install -m 755 -- "$root/target/release/jikanda" "$stage/jikanda"
cp -- "$root/README.md" "$root/assets/fonts/OFL-PlusJakartaSans.txt" "$stage/"
cp -R -- "$root/docs" "$stage/docs"
cp -- "$root/LICENSING.md" "$stage/"
if [[ -f "$root/LICENSE" ]]; then cp -- "$root/LICENSE" "$stage/"; fi
printf '%s\n' 'PRE-RELEASE REVIEW BUILD. Code: GPL-3.0-only; label does not restrict GPL rights. See LICENSE and LICENSING.md. Source dependency completeness, asset audit and platform tests remain release tasks.' > "$stage/INTERNAL-BUILD.txt"
mkdir -p -- "$stage/source/jikanda"
for folder in src ui assets docs scripts .github; do cp -R -- "$root/$folder" "$stage/source/jikanda/$folder"; done
for file in Cargo.toml Cargo.lock build.rs LICENSE LICENSING.md README.md CONTRIBUTING.md CHANGELOG.md AGENTS.md .gitignore 'Jikanda Brand Identity Sheet.png'; do cp -- "$root/$file" "$stage/source/jikanda/$file"; done
cargo metadata --locked --format-version 1 --manifest-path "$root/Cargo.toml" > "$stage/dependency-metadata.json"
tar -czf "$root/artifacts/packages/$name.tar.gz" -C "$stage" .
sha256sum "$root/artifacts/packages/$name.tar.gz" > "$root/artifacts/packages/$name.tar.gz.sha256"
printf '%s\n' "$root/artifacts/packages/$name.tar.gz"
