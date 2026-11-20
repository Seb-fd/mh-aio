# 014 — Plan — MHWilds small monsters + endemic life (shipped)

## Approach

Curated small-monster list merged into the existing `monsters` table with `size = 'small'` and a dedicated 21001+ id offset (clear of the 004 large range 20001–20034); curated endemic list into the new `endemic_life` table. Both rendered by existing monster detail + the shared `/[game]/endemic` route (enabled for `mhw` + `mhwilds`), so the only frontend touch is the endemic hub entry. Warm-boot installs backfilled by `patch_mhwilds_012_014`.

## Files to touch

- Backend (`src-tauri/src/...`): `db/schema.rs` (`endemic_life` table + indexes), `db/seed.rs` (`seed_mhwilds_small_monsters`, `seed_mhwilds_endemic_life`, `patch_mhwilds_012_014`), `db/queries.rs` (`get_endemic_life` + test `mhwilds_013_014_charms_small_endemic`), `commands/mod.rs` + `lib.rs` (`get_endemic_life` registration).
- Frontend (`src/...`): `src/routes/[game]/endemic/+page.svelte` (shared route), `src/routes/[game]/+page.svelte` (endemic hub entry), `src/lib/api.ts` (`getEndemicLife`).
- Data (`src-tauri/data/...`): `mhwilds_small_monsters.json`, `mhwilds_endemic_life.json` (new).
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): counts via generated `docs/data-coverage.md`; this spec folder.

## Data / migrations

- `schema.rs`: new `endemic_life` table (`CREATE TABLE IF NOT EXISTS`); no column migration needed.
- `add_idempotency_constraints()`: UNIQUE `uq_endemic_life_game_id` (`game_id, id`) + `idx_endemic_life_game`.
- `seed.rs`: smalls after larges (offset avoids collision); `INSERT OR IGNORE`; patch `mhwilds_012_014_backfill` for warm-boot.
- Ordering (`ORDER BY`, `sort_order`, Book order): source list order.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria:
  - `SELECT COUNT(*) FROM monsters WHERE game_id = 3 AND size = 'small'` → 34 (68 total); `endemic_life WHERE game_id = 3` → 70.
  - Open `/mhwilds/monsters` (smalls listed) and `/mhwilds/endemic` (70 rows, kind filter works).
