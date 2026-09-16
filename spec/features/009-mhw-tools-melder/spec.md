# 009 — MHW tools (Mantles/Boosters/Palico) + Elder Melder — DONE

## Context

- Problem: MHW-only Tools (Specialized Tools: Mantles/Boosters; Palico Gadgets) + Elder Melder recipes were implemented with no dedicated spec — they "rode along" with 002. Other games have no equivalent data, and the UI gates Tools to `mhw`.
- Current state (all implemented): tables `mhw_mantles`, `palico_gadgets`, `palico_gadget_levels`, `melder_recipes`; commands `get_melder_recipes`, `get_mhw_mantles/_detail`, `get_palico_gadgets/_detail` (`commands/mod.rs`, registered in `lib.rs`); queries `get_melder_recipes_by_game`, `get_mhw_mantles_by_game/_detail`, `get_palico_*` + `global_search` coverage; types `MhwMantle`, `PalicoGadget(/Level/Detail)`, `MelderRecipe`, `ItemDetail.melder` (`api.ts`); routes `tools/+layout` (non-`mhw` shows "exclusive to MHW"), `tools/+page` → redirect `tools/mantles`, `tools/mantles(+[id])`, `tools/boosters(+[id])`, `tools/palico(+[id])`; Melder block in `items/[id]`; sidebar/dashboard `gameOnly: 'mhw'`.
- Related docs: `002-mhw-data-import` (core), `002b-mhw-gaps` (no tool changes), `000-mh2g-baseline` (tools hidden for mh2g).

## User stories

- As a World player, I want Mantles/Boosters/Palico Gadgets browsable offline with durations/cooldowns/slots/unlocks so that loadout planning includes Specialized Tools.
- As a player, I want Elder Melder recipes (research + melding cost, unlock) visible from the item detail so that melding is actionable.

## Scope

### In scope (completed — retro-anchor)

- Mantles 20 (17 mantles + 3 boosters split in UI by `tool_type`), Palico Gadgets 8 + 38 proficiency levels, Melder 211 recipes (Astera/Seliana via `melder_type`).
- UI: Tools section MHW-only; mantles list filters `tool_type === 'mantle'` vs `'booster'`; palico list + `[id]` with proficiency levels; item-detail Melder block; global search includes mantles/palico.

### Out of scope

- Rise/Wilds equivalents (no data; track under 003/004 if Capcom-parity tools are ever scoped). Wirebug / Focus Mode widgets (not scoped anywhere yet — open a new spec if needed). Decorations/weaknesses (002b).

## Acceptance criteria

- [x] `/mhw/tools/mantles` lists 17 mantles (effect, duration/cooldown incl. upgraded, slots, acquisition, upgrade quest/effect); `/mhw/tools/boosters` lists 3.
- [x] `/mhw/tools/palico` lists 8 gadgets (tribe, effect, acquisition); `[id]` shows proficiency levels with unlock conditions.
- [x] Non-`mhw` games show "Tools exclusive to MHW" layout guard; Tools hidden in sidebar/dashboard for `mhp3rd`/`mh2g`/`mhr`/`mhwilds`.
- [x] Item `[id]` shows Melder block when a recipe exists (research_cost, melding_cost, unlock, type).
- [x] Global search returns mantles/palico results for `mhw`.
- [x] Double-seed stable (idempotent).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Tables: `mhw_mantles` (incl. `tool_type mantle|booster`, `duration_sec`, `cooldown_sec`, `cooldown_upgraded_sec`, `slots`, `acquisition`, `upgrade_quest/effect`, `sort_order`), `palico_gadgets` (+ `palico_gadget_levels` with `proficiency`, `ability_name`, `unlock_condition`), `melder_recipes` (`research_cost`, `melding_cost`, `unlock_condition`, `melder_type`).
- Seed: `INSERT OR IGNORE` in MHW orchestrator (`seed_mhw_*`); idempotency via PK + `(game_id, id)` guards.
- Ordering: `sort_order` where present; mantles/boosters split is a UI filter, not a table split.

## Constraints

- Idempotent non-destructive seed; FK parents first; English-primary UI; theming via `var(--theme-*)` only; no new tables for Rise/Wilds in this spec.
