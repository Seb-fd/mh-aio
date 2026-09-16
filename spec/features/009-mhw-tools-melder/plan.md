# 009 — Plan — MHW tools + Melder (retro-anchor, no code changes)

## Approach

No code changes: retro-documents the already-shipped MHW Tools + Melder stack so 002 stays about core data and future Rise/Wilds tool equivalents have a pattern to copy. Any new tool type after this point needs its own spec.

## Files to touch

- Backend (`src-tauri/src/...`): none (reference only: `commands/mod.rs` tool commands, `db/queries.rs` tool queries + `global_search` coverage, `db/seed.rs` MHW tool seed fns).
- Frontend (`src/...`): none (reference only: `routes/[game]/tools/*`, `sidebar.svelte` `gameOnly`, `api.ts` tool types).
- Data (`src-tauri/data/...`): none (reference only: `mhw_mantles.json`, `mhw_melder_recipes.json`, `mhw_palico_gadgets.json` + `_levels`).
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): this spec only.

## Data / migrations

- `schema.rs`: no change (tables `mhw_mantles`, `palico_gadgets(/_levels)`, `melder_recipes` already exist with `CREATE TABLE IF NOT EXISTS`).
- `seed.rs`: MHW tool seed order mantles → palico (+levels) → melder, `INSERT OR IGNORE`.
- Ordering (`ORDER BY`, `sort_order`, Book order): `sort_order` for display; mantle/booster split via `tool_type` filter.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria: counts (`mhw_mantles` 20, `palico_gadgets` 8, `melder_recipes` 211); UI smoke `/mhw/tools/mantles|boosters|palico` + one `[id]` each + item with Melder block; visit `/mhp3rd/tools` and `/mh2g/tools` to confirm the exclusivity guard; one `global_search` query hitting a mantle.
