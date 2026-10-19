# Fidelity Report — MH2G Data

**Verdict:** The MH2G dataset in `src-tauri/data/mh2g_*.json` is faithful to the retail UMD (English Patched). The MHFU-ASS 2017 CSV snapshot is stale and is used only as algorithm reference, never as a data source.

## Summary

| Metric                                      | Result                                                                                                                                 |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Armor                                       | 2075 pieces (head/chest/arms/waist/legs), 949 sets                                                                                     |
| Items                                       | 1244 (1083 + 161 upstream books/ores/fish/meats/insects/seeds/herbs/mushrooms/eggs; 12,751 `item_sources` rows + 432 combines)         |
| Item categories                             | `Consumable 91 / Material 913 / Ammo 79` (67 fixes, `subcategory` Charm/Husk/Coating etc., `Powercharm` → `Consumable • Charm`)        |
| Combine recipes                             | 432 (147 Normal + 18 Alchemy + 7 Treasure, with `chance` + Book order)                                                                 |
| Skill families / abilities                  | 99 / 214                                                                                                                               |
| Decorations                                 | 192                                                                                                                                    |
| Weapons                                     | 1500 (11 types)                                                                                                                        |
| Monsters                                    | 83 (54 Large, 25 Small, 4 Giant)                                                                                                       |
| Quests                                      | 610 (95 Elder, 62 Nekoto incl. restored 9★, 89 Guild Low, 77 Guild High, 89 Guild G, 140 Training, 7 Treasure, 37 Event, 14 Challenge) |
| Weapon materials / craft                    | 5137 / 1500 weapons with forge+upgrade rows (was 1075; SNS/gunner backfilled)                                                          |
| Monster weaknesses / drops / equipment      | 420 / 3402 / 9019 (weaknesses: Scarred/Rusted + thin + smalls Body-only; equipment join-derived family-capped)                         |
| Defense / rarity / slots (armor)            | **100% match** vs retail (0 mismatches)                                                                                                |
| Skill points (normalized, 1828 overlapping) | 40 diffs — **all favor our DB** (ASS has ±1 and sign flips, e.g. Kirin Crest X Protection -2 vs +2)                                    |
| Decorations (overlapping)                   | 0 mismatches on slot/points                                                                                                            |
| Weapons per type                            | **100% match** vs game-extracted DB (see below)                                                                                        |
| Monster order                               | In-game order (Felyne 1, Melynx 2, Shakalaka 3 … Rathian 31, Rathalos 34 … White Fatalis 83)                                           |

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
- **Weaknesses:** 420 rows (was 163): Scarred (7 parts) + Rusted (7) filled, thin larges expanded to upstream part lists, 25 smalls Body-only. Existing guide-derived values kept; upstream diffs logged in `scripts/mh2g_weaknesses.log`. Part flinch HP (`stagger_hp`, 244 rows) backfilled from `basic-parts-duration`. Windwrap/enraged/guard state variants skipped (single-row schema).
- **Hunt reference (new tables):** `monster_ailments` 214 rows (Poison/Paralysis/Sleep/KO tolerances, 69 monsters incl. 16 KO smalls) + `monster_tools` 354 rows (Pitfall/Shock/Flash/Sonic seconds, Dung/Meat bools, 59 monsters) from `status/item-effectiveness.json`; new Ailments + Traps & Items detail sections. Stats `appear` arrays (no quest keys) + crown `size.json` deferred.
- **Drops / Equipment:** 3402 / 9019 rows (equipment join-derived: gear links to monsters dropping its mats, family-trio capped to exclude ores/bones; all 65 drop-bearing monsters covered, 18 dropless smalls correctly zero). Source: `Monsters/monsters-material.json` (game-extracted, matches GameFAQs FAQ 78652 cell-for-cell on Scarred One-Eyed; `Daimyo Jewel` typo aliased to `Daora Jewel`); 18 unrated smalls (e.g. `Kelbi`) stay item-side only, `Felyne` has no drops anywhere. Generator: `scripts/generate_mh2g_monster_drops.py`, existing installs backfilled via `patch_mh2g_monster_drops` (`seed.rs`). **Rate correction pass (extraction-arbitrated, mhfu-db wins):** 133 pre-existing rows corrected + 33 shadow-rank rows deleted (3435 → 3402) via `scripts/correct_mh2g_drop_rates.py` + `patch_mh2g_drop_corrections` (166-op audit in `scripts/mh2g_drop_corrections.log`). Fixed: 3 swapped pairs (Rathalos Head/Low Shell↔Scale, Purple High Nova↔Light, Diablos Tail/Low Shell↔Tail), Gendrome/Velocidrome rank shadows, regular Garuga G Body Thick 64% → 53%. No MHF1 contamination found (deviations were swaps/shadows/merge errors, verified vs Fandom MHF2/U tabs). 555 alternate-item-name rows (Fatalis Shell vs HvyFatalisShl etc.) deliberately kept, logged REVIEW-KEEP. Descriptions backfilled for all 83 via `seed.rs:114`.

