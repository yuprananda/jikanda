# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
"""Inventory actual Windows normal/build crates and retain upstream license texts.

Requires Cargo; --fetch-missing uses an authenticated gh CLI for pinned upstream
texts omitted from a published crate. No registry/vendor sources are modified.
"""
import argparse
import base64
import json
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
TARGET = "x86_64-pc-windows-msvc"


def command(*args):
    return subprocess.check_output(args, cwd=ROOT, encoding="utf-8")


def selected(expression):
    if expression == "(MIT OR Apache-2.0) AND Unicode-3.0":
        return "MIT AND Unicode-3.0"
    if expression == "Apache-2.0 AND MIT":
        return expression
    normalized = expression.replace("/", " OR ")
    choices = normalized.split(" OR ")
    for license_name in ["GPL-3.0-only", "MIT", "Apache-2.0", "BSD-3-Clause", "BSD-2-Clause", "ISC", "Zlib", "BSL-1.0", "CC0-1.0", "Unlicense", "Unicode-3.0", "0BSD"]:
        if license_name in choices:
            return license_name
    raise RuntimeError(f"License expression needs manual review: {expression}")


def license_entries(folder):
    return [p for p in folder.iterdir() if re.match(r"LICENSE|COPYING|NOTICE|UNLICENSE", p.name, re.I)]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fetch-missing", action="store_true")
    parser.add_argument("--all-lockfile", action="store_true", help="Retain notices for unlinked platform sources in the vendor archive too")
    args = parser.parse_args()
    metadata_args = [] if args.all_lockfile else ["--filter-platform", TARGET]
    metadata = json.loads(command("cargo", "metadata", "--locked", "--format-version", "1", *metadata_args))
    tree = command("cargo", "tree", "--locked", "--target", TARGET, "--edges", "normal,build", "--prefix", "none", "--format", "{p}")
    used = set(re.findall(r"^(\S+) v(\S+)", tree, re.M))
    packages = sorted((p for p in metadata["packages"] if (args.all_lockfile or (p["name"], p["version"]) in used) and p["name"] != "jikanda"), key=lambda p: (p["name"], p["version"]))
    output = ROOT / "docs" / "licenses"
    output.mkdir(parents=True, exist_ok=True)
    rows = []
    upstream_cache = {}
    for package in packages:
        folder = Path(package["manifest_path"]).parent
        entries = license_entries(folder)
        explicit = package.get("license_file")
        if explicit and (folder / explicit).is_file():
            entries.append(folder / explicit)
        destination = output / f"{package['name']}-{package['version']}"
        destination.mkdir(exist_ok=True)
        for entry in set(entries):
            if entry.is_dir():
                shutil.copytree(entry, destination / entry.name, dirs_exist_ok=True)
            else:
                shutil.copy2(entry, destination / entry.name)
        provenance = "Published Cargo crate (unmodified); original notices retained."
        if not entries:
            if (destination / "UPSTREAM-SOURCE.json").is_file():
                provenance = "Recorded upstream provenance; see UPSTREAM-SOURCE.json."
            elif not args.fetch_missing:
                if not (destination / "UPSTREAM-SOURCE.json").is_file():
                    raise RuntimeError(f"Missing text for {package['name']}; rerun with --fetch-missing")
                provenance = "Pinned upstream text; see UPSTREAM-SOURCE.json."
            else:
                repository = re.sub(r"^https?://github.com/", "", package["repository"]).removesuffix(".git").strip("/")
                if len(repository.split("/")) != 2:
                    raise RuntimeError(f"Review non-GitHub upstream manually: {repository}")
                vcs = json.loads((folder / ".cargo_vcs_info.json").read_text())
                commit = vcs["git"]["sha1"]
                key = (repository, commit)
                if key not in upstream_cache:
                    listing = json.loads(command("gh", "api", f"repos/{repository}/contents?ref={commit}"))
                    texts = {}
                    for entry in listing:
                        if entry["type"] == "file" and re.match(r"LICENSE|COPYING|NOTICE|UNLICENSE", entry["name"], re.I):
                            data = json.loads(command("gh", "api", f"repos/{repository}/contents/{entry['path']}?ref={commit}"))
                            texts[entry["name"]] = base64.b64decode(data["content"])
                    if not texts and args.all_lockfile and (package["name"], package["version"]) not in used:
                        # Some inactive historical platform crates declare a license
                        # in Cargo.toml but omit its standalone text even upstream.
                        # Retain their source/headers and record this transparently;
                        # never use this fallback for a Windows-linked package.
                        canonical = selected(package["license"] or "")
                        if canonical not in {"MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "BSL-1.0"}:
                            raise RuntimeError(f"Review omitted upstream text manually: {repository}")
                        spdx_ref = "31846481de7ea17d92f2f46fad76abe58b2e9bb0"
                        data = json.loads(command("gh", "api", f"repos/spdx/license-list-data/contents/text/{canonical}.txt?ref={spdx_ref}"))
                        texts[f"CANONICAL-{canonical}.txt"] = base64.b64decode(data["content"])
                        declaration = {"note": "Inactive source only, not linked into Windows. Upstream omits standalone text; original source/headers and declared license retained. Canonical terms supplied without inventing copyright holders or dates.", "declared_license": package["license"], "authors_from_cargo": package["authors"], "canonical_source": f"https://github.com/spdx/license-list-data/blob/{spdx_ref}/text/{canonical}.txt"}
                        texts["DECLARATION.json"] = (json.dumps(declaration, indent=2) + "\n").encode("utf-8")
                    if not texts:
                        raise RuntimeError(f"No license texts at {repository}@{commit}; review manually")
                    upstream_cache[key] = texts
                for name, content in upstream_cache[key].items():
                    (destination / name).write_bytes(content)
                (destination / "UPSTREAM-SOURCE.json").write_text(json.dumps({"repository": "https://github.com/" + repository, "commit": commit, "files": list(upstream_cache[key])}, indent=2) + "\n")
                provenance = f"Pinned upstream {commit}; see UPSTREAM-SOURCE.json."
        choice = "Unlinked source collection; original terms retained" if args.all_lockfile else selected(package["license"] or "")
        rows.append({"name": package["name"], "version": package["version"], "declared": package["license"], "selected": choice, "repository": package["repository"], "texts": destination.relative_to(ROOT).as_posix(), "provenance": provenance})
    filename = "review-all-source.json" if args.all_lockfile else "review.json"
    (output / filename).write_text(json.dumps({"target": "All lockfile sources" if args.all_lockfile else TARGET, "scope": "Unlinked source collection" if args.all_lockfile else "Actual cargo tree normal/build dependencies; not inactive optional packages", "packages": rows}, indent=2) + "\n", encoding="utf-8")
    lines = ["# Windows beta dependency license review", "", f"Target: `{TARGET}`. {len(rows)} normal/build dependency packages from Cargo.lock.", "", "Slint uses the GPL-3.0-only branch, not a Royalty-free/commercial option.", "Permissive branches are selected explicitly below; AND obligations are retained.", "All original copied texts remain under their upstream terms. Vendored source is", "distributed separately with original headers. Toolchain/system libraries are not bundled.", "", "This is a distribution inventory and recorded review, not trademark clearance or", "a warranty about third-party rights. Fonts and project artwork are documented in LICENSING.md.", "", "| Package | Declared | Selected | Texts |", "| --- | --- | --- | --- |"]
    for row in rows:
        lines.append(f"| {row['name']} {row['version']} | {row['declared']} | {row['selected']} | [notices](licenses/{row['name']}-{row['version']}/) |")
    if args.all_lockfile:
        lines[:12] = ["# Vendored source license inventory", "", "All lockfile crates are included as a source collection, including inactive", "optional/platform crates. Their original license terms and notices are retained.", "This is not a claim that all of them are linked into the Windows binary.", "For Windows branch selections, see license-review.md.", ""]
    (ROOT / "docs" / ("source-license-inventory.md" if args.all_lockfile else "license-review.md")).write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Reviewed {len(rows)} {'source' if args.all_lockfile else 'Windows'} packages; notices and provenance recorded.")


if __name__ == "__main__":
    main()
