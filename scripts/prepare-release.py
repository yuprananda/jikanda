# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
"""Prepare Windows beta ZIP, complete vendored source ZIP and SHA-256 manifest.

Run after checks and review-licenses.py. Nothing is uploaded or published here.
Python 3.11+; system SDK/toolchain are intentionally not bundled.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def copy_file(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)


def archive(folder, destination):
    with zipfile.ZipFile(destination, "w", zipfile.ZIP_DEFLATED, compresslevel=6, strict_timestamps=False) as output:
        for file in sorted(folder.rglob("*")):
            if file.is_file():
                output.write(file, Path(folder.name) / file.relative_to(folder))


def main():
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"]
    if "-beta." not in version:
        raise RuntimeError("This preparer targets beta releases; review stable release policy separately")
    executable = ROOT / "target" / "release" / "jikanda.exe"
    vendor = ROOT / "artifacts" / f"vendor-{version}"
    if not executable.is_file() or not vendor.is_dir():
        raise RuntimeError("Build release and run cargo vendor --locked --versioned-dirs artifacts/vendor-<version> first")
    review = ROOT / "docs" / "licenses" / "review.json"
    reviewed = json.loads(review.read_text())
    release = ROOT / "artifacts" / "releases" / f"v{version}"
    release.mkdir(parents=True, exist_ok=True)
    stage = ROOT / "artifacts" / "release-staging" / f"v{version}"
    binary = stage / f"Jikanda-{version}-windows-x64"
    source = stage / f"Jikanda-{version}-source"
    binary.mkdir(parents=True, exist_ok=True)
    source.mkdir(parents=True, exist_ok=True)
    for name in ["README.md", "LICENSE", "LICENSING.md", "CHANGELOG.md", "CONTRIBUTING.md"]:
        copy_file(ROOT / name, binary / name)
    copy_file(executable, binary / "jikanda.exe")
    copy_file(ROOT / "scripts" / "install-windows.ps1", binary / "install-windows.ps1")
    shutil.copytree(ROOT / "docs", binary / "docs", dirs_exist_ok=True)
    shutil.copytree(ROOT / "assets" / "brand" / "exports", binary / "assets" / "brand" / "exports", dirs_exist_ok=True)
    copy_file(ROOT / "assets" / "fonts" / "OFL-PlusJakartaSans.txt", binary / "FONT-LICENSE.txt")
    copy_file(ROOT / "assets" / "audio" / "README.md", binary / "AUDIO-LICENSE.md")
    notice = [f"Jikanda {version} - Windows dependency notices", "", "GPL-3.0-only application; Slint GPLv3 option.", "Full upstream texts: docs/licenses. Font: FONT-LICENSE.txt. Audio: AUDIO-LICENSE.md.", "Complete corresponding source with vendor crates is provided as a separate asset in the same release.", ""]
    for package in reviewed["packages"]:
        notice.append(f"{package['name']} {package['version']} | {package['selected']} | {package['repository']}")
    (binary / "THIRD-PARTY-NOTICES.txt").write_text("\n".join(notice) + "\n", encoding="utf-8")
    for name in ["src", "ui", "assets", "docs", "scripts", ".github"]:
        shutil.copytree(ROOT / name, source / name, dirs_exist_ok=True, ignore=shutil.ignore_patterns("__pycache__", "*.pyc"))
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", "LICENSE", "LICENSING.md", "README.md", "CHANGELOG.md", "CONTRIBUTING.md", "AGENTS.md", ".gitignore", ".gitattributes", "Jikanda Brand Identity Sheet.png"]:
        copy_file(ROOT / name, source / name)
    vendor_destination = source / "vendor"
    if not vendor_destination.exists():
        # Same-volume hardlinks avoid a second large copy. ZIP entries still carry
        # full independent contents; no external links are used in the archive.
        try:
            shutil.copytree(vendor, vendor_destination, copy_function=os.link)
        except OSError:
            shutil.copytree(vendor, vendor_destination, dirs_exist_ok=True)
    config = source / ".cargo" / "config.toml"
    config.parent.mkdir(exist_ok=True)
    config.write_text('[source.crates-io]\nreplace-with = "vendored-sources"\n\n[source.vendored-sources]\ndirectory = "vendor"\n', encoding="utf-8")
    copy_file(binary / "THIRD-PARTY-NOTICES.txt", source / "THIRD-PARTY-NOTICES.txt")
    (source / "SOURCE-README.txt").write_text("Corresponding source for Jikanda " + version + ".\nFrom this folder: cargo build --release --locked --offline\nRequires stable Rust >=1.92, MSVC C++ tools and Windows SDK on Windows.\nNo Rust toolchain or system libraries are bundled.\n", encoding="utf-8")
    for folder in [binary, source]:
        destination = release / f"{folder.name}.zip"
        archive(folder, destination)
        print(f"Created {destination.name}: {destination.stat().st_size / 1048576:.1f} MiB", flush=True)
    files = sorted(release.glob("*.zip"))
    checksums = []
    for file in files:
        with file.open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        checksums.append(f"{digest}  {file.name}")
    (release / "SHA256SUMS.txt").write_text("\n".join(checksums) + "\n", encoding="ascii")
    print(f"Prepared locally: {release}")


if __name__ == "__main__":
    main()
