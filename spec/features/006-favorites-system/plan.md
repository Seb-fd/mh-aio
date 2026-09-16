# 006 — Plan — Favorites system — DONE (as built)

## Approach (as built)

App-data JSON store mirroring `builds.rs` (`favorites.rs`, no DB migration). Frontend: per-game cache store (`stores/favorites.ts`, single `listFavorites` per game, refresh on toggle) + shared `FavoriteButton.svelte` (Star, stopPropagation, themed) + optional `favKind`/`favId` props on `DetailHeader` (one component covers 8 detail pages). Lists got a ★ filter chip plus per-row star overlays (relative wrapper + absolute button; weapons tree uses a flex sibling instead).

## Files to touch

- Backend (`src-tauri/src/...`): `favorites.rs` (new), `commands/mod.rs` + `lib.rs` (3 commands).
- Frontend (`src/...`): `lib/api.ts` (`Favorite`, `FavoriteKind`, `favoriteRoute`), `lib/stores/favorites.ts`, `lib/components/favorite-button.svelte`, `lib/components/detail-header.svelte`, 7 list pages + 8 detail pages (fav props/buttons only).
- Data (`src-tauri/data/...`): none.
- Docs to update (`STATUS.md`, SDD spec itself): this spec + STATUS/roadmap ticks.

## Data / migrations

- No DB migration (app-data `{app_data_dir}/favorites.json` — explicit decision over localStorage/SQLite, see spec).
- Validation: game 1..=5, 8-kind whitelist, id > 0, name trimmed 1..=120.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Manual: toggle → restart → still favorited; per-game scoping check (different games keep separate stars).
