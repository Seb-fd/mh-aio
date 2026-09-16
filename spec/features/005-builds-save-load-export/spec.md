# 005 — Builds save/load + export — DONE

## Context

- Problem: Armor Set Search (ASS port in `src-tauri/src/ass.rs` + `search_armor_sets` command + `src/routes/[game]/builds/`) worked but had no save/load or export (roadmap Phase 3 follow-up).
- Current state (as built): `src-tauri/src/builds.rs` store + 5 commands + `api.ts` types + `builds/` Saved-builds section. Solver untouched except `Serialize`+`Deserialize` derives on its DTOs.
- Related docs: `roadmap.md` Phase 3, `STATUS.md` Next Steps.

## User stories

- As a hunter, I want to save and reload custom builds so that I don't lose planning work.
- As a hunter, I want to export a build to JSON / share link so that I can share it.

## Scope

### In scope (completed)

- Save/list/load/delete per game, persisted in Tauri **app-data** (`{app_data_dir}/builds.json`, atomic temp+rename writes) — chosen over localStorage for cache-clear survival and trivial backup.
- Export JSON download + share code (base64url of the same JSON, clipboard with textarea fallback) + import from file content or pasted code/text.
- Versioned DTO (`version: 1`); import validates version/name/game/skills/query/solution and mints a fresh id.

### Out of scope

- Cloud sync. Favorites (006). Solver algorithm changes (none made).

## Acceptance criteria

- [x] User can save, list, load, delete a build (per-game list, newest first).
- [x] Export JSON round-trips (export → import = identical content apart from id/timestamps; covered by `import_validates_and_assigns_fresh_id`).
- [x] No solver regression (all pre-existing ASS tests still pass; 18/18 total).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (18/18 incl. 4 new `builds::` tests).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Build schema v1: `{version, id, name, game_id, query: AssQueryInput, solution: AssSolutionView, created_at, updated_at}` (`src-tauri/src/builds.rs::SavedBuild`, mirrored in `api.ts`).
- Storage: `{app_data_dir}/builds.json` array; no DB migration; no new Tauri capabilities (Rust `std::fs` only).
- Commands: `list_builds(game_id?)`, `save_build`, `get_build`, `delete_build`, `import_build`.

## Constraints

- Do not change solver semantics (only serde derives added); English UI; theming via vars.
