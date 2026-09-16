# 004 — Plan — MHWilds data import

## Approach

Same pipeline as 003: dev-only fetch into `tmp/`, curated `src-tauri/data/mhwilds_*.json`, idempotent seed section.

## Files to touch

- Backend: `src-tauri/src/db/seed.rs`, `schema.rs`/`queries.rs` if needed.
- Data: `src-tauri/data/mhwilds_*.json` (new).
- Frontend: only for new filters/badges.
- Docs: fidelity report, status, this spec.

## Data / migrations

- Reuse tables; `sort_order` at import; `INSERT OR IGNORE` + backfills.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Counts, orphan-FK check, double-seed stability, UI smoke.