## Quests

- **Base game (ISO):** 559 quests validated against the `DATA.BIN` string table (`Mountain Herb Picking` at `335602928`, etc.). Each quest record = 13 string-pointer offsets + numeric block; name / objective / description / monsters / client resolve cleanly from the record base (e.g. `Hunt the Carnivore!` → `Slay 5 Giaprey`, client `Pokke Village Guard`). Hub distribution: `Elder 95, Nekoto 62 (incl. restored 9★), Guild Low 89, Guild High 77, Guild G 89, Training 140, Treasure 7`.
- **Guild rank fix (from ISO schema):** `guild_high` previously held 165 quests with `stars` 6–11, mixing HR4-6 (★6-8) with G-rank quests (`stars` 9–11) that belong in `guild_g`. Corrected: moved the 88 `stars >= 9` quests to `guild_g` and normalized to `G★1/2/3` (`stars` 9→1, 10→2, 11→3). Result: `guild_high` 77 (★6-8 = HR4-6), `guild_g` 116 (G★1-3). `stars` is now the faithful MHFU difficulty tier, not a global lineup index.
- **Nekoht-9 restore:** 27 Village-9★ quests (`Secret Request: Nargacuga!`, `Monster Hunter`, `Land of the Tremors`…) were re-homed into `guild_g` by the rank fix; moved back (`nekoto` 35→62, `guild_g` 116→89). 11 dual-hub names exist in both upstream `nekoht.json` and `gal-3.json` — the gal-3-difficulty-matching copy stays (e.g. G3 `Land of the Tremors`), the other copy moved. Urgent/optional same-name doubles keep the flag on the key (lower-id) copy only. Result: 20 urgents flagged (upstream: 21 markers; elder `Absolute Power` has no counterpart row).
- **Quest details:** `is_urgent` backfilled (was all-false; UI badge now live), 32 event requirements + 6 challenge descriptions filled from monsterhunterwiki.org, `Fortess` typo + whitespace fixed, JUMP Emperor (565) G9→G3 (confirmed `JUMP 爆炎の帝`). `name_original` kept (truncated real JP, not mojibake). Training (140, completion-only), Paw-Pass gathering (12z zenny-only) and early zenny-only quests verified correct-zero (wiki shows money-only, no reward tables).
- **Quest rewards:** 1273 rows (was 1269): 4 sourced Special-Reward tickets (Puppet/Hypno/Lavasioth/SpecPuppet, NULL rate = unknown). Hunting Basic/Bonus % tables need GameFAQs FAQ 59556 (bot-blocked; deferred with source documented in `scripts/mh2g_quest_rewards.log`).
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

