# 002 — MHW data import (core) — DONE

## Context

- Status: DONE (core). The old "dataset is pending" statement is superseded: World+Iceborne core (items / monsters / weapons / armor / quests / skills / drops / materials / melder / mantles / palico) is seeded and browsable offline. Remaining gaps moved to `002b-mhw-gaps` (decorations, weaknesses/equipment, skill-points, shop/gather sources).
- Current state: `src-tauri/src/db/seed.rs` MHW section (`const MHW: i32 = 1`), `src-tauri/data/mhw_*.json` (20 files), `scripts/generate_mhw_*.py` + `scripts/scrape_mhw_*.py` + `scripts/download_*mhw*.py` pipeline (dev-only), `docs/fidelity-report.md` § MHW.
- Related docs: `000-mh2g-baseline` (seed pattern), `009-mhw-tools-melder` (tools/melder retro-spec), `002b-mhw-gaps` (follow-up).

## User stories

- As a player, I want MHW monsters/weapons/armor/quests/items/skills browsable offline so that World+Iceborne is usable like MH2G.

## Scope

### In scope (completed)

- Import from MHWorldData (+ Fandom extras for drops/icons): items, monsters, weapons, armor (+sets/materials), quests (+rewards), skills/levels, drops/materials/craft, combines (Normal only — correct for MHW, see 009 for Melder), mantles/palico (detailed in 009).
- Game-faithful ordering (Chest / Smith tree DFS) via `sort_order`, per-rarity styling (12-HEX r1..r12 icon set; the old "8-color" label is superseded).
- Weapons list without `All` (default `Great Sword`).

### Out of scope

- Decorations + weaknesses/equipment + skill-points + full shop/gather sources → `002b-mhw-gaps`. Focus Mode UI adaptations (track with 004-pattern if needed). MHR/Wilds (003/004). Tools detail → `009-mhw-tools-melder`.

## Acceptance criteria

- [x] All entity lists render for `mhw` with no orphan FK rows (quests use `100101+` offset to avoid PK collision with MH2G `1..610`).
- [x] Detail views show materials/drop sources (drops carry `rank` Low/High/Master, `probability` %, `method` carve/break/reward, `part`).
- [x] Core counts stable across re-seed: items 1359 (Chest `sort_order` 1–1339 + 2000+ extras, 343 Fandom icons), monsters 94 (Small 23 + Large 71 incl. variants, 94 offline icons), weapons 3544 (14 types Great Sword→Bow, Smith DFS `sort_order`), monster_drops 5862 (5680 MHWorldData + 182 Fandom), weapon_craft 10056 / weapon_materials 9719, quests 521 / quest_rewards 4176, skills 178 / levels 418, armor 1595 / sets 351 / armor_mats 5887, item_combine 188 (Normal), melder 211 (see 009), mantles 20, palico 8 + 38 levels, item_sources_extra 107 (+ rows derived from drops via `seed_mhw_item_sources_from_drops`).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Tables: all content tables + junctions (`weapon_materials`, `weapon_craft`, `armor_materials`, `monster_drops`, `quest_rewards`, `item_sources`, `item_combine`, `melder_recipes`, `mhw_mantles`, `palico_gadgets/_levels`).
- Seed idempotency: `(game_id, id)` guards + UNIQUE junction indexes; `BEGIN IMMEDIATE` + `INSERT OR IGNORE`.
- Ordering: Chest order items (`COALESCE(sort_order,id)`), Smith DFS weapons (`CASE weapon_type` + `COALESCE(sort_order,id)`), monsters Small→Large (`queries.rs`), combine `ORDER BY item_combine.id`.

## Constraints

- Non-destructive idempotent seed; FK parents first; English-primary UI; rarity icons 12-HEX (not 8-color).
