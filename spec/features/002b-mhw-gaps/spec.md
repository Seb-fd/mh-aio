# 002b — MHW gaps (decorations, weaknesses, sources) — DONE

## Context

- Problem: MHW core was DONE (see `002-mhw-data-import`), but four gaps kept it below MH2G parity: (1) decorations empty; (2) no `monster_weaknesses`/`monster_equipment`/`weapon_skill_points`; (3) thin shop/gather `item_sources`; (4) Normal-only combine unexplained. All closed below.
- Current state: `src-tauri/data/mhw_decorations.json` (404), `mhw_monster_weaknesses.json` (788), `mhw_item_sources_gather.json` (911) via `scripts/generate_mhw_decorations_weaknesses.py` (MHWorldData); seed fns `seed_mhw_decorations`, `seed_mhw_monster_weaknesses`, `seed_mhw_weapon_skill_points`, `seed_mhw_monster_equipment`, `seed_mhw_gather_item_sources`; test `mhw_002b_coverage_decorations_weaknesses_skills_equipment` (14/14 green).
- Related docs: `002-mhw-data-import` (core DONE), `009-mhw-tools-melder`, `000-mh2g-baseline` (parity target).

## User stories

- As a player, I want MHW decorations + monster weaknesses browsable so that build planning reaches MH2G parity.
- As a maintainer, I want the "8-color" doc label corrected to 12-HEX everywhere so icon docs match code (done in step 1: STATUS/AGENTS/fidelity-report).

## Scope

### In scope (completed)

- `mhw_decorations.json` (404, slot/rarity/skill1+2/levels/icons) + `seed_mhw_decorations` (skips primary-unresolved, never FK NULL; materials stay empty — no upstream data).
- `monster_weaknesses` (788 rows / 88 monsters) from per-part hitzones (real cut/impact/shot + elements, mh2g scale); status ailments dropped (no column).
- `weapon_skill_points` for 638 special-skill weapons: bare-name fallback (points = max_level, all rank-1 specials) + added missing `Kulve Taroth Essence` skill (20179) + level (20419).
- `monster_equipment` derived from material drops (~6461 armor + ~13923 weapon links); powers monster detail tabs + dedicated sets.
- Gather `item_sources` (911 rows, 5 maps, area + rank in conditions; no Hoarfrost/Guiding Lands upstream).
- Combine documented as Normal-only by design (crafting + Melder model, see 009).

### Out of scope

- MHR/Wilds (003/004). Melder/tools behavior (009). Solver tuning for MHW (no spec yet — open one if needed). Deco crafting materials (don't exist upstream).

## Acceptance criteria

- [x] `decorations` list + `[id]` render for `mhw` (404, materials empty by design).
- [x] Monster `[id]` shows weaknesses table for MHW (88 monsters; 6 small monsters have no hitzones upstream).
- [x] Skill `[id]` shows MHW armor + weapon refs (armor_skill_points pre-existing + new weapon_skill_points).
- [x] `item_sources` per-type counts documented (gather 911 + shop 50 / farm 37 / melder 10 / steamworks 10 + derived carve/capture/drop/break/reward).
- [x] Double-seed stable (UNIQUE indexes; `seed_is_idempotent_and_non_destructive` covers `decorations`/`item_sources`).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (14/14 incl. new 002b test).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- New files: `src-tauri/data/mhw_decorations.json`, `mhw_monster_weaknesses.json`, `mhw_item_sources_gather.json` (+ `mhw_skills.json` +1 skill, `mhw_skill_levels.json` +1 level); generator: `scripts/generate_mhw_decorations_weaknesses.py`.
- Tables: `decorations` (+rarity now populated for MHW), `monster_weaknesses`, `weapon_skill_points`, `monster_equipment`, `item_sources` (extended), `skills`/`skill_levels` (+1 each).
- Seed idempotency: `uq_decorations_game_id (game_id, id)` + `uq_item_sources` + `uq_monster_weaknesses` + `uq_monster_equipment` + PKs on skill-points tables; `INSERT OR IGNORE` only.
- Ordering: weaknesses `ORDER BY id`; decorations existing query order.

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: parents before children (new fns run after materials/drops/skills in the MHW orchestrator).
- English-primary UI; theming via `var(--theme-*)` only.
