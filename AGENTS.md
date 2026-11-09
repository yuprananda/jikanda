# AGENTS.md — Jikanda Desktop Focus Companion

## 1. Project overview

This repository is an open-source, native desktop focus companion developed under **Yu Prananda**. The confirmed product name is **Jikanda**. The visual wordmark is styled **jikanda.** (lowercase with a terminal dot). Do not rename the product without explicit maintainer approval.

The product helps people intentionally spend time on work, study, and creative projects, understand their progress, and celebrate it through beautiful, shareable achievement cards.

**Product promise:** A calm, beautiful, lightweight desktop companion that makes time spent on meaningful work visible.

This is **not** a SaaS product, team productivity platform, social network, task-management suite, or a Pomodoro-only app.

## 2. Non-negotiable principles

1. **Visual quality is a feature.** Thoughtful typography, spacing, color, states, motion, and overall polish matter as much as functionality. Avoid generic enterprise dashboards and default-looking desktop controls.
2. **Native and lightweight.** Prefer efficient Rust code and Slint-native UI. Minimize background activity, dependencies, memory allocations, and unnecessary redraws.
3. **Local-first.** No mandatory account, cloud backend, internet connection, analytics, telemetry, or remote data transfer.
4. **Calm, not competitive.** Celebrate progress without shame, manipulative streak pressure, rankings, or dark patterns.
5. **Small scope, excellent execution.** Deliver a reliable, cohesive MVP before considering extensions.
6. **Cross-platform by design.** Prioritize Windows and Linux. Do not assume that window management behaves identically across operating systems or Linux display servers.
7. **Accessible and usable.** Support keyboard navigation, legible contrast, sensible scaling, clear feedback, and reduced-motion preferences where feasible.
8. **Honest progress.** Focus duration measures time in sessions, not objectively verified productivity. Never imply otherwise.

## 3. Technology decisions

- **Core language:** Rust (stable toolchain).
- **UI:** Slint using `.slint` components and Rust integration.
- **Build:** Cargo, with the standard Slint build integration when needed.
- **Platforms:** Windows and Linux initially.
- **Persistence:** Local storage only. Start with a minimal, well-defined storage abstraction. Select SQLite only when the data model justifies it; do not introduce a database prematurely.
- **Graphics/export:** Choose a local PNG generation approach after a small proof of concept. Avoid introducing a browser runtime just to render share cards.
- **Dependencies:** Keep them few, actively maintained, appropriately licensed, and justified.

Do not introduce Electron, Tauri, React, web servers, external authentication, cloud databases, or a heavy UI framework without explicit maintainer approval.

## 4. MVP scope

### 4.1 Focus sessions

Support three user-facing modes:

- **Interval:** configurable focus/break cycles, including a Pomodoro-style preset.
- **Deep Focus:** a single configurable countdown session without forced interval breaks.
- **Flow:** a count-up session with no predetermined end time.

Users must be able to start, pause, resume, and stop a session. Handle completion and interruption explicitly; do not silently treat every started session as completed.

### 4.2 Floating timer

Provide a compact, draggable, readable timer that can remain visible while the user works, subject to OS/window-manager support. It should expose only essential controls and a clear path back to the main window.

Do not assume always-on-top, translucency, precise positioning, click-through, or decorations work universally. Validate behavior on Windows, Linux X11, and Linux Wayland where test environments are available. Document unsupported behavior and provide graceful fallbacks.

### 4.3 Session history and progress

Persist completed or explicitly confirmed focus sessions locally. Record at least session mode, timestamps, elapsed focus duration, completion/interruption state, and an optional short note.

Display useful daily and weekly summaries. Make it possible to correct or delete inaccurate entries. Avoid gamification that pressures the user.

### 4.4 Shareable achievement cards

Generate aesthetically polished PNG cards using locally stored progress data. Support a small number of carefully designed templates rather than a complicated editor. Users should control the time range and which optional personal details appear. Export must work offline and must never upload data automatically.

### 4.5 Preferences

