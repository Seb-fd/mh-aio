# Feature plan template

> Copy to `spec/features/NNN-short-name/plan.md`. Fill after `spec.md` is validated.

## Approach

- Summary of the technical approach (1 paragraph):

## Files to touch

- Backend (`src-tauri/src/...`):
- Frontend (`src/...`):
- Data (`src-tauri/data/...`):
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself):

## Data / migrations

- `schema.rs`: new tables (`CREATE TABLE IF NOT EXISTS`) / `apply_migrations()` columns:
- `add_idempotency_constraints()`: UNIQUE indexes needed:
- `seed.rs`: insert order + backfill UPDATEs:
- Ordering (`ORDER BY`, `sort_order`, Book order):

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria:
