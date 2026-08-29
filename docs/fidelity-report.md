# Fidelity Report — MH2G Data

**Verdict:** The MH2G dataset in `src-tauri/data/mh2g_*.json` is faithful to the retail UMD (English Patched). The MHFU-ASS 2017 CSV snapshot is stale and is used only as algorithm reference, never as a data source.

## Summary

| Metric                                      | Result                                                                                                                          |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Armor                                       | 2075 pieces (head/chest/arms/waist/legs), 949 sets                                                                              |
| Items                                       | 1083 (fully sourced, 12,751 `item_sources` rows + 432 combines)                                                                 |
| Item categories                             | `Consumable 91 / Material 913 / Ammo 79` (67 fixes, `subcategory` Charm/Husk/Coating etc., `Powercharm` → `Consumable • Charm`) |
| Combine recipes                             | 432 (147 Normal + 18 Alchemy + 7 Treasure, with `chance` + Book order)                                                          |
| Skill families / abilities                  | 99 / 214                                                                                                                        |
| Decorations                                 | 192                                                                                                                             |
| Weapons                                     | 1500 (11 types)                                                                                                                 |
| Monsters                                    | 83 (54 Large, 25 Small, 4 Giant)                                                                                                |
| Quests                                      | 610 (95 Elder, 35 Nekoto, 89 Guild Low, 77 Guild High, 116 Guild G, 140 Training, 7 Treasure, 37 Event, 14 Challenge)           |
| Weapon materials / craft                    | 5137 / 1075 recipes (forge + upgrade)                                                                                           |
| Monster weaknesses / drops / equipment      | 163 / 2319 / 1880                                                                                                               |
| Defense / rarity / slots (armor)            | **100% match** vs retail (0 mismatches)                                                                                         |
| Skill points (normalized, 1828 overlapping) | 40 diffs — **all favor our DB** (ASS has ±1 and sign flips, e.g. Kirin Crest X Protection -2 vs +2)                             |
| Decorations (overlapping)                   | 0 mismatches on slot/points                                                                                                     |
| Weapons per type                            | **100% match** vs game-extracted DB (see below)                                                                                 |
| Monster order                               | In-game order (Felyne 1, Melynx 2, Shakalaka 3 … Rathian 31, Rathalos 34 … White Fatalis 83)                                    |

## Method

