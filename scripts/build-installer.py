# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
"""Build an installer from the already published, verified portable ZIP.

Never rebuild/change the app binary or overwrite the original beta source ZIP.
The additional source archive supplies the installer script and build instructions.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[1]
VERSION = "0.1.0-beta.1"
PORTABLE_SHA256 = "0af72c10633de277e632371c27a2bff9a49c14842f47e7adc6d7da3c258faa47"


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--compiler", required=True, type=Path)
    args = parser.parse_args()
    release = ROOT / "artifacts" / "releases" / f"v{VERSION}"
    portable = release / f"Jikanda-{VERSION}-windows-x64.zip"
    if digest(portable) != PORTABLE_SHA256:
        raise RuntimeError("Portable ZIP differs from the published beta; do not substitute an unreviewed app build")
    staging = ROOT / "artifacts" / "installer-staging" / VERSION
    staging.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(portable) as archive:
        if archive.testzip() is not None:
            raise RuntimeError("Portable ZIP integrity check failed")
        archive.extractall(staging)
    payload = staging / f"Jikanda-{VERSION}-windows-x64"
    notices = ROOT / "installer" / "notices"
    shutil.copytree(notices, payload / "installer-notices", dirs_exist_ok=True)
    subprocess.run([str(args.compiler.resolve()), f"/DAppVersion={VERSION}",
                    f"/DPayloadDir={payload}", f"/DReleaseDir={release}",
                    f"/DIconFile={ROOT / 'assets/brand/exports/jikanda.ico'}",
                    str(ROOT / "installer/jikanda.iss")], check=True)
    source_zip = release / f"Jikanda-{VERSION}-installer-source.zip"
    with zipfile.ZipFile(source_zip, "w", zipfile.ZIP_DEFLATED) as archive:
        for file in sorted((ROOT / "installer").rglob("*")):
            if file.is_file():
                archive.write(file, file.relative_to(ROOT))
        archive.write(ROOT / "scripts/build-installer.py", "scripts/build-installer.py")
        archive.write(ROOT / "scripts/verify-installer.ps1", "scripts/verify-installer.ps1")
        archive.write(ROOT / "scripts/verify-native.ps1", "scripts/verify-native.ps1")
        archive.write(ROOT / "LICENSE", "LICENSE")
        archive.write(ROOT / "assets/brand/exports/jikanda.ico", "assets/brand/exports/jikanda.ico")
    # Keep all original asset hashes unchanged; add the setup EXE and its source.
    assets = sorted([*release.glob("*.zip"), *release.glob("*.exe")])
    (release / "SHA256SUMS.txt").write_text("".join(f"{digest(file)}  {file.name}\n" for file in assets), encoding="ascii")
    exe = release / f"Jikanda-{VERSION}-windows-x64-setup.exe"
    print(json.dumps({"setup": exe.name, "sha256": digest(exe),
                      "payload_exe_sha256": digest(payload / "jikanda.exe"),
                      "size_bytes": exe.stat().st_size}, indent=2))


if __name__ == "__main__":
    main()
