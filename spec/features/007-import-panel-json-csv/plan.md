# 007 — Plan — Import panel JSON/CSV — DONE (as built)

## Approach (as built)

New `import.rs` module (no seed refactor needed — validators mirror seed shapes + FK guards independently). Frontend `[game]/import` page parses JSON natively and CSV via a small quoted-cell parser, then drives `preview_import` (dry-run table) → `apply_import` (counts). Sidebar entry added (power-user but discoverable).

## Files to touch

- Backend (`src-tauri/src/...`): `import.rs` (new), `commands/mod.rs` + `lib.rs` (2 commands).
- Frontend (`src/...`): `lib/api.ts` (`ImportPreview/Result`), `routes/[game]/import/+page.svelte` (new), `lib/components/sidebar.svelte` (entry).
- Data (`src-tauri/data/...`): none.
- Docs to update (`STATUS.md`, SDD spec itself): this spec + STATUS/roadmap ticks.

## Data / migrations

- No schema change; reuse UNIQUE indexes as conflict targets.
- Transaction: `BEGIN IMMEDIATE` … `COMMIT` per apply; `ROLLBACK` on unexpected SQLite failure (row errors are collected, not fatal).

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Fixtures: valid + invalid JSON rows; FK-violation rows rejected; double-apply stability (all covered in `import::` tests).
