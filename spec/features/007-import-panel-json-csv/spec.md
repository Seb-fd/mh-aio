# 007 — Import panel JSON/CSV — DONE

## Context

- Problem: no import panel for JSON/CSV (roadmap Phase 4 follow-up). Data curation required editing `src-tauri/data/*.json` by hand.
- Current state (as built): `src-tauri/src/import.rs` (validate + transactional apply) + `preview_import`/`apply_import` commands + `[game]/import` route + sidebar entry.
- Related docs: `roadmap.md` Phase 4.

## User stories

- As a maintainer, I want to validate and preview a JSON/CSV import so that curation is safer.
- As a maintainer, I want failed rows reported without corrupting the DB.

## Scope

### In scope (completed)

- Per-game import page: kind selector (items, monsters, skills, decorations, item_sources, monster_drops), file picker (.json/.csv) + paste box, field reference, dry-run preview (valid count + per-row errors), transactional apply with inserted/skipped/rejected report.
- CSV parsed frontend-side (header row + quoted cells, numeric auto-coerce) into the same row objects as JSON.
- Backend validation mirrors seed shapes + FK existence checks; decorations resolve `skill_id` against the game's skills (never FK NULL).

### Out of scope

- Remote fetching. Auto-update mechanism. Weapons/armor/quests imports (complex trees/materials — hand-edit JSON, open a new spec if needed).

## Acceptance criteria

- [x] Invalid files are rejected with clear per-row errors; DB unchanged (preview writes nothing; apply skips invalid rows).
- [x] Valid imports are idempotent (re-import = stable counts via `INSERT OR IGNORE`; tested).
- [x] FK violations are caught pre-apply with actionable messages (`... does not exist for game N`, unknown method/source_type).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (27/27 incl. 5 new `import::` tests).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Row shapes mirror `src-tauri/data/*.json` seed shapes (documented per kind in the UI); `id` required for content tables (idempotency key), FK ids validated against the target game.
- Commands: `preview_import(game_id, kind, rows)`, `apply_import(game_id, kind, rows)`; max 5000 rows per call (UI guard).
- No schema change; UNIQUE indexes are the conflict targets; single `BEGIN IMMEDIATE` transaction per apply (rollback on unexpected failure).

## Constraints

- Offline-first; never DELETE reference rows; transactional apply; English UI.