| Aspect               | State                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Items                | **1044** — real names, rarity, sell price; 0 duplicate names                                                                                                                                                                                                                                                                                                                                                                                                           |
| Categories           | `Consumable 55 / Material 943 / Ammo 46`; subcategories from section taxonomy + name heuristics (`Recovery, Buff, Food, Charm, Coating, Husk, Ore, Bone, Sac, Monster Material, …`)                                                                                                                                                                                                                                                                                    |
| Descriptions         | **291** (274 with CJK) — 17 EN + 274 **Japanese** (verified real game text, not mojibake — earlier report was a terminal artifact; kept faithfully, flagged with a **`JP`** badge in the detail UI via CJK detection); 753 catalog items have no description (genuinely missing source text, not a seed bug)                                                                                                                                                           |
| Buy prices           | **150 items** have `buy_price`; sell/rarity corrected against the wiki                                                                                                                                                                                                                                                                                                                                                                                                 |
| Combine list         | **263 recipes** (202 Normal in `調合リスト` book order + 61 Alchemy) with `chance`; verified #1 Potion = Herb + Blue Mushroom 95%                                                                                                                                                                                                                                                                                                                                      |
| Monster drops        | **1949 rows** across **60/60 monsters** (small pages + 6 new guide pages parsed; 200 unsourced xeno rows pruned — foreign-monster items untraceable to any wiki token; subspecies High tables blocked by wiki login walls, documented)                                                                                                                                                                                                                                 |
| Item ids             | Re-indexed to the MHP3rd **item-box (chest) order** (`scripts/reindex_mhp3rd_items.py`): matched against the per-game ordered list (books → consumables → plants/tools/baits/insects/ores/bones → ammo → tickets → monster materials), then all `item_id`/`result_item_id`/`component_item_id` references remapped. **0 dangling references** across weapon/armor materials, craft, combine, monster_drops, quest_rewards, item_sources; 0 duplicate ids (10001–11065) |
| Monsters             | **60** (Large + Small) with rewritten hitzones / trap tools / real descriptions / equipment links / charms + comrades browsers                                                                                                                                                                                                                                                                                                                                         |
| Weapons / Armor      | **973 weapons** (incl. previously missing Ludroth Splashammer; element/status from mhp3db.github.io `Special` fields; all with forge/upgrade craft rows) / **1106 armor pieces** (5 BOM rows removed; sets via `derive_set_name`) + forge/upgrade materials                                                                                                                                                                                                            |
| Quests               | **378** (`village 96 · guild_low 88 · guild_high 100 · event 52 · hot_spring 7 · drink 16 · nyanta 3 · training 10 · challenge 6`); all 378 carry `name_original` (JP quest-board title = in-game order). Bilingual fields: `location_original`/`objective_original`/`description_original`                                                                                                                                                                            |
| Quest rewards        | **1867 rows** (incl. 6 S-rank rows with conditions; event Special-Reward tickets need per-quest mapping, deferred) — JP reward material → `item_id` (Fandom `MHP3: Item List` + curated monster-material map); unresolved logged never orphaned                                                                                                                                                                                                                        |
| Item sources (extra) | **2016 rows** — `trade` 528 / `drop` 515 / `carve` 374 / `farm` 120 / `gather` 110 / `shop` 102 (`Yukumo Village Shop`) / `mining` 100 / `capture` 98 / `bug` 36 / `fish` 30 / `other` 3; shop/trade/farm fully populated (SDD 001 verified — earlier "gather-only" state is resolved)                                                                                                                                                                                 |
| Seed / schema        | Idempotent (`add_idempotency_constraints` dedup + UNIQUE indexes, `clear_game` removed; `schema_version` table); `norm_key` registered as SQLite scalar for `get_global_search`                                                                                                                                                                                                                                                                                        |

## Pipeline

- `scripts/fetch_mhp3rd_fandom.py` → caches Fandom `MHP3:` wikitext.
- `scripts/fetch_mhp3rd_wiki_data.py` (Playwright/Chromium) → renders `index.php?<page>` and caches `tmp_mhp3_upstream/wikipages/`.
- `scripts/generate_mhp3rd_items.py` → base catalog + EN/JP descriptions.
- `scripts/generate_mhp3rd_item_sources.py` → buy/sell/rarity + gather sources + combine list.
- `scripts/generate_mhp3rd_monster_drops.py` → full carve/break/capture/drop tables (incl. small-monster page parsing + 6 thin-large pages).
- `scripts/generate_mhp3rd_weaknesses.py` → hitzone rewrite from 肉質 tables (first/normal-state block only).
- `scripts/generate_mhp3rd_charms.py` → 50,635 excavated charms (deduped) + `roundout_mhp3rd_monsters.py` (Fandom descriptions) + `generate_mhp3rd_comrades.py` (92/154/52 gear).

