# 008 — Mobile & distribution

## Context

- Problem: Phase 6 pending — mobile build, auto-updater, tray, cross-platform packaging (roadmap Phase 6). Lib already supports `cdylib` + `staticlib`; `src-tauri/gen/` is git-ignored (mobile targets not initialized).
- Current state: desktop-first Tauri v2 (Windows/macOS/Linux via `.github/workflows/release.yml` semver tags).
- Related docs: `roadmap.md` Phase 6, `AGENTS.md` Mobile Support + Release & Versioning.

## User stories

- As a user, I want signed installers/auto-updates so that I stay current.
- As a mobile user, I want the encyclopedia usable on Android/iOS where feasible.

## Scope

### In scope

- Mobile targets init + dev/build smoke (Android first, iOS where macOS+Xcode available).
- Auto-updater wiring, tray (if approved), release packaging matrix.

### Out of scope

- PWA (not planned — Tauri-focused). Store listings copy.

## Acceptance criteria

- [ ] `npx tauri android dev/build` path documented and reproducible (arch-matched emulator).
- [ ] Version sync (`package.json` + `src-tauri/tauri.conf.json`) enforced via `scripts/check-version.js`.
- [ ] Release workflow (`release.yml`) understood; no local tag pushed without explicit request.
- [ ] `npm run check` passes with 0 errors, 0 warnings.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- None (no DB change expected).

## Constraints

- STRICT git workflow: no commit/tag/push/PR without explicit user instruction; pre-push CI gate mandatory.
- `src-tauri/gen/` stays git-ignored; mobile reqs (SDK/NDK, `ANDROID_HOME`, JDK; Xcode for iOS) documented, not committed.
