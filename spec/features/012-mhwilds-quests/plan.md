# 012 — Plan — MHWilds quests (shipped)

## Approach

Fextra quest lists (Assignments + Optional + Event) curated into `src-tauri/data/mhwilds_quests.json`, then seeded into the existing `quests` table by `seed_mhwilds_quests`, reusing the `Assignment/Optional/Event` category enum so the quest list/detail UI works with zero frontend changes. Reward text stored as `description`; `quest_rewards` deliberately left empty (documented gap). Warm-boot installs backfilled by `patch_mhwilds_012_014`.

## Files to touch

- Backend (`src-tauri/src/...`): `db/seed.rs` (`seed_mhwilds_quests`, `patch_mhwilds_012_014`), `db/queries.rs` (test `mhwilds_012_quests` only — list/detail commands reused as-is).
- Frontend (`src/...`): none (existing quest routes/filters reused).
- Data (`src-tauri/data/...`): `mhwilds_quests.json` (new).
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): counts via generated `docs/data-coverage.md`; this spec folder.

## Data / migrations

- `schema.rs`: none (existing `quests` table).
- `add_idempotency_constraints()`: existing `(game_id, id)` guard (`uq_quests_game_id`).
- `seed.rs`: `seed_mhwilds_quests` after monsters (name lookup needs `monsters` rows); `INSERT OR IGNORE`; patch `mhwilds_012_014_backfill` for warm-boot.
- Ordering (`ORDER BY`, `sort_order`, Book order): Fextra list order; no canonical retail order published for Wilds quests.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria:
  - `SELECT COUNT(*) FROM quests WHERE game_id = 3` → 174; per-category 33/81/60; 0 `quest_rewards` rows for `game_id = 3`.
  - Open `/mhwilds/quests`: list renders, category filter works; open an Assignment detail: main monster + reward text visible.
