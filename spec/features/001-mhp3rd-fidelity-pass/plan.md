# 001 — Plan — MHP3rd fidelity pass

## Approach

Extend the MHP3rd seed with shop/trade/farm sources scraped from the same wiki pipeline (`fetch_mhp3_wiki_data.py` Playwright flow), plus a numeric audit pass comparing prices/rarity/probabilities against `mhp3rd_*.json` canonical data. All inserts via `INSERT OR IGNORE` + backfill UPDATEs.

## Files to touch

- Backend: `src-tauri/src/db/seed.rs`, `src-tauri/src/db/schema.rs` (only if a new UNIQUE index is needed), `src-tauri/src/db/queries.rs` (only if a list/detail filter changes).
- Data: `src-tauri/data/mhp3rd_*.json` (+ generation scripts in `scripts/` — dev-only, git-ignored except `check-version.js`).
- Frontend: likely none; only `src/routes/[game]/items/` if a new `source_type` badge is needed.
- Docs: `docs/fidelity-report.md`, `STATUS.md`, this spec.

## Data / migrations

- `schema.rs`: add UNIQUE index only if new `(item_id, source_type, ...)` combinations can duplicate; reuse `uq_item_sources`.
- `seed.rs`: insert order items → `item_sources` → `monster_drops` → `quest_rewards` → `item_combine`; backfill descriptions/categories.
- Ordering: preserve `item_combine.id` Book order and item Chest order (`COALESCE(sort_order,id)`).

## Outcome note (added when closing)

No schema/query change was needed except the `get_item_sources` visibility fix (`source_id IS NOT NULL` guard). Shop/trade/farm data was already present; T2–T4 became verification rather than extension. `carry_limit` / `icon_*` / `sort_order` remain unseeded for MHP3rd by design (see spec).

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries: count MHP3rd items without `item_sources`; count by `source_type`; spot-check `buy_price`; re-run app twice and diff counts (must be stable).
