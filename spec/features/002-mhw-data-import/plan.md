# 002 — Plan — MHW data import (core — DONE)

## Approach (as built)

Dev-only pipeline (`scripts/generate_mhw_*.py`, `scrape_mhw_*.py`, `download_*mhw*.py`) against MHWorldData (+ Fandom extras for drops/icons) emitted `src-tauri/data/mhw_*.json` (20 files); `seed.rs` MHW section follows the MH2G/MHP3rd idempotent pattern. Rarity styling migrated from 8-color to 12-HEX r1..r12 during implementation.

## Outcome note (added when closing)

Core is DONE and verified (counts in spec). Decorations, monster weaknesses/equipment, weapon/decoration skill-points, and full shop/gather sources were deferred to `002b-mhw-gaps`. Tools/melder detail retro-spec lives in `009-mhw-tools-melder`.

## Files to touch

- Backend: `src-tauri/src/db/seed.rs`, `src-tauri/src/db/schema.rs` (indexes only if needed), `src-tauri/src/db/queries.rs` (ordering).
- Data: `src-tauri/data/mhw_*.json` (new).
- Frontend: `src/lib/stores/game.ts` (counts/badges only if needed), list pages only if a new filter is required.
- Docs: `docs/fidelity-report.md`, `STATUS.md`, this spec.

## Data / migrations

- `schema.rs`: reuse existing tables; add UNIQUE coverage only for new junction shapes.
- `seed.rs`: MHW section after MH2G/MHP3rd; `BEGIN IMMEDIATE` + `INSERT OR IGNORE` + backfills.
- Ordering: compute `sort_order` at import (Chest DFS / Smith DFS).

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries: per-table counts for `game_id = mhw`; orphan-FK check; double-seed stability; UI smoke per entity.
