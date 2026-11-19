# 013 — MHWilds charms (talismans) — SHIPPED (backfill)

> Backfill record. Shipped via `patch_mhwilds_012_014` (`src-tauri/src/db/seed.rs:280`, patch name `mhwilds_012_014_backfill`) together with 012/014. Read-only history — do not add scope here.

## Context

- Problem: spec `004-mhwilds-data-import` deferred Wilds charms (no `charms` usage for Wilds in the UI at the time).
- Current state: `charms` holds 76 Wilds talisman rows (`game_id = 3`, `charm_type = 'wilds_talisman'`) seeded by `seed_mhwilds_charms` (`src-tauri/src/db/seed.rs:7119`) from `src-tauri/data/mhwilds_charms.json`; the generic charms browser (`src/routes/[game]/charms/+page.svelte`, with `mhwilds` in the charms `gameOnly` list at `src/routes/[game]/+page.svelte:104`) serves them via `get_charms` (`src/lib/api.ts:865`).
- Related docs: `004-mhwilds-data-import` (core import), `docs/data-coverage.md` (generated counts).

## User stories

- As a player, I want to browse Wilds talismans by skill so that I can plan builds around talisman skill points.

## Scope

### In scope (shipped)

- 76 `wilds_talisman` rows in `charms` (`skill_sig = "<skill>|<max_level>"`, `slots = 0`, `charm_table` NULL); skill links into `charm_skills` resolved case-insensitively against Wilds `skills` at seed time (`src-tauri/src/db/seed.rs:7131-7156`).
- Talisman viewer enabled for `mhwilds` (shared charms browser with a `charm_type` filter chip; no Wilds-specific page needed).

### Out of scope (documented)

- Per-level talisman upgrade trees / craft materials: N/A (upstream exposes a flat talisman list; `charm_table`/slots unused for Wilds).
- MHW-style hunter-charm rank tree (I→V): N/A for Wilds (MHW-only UI branch, `isMhw` in `src/routes/[game]/charms/+page.svelte:18`).

## Acceptance criteria

- [x] `SELECT COUNT(*) FROM charms WHERE game_id = 3 AND charm_type = 'wilds_talisman'` = 76 (`src-tauri/src/db/queries.rs:4516`, test `mhwilds_013_014_charms_small_endemic`).
- [x] Most talismans link a game-3 skill: `charm_skills` joins > 60/76 (`src-tauri/src/db/queries.rs:4523-4531`); unlinked names are logged to stderr and never fail the seed.
- [x] `get_charms_by_game(3, ...)` serves at least one `wilds_talisman` (`src-tauri/src/db/queries.rs:4534`); `/mhwilds/charms` renders with the type filter.
- [x] Gaps hold: `charm_table` NULL and `slots = 0` for all 76 rows.
- [x] Idempotent re-seed (`INSERT OR IGNORE` + patch self-guard).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- File: `src-tauri/data/mhwilds_charms.json` (`id, game_id, charm_type, skill_name, max_level`).
- Tables: existing `charms` + `charm_skills`; no new tables, no migration.
- Seed idempotency: `INSERT OR IGNORE` on PKs + patch `mhwilds_012_014_backfill`; skill resolve is best-effort (unresolved names counted to stderr, never fatal).
- Ordering: source list order.

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: `skills` (004) before `charms`/`charm_skills`.
- English-primary UI.
