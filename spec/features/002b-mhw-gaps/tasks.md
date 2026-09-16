# 002b — Tasks — MHW gaps

- [x] T1 — Emit `mhw_decorations.json` (404) and `seed_mhw_decorations` (rarity + icons; materials empty — no upstream data).
- [x] T2 — Seed `monster_weaknesses` (788 rows / 88 monsters, per-part hitzones) + derived `monster_equipment` (~6461 armor + ~13923 weapon links).
- [x] T3 — Derive `weapon_skill_points` (638 weapons; bare-name fallback points = max_level; added `Kulve Taroth Essence` skill 20179 + level 20419).
- [x] T4 — Extend `item_sources` with 911 gather rows + document per-type counts.
- [x] T5 — Document Normal-only combine for MHW (crafting+Melder model, link 009).
- [x] T6 — Doc sweep: 8-color → 12-HEX r1..r12 in fidelity report + status (+ AGENTS.md).
- [x] T7 — Double-seed stability check (`seed_is_idempotent_and_non_destructive` green).
- [x] T8 — Run `npm run check` (0 errors, 0 warnings).
- [x] T9 — Run `cargo test --manifest-path src-tauri/Cargo.toml` (14/14, new `mhw_002b_coverage_*` test).
- [x] T10 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T11 — Verify against `spec.md` acceptance criteria; update fidelity report + status.
