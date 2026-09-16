# 000 — Plan — MH2G baseline (retro-anchor, no code changes)

## Approach

No code changes: this feature retro-documents the already-verified MH2G MVP so future work has a stable anchor. Any MH2G data fix after this point must open a new `00X` spec instead of editing history.

## Files to touch

- Backend (`src-tauri/src/...`): none (reference only: `db/seed.rs` MH2G section, `db/queries.rs` ordering, `ass.rs` solver, `commands/mod.rs` list/detail).
- Frontend (`src/...`): none (reference only: `routes/[game]/*`, `stores/game.ts` `mh2g` theme, `api.ts`).
- Data (`src-tauri/data/...`): none (reference only: `mh2g_*.json`, 20 files).
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): this spec only; root docs already describe MH2G as DONE.

## Data / migrations

- `schema.rs`: no change (UNIQUE indexes `uq_item_combine`, `uq_item_sources`, `uq_monster_equipment`, `(game_id, id)` guards already cover MH2G).
- `seed.rs`: insert order monsters → items → drops → sources → equipment → weaknesses → combine → weapons → weapon_mats/craft → armor_sets (DELETE+reinsert, documented exception) → armor → armor_mats → quests → quest_rewards → skills/levels → decorations → skill_points.
- Ordering (`ORDER BY`, `sort_order`, Book order): Chest `COALESCE(sort_order,id)`; Smith `CASE weapon_type Great Sword → Bow`; monsters Hunter's Notes `ORDER BY id` (frontend preserves); quests hub `CASE`; combines `ORDER BY item_combine.id`.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria: per-table `SELECT COUNT(*) WHERE game_id = 5`; double-seed stability (counts identical); UI smoke `/mh2g/<section>` + one detail per entity + ASS example (Attack+Sharpness) + global search `norm_key` query.
