# 013 — Plan — MHWilds charms (shipped)

## Approach

Curated Wilds talisman list (`src-tauri/data/mhwilds_charms.json`) seeded into the existing `charms` table as `charm_type = 'wilds_talisman'`, with skill links resolved best-effort against Wilds `skills` into `charm_skills`. The existing generic charms browser serves them via `get_charms` — the only frontend touch is the `mhwilds` entry in the charms `gameOnly` list. Warm-boot installs backfilled by `patch_mhwilds_012_014`.

## Files to touch

- Backend (`src-tauri/src/...`): `db/seed.rs` (`seed_mhwilds_charms`, `patch_mhwilds_012_014`), `db/queries.rs` (test `mhwilds_013_014_charms_small_endemic` only — `get_charms_by_game` reused).
- Frontend (`src/...`): `src/routes/[game]/+page.svelte` (charms `gameOnly` includes `mhwilds`); no new routes.
- Data (`src-tauri/data/...`): `mhwilds_charms.json` (new).
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): counts via generated `docs/data-coverage.md`; this spec folder.

## Data / migrations

- `schema.rs`: none (existing `charms` + `charm_skills` tables).
- `add_idempotency_constraints()`: existing PK/UNIQUE guards; no new index.
- `seed.rs`: `seed_mhwilds_charms` after Wilds skills (name→id map needs `skills` rows); `INSERT OR IGNORE`; patch `mhwilds_012_014_backfill` for warm-boot.
- Ordering (`ORDER BY`, `sort_order`, Book order): source list order.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria:
  - `SELECT COUNT(*) FROM charms WHERE game_id = 3 AND charm_type = 'wilds_talisman'` → 76; `charm_skills` joins > 60.
  - Open `/mhwilds/charms`: talismans render, skill search + type filter work.
