# 004 — Tasks — MHWilds data import

- [x] T1 — Define Wilds schemas (`mhwilds_*.json`, 14 files, dedicated PK offsets).
- [x] T2 — Fetch from MHDB Wilds API (`scripts/generate_mhwilds_from_mhdb.py`, dev-only; Kiranico/Game8 not needed).
- [x] T3 — Emit `src-tauri/data/mhwilds_*.json` (34 monsters, 773 items, 1188 weapons, 714 armor, 179 skills, 361 decos + junctions).
- [x] T4 — Seed all entities + junctions (Wilds section in `seed.rs`; equipment/skill-points derived; quests/charms deferred — no upstream/schema support).
- [x] T5 — Double-seed stability check (`seed_is_idempotent_and_non_destructive` green).
- [x] T6 — Run `npm run check` (0 errors, 0 warnings).
- [x] T7 — Run `cargo test --manifest-path src-tauri/Cargo.toml` (19/19 incl. new `mhwilds_004_coverage_*`).
- [x] T8 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T9 — Verify against `spec.md`; update docs.
