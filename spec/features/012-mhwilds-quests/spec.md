# 012 — MHWilds quests — SHIPPED (backfill)

> Backfill record. Shipped via `patch_mhwilds_012_014` (`src-tauri/src/db/seed.rs:280`, patch name `mhwilds_012_014_backfill`) together with 013/014. Read-only history — do not add scope here.

## Context

- Problem: spec `004-mhwilds-data-import` left the Wilds quest list empty (the MHDB Wilds API used by the 004 generator has no quest endpoint).
- Current state: `quests` holds 174 Wilds rows (`game_id = 3`) seeded from Fextra quest lists by `seed_mhwilds_quests` (`src-tauri/src/db/seed.rs:7255`) from `src-tauri/data/mhwilds_quests.json`; served by the existing generic `get_quests_by_game` / `get_quest_detail` commands with no frontend changes.
- Related docs: `004-mhwilds-data-import` (core import), `docs/data-coverage.md` (generated counts).

## User stories

- As a player, I want to browse Wilds Assignments, Optional and Event quests offline so that I can track progression and quest targets.

## Scope

### In scope (shipped)

- 174 Wilds quests: Assignment 33 / Optional 81 / Event 60, using the existing `quests.category` enum so the quest UI filter works unchanged.
- Per-quest `main_monsters` name resolved from `monsters` at seed time (`src-tauri/src/db/seed.rs:7276`); reward info kept as `description` text (`possible_rewards_text`); per-category MH2G quest icon/color reuse.

### Out of scope (upstream gaps, documented)

- `quest_rewards` per-item rows: NONE for Wilds (`game_id = 3` has 0 rows). Fextra exposes reward info as text, not per-item counts/probabilities, and the upstream rewards page is bot-blocked — deferred, needs a new spec with a new source if ever scoped.
- `hub` stays NULL (Wilds quests are not grouped by hub; `location` is stored instead).

## Acceptance criteria

- [x] `SELECT COUNT(*) FROM quests WHERE game_id = 3` = 174 (`src-tauri/src/db/queries.rs:4662`, test `mhwilds_012_quests`).
- [x] Category split: Assignment 33 / Optional 81 / Event 60 (`src-tauri/src/db/queries.rs:4663-4671`); no NULL/empty `name`/`category`.
- [x] No cross-game id collisions (`uq_quests_game_id` orphans = 0).
- [x] `get_quests_by_game(3)` non-empty, all with `category`; `get_quest_detail` smoke on an Assignment with `main_monsters` set.
- [x] Gaps hold: 0 `quest_rewards` rows for `game_id = 3`; `hub` is NULL.
- [x] Idempotent re-seed (`INSERT OR IGNORE` + `patch_mhwilds_012_014` self-guard via `data_patches`).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- File: `src-tauri/data/mhwilds_quests.json` (`id, name, category, rank, difficulty, location, objective, monster_id, possible_rewards_text, possible_reward_item_ids`; the id list is unused — no `quest_rewards` rows are written).
- Table: existing `quests` only; no new tables, no migration.
- Seed idempotency: `INSERT OR IGNORE` on PK + `(game_id, id)` UNIQUE guard; patch recorded as `mhwilds_012_014_backfill` in `data_patches`.
- Ordering: Fextra list order; no canonical retail order published for Wilds quests.

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: `monsters` (parents, from 004) before `quests` name resolution.
- English-primary UI (Fextra EN names).
