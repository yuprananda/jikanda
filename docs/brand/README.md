# Jikanda beta brand assets

The maintainer supplied `Jikanda Brand Identity Sheet.png` and authorized the existing
mascot for the GitHub beta on 9 November 2026. Gesture, silhouette, and wordmark are
preserved. No replacement logo, tracing, auto-vectorization, or new mascot geometry
has been introduced.

The original sheet is retained. `scripts/extract-brand.ps1` exports its existing
mascot/wordmark masks. `scripts/export-app-icons.ps1` renders that mascot in white
on a dark tile, preserving proportions and clear space. The monochrome UI is kept.

Exports: PNG 16, 24, 32, 48, 64, 128, 256 px; multi-resolution Windows ICO. The ICO is
embedded in the executable; PNG is used for native window icons. The optional Start
menu shortcut uses the executable icon.

These are raster beta assets, not SVG masters. Precision vectors and further tiny
icon refinement remain future brand work; no trademark clearance is asserted.
Maintainer-supplied artwork is distributed with this project under GPL-3.0-only;
see LICENSE/LICENSING.md. The bundled font retains its OFL license.
