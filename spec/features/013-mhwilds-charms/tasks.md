# 013 — Tasks — MHWilds charms (shipped)

- [x] T1 — Curate `src-tauri/data/mhwilds_charms.json` (76 talismans with skill names + max levels).
- [x] T2 — Seed via `seed_mhwilds_charms` (`src-tauri/src/db/seed.rs:7119`) into existing `charms`/`charm_skills` (best-effort skill resolve).
- [x] T3 — Enable the charms viewer for `mhwilds` (charms `gameOnly` in `src/routes/[game]/+page.svelte:104`).
- [x] T4 — Backfill warm-boot installs (`patch_mhwilds_012_014`, `src-tauri/src/db/seed.rs:280`).
- [x] T5 — Add coverage test `mhwilds_013_014_charms_small_endemic` (`src-tauri/src/db/queries.rs:4509`).
- [x] T6 — Run `npm run check` (0 errors, 0 warnings).
- [x] T7 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] T8 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T9 — Verify against `spec.md` acceptance criteria; document gaps (no upgrade trees/materials, no rank tree).
