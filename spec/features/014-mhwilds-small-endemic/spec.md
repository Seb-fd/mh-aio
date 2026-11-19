# 014 — MHWilds small monsters + endemic life — SHIPPED (backfill)

> Backfill record. Shipped via `patch_mhwilds_012_014` (`src-tauri/src/db/seed.rs:280`, patch name `mhwilds_012_014_backfill`) together with 012/013. Read-only history — do not add scope here.

## Context

- Problem: 004 imported large monsters only (the MHDB Wilds API is large-only; small fauna and non-monster endemic creatures had no home).
- Current state: 34 small monsters merged into `monsters` with `size = 'small'` (id offset 21001+, seeded by `seed_mhwilds_small_monsters`, `src-tauri/src/db/seed.rs:7170`, from `src-tauri/data/mhwilds_small_monsters.json`) → 68 Wilds monsters total; 70 endemic rows in the new `endemic_life` table (seeded by `seed_mhwilds_endemic_life`, `src-tauri/src/db/seed.rs:7199`, from `src-tauri/data/mhwilds_endemic_life.json`), rendered by the shared `/[game]/endemic` route (`src/routes/[game]/endemic/+page.svelte`, enabled for `mhw` + `mhwilds` via `hasData` at line 16; hub entry `gameOnly: ['mhw', 'mhwilds']` at `src/routes/[game]/+page.svelte:116-123`) via `get_endemic_life` (`src-tauri/src/db/queries.rs:2515`, `src/lib/api.ts:878`).
- Related docs: `004-mhwilds-data-import` (34 larges), `docs/data-coverage.md` (generated counts).

## User stories

- As a player, I want Wilds small monsters and endemic creatures browsable alongside large monsters so that field fauna is covered too.

## Scope

### In scope (shipped)

- 34 small monsters in `monsters` (`game_id = 3`, `size = 'small'`, 21001–21034, clear of the 20001–20034 large range); placeholder icon reuses a neutral monster icon (no upstream icons for smalls).
- 70 `endemic_life` rows (`game_id = 3`); new table with the `uq_endemic_life_game_id` UNIQUE index; shared UI with MHW endemic rows.
- Small-monster detail smoke: description present, `size = 'small'`.

### Out of scope (documented)

- Drops/weaknesses for small monsters: N/A (upstream exposes none; smalls carry description/species only).
- Per-item rewards or quest links for endemic rows: N/A (observed/collected fauna, not quest objectives).

## Acceptance criteria

- [x] `SELECT COUNT(*) FROM monsters WHERE game_id = 3 AND size = 'small'` = 34; total Wilds monsters = 68 (`src-tauri/src/db/queries.rs:4538-4542`, test `mhwilds_013_014_charms_small_endemic`).
- [x] `SELECT COUNT(*) FROM endemic_life WHERE game_id = 3` = 70 (`src-tauri/src/db/queries.rs:4544-4547`).
- [x] Small-monster detail smoke: description present, `size = 'small'` (`src-tauri/src/db/queries.rs:4548-4558`).
- [x] `/mhwilds/endemic` renders the 70 rows; `/mhw/endemic` unaffected (shared route, per-game `gameId` query).
- [x] Gaps hold: smalls carry description/species only (no drops/weaknesses imported — no upstream data).
- [x] Idempotent re-seed (`INSERT OR IGNORE` + patch self-guard).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Files: `src-tauri/data/mhwilds_small_monsters.json` (`id, name, size, species, description`), `src-tauri/data/mhwilds_endemic_life.json` (`id, name, type, description`).
- Tables: existing `monsters` (smalls) + new `endemic_life` (`CREATE TABLE IF NOT EXISTS`, `src-tauri/src/db/schema.rs:309`; `idx_endemic_life_game`, UNIQUE `uq_endemic_life_game_id`).
- Seed idempotency: `INSERT OR IGNORE` + `(game_id, id)` UNIQUE guards; patch `mhwilds_012_014_backfill`.
- Ordering: source list order.

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: no new FK junctions added.
- Theming via `var(--theme-*)` only; English-primary UI.
