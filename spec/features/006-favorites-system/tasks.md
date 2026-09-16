# 006 — Tasks — Favorites system

- [x] T1 — Decide storage (app-data `favorites.json`, consistent with 005) and document.
- [x] T2 — Implement favorites store + toggle (`favorites.rs` + commands + Svelte cache store + button).
- [x] T3 — Wire list + detail UIs (7 lists with ★ filter + row stars; 8 details via `DetailHeader` props).
- [x] T4 — Favorites view/filter (per-list ★ chip; no separate route by design).
- [x] T5 — Persistence-across-restart check (`favorites_survive_reload` + app-data file).
- [x] T6 — Run `npm run check` (0 errors, 0 warnings).
- [x] T7 — Run `cargo test --manifest-path src-tauri/Cargo.toml` (22/22).
- [x] T8 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T9 — Verify against `spec.md`; update docs.
