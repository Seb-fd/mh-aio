# 003 — Plan — MHR data import

## Approach

Scrape/curate Kiranico (+ Game8 cross-check) via dev-only scripts into `src-tauri/data/mhr_*.json`, then add an idempotent MHR seed section mirroring 002.

## Files to touch

- Backend: `src-tauri/src/db/seed.rs`, `schema.rs` (indexes if needed), `queries.rs` (ordering).
- Data: `src-tauri/data/mhr_*.json` (new).
- Frontend: only if new filters/badges required.
- Docs: fidelity report, status, this spec.

## Data / migrations

- Reuse tables; compute `sort_order` at import; `INSERT OR IGNORE` + backfills.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Counts per table for `mhr`, orphan-FK check, double-seed stability, UI smoke.
