# 014 — Tasks — MHWilds small monsters + endemic life (shipped)

- [x] T1 — Curate `src-tauri/data/mhwilds_small_monsters.json` (34 smalls, 21001+ ids, descriptions).
- [x] T2 — Curate `src-tauri/data/mhwilds_endemic_life.json` (70 endemic rows).
- [x] T3 — Add `endemic_life` table + indexes (`src-tauri/src/db/schema.rs`), `get_endemic_life` query/command/API wiring.
- [x] T4 — Seed via `seed_mhwilds_small_monsters` (`src-tauri/src/db/seed.rs:7170`) + `seed_mhwilds_endemic_life` (`src-tauri/src/db/seed.rs:7199`).
- [x] T5 — Shared `/[game]/endemic` route for `mhw` + `mhwilds` + hub entry.
- [x] T6 — Backfill warm-boot installs (`patch_mhwilds_012_014`, `src-tauri/src/db/seed.rs:280`).
- [x] T7 — Add coverage test `mhwilds_013_014_charms_small_endemic` (`src-tauri/src/db/queries.rs:4509`).
- [x] T8 — Run `npm run check` (0 errors, 0 warnings).
- [x] T9 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] T10 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T11 — Verify against `spec.md` acceptance criteria; document gaps (no small-monster drops/weaknesses).