## 100% pass (MHP3rd gaps closed)

- Weaknesses 120 → 213 real hitzone rows (40/40 larges; 9 login-walled pages keep legacy rows; smalls have no hitzone source).
- New tables `monster_ailments`/`monster_tools` (MHFU) + `monster_tools` for MHP3rd (180 trap rows); `charms`/`charm_skills` (50,635); `comrade_weapons/armor/skills` (92/154/52); `stagger_hp` on weaknesses. Charms browser (skill search) + Comrades browser, MHP3rd-only sidebar tabs.
- Element/status filled for 405 weapons (mhp3db.github.io `Special`); all 973 weapons have craft rows; 139 zero-mat weapons backfilled; 5+2+1+1 junk rows removed; deco rarity 158/164; skill-level descriptions 182/215 (concept-mapped, uncertain pairs skipped).
- Quests: 44 key flags wiki-verified (+1 Hunt 1 Barroth); training/gathering zero-rewards verified correct (money-only); urgent/client/requirements have no wiki columns (documented).
- Items: 10526 kept as unresolved ticket placeholder (6 Ludroth U armor rows depend on it); farm strings fixed Pokke→Yukumo.
- Deferred with sources: hunting Basic/Bonus % (GameFAQs 59556, bot-blocked), subspecies High drops (wiki login), crown sizes, quest HP tables (no quest keys), armor/skill/weapon EN flavor (no source), comrade EN names (Fandom warns unreliable).

## Known gaps

- **Shop/trade/farm source rows** were seeded all along (`mhp3rd_item_sources_extra.json`, 2016 rows) — SDD 001 verified coverage and fixed the real gap, which was visibility: `get_item_sources` (`queries.rs`) excluded all `carve/capture/drop/break` junction rows, hiding 987 small-monster rows (`source_id NULL`, e.g. `Conga - carving`). The exclusion now applies only with `source_id NOT NULL` (mirrored `monster_drops` dupes stay excluded); covered by `small_monster_sources_are_visible_without_duplicates` test. Small-monster carves from the wiki `小型モンスター` page are absorbed.
- **Chest order** is derived from the per-game ordered item list (kouryaku.ohuda.com, game category order) — a faithful proxy; ~323/575 box items were matched by JP→EN, the remaining catalog items stay at their prior relative order after the matched block.
- **Unresolved JP names** are logged (never orphaned): `scripts/mhp3rd_items.log`, `mhp3rd_item_sources.log`, `mhp3rd_monster_drops.log`. Some monster-material JP names lack an EN mapping in the current catalog.

## MHW + Iceborne — Catalog & Order

**Verdict:** MHW+Iceborne in `src-tauri/data/mhw_*.json` is seeded from `MHWorldData` (1339 base items, 93 base monsters, 3544 weapons, 5680 rewards) + Fandom `MHWI` wiki (543 extra rows for icons/how-to-get). All 1359 items have `sort_order` (Chest order `1-1339` MHWorldData + `2000+` extras), 94 monsters have `sort_order` (Small 1-23 then Large 1001-1071, species corrected), 3544 weapons have `sort_order` per-type DFS (Smith tree `Great Sword → Bow` with `Charge Blade`/`Insect Glaive`).

