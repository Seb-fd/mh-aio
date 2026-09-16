# 002 — Tasks — MHW data import (core)

- [x] T1 — Define MHW `game_id` + JSON schemas (`mhw_*.json`, 20 files).
- [x] T2 — Fetcher pipeline (MHWorldData + Fandom extras) into `scripts/` (dev-only).
- [x] T3 — Emit curated `src-tauri/data/mhw_*.json`.
- [x] T4 — Seed items + categories/descriptions (1359, Chest `sort_order`).
- [x] T5 — Seed monsters + drops (94 / 5862; weaknesses/equipment deferred to 002b).
- [x] T6 — Seed weapons (+ materials/craft) in Smith order (3544 / 9719 / 10056).
- [x] T7 — Seed armor (+ materials) + skills/levels (1595 / 351 sets / 5887 mats; 178 / 418; decorations deferred to 002b).
- [x] T8 — Seed quests + rewards + sources/combines/melder/mantles/palico (521 / 4176 / 107+derived / 188 Normal / 211 / 20 / 8+38).
- [x] T9 — Double-seed stability check (quest `100101+` offset guard vs MH2G PKs).
- [x] T10 — Run `npm run check` (0 errors, 0 warnings).
- [x] T11 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] T12 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T13 — Verify against `spec.md`; update fidelity report + status (core counts; gaps tracked in 002b).
