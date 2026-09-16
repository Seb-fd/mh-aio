# 002b — Plan — MHW gaps — DONE (as built)

## Approach (as built)

Dev-only `scripts/generate_mhw_decorations_weaknesses.py` pulled `decoration_base.csv` + `monster_hitzones.csv` + `location_items.csv` from MHWorldData GitHub raw and emitted three curated JSONs (deco ids offset +20000; gather rank folded into conditions). Seed extends the MHW orchestrator with five fns (decorations / weaknesses / weapon_skill_points with bare-name fallback / derived equipment via materials ⨝ drops / gather sources). One skill+level row added (`Kulve Taroth Essence`, 362 weapons). Finished with the 8-color → 12-HEX doc sweep.

## Files to touch

- Backend (`src-tauri/src/...`): `db/seed.rs` (new `seed_mhw_decorations`, `seed_mhw_monster_weaknesses`, `seed_mhw_monster_equipment`, skill-points derivation), `db/schema.rs` (UNIQUE only if needed), `db/queries.rs` (only if a filter changes — likely none).
- Frontend (`src/...`): likely none (lists/details already generic); only badges if a new `source_type` appears.
- Data (`src-tauri/data/...`): `mhw_decorations.json` (new), optional `mhw_monster_weaknesses.json` / `mhw_monster_equipment.json`, extended `mhw_item_sources_extra.json`.
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): fidelity §MHW (12-HEX, new counts), this spec.

## Data / migrations

- `schema.rs`: reuse tables; add UNIQUE coverage only for new junction shapes.
- `seed.rs`: new seed fns called from the MHW orchestrator after `seed_mhw_skills`; `BEGIN IMMEDIATE` + `INSERT OR IGNORE` + backfills; keep quest `100101+` offset untouched.
- Ordering (`ORDER BY`, `sort_order`, Book order): no new ordering; weaknesses `ORDER BY id`.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria: `SELECT COUNT(*) FROM decorations WHERE game_id = 1`; weaknesses non-empty for Large MHW monsters; skill detail shows weapon refs; `item_sources` grouped by `source_type`; double-seed diff (stable); UI smoke `/mhw/decorations`, `/mhw/monsters/<id>`, `/mhw/skills/<id>`.
