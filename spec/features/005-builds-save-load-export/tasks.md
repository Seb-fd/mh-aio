# 005 — Tasks — Builds save/load + export

- [x] T1 — Define versioned build DTO (game, pieces, decos, skills, options) — `SavedBuild` v1 shared Rust↔TS.
- [x] T2 — Implement save/list/load/delete in app-data (`builds.rs` + 5 commands).
- [x] T3 — Implement export JSON + share code + import with validation.
- [x] T4 — UI wiring in `builds/` (Saved builds card + per-result save, themed, English).
- [x] T5 — Round-trip test (export → import equality — `import_validates_and_assigns_fresh_id` + garbage rejection).
- [x] T6 — Run `npm run check` (0 errors, 0 warnings).
- [x] T7 — Run `cargo test --manifest-path src-tauri/Cargo.toml` (18/18).
- [x] T8 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T9 — Verify against `spec.md`; update docs.