Provide practical settings for session durations, notifications, theme preferences, and floating-window behavior where supported.

## 5. Out of scope for the MVP

- User accounts, synchronization, cloud backups, or public profiles.
- Leaderboards, social feeds, friend lists, or in-app messaging.
- Team features, billing, subscriptions, or advertisements.
- AI features, chatbots, or unnecessary integrations.
- Full calendar, task manager, or project management features.
- Excessive themes, plugin systems, and elaborate customization.
- Automatic activity surveillance, screenshots, keystroke logging, or claims to detect actual concentration.

Suggest such features only as future possibilities; do not implement them without approval.

## 6. UI and interaction direction

Aim for a **calm, refined, modern desktop aesthetic** with personality. Use the Jikanda brand direction defined in Section 6A, while treating exact visual specifications and logo artwork as provisional until the maintainer approves production-ready vector assets.

- Establish reusable design tokens for colors, typography, spacing, corner radii, and motion.
- Keep the main window focused and uncluttered.
- Make the floating timer beautiful even at small dimensions.
- Prefer intentional hierarchy, generous breathing room, and subtle feedback over decorative complexity.
- Use animation sparingly and avoid continuous effects that waste CPU/GPU while idle.
- Make progress visuals informative without resembling a corporate analytics dashboard.
- Treat exported share cards as first-class visual artifacts, with intentional composition and typography.
- Avoid placeholder icons, arbitrary gradients, excessive glass effects, and stock dashboard layouts.
- Do not claim a design is final until the maintainer reviews it.

## 6A. Jikanda brand identity and asset rules

### Name, story, and positioning

