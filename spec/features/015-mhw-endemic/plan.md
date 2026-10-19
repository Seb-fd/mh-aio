# 015 — Plan — MHW + Iceborne Endemic Life

## Approach

Mirror the 014-mhwilds-endemic pattern (`endemic_life` table, list-only UI, Slinger-ammo `kind` taxonomy) for `mhw`. MHW has no canonical "Endemic Life" in-game category, so curate ~35 small creatures + bug/ammo-yield pickups from Fandom MHW+Iberence + Kiranico (already cached in `tmp_mhw_100/`). New JSON committed via `scripts/generate_mhw_endemic.py`; seed stays `INSERT OR IGNORE`; warm-boot backfill via `patch_mhw_015` so pre-015 installs pick it up without a `DATA_VERSION` bump.

## Files to touch

- Data: `src-tauri/data/mhw_endemic_life.json` (new, ~35 rows, IDs 90001-90035).
- Backend: `src-tauri/src/db/seed.rs` (`seed_mhw_endemic_life` + `run(conn, "mhw_015_endemic", …)`); `schema.rs` (add `"endemic_life"` to `content_tables` UNIQUE list).
- Frontend: `src/lib/components/sidebar.svelte` line 115 (`gameOnly: ['mhw', 'mhwilds']`); `src/routes/[game]/endemic/+page.svelte` (drop `isWilds` gate, widen empty-state message); `src/lib/prefetch.ts` (register `/endemic`).
- Scripts (gitignored): `scripts/generate_mhw_endemic.py` (idempotent emitter, regenerates the JSON).
- Tests: `src-tauri/src/db/queries.rs` new `mhw_015_endemic` test (count ≥ 30, every row has a description).
- Docs: `AGENTS.md` (MHW paragraph +35 endemic rows), `roadmap.md` (015 done), `STATUS.md`, `docs/fidelity-report.md` (drop from Phase 7 gap list).

## Data / migrations

- No new table. `endemic_life` already game-agnostic (`game_id` discriminator). New rows for `game_id = 1`.
- Schema migration: add `"endemic_life"` to `content_tables` in `add_idempotency_constraints` → creates `uq_endemic_life(game_id, id)` UNIQUE index (defense-in-depth, current `INSERT OR IGNORE` already protects on PK).
- ID offset: 90001-90035 (no collision with Wilds 80001-80070 or any other content table).

## Verification

```bash
py scripts/generate_mhw_endemic.py     # idempotent JSON regen
cargo test --manifest-path src-tauri/Cargo.toml
npm run check ; npm run format:check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo build --manifest-path src-tauri/Cargo.toml
node scripts/check-version.js
npm audit --audit-level=moderate
```

- `cargo test` must report `mhw_015_endemic` green + 30 ≤ count ≤ 40 rows for game_id=1.
- `/mhw/endemic` lists the new entries with kind filter chips working; `/mhwilds/endemic` unchanged.
- Warm-boot test: rerun `seed()` on a pre-existing DB → no INSERT work, no schema delta.
