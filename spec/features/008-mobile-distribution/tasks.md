# 008 — Tasks — Mobile & distribution

- [ ] T1 — Read `.github/workflows/release.yml` + document current matrix.
- [ ] T2 — Verify `npm run version:check` (package.json ↔ tauri.conf.json sync).
- [ ] T3 — Android init docs (SDK/NDK, `ANDROID_HOME`, JDK) — no commit of `gen/`.
- [ ] T4 — Android dev smoke on arch-matched emulator.
- [ ] T5 — Updater wiring assessment (signed artifacts path).
- [ ] T6 — Tray assessment (only if approved).
- [ ] T7 — Run `npm run check` (0 errors, 0 warnings).
- [ ] T8 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [ ] T9 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [ ] T10 — Verify against `spec.md`; update docs. No tag/push without explicit request.
