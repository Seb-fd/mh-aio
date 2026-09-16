# 006 — Favorites system — DONE

## Context

- Problem: no favorites system (roadmap Phase 4 follow-up). Users could not pin monsters/weapons/armor/items/quests/skills/decorations.
- Current state (as built): `src-tauri/src/favorites.rs` store + 3 commands + `api.ts` types + `stores/favorites.ts` cache + `FavoriteButton.svelte` + `DetailHeader` fav props; wired on all 7 lists (row button + ★ filter) and 8 details (header star, incl. armor sets).
- Related docs: `roadmap.md` Phase 4.

## User stories

- As a player, I want to favorite entities per game so that I can jump back quickly.

## Scope

### In scope (completed)

- Favorite/unfavorite from list (row star) + detail (header star); ★ Favorites-only filter per list; local persistence per game.
- Storage: Tauri **app-data** (`{app_data_dir}/favorites.json`, atomic temp+rename) — consistent with 005, survives cache clears.
- Kinds: monster, weapon, armor, armor_set, quest, item, skill, decoration. Name snapshot at toggle time (routes resolve by id).

### Out of scope

- Cloud sync. Import panel (007). Tools/palico/mantle favorites (no kinds defined — follow-up if scoped).

## Acceptance criteria

- [x] Favorites persist across restarts (app-data file; `favorites_survive_reload` test).
- [x] Favorites are scoped per game and entity type (`list_favorites(game_id)`; toggle test asserts scoping).
- [x] UI is themed and English-primary (star uses `--theme-accent`, aria-pressed/labels).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (22/22 incl. 3 new `favorites::` tests).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Favorite: `{game_id, kind, id, name, created_at}` (`favorites.rs`, mirrored in `api.ts` + `favoriteRoute()` helper).
- Commands: `list_favorites(game_id?)`, `toggle_favorite` (returns new state), `remove_favorite`. Validation: game 1..=5, kind whitelist (8), id > 0, name 1..=120 chars.
- No DB migration; no new Tauri capabilities.

## Constraints

- Offline-first; no network; theming via vars.
