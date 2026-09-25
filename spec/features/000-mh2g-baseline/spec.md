# 000 — MH2G (Freedom Unite) baseline — DONE

## Context

- Problem: MH2G (`mh2g`, game_id 5) is the verified MVP but had no dedicated spec; its state was only anchored in `roadmap.md` Phase 1/2 + `docs/fidelity-report.md` + `STATUS.md`.
- Current state: fully seeded and verified. Seed: `src-tauri/src/db/seed.rs` (`const MH2G = 5`), data: `src-tauri/data/mh2g_*.json` (20 files). Queries: `src-tauri/src/db/queries.rs` generic `WHERE game_id = ?1`. Routes: `src/routes/[game]/*` (all sections work for `mh2g`; `tools/*` hidden by `gameOnly: 'mhw'`). Commands: list + detail + `get_combinations` + `get_monster_dedicated_sets` + `search_armor_sets` + `global_search`.
- Related docs: `roadmap.md` Phase 1/2 (DONE), `STATUS.md` Completed, `docs/fidelity-report.md` (MH2G audit), `spec/constitution/mission.md`.

## User stories

- As a player, I want the full Freedom Unite catalog browsable offline (monsters / weapons / armor / quests / items / skills / decorations) with game-faithful ordering so that reference matches the retail UMD.
- As a hunter, I want Armor Set Search + global per-game search + combine Book view so that build planning works offline.

## Scope

### In scope (already implemented — this spec is the retro-anchor)

- Full dataset per table below; game-faithful ordering; filtered browsers; detail views with cross-navigation; ASS; global search; combine view.

### Out of scope

- Other games (001/002/002b/003/004/009). Builds save/load (005). Favorites (006). Import panel (007).

## Acceptance criteria

- [x] Counts stable across double-seed (idempotent `INSERT OR IGNORE` + UNIQUE indexes; see known exception below): items 1244 (1083 + 161 upstream books/ores/fish/meats/insects/seeds/herbs/mushrooms/eggs), `item_sources` 12751, combines 432 (42 Lvl-ammo chances filled, 2 NULL left: Crag S Lv3 comps), monsters 83 (54 Large + 25 Small + 4 Giant), weapons 1500 (11 types, all with forge/upgrade craft rows), armor 2075 / sets 949 derived, quests 610 (95 Village Elder + 62 Nekoto incl. restored 9★ + 89 Guild Low + 77 Guild High + 89 Guild G + 140 Training + 7 Treasure + 37 Event + 14 Challenge), skills 99 / levels 214, decorations 192, drops 3402 (133 extraction-arbitrated corrections + 33 shadow deletes; see `scripts/mh2g_drop_corrections.log`), weaknesses 420 + ailments 214 + tools 354, equipment 9019, weapon_mats 5137 / craft split complete, quest_rewards 1273 (incl. 4 Special-Reward tickets).
- [x] Item taxonomy ISO-derived: `Consumable 128 / Material 1037 / Ammo 79` + `subcategory` (Recovery/Buff/Food/Charm/Husk/Coating/Ore/Monster Material…; `Powercharm/Powertalon` → `Consumable • Charm`, `Huskberry/Sm Bone Husk` → `Ammo • Husk`).
- [x] Ordering: items Chest (`COALESCE(sort_order,id)`), weapons Smith `Great Sword → Bow` (`ORDER BY CASE weapon_type`), monsters Hunter's Notes `Felyne 1 … White Fatalis 83` (`ORDER BY id`, frontend Large/Small selector order-preserving), quests hub order (`elder → nekoto → guild_low → guild_high → guild_g → training → treasure → event`), combines Book order (`ORDER BY item_combine.id`).
- [x] Browsers: monsters Large/Small/All (Large default), armor Sets/Pieces + Both/Male/Female + All/Blademaster/Gunner, quests collapsible difficulty accordions, items `category • subcategory` + Chest order, combine single view (Normal/Alchemy/Treasure badge + `success %`).
- [x] Details: monster (weaknesses, drops, Dedicated ≥40% + Uses-1-Material tabs), weapon (tree/stats/materials), armor piece + `sets/[id]`, quest (rewards), item (sources + clickable `A x1 + B x1 = Result x1 • %`), skill (levels/decos/armor/weapons), decoration.
- [x] ASS (`ass.rs` + `search_armor_sets` + `builds/`): equivalences, jewel solver, Torso Inc, bad-skill auto-fix, `MAX_LIMIT 1000`, HR/Elder gate, gender, weapon slots, piercings.
- [x] Global search per game (accent-insensitive `norm_key`, debounced, grouped suggestions).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Tables: `games, items, item_sources, item_combine, monsters, monster_weaknesses, monster_drops, monster_equipment, weapons, weapon_materials, weapon_craft, armor, armor_sets (derived via derive_set_name), armor_materials, quests, quest_rewards, skills, skill_levels, decorations, armor_skill_points, weapon_skill_points`.
- Seed idempotency: `uq_item_combine(result_item_id, component_item_id, combine_type)`, `uq_item_sources(...)`, `uq_monster_equipment(game_id, monster_id, equipment_kind, equipment_id)`, `(game_id, id)` guards; `INSERT OR IGNORE` + backfill UPDATEs.
- Ordering: Chest / Smith / Hunter's Notes / hub / Book (see criteria).

## Known exceptions / gaps (accepted)

- `seed_armor_sets` does `DELETE FROM armor_sets WHERE game_id = 5` + reinsert (only non-`INSERT OR IGNORE` spot; no source JSON).
- Combine breakdown cited in root docs (`147 + 18 + 7 = 172 ≠ 432`) is stale; total 432 is authoritative — real `GROUP BY combine_type` pending documentation.
- `mh2g_item_descriptions.json` covers 43 rows (partial by design, not claimed 100%).
- Monster SQL orders Small-first (`queries.rs`); Hunter's Notes `ORDER BY id` is preserved by the frontend selector.
- PK collision guard: MHW quest offset `100101+` exists because MH2G occupies `1..610`.

## Constraints

- Idempotent non-destructive seed (never DELETE reference rows — except the documented `armor_sets` re-derive); no `clear_game`; no count early-returns.
- FK order: parents before children (`PRAGMA foreign_keys = ON`).
- English-primary UI; theming via `var(--theme-*)` (`medieval`, `#b91c1c`/`#d4a017`, sigil `II`).
