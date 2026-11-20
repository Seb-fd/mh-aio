# 012 — Tasks — MHWilds quests (shipped)

- [x] T1 — Curate `src-tauri/data/mhwilds_quests.json` from Fextra (Assignment 33 / Optional 81 / Event 60 = 174 rows).
- [x] T2 — Seed via `seed_mhwilds_quests` (`src-tauri/src/db/seed.rs:7255`) into existing `quests` (reward text → `description`, no `quest_rewards`).
- [x] T3 — Backfill warm-boot installs (`patch_mhwilds_012_014`, `src-tauri/src/db/seed.rs:280`).
- [x] T4 — Add coverage test `mhwilds_012_quests` (`src-tauri/src/db/queries.rs:4653`).
- [x] T5 — Run `npm run check` (0 errors, 0 warnings).
- [x] T6 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] T7 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T8 — Verify against `spec.md` acceptance criteria; document gaps (`quest_rewards` empty, `hub` NULL).