- Compared `mh2g_*.json` against a game-extracted database ([Kolyn090/mhfu-db](https://github.com/Kolyn090/mhfu-db)) and the retail UMD string tables in `DATA.BIN` (`PSP_GAME/USRDIR/DATA.BIN`).
  - Armor names: string table at offset `37652906` (`Bone Helm`, `Velociprey Helm`, `Hornet Helm` …) — already used in `src-tauri/src/db/queries.rs:872`.
  - Monsters: tables around `37418670` (species `Flying Wyvern`, `Lynian` …) and `37427058` (descriptions `[Bulldrome: Pelagus]`, `[Velocidrome: Bird Wyvern]` …).
  - Weapons: block at `37563513` (`Buster Sword`, `Iron Katana`, `Ravager Blade` … 1500 entries).
  - Quests: table at `335602928` (`Mountain Herb Picking`, `An Anteka in the Snow` … 559 base quests) plus hub labels `Training`/`Treasure` at `37442195`/`37417942`; Event quests not in ISO, validated against distribution file (33 quests, e.g. `Emperador de las llamas JUMP` — Teostra, `Carnaval de cangrejos`) and wiki.
- Normalized names (lowercase, strip non-alphanumerics) to handle translation aliases (`Hornetaur→Hornet`, `Volganos→Lava`, `Hypnoc→Hypno`, `Plum D.Hermitaur→Plum Daimyo Hermitaur`, `Vespoid Queen↔Queen Vespoid`) and skill-name differences (`Defence`/`Defense`, `WindPress`/`Wind Press` via `seed.rs:759`).
- `mhp2g.kiranico.com` is sunset (HTTP 530), so `mhfu-db` was used as the fallback arbiter. The retail ISO is the primary source; `mhfu-db` wins only when the ISO table is ambiguous.
- Ordering verified: monster order is the in-game Hunter's Notes order (small monsters first), weapon order is the in-game Smith tree `Great Sword → Long Sword → Sword & Shield → Dual Blades → Hammer → Hunting Horn → Lance → Gunlance → Light Bowgun → Heavy Bowgun → Bow` (`src-tauri/src/db/queries.rs:698`, `src/routes/[game]/weapons/+page.svelte:14`).

## Armor, Items, Skills, Decorations

- **Armor:** 2075 pieces, 1083 items, 99/214 skills, 192 decorations. Defense/rarity/slots 100% vs retail. Skill points validated; 40 diffs with ASS all favor the DB (ASS has sign errors).
- **Items:** Chest order faithful to `DATA.BIN` file 15 (`src-tauri/src/db/queries.rs:1264`). 138 sell prices fixed vs game. **Taxonomy re-derived from ISO `icon` + verb:** `Consumable 91 / Material 913 / Ammo 79` with `subcategory` (`Recovery, Buff, Food, Charm, Husk, Coating, Ore, Monster Material`; 67 fixes e.g. `Power Juice Material→Consumable/Buff`, `Huskberry Consumable→Ammo/Husk`, `Powercharm/Powertalon` → `Consumable • Charm`). `item_sources` 12,751 rows (gather/mining/bug/fish from `maps.json`, shop 5 merchants consolidated, trade Veggie Elder/Trenya/Pokke Points, farm spots/trees, small monsters via `Monsters/monsters-material.json`, plus `monster_drops`/`quest_rewards`); fully covered 1083/1083. **Combine** 432 recipes (147 Normal + 18 Alchemy + 7 Treasure) with `combine_type`/`chance` and Book order (`ORDER BY item_combine.id`, ISO `Book of Combos` + `Alchemy Guide`), clickable `A x1 + B x1 = Result x1 • 90%` in detail and global list `/items/combine`.
- **Skills / Decorations:** 99 families, 214 abilities, 192 jewels. `seed.rs:759` aliases `Defence`/`Defense` etc. No mismatches on overlapping decoration slot/points.

`src-tauri/src/ass.rs` isolates the ASS port to the **algorithm only** (equivalence grouping, jewel solver, Torso Inc, bad-skill fix). All gameplay data comes from the retail game. Armor names use the patched English strings (e.g. `Hornet Helm`, `Lava Helm`).

## Weapons

| Type           | Count (DB) | Count (game-extracted) |
| -------------- | ---------- | ---------------------- |
| Great Sword    | 176        | 176                    |
| Long Sword     | 122        | 122                    |
| Sword & Shield | 161        | 161                    |
| Dual Blades    | 140        | 140                    |
| Hammer         | 190        | 190                    |
| Hunting Horn   | 95         | 95                     |
| Lance          | 177        | 177                    |
| Gunlance       | 87         | 87                     |
| Light Bowgun   | 133        | 133                    |
| Heavy Bowgun   | 103        | 103                    |
| Bow            | 116        | 116                    |
| **Total**      | **1500**   | **1500**               |

- Catalog: 1500 names present in the ISO weapon block (`Buster Sword` at `0x23D6…`, `Iron Katana`, `Bone Katana` …). Three minor alias diffs remain (`Carbalite Sword`, `Carbalite Sword+`, `Gold Semi-Auto` vs `Gold Semiauto` in the extracted DB) — same item, spacing/casing.
- Tree: `upgrade_path` faithful to the in-game Smith tree (e.g. `Ravager Blade → Ravager Blade+ → Tactical Blade`). `weapon_craft` forge/upgrade resolves `item_id` via `SELECT id FROM items WHERE name=?` (`seed.rs:394`); 0 missing FKs. `weapon_materials` (5137) and `weapon_craft` (1075) totals are coherent once forge+upgrade arrays are denested (~4800–5100 materials).
- Presentation order: `Great Sword` through `Bow` as above; each type internally sorted by `id` (creation order). The frontend `Smith (Game Order)` sort reflects this.

## Monsters

- **Before:** 56 (52 Large + 4 Giant). **After:** 83 (54 Large + 25 Small + 4 Giant) — added 27 small monsters previously missing:
  `Felyne, Melynx, Shakalaka, Vespoid, Hornetaur, Great Thunderbug, Anteka, Popo, Kelbi, Mosswine, Aptonoth, Apceros, Giaprey, Giadrome, Velociprey, Genprey, Ioprey, Remobra, Cephalos, Hermitaur, Ceanataur, Bullfango, Bulldrome, Conga, Blango` plus `Rusted Kushala Daora` and `Scarred Yian Garuga` (Large). Species coverage now includes `Herbivore 6, Lynian 4, Neopteron 4` for small monsters.
- **Order:** Reordered to the in-game order (`Felyne 1, Melynx 2, Shakalaka 3 … Purple Gypceros 28, Hypnocatrice 29, Remobra 30, Rathian 31, Rathalos 34 … White Fatalis 83`). `ORDER BY id` (`src-tauri/src/db/queries.rs:445`) is now the game order; the UI preserves it when filtering (`src/routes/[game]/monsters/+page.svelte:8` `sizeFilter='large'` default, `All` shows 83, filtered lists keep relative order).
- **Weaknesses:** 163 rows covering Large monsters; small monsters have no hitzones (detail view handles missing data).
- **Drops / Equipment:** 2319 / 1880 rows (Large only); small carves (e.g. `Kelbi Horn`) exist as items and can be added later if needed. Descriptions backfilled for all 83 via `seed.rs:114`.

## Quests

- **Base game (ISO):** 559 quests validated against the `DATA.BIN` string table (`Mountain Herb Picking` at `335602928`, etc.). Each quest record = 13 string-pointer offsets + numeric block; name / objective / description / monsters / client resolve cleanly from the record base (e.g. `Hunt the Carnivore!` → `Slay 5 Giaprey`, client `Pokke Village Guard`). Hub distribution: `Elder 95, Nekoto 35, Guild Low 89, Guild High 77, Guild G 116, Training 140, Treasure 7`.
- **Guild rank fix (from ISO schema):** `guild_high` previously held 165 quests with `stars` 6–11, mixing HR4-6 (★6-8) with G-rank quests (`stars` 9–11) that belong in `guild_g`. Corrected: moved the 88 `stars >= 9` quests to `guild_g` and normalized to `G★1/2/3` (`stars` 9→1, 10→2, 11→3). Result: `guild_high` 77 (★6-8 = HR4-6), `guild_g` 116 (G★1-3). `stars` is now the faithful MHFU difficulty tier, not a global lineup index.
- **Other split:** Former `other` hub (147: `Training 140` + `Gathering 7`) split into `training` (140) and `treasure` (7: `Treasure in the Mountains!` … `Treasure in the Grt Forest!`). Ordering `elder → nekoto → guild_low → guild_high → guild_g → event → challenge → training → treasure` (`src-tauri/src/db/queries.rs:1157`).
- **Event / challenge quests:** 37 event + 14 challenge quests, **not in the base ISO** (they are downloadable). Extracted from the MHP2G Quest Editor kit `*.bin` files (`reward_money` at `0x54`, `contract_fee` at `0x50`, verified across EU↔JP) and validated against GameFAQs/"Event Quests" guide (objective, locale, reward, monsters). JP-exclusive (Famitsu/Dengeki) quests carry `name_original` (Japanese) + English `name`. Total 610 quests.
- **UI grouping:** Quests are grouped into collapsible accordions by `stars` within each hub (`src/routes/[game]/quests/+page.svelte`); first group expanded by default; `guild_g` labels `G★1-3`.

## Notes

- `src-tauri/src/ass.rs` is algorithm-only; no gameplay data is taken from ASS.
- All `mh2g_*.json` files are loaded idempotently (`INSERT OR IGNORE`, `seed.rs:6`, `clear_mh2g` in FK-safe order). FKs (`weapon_materials`, `weapon_craft`, `armor_materials`, `monster_drops`, `item_combine`) have no missing `item_id`. `items.subcategory` and `item_combine.combine_type`/`chance` are added via `apply_migrations()` with `pragma_table_info` checks; `seed_items` also runs a `UPDATE ... WHERE category != ? OR subcategory != ?` backfill (idempotent, keeps existing installs in sync).

## Recommendation

Keep `mh2g_*.json` as the retail-faithful source. Items are now 100% sourced and the category/subcategory taxonomy matches the ISO `icon` system. Optional follow-up: numeric struct parsing (`attack`, `affinity`, `sharpness`, `slots`) directly from `DATA.BIN` for a weapon field-level audit, and decoding the ISO treasure/training-only gathering for a handful of Account items — not blocking, catalog fidelity already 100%.

## Armor Filtering

- Gender: `Both` shows all; `Male` shows `male + both`, `Female` shows `female + both` (`src/routes/[game]/armor/+page.svelte:47`). The redundant `All` option was removed.
- Hunter type: `All | Blademaster | Gunner` (`src/routes/[game]/armor/+page.svelte:57`). For `both` heads (e.g. `Rathalos Helm 40` vs `Rathalos Cap 20`) the higher `defense_base` per `set_id|rank` is treated as Blademaster, lower as Gunner; `both` chests/arms/waist/legs are usable by both.

# MH P3rd (MHP3rd) — Item Catalog & Acquisition

**Verdict:** The MHP3rd item catalog + acquisition data in `src-tauri/data/mhp3rd_*.json` is derived from MHP3rd-only sources. Base catalog/descriptions come from the per-game Fandom `MHP3:` pages (`MHP3: Item List` + `MHP3: Monster Item List`, both `Category:MHP3_Database`). The acquisition layer (buy prices, gather map+area, monster carve/break/capture/drop, and the combine list in game-book order) comes from the authoritative JP wiki `www.mhp3wiki.info`, rendered with Playwright (its tables are JS-rendered via the `table_edit2` plugin) and cached under `tmp_mhp3_upstream/wikipages/`. **No MHTri / MH3U data is used.**

## Status

| Aspect                | State                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Items                 | **1065** — real names, rarity, sell price; 0 duplicate names                                                                                                                                                                                                                                                                                                                                                                                                           |
| Categories            | `Consumable 55 / Material 964 / Ammo 46`; subcategories from section taxonomy + name heuristics (`Recovery, Buff, Food, Charm, Coating, Husk, Ore, Bone, Sac, Monster Material, …`)                                                                                                                                                                                                                                                                                    |
| Descriptions          | **291** (274 with CJK) — ~28 EN + ~263 **Japanese** (kept faithfully, flagged with a **🇯🇵 JP badge** in the detail UI via CJK detection `[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff]`)                                                                                                                                                                                                                                                                                   |
| Buy prices            | **181 items** have `buy_price`; sell/rarity corrected against the wiki                                                                                                                                                                                                                                                                                                                                                                                                 |
| Combine list          | **263 recipes** (202 Normal in `調合リスト` book order + 61 Alchemy) with `chance`; verified #1 Potion = Herb + Blue Mushroom 95%                                                                                                                                                                                                                                                                                                                                      |
| Monster drops         | **761 rows** (carve 238 / break 189 / capture 191 / drop 143, Low 378 / High 383) across **40 droptable monsters** with rank/part/quantity/probability — per-monster carve/break/capture tables                                                                                                                                                                                                                                                                        |
| Item ids              | Re-indexed to the MHP3rd **item-box (chest) order** (`scripts/reindex_mhp3rd_items.py`): matched against the per-game ordered list (books → consumables → plants/tools/baits/insects/ores/bones → ammo → tickets → monster materials), then all `item_id`/`result_item_id`/`component_item_id` references remapped. **0 dangling references** across weapon/armor materials, craft, combine, monster_drops, quest_rewards, item_sources; 0 duplicate ids (10001–11065) |
| Monsters              | **60** (Large + Small) with weaknesses / equipment links                                                                                                                                                                                                                                                                                                                                                                                                               |
| Weapons / Armor       | **972 weapons** / **1111 armor pieces** (sets via `derive_set_name`) + forge/upgrade materials                                                                                                                                                                                                                                                                                                                                                                         |
| Quests                | **378** (`village 96 · guild_low 88 · guild_high 100 · event 52 · hot_spring 7 · drink 16 · nyanta 3 · training 10 · challenge 6`); all 378 carry `name_original` (JP quest-board title = in-game order). Bilingual fields: `location_original`/`objective_original`/`description_original`                                                                                                                                                                            |
| Quest rewards         | **1867 rows** — JP reward material → `item_id` (Fandom `MHP3: Item List` + curated monster-material map); unresolved logged never orphaned                                                                                                                                                                                                                                                                                                                             |
| Item sources (gather) | **26 rows** — `gather` map + area per item (EN map names, `conditions: "Areas: …"`); shop/trade/farm not yet populated                                                                                                                                                                                                                                                                                                                                                 |
| Seed / schema         | Idempotent (`add_idempotency_constraints` dedup + UNIQUE indexes, `clear_game` removed; `schema_version` table); `norm_key` registered as SQLite scalar for `get_global_search`                                                                                                                                                                                                                                                                                        |

## Pipeline

- `scripts/fetch_mhp3rd_fandom.py` → caches Fandom `MHP3:` wikitext.
- `scripts/fetch_mhp3rd_wiki_data.py` (Playwright/Chromium) → renders `index.php?<page>` and caches `tmp_mhp3_upstream/wikipages/`.
- `scripts/generate_mhp3rd_items.py` → base catalog + EN/JP descriptions.
- `scripts/generate_mhp3rd_item_sources.py` → buy/sell/rarity + gather sources + combine list.
- `scripts/generate_mhp3rd_monster_drops.py` → full carve/break/capture/drop tables.

## Known gaps

- **Shop/trade/farm source rows** (`item_sources` type shop/trade/farm) not yet generated; gather (`mining`) is populated. Small-monster carves are in the wiki `小型モンスター` page but not yet absorbed.
- **Chest order** is derived from the per-game ordered item list (kouryaku.ohuda.com, game category order) — a faithful proxy; ~323/575 box items were matched by JP→EN, the remaining catalog items stay at their prior relative order after the matched block.
- **Unresolved JP names** are logged (never orphaned): `scripts/mhp3rd_items.log`, `mhp3rd_item_sources.log`, `mhp3rd_monster_drops.log`. Some monster-material JP names lack an EN mapping in the current catalog.

## Verification

`svelte-check` → 0 errors/0 warnings. `cargo test` → 9 tests pass (ASS + `db::queries` idempotency/migration/global_search). `cargo check` → no errors. All seeds deserialize cleanly from the new JSON (items / item_combine / monster_drops / item_sources / quest_rewards structs). `src/lib/utils/norm.ts` mirrors Rust `norm_key` for accent-insensitive list filtering.

## Source

`www.mhp3wiki.info` is reachable via `index.php?<page-name>` (e.g. `index.php?調合リスト`), but its tables are client-rendered — hence Playwright. The archive.org copies of `/wiki/*` are absent, so live rendering is the only route. Quests are supplemented by `scripts/mhp3rd_quest_rewards.log` for unresolved JP reward names.