| Aspect         | Count                                                                                                                                    | Source                                                                                             |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Items          | 1359 (World+Iceborne incl. 20 event/collab)                                                                                              | `MHWorldData/item_base.csv:1339` + Fandom `MHWI:_Item_List` 549 + `MHW:_Item_List` 267             |
| Monsters       | 94 (Small 23 + Large 71 incl. variants)                                                                                                  | `MHWorldData/monster_base.csv:93` + `Grimalkyne` (Fandom Lynian)                                   |
| Weapons        | 3544 (14 types, 12-HEX per-rarity icons r1..r12)                                                                                         | `MHWorldData/weapon_base.csv`                                                                      |
| Drops          | 5862                                                                                                                                     | `MHWorldData/monster_rewards.csv:5680` + 182 Fandom                                                |
| Icons          | 343 item + 94 monster + 294 weapon (12 rarities × types, incl. blue variants) + 36 mantles + 9 palico + 100 armor                        | Fandom `Category:Weapon_Icons` / `Item_Icons`                                                      |
| Decorations    | 404 (slot, rarity, skill1+2 with levels, skill icons)                                                                                    | MHWorldData `decorations/decoration_base.csv` via `scripts/generate_mhw_decorations_weaknesses.py` |
| Weaknesses     | 788 rows / 88 monsters (per-part cut/impact/shot + fire/water/thunder/ice/dragon; ailments dropped — no column)                          | MHWorldData `monsters/monster_hitzones.csv` (not the 0-3 star summary)                             |
| Equipment      | ~6461 armor + ~13923 weapon links, derived from material drops                                                                           | seed derivation (`armor/weapon_materials` ⨝ `monster_drops`)                                       |
| Gather sources | 911 rows (Ancient Forest / Wildspire / Coral / Rotten / Elder's Recess; area + rank in conditions; no Hoarfrost/Guiding Lands upstream)  | MHWorldData `locations/location_items.csv`                                                         |
| Skills         | 179 (+`Kulve Taroth Essence`) / 419 levels; `weapon_skill_points` for 638 special-skill weapons (bare-name fallback, points = max_level) | seed derivation from `weapons.skills`                                                              |

Chest order `COALESCE(sort_order,id)` (`queries.rs:1702` items, `queries.rs:494` monsters) and Smith order (`queries.rs:916` weapons) are faithful to in-game box/tree. Weapons filter no longer shows `All` (`weapons/+page.svelte:76` default `Great Sword`).

> MHW remaining notes: combine is Normal-only by design (crafting + Melder model, see spec `009`); deco crafting materials don't exist upstream (`decoration_materials` empty for MHW).

## MHW + Iceborne — 100% pass (spec 010 DONE)

**Verdict:** all twelve audited gaps closed with pinned upstreams (MHWorldData frozen master, Kiranico static HTML, Fandom api.php). Generators re-run converge; `mhw_010_phase_{a,b,c}` tests bank the counts (36/36 suite green).

| Aspect                   | Count                                                                                                                                                                                   | Source                                                                                                                 |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Ailments                 | 721 rows / 93 monsters (18 labels; 501 with buildup numbers)                                                                                                                            | `monster_ailments.csv` tolerances + Kiranico `Ailments` buildup/decay/damage/duration (`generate_mhw_kiranico.py`)     |
| Traps                    | 465 rows / 5 tools (Pitfall/Shock/Vine + Flash Bomb + Dung; 386 with seconds, 186 diminishing chains)                                                                                   | `monster_base.csv` trap cols + Kiranico trap durations (`Blind` → Flash)                                               |
| Stagger proxy            | 317/788 hitzone rows carry Part-Break values (exact part-name matches; COALESCE so real values win)                                                                                     | Kiranico `Part Breakability`; **proxy caveat**: break HP ≠ flinch HP                                                   |
| Hunter charms            | 317 charms / 1247 materials / 207 previous-links (rank tree I→V, rank-tree browser + sidebar entry)                                                                                     | `charms/charm_base.csv` + `charm_craft.csv` (new `hunter_charms` tables — excavated `charms` shape doesn't fit)        |
| Quest roster             | 517/521 with monsters (4 gathering quests correctly empty; targets-only rosters kept over locale lists)                                                                                 | `quests/quest_monsters.csv` (1411 rows; diffs logged REVIEW)                                                           |
| Set bonuses              | skill1 verified everywhere + 58 skill2 fills (new `bonus_skill2/required2`; 27 two-skill sets) + display on set detail, list cards, piece detail, skill reverse refs (hidden when NULL) | `armorset_base.csv` → `armorset_bonus_base.csv` (1:1 positional join, 351/351)                                         |
| Descriptions             | weapons 3544/3544 + armor 1595/1595 (deterministic stat summaries — World has no gear flavor text) + monsters 94/94                                                                     | generated from stats (hidden element, elderseal, phial/shelling/kinsect, horn notes, sharpness max); 4 monsters Fandom |
| Deco acquisition         | 398/404 (24 meldable jewels with RP/MP costs + per-feystone odds; 5 event jewels + 1 have no row)                                                                                       | Kiranico `/en/melder` + `/en/decorations` (new `decorations.acquisition`; materials stay empty — none exist)           |
| Melder sources           | 212 rows with RP/MP costs in conditions (vs 10 before)                                                                                                                                  | Kiranico `/en/melder` (`mhw_item_sources_melder.json`)                                                                 |
| Shop / buy prices        | 56 shop rows; buy prices for all (6 corrected)                                                                                                                                          | Kiranico `/en/provisions` (Fandom How-To-Get cross-check: 0 gaps)                                                      |
| Hoarfrost/Guiding gather | 75 rows with region/node conditions (Forest/Wildspire/Coral/Rotten/Volcanic/Tundra)                                                                                                     | Fandom `MHWI:_Item_List` How-To-Get (`mhw_item_sources_gather_fandom.json`)                                            |
| Combine                  | 188 JSON rows (wiki 98/98 covered; 4 legitimate local extras); DB 186/188 — `uq_item_combine2` (+qty) so yield variants land                                                            | upstream `item_combination_list.csv` + Fandom `MHW:_Combination_List` arbiter                                          |
| Drops                    | 5826 rows (5680 upstream + 182 Fandom −1 phantom −35 xeno; 32 Kiranico-filled %, 116 NULL unknown-rate; `uq_monster_drops2` keeps stack variants)                                       | `monster_rewards.csv` + Kiranico per-monster tables; Fandom merge audited (mhp3rd xeno-prune pattern)                  |
| Phantom delete           | Uragaan + Fulgurbug `capture` row removed (0 pure-capture rows upstream; Fulgurbug Zinogre-only; 0 Fandom mentions)                                                                     | Phase 0 proof (`mhw_100_phase0.log` §8)                                                                                |

By design (documented, not built): World combines never fail (`chance` NULL, badge hidden); no urgent-quest system (`is_urgent=0`, mhp3rd precedent); Sonic/Meat trap seconds (no numeric source); weapon augments (no upstream CSV); endemic life/canteen/layered armor.

## MH Wilds — Catalog (spec 004 DONE)

**Verdict:** Wilds core seeded from the MHDB Wilds API (`scripts/generate_mhwilds_from_mhdb.py`, browser UA — the API 403s the default python UA). 14 curated files, dedicated PK offsets (monsters 20001+, items/skills/decos 30001+, weapons 40001+, armor 50001+, sets 30000+, combine 900000+).

| Aspect      | Count                                                                                                                       | Source                                                             |
| ----------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Monsters    | 34 Large (descriptions; no smalls upstream)                                                                                 | `/en/monsters`                                                     |
| Weaknesses  | 340 rows / 34 monsters (per-part multipliers ×100, mh2g scale)                                                              | monster detail `parts[]`                                           |
| Drops       | 1775 (carve 583 / break 431 / reward 761; Low 477 / High 1298)                                                              | monster detail `rewards[]` (carve-_/wound-_/broken-* mapped)       |
| Items       | 773 (Material + descriptions; no categories upstream)                                                                       | `/en/items`                                                        |
| Combines    | 121 (crafting recipes, Normal)                                                                                              | item detail `recipes[]`                                            |
| Skills      | 179 / 442 levels (armor/weapon/set/group kinds)                                                                             | `/en/skills` (+ ranks)                                             |
| Decorations | 361 (slot/rarity/skills; no materials upstream)                                                                             | `/en/decorations`                                                  |
| Weapons     | 1188 (14 types, per-type sort, sharpness/slots/elements/status/skills, forge+upgrade materials; elements from `specials[]`) | `/en/weapons` (edges) + Kiranico Wilds list order (Smith sequence) |
| Armor       | 714 (183 sets, resistances/skills/materials)                                                                                | `/en/armor`                                                        |
| Equipment   | ~1273 armor + ~2499 weapon links derived from material drops                                                                | seed derivation (002b pattern)                                     |

Ordering: Chest `sort_order` is an alphabetical proxy (no retail box order published); weapons follow the Kiranico Wilds Smith DFS sequence (scripts/reorder_mhwilds_weapons.py Playwright tab scrape; MHDB previous/branches kept as the edge source, 0 orphans); monsters alphabetical. Icons: no Wilds sets offline — monsters reuse mhw slug paths (UI fallback), weapons/armor reuse the mhw rarity scheme, items NULL, decos mhfu path.

Gaps (no upstream data, not seed bugs): quests (empty list), charms (64 upstream, no table), small monsters. Tracked as Phase 7 in `roadmap.md` and as specs `012`–`014` (Wilds quests / charms / small monsters + endemic).

## MH Rise — Base catalog (spec 003 Phase A bulk)

Phase A content was fully superseded by the Phase B Sunbreak v16 Kiranico scrape (see below); the bulk scripts remain as provenance (`scripts/generate_mhrise_bulk.py`).

## MH Rise: Sunbreak — Full catalog (spec 003 Phase B DONE)

**Verdict:** full Sunbreak v16 from Kiranico (`scripts/fetch_mhrise_kiranico.py` — cached, throttled, resumable; ~6900 detail pages) merged by `scripts/merge_mhrise_phaseb.py`. Dedicated PK offsets (monsters 30001+, items/skills/decos 40001+, weapons 50001+, armor 60001+, sets 40001+, quests 200001+, rewards 900001+).

| Aspect      | Count                                                                                                                                                       | Source                                                                                                                                                                                                           |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Monsters    | 112 (78 Large + 34 Small, descriptions)                                                                                                                     | CrimsonNynja (names) — Kiranico physiology/drops keyed by exact name match (112/112)                                                                                                                             |
| Weaknesses  | 629 rows / 112 monsters (state-0 hitzones: slash/strike/shell + elements, mh2g scale)                                                                       | Kiranico monster `Physiology` table                                                                                                                                                                              |
| Drops       | 7092 (carve/break/reward/drop/palico/capture, Low/High/Master with %)                                                                                       | Kiranico monster drop tables (shape-detected, no headers)                                                                                                                                                        |
| Items       | 1642 (taxonomy + descriptions backfilled + 74 gather rows)                                                                                                  | Badge87 + 554 Sunbreak materials from drops/quests/mats                                                                                                                                                          |
| Quests      | 946 (UNION CrimsonNynja 327 incl. retired Rampage + Kiranico 619; all hubs incl. Anomaly/Follower; 7508 rewards)                                            | CrimsonNynja + Kiranico quest reward tables                                                                                                                                                                      |
| Skills      | 147 / 459 levels (Kiranico level text; Badge87 fallback)                                                                                                    | Badge87 + Kiranico skill pages (+ deco price cross-ref)                                                                                                                                                          |
| Decorations | 243 (slot, Kiranico skill levels, materials, prices)                                                                                                        | Kiranico deco pages (union by name)                                                                                                                                                                              |
| Weapons     | 3953 (14 types, Smith sort, attack/affinity/element/status/sharpness/rarity/defense/materials/tree/slots (2508 backfilled from Kiranico list decoN badges)) | Kiranico weapon pages for stats/materials (element codes anchored: Rathalos→1 Fire … Magnamalo→9 Blast) + Game8 tree pages for upgrade edges (scripts/parse_game8_rise_trees.py, scripts/rebuild_mhr_weapons.py) |
| Armor       | 1591 (410 stem-derived sets, defense/res/slots/skill-levels/materials; Low ≤R3 < High ≤R7 < Master)                                                         | Kiranico armor pages (slots from `decoN.png`, skills `Name Lv N`)                                                                                                                                                |
| Equipment   | derived from material drops (002b pattern; runs AFTER drops in seed order)                                                                                  | seed derivation                                                                                                                                                                                                  |

Ordering: alphabetical proxy for non-weapon lists (no retail order published); weapons follow the Game8 Smith DFS sequence (matches the Kiranico list order except single-node Primordial/event trees). Icons: no Rise sets offline — same fallback scheme as Wilds.

Gaps (documented, not seed bugs): talismans (no table), weapon descriptions (not on Kiranico detail). Lost Code / Stuffed event weapons (30) have no tree upstream and stay roots; one Game8/Kiranico order quirk (Sinister Soulpiercer pair) keeps the Game8 edge.

Lesson learned (FK failure during merge): NEVER layer merges on previously-merged files — name cleanup can collapse rows and orphan FK children. Skills/levels regenerate deterministically from source with an orphan-level self-check assert.

## MH Rise: 100% — Phase A–E (spec 011)

Builds on the 003 baseline. New tables, seeds, and tests for everything Kiranico v16 publishes but 003 didn't import. **Verdict:** all five phases DONE — `verify_mhr_100.py` PASSED, `patch_mhr_011` warms up existing installs, gate clean.

| Aspect             | Count                                                                                                                          | Source                                      |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------- |
| Status ailments    | 1120 (10 ailments × 112 monsters, zero nulls)                                                                                  | Kiranico monster detail Status table        |
| Part-break         | 468 rows / 72 monsters (541 raw → UNIQUE dedupe; multi-break dupes like Aknosom Wing×2 collapse)                               | Kiranico monster detail Part table          |
| Trap effectiveness | 270 rows / 54 monsters (Effective→NULL, Ineffective→0; remaining 58 stay absent)                                               | Fextralife item-effectiveness list          |
| Buddy gear         | 322 weapons (161 palico + 161 palamute) + 362 armor (181 + 181, upstream x2 dupes deduped); EN names, melee/ranged split stats | Kiranico Palicoes/Palamutes list views      |
| Rampage skills     | 247 unique (32 Kiranico dupes from same-name weapon-type pages collapsed) + 6822 (skill, weapon) links (33 dupes deduped)      | Kiranico 279 rampage-skill detail pages     |
| Rampage decos      | 26 (stored as Decoration items, generic mhfu icon, item id 60001+)                                                             | Kiranico rampage-decorations list           |
| Talismans          | 11,849 generator rows (1 per skill pair, max lvl/slots); 4 base methods (anima/rebirth/reincarnation/wisp-of-mystery)          | `3geek14/mhr-charms` GitLab CSV (datamined) |
| Item-mix           | 178 (result, component) combine rows, chance 100 (Rise crafting always succeeds); Potion=Herb spot-verified                    | Kiranico item-mix view                      |

Gaps (documented): Sunbreak talisman methods `eve`/`haze` aren't in upstream CSV (only the 4 base methods ship); wirebug widgets, JP motion values, anomaly investigations, qurious crafting (procedural — unbounded). Buddy gear has no materials upstream (stats-only); `rampage_decorations` materials absent from Kiranico.

ID offsets (game-2 space): buddy weapons 70001+, buddy armor 70501+, rampage skills 80001+, rampage decos (= items) 60001+, talismans 60000+. No collisions with the 003 ranges (monsters 30001+, items 40001+, weapons 50001+, armor 60001+, quests 200001+).

## Verification

Current `cargo test` count is 43 (ASS + `db::queries` idempotency / migration / global_search / small-monster-visibility / MHW quest offset / MHW 010 phases A·B·C / set-bonus display / MHP3rd charms / MHR 011 phases A·B·C·D·E). Full gate: `npm run check` 0/0, ESLint 0 errors, Prettier repo-wide, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo build`, `node scripts/check-version.js` 0.1.1 OK, `npm audit --audit-level=moderate` (4 pre-existing transitive findings, accepted). `src/lib/utils/norm.ts` mirrors Rust `norm_key` for accent-insensitive list filtering.

## Source

`www.mhp3wiki.info` is reachable via `index.php?<page-name>` (e.g. `index.php?調合リスト`), but its tables are client-rendered — hence Playwright. The archive.org copies of `/wiki/*` are absent, so live rendering is the only route. Quests are supplemented by `scripts/mhp3rd_quest_rewards.log` for unresolved JP reward names. MHW uses `MHWorldData` + Fandom `MHWI` wiki via `api.php` (icons + how-to-get) and `Category:Weapon_Icons` for per-rarity weapon icons.