- **Product name:** Jikanda (official name in documentation, release titles, and application metadata).
- **Visual wordmark:** `jikanda.` in lowercase with a terminal period; do not append the dot mechanically to every textual mention of the product.
- **Tagline:** *Make time for what matters.*
- **Brand signature:** *by Yu Prananda*, where appropriate and unobtrusive.
- **Brand pillars:** calm, thoughtful, approachable, subtly playful, polished, personal, and lightweight.
- **Brand idea:** Help people make room for focused work, reflect on time spent, and celebrate their progress without productivity guilt.
- **Origin:** The coined name was inspired by the Japanese expression *soro-soro, jikan da* (roughly, “it's about time”), particularly a humorous meme involving someone checking their wristwatch. This is an internal creative origin, **not** a claim that “Jikanda” itself is a standard Japanese word. The joke may inform tasteful Easter eggs, but should not dominate the public identity.
- **Positioning:** A premium-feeling, open-source, native desktop focus companion; not a generic Pomodoro brand or an anime/Japanese-themed app.

### Approved logo direction (concept approval, not final vector approval)

The maintainer selected a **minimal stick-figure mascot checking a wristwatch** as the primary logo concept. This is the most distinctive Jikanda visual. Preserve the recognizable wrist-checking gesture.

- Simple round head, minimal two-eye expression, rounded and consistent stroke weight.
- One arm raised/bent toward the wristwatch; pose must read clearly without relying on intricate details.
- Friendly but restrained, professional rather than childish or heavily illustrated.
- Flat, preferably one-color geometry, reproducible as clean SVG paths; no required gradients, shadows, raster textures, or excessive decoration.
- Balance negative space, stroke joins, visual weight, and proportion on a geometric grid.
- Must remain recognizable in small desktop/taskbar/tray contexts; test at 16, 24, 32, 48, and 64 px, and simplify for tiny sizes if needed.
- Avoid replacing this concept with a generic clock circle, stopwatch, tomato, anime character, or an unrelated mascot.
- The reference brand sheet is a **visual concept only**. Do not trace or auto-vectorize it blindly; redraw precise vector assets and obtain approval before treating them as final.

### Logo system — four requested variants

1. **Full lockup:** wristwatch-checking stickman + `jikanda.` wordmark, with carefully balanced spacing; primary use on landing pages, about screens, README, and release artwork.
2. **Mascot / symbol only:** the stickman without lettering; intended for app icons, avatars, tray/dock use (with a simplified micro-icon if necessary).
3. **Full wordmark only:** `jikanda.` using the chosen type treatment; suitable for narrow headers and branded text areas.
4. **Abbreviated wordmark / monogram:** `j.` as a compact secondary mark, not a replacement for the primary mascot.

Provide monochrome dark-on-light and light-on-dark versions. Prefer transparent SVG master assets and export raster sizes from those masters. Preserve a safe clear-space margin and never stretch, skew, add outlines, or change proportions casually.

### Color direction (provisional concept-sheet tokens)

- **Primary purple:** `#4B2E83`
- **Accent lavender:** `#A78BFA`
- **Soft background:** `#F5F1FF`
- **Dark ink:** `#2D1F4F`

These hex values are taken from the exploratory brand sheet and **are not yet an approved production palette**. Use them as initial design tokens, check contrast in context, and present refinements for approval. Design both light and dark variants with equal care. Do not create a gradient-heavy, neon, or overly decorative identity.

### Typography direction (provisional)

Explore **Nunito Sans** and **Plus Jakarta Sans** or similarly well-crafted, legally usable rounded geometric sans-serif typefaces. The wordmark should be friendly but mature, with intentional kerning and proportion; do not assume ordinary font rendering alone is the final custom logotype. Verify font licensing and bundle only the required assets. Do not redistribute unlicensed font files.

### Brand usage in product UI

- The mascot is a quiet companion, not an intrusive assistant or a mandatory animated character.
- Use the mascot selectively in welcome, empty, break, and completed-session states, without visual clutter.
- Achievement cards should highlight the **user's accomplishment**; keep the Jikanda signature small and secondary.
- Motion should be subtle, purposeful, optional/reduced-motion aware, and low-cost when idle.
- Keep the interface mature and uncluttered; avoid kawaii overload, kanji decoration, anime styling, and forced Japanese references.
- Do not use the original meme image, a real person's likeness, or copyrighted meme artwork as a brand asset.
- Public wording should be truthful: session time is recorded focus time, not verified work output.

### Brand approval and repository asset conventions

- Maintain original editable source assets (SVG) and an explicit export process; don't rely on image-generation previews as the sole production source.
- Suggested structure, subject to the existing repository: `assets/brand/source/` for editable masters, `assets/brand/exports/` for PNG/ICO and other generated outputs, and `docs/brand/` for usage guidelines.
- If concept images are available, treat them as visual references only; do not invent paths or claim files exist without checking.
- Ask the maintainer to approve the final mascot geometry, wordmark, palette, and icon-size variants before release.
- Name availability was checked only preliminarily; **do not claim Jikanda is legally cleared**. Conduct appropriate trademark checks before public launch.

## 7. Architecture and code quality

Keep the implementation modular without overengineering. Separate at least these responsibilities:

- **Domain:** session modes, lifecycle, time accounting, validation, and statistics.
- **Application:** commands and state coordination.
- **Persistence:** reading/writing settings and session records.
- **Platform integration:** windows, notifications, startup behavior, and OS-specific capabilities.
- **Presentation:** Slint views, components, styling, and bindings.
- **Export:** achievement-card composition and PNG generation.

Favor explicit types, small functions, meaningful names, and straightforward control flow. Avoid premature abstractions, global mutable state, unnecessary threads, and blocking work on the UI thread.

Keep OS-specific code behind clear interfaces. Document platform limitations instead of hiding them with unreliable workarounds.

## 8. Timer correctness

Timer accuracy is essential.

- Use a monotonic clock for measuring elapsed time while the application runs; do not derive elapsed focus time by counting UI refresh ticks.
- Keep timer state separate from the rendering refresh rate.
- Handle pause/resume without double-counting elapsed time.
- Decide and document how suspend/resume, application restart, clock changes, and interrupted sessions behave before implementing persistence of active sessions.
- Never fabricate completed time after a crash or unverified interruption.
- Update the UI only as often as needed for a responsive display.
- Test boundary conditions such as zero duration, rapid pause/resume, and session completion.

## 9. Performance targets

These are **aspirational targets, not guaranteed Slint benchmarks**:

- Idle RAM: target **50 MB or less** for a release build in a representative environment.
- Idle CPU: approximately **0%** when no interaction or animation is occurring.
- Startup: aim for **1 second or less** on a reasonable reference machine.
- Timer remains responsive and accurate during ordinary desktop usage.

Measure real process memory and CPU on actual Windows/Linux builds. Record OS, hardware, build profile, measurement method, and whether main/floating windows are open. Do not optimize solely from assumptions or report targets as achieved results.

## 10. Privacy and data handling

- Store data on the user's device.
- Make the storage location discoverable and documented.
- Never send session history, notes, or usage events to remote services.
- Do not collect credentials or sensitive information unnecessarily.
- Provide a clear way to delete local history.
- Export only what the user explicitly chooses.
- Avoid logging personal session notes in production diagnostics.

## 11. Development workflow for coding agents

Before coding:

1. Inspect the repository and existing decisions. Do not assume a blank project.
2. Explain the intended change, impacted files, and any major trade-offs.
3. If a requirement is ambiguous or affects product direction, ask the maintainer instead of inventing behavior.
4. For major UI changes, propose a compact design direction or prototype first.

During implementation:

1. Work in small, reviewable increments.
2. Keep changes limited to the requested scope.
3. Prefer existing patterns and reusable components.
4. Add or update tests for domain logic and bug fixes.
5. Keep documentation in sync with actual behavior.
6. Do not silently change product naming, scope, licenses, dependencies, or platform support.

After implementation:

1. Run formatting, linting, tests, and build checks appropriate to the repository (for example `cargo fmt --check`, `cargo clippy`, and `cargo test`).
2. Clearly state which checks passed, failed, or could not run.
3. Summarize what changed, why, and any remaining limitations.
4. Never claim that a visual or platform behavior was tested if it was not actually tested.

## 12. Suggested delivery sequence

**Phase 0 — Technical feasibility spikes**
- Confirm Slint rendering quality with a small polished timer prototype.
- Verify floating-window behavior on target environments.
- Benchmark basic idle memory/CPU in release mode.
- Verify a minimal local PNG export path.

**Phase 1 — Visual foundation**
- Establish design tokens and core components.
- Build main-window and floating-timer prototypes.
- Review visual direction before expanding features.

**Phase 2 — Reliable focus engine**
- Implement Interval, Deep Focus, and Flow.
- Implement pause/resume/stop and accurate time accounting.
- Integrate floating controls and notifications.

**Phase 3 — Local progress**
- Persist sessions and settings.
- Add history, optional notes, and daily/weekly summaries.

**Phase 4 — Share experience**
- Build polished achievement-card templates.
- Export PNG locally and validate output at intended share sizes.

**Phase 5 — Release readiness**
- Test Windows/Linux packages and OS-specific behavior.
- Improve accessibility, error handling, and performance.
- Add installation instructions, screenshots, contribution guidelines, and release notes.

Do not start a later phase just because it is listed here; obtain maintainer agreement when moving between major phases.

## 13. Open-source and licensing

The project is intended to be free and open-source. **Do not select a final application license without maintainer approval.** Before release, verify that the chosen application license is compatible with the exact Slint licensing option, third-party dependencies, fonts, icons, and assets used. Include required attribution and notices.

Avoid incorporating unlicensed artwork, fonts, or code. Keep branding assets and licensing decisions explicit.

## 14. Definition of done

A feature is complete only when:

- It works for its intended scenario and handles expected errors.
- Relevant logic has tests where practical.
- UI states are visually coherent and usable at supported sizes.
- It does not introduce unnecessary idle CPU/memory usage.
- It respects offline/privacy constraints.
- Cross-platform limitations are documented.
- The maintainer has a clear summary of what was verified and what still needs testing.

**Final reminder:** Build a product people enjoy keeping open, feel proud to use, and willingly recommend—not merely a timer that happens to work.
