# 003 — MHR data import — DONE (Phase A bulk + Phase B Sunbreak v16)

## Context

- Status: DONE. Strategy was A→B: Phase A bulk base-Rise JSONs (fast, playable), Phase B full Sunbreak v16 Kiranico scrape (complete).
- Phase A sources: `Badge87/MHRiseScraperData` v1.0.3 (base Rise) + `CrimsonNynja/monster-hunter-DB` (112 monsters, 327 quests) via `scripts/generate_mhrise_bulk.py`.
- Phase B source: Kiranico DB v16 (`mhrise.kiranico.com`, scheme `/data/<plural>?view=X` + `/data/<id>`) via `scripts/fetch_mhrise_kiranico.py` (cached, throttled, resumable) + `scripts/merge_mhrise_phaseb.py`.
- Current state: `src-tauri/src/db/seed.rs` Rise section (`MHR = 2`), 16 `mhr_*.json` files, tests `mhr_003a_*` + `mhr_003b_*` (29/29 green).
- Related docs: `004` (MHDB pattern), `009` (pattern for future Rise tools).

## User stories

- As a player, I want Rise+Sunbreak data browsable offline with skill data intact.

## Scope

### In scope (completed)

- 112 monsters (78L+34S, descriptions, 629 per-part weakness rows, 7092 drops Low/High/Master), 1642 items (taxonomy + descriptions + 121... no combines — Rise has none upstream; gathering 74 rows), 946 quests (all hubs incl. Anomaly/Follower, 7508 rewards), 147 skills / 459 levels, 243 decorations (levels/materials/prices), 3953 weapons (14 types, Smith sort, attack/affinity/element/status/sharpness/rarity/defense/materials/tree), 1591 armor (410 stem-derived sets, defense/res/slots/skill-levels/materials), derived equipment + skill points.
- Ordering: alphabetical proxy for monsters/items (no retail order published); weapons per-type Smith order.

### Out of scope (documented gaps)

- Wirebug-specific UI widgets (new spec if scoped). Talismans (no table — same gap class as Wilds charms). Rampage/Switch/Dango skills UI. Deco slot SIZES for Kiranico weapons (badges carry no numbers; null by design). Weapon descriptions (not on Kiranico detail; null).

## Acceptance criteria

- [x] All entity lists + details work for `mhr`, no orphan FKs (name→id resolution, FK guards, dedicated PK offsets; self-check asserts no orphan levels).
- [x] Detail views show materials/drop sources/weaknesses/equipment.
- [x] Idempotent re-seed (stable counts; merge scripts byte-identical on re-run).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (29/29).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Files: 16 `mhr_*.json` (monsters, weaknesses, drops, items, quests, rewards, skills, levels, decos, weapons, wmat, wcraft, armor, sets, amat + combine n/a).
- ID offsets: monsters 30001+, items 40001+, skills/levels/decos 40001+ (own spaces), weapons 50001+, armor 60001+, sets 40001+, quests 200001+, rewards 900001+.
- Seed idempotency: `(game_id, id)` guards + UNIQUE junctions; `INSERT OR IGNORE` only. Rule learned the hard way: NEVER layer merges on previously-merged files (name cleanup can collapse rows and orphan FK children) — regenerate skills/levels deterministically from source.
- Icons: no Rise sets offline — same fallback scheme as Wilds.

## Constraints

- Non-destructive idempotent seed; FK parents first (equipment AFTER drops); English-primary UI.
