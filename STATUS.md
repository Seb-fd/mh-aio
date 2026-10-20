# MH-AIO - Project Status

## Current Version: v0.1.1

> **All counts below are sourced from [`docs/data-coverage.md`](docs/data-coverage.md)** (generated per-table × per-game matrix). Regenerate with `py scripts/gen_data_coverage.py`; verify with `--check`.

**Games at 100% (content complete + verified):**

- **MH2G / Freedom Unite** (`mh2g`, dbId 5) — 1244 items, 432 combines (379 Normal + 39 Alchemy + 14 Treasure), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons, 2075 armor (348 sets), 610 quests, 99 skills, 192 decos, 15,671 `item_sources`, 3353 drops
- **MHP3rd** (`mhp3rd`, dbId 4) — 1065 items (Material 964 / Consumable 55 / Ammo 46), 398 combines (276 Normal + 122 Alchemy), 60 monsters (40 Large + 20 Small), 2200 drops, 972 weapons, 1106 armor (345 sets), 378 quests, 50,635 charms, 180 trap tools, Comrades gear (92/154/52)
- **MHW + Iceborne** (`mhw`, dbId 1) — 1359 items, 186 combines, 94 monsters (23 Small + 71 Large), 5826 drops, 3544 weapons, 1595 armor (351 sets), 521 quests, 404 decos, 788 weakness rows, 721 ailment rows, 465 trap rows, 317 hunter charms, Melder (211 recipes), Mantles/Boosters (20), Palico Gadgets (8 + 38 levels), **33 endemic creatures**
- **MH Rise + Sunbreak** (`mhr`, dbId 2) — 1668 items, 178 combines, 112 monsters (78 Large + 34 Small), 6403 drops, 3953 weapons, 1591 armor (410 sets), 946 quests / 6426 rewards, 147 skills, 243 decos, status ailments + part-break + traps, buddy gear (322 Palico + 362 Palamute), rampage (247 skills / 26 decos / 6822 links), 11,849 talismans

**Games seeded beyond core:**

- **MH Wilds** (`mhwilds`, dbId 3) — 773 items, 120 combines, 68 monsters (34 Large + 34 Small), 1399 drops, 1188 weapons, 714 armor (183 sets), 179 skills, 361 decos, 174 quests (Optional 81 / Event 60 / Assignment 33), 76 Wilds talismans, **70 endemic creatures**, 324 weakness rows. Seeded via specs `012` (quests) + `013` (charms) + `014` (small monsters + endemic). _Documented gap: no `quest_rewards` (upstream exposes reward text, not per-item counts)._

Specs done: `000` MH2G · `001` MHP3rd · `002`+`002b` MHW core+gaps · `003` MHR · `004` MHWilds core · `005` builds · `006` favorites · `007` import panel · `008` mobile (spec written) · `009` MHW tools/Melder · `010` MHW 100% · `011` MHR 100% · `012` Wilds quests · `013` Wilds charms · `014` Wilds small/endemic · `015` MHW endemic life.

---

## ✅ Completed

### Initial Setup

- [x] Node.js + Rust + Tauri v2 + Svelte 5 + Tailwind v4 stack
- [x] `svelte.config.js` with adapter-static + fallback
- [x] `vite.config.ts` with Tailwind v4 plugin
- [x] `src/app.css` with Tailwind theme + themed-bg utilities per game

### Game System

- [x] `src/lib/stores/game.ts` — 5 games, each with `theme` object
- [x] `game-selector.svelte`, `sidebar.svelte`, `header.svelte` (themed, CSS vars)

### Frontend (Svelte 5) — List + Detail Views

- [x] `monsters/` — list + `[id]` (weaknesses, drops, ailments, traps, dedicated armor sets) — Large/Small/All selector (Large default, order-preserving)
- [x] `weapons/` — list + `[id]` (tree, stats, materials)
- [x] `armor/` — list (`Sets`/`Pieces` toggle + Both/Male/Female + Blademaster/Gunner + rank filters), `sets/[id]`, `[id]`
- [x] `quests/` — list + `[id]` — grouped into collapsible difficulty accordions (★/G★, first expanded), hub order
- [x] `items/` — list (game-order Chest sort + name/rarity/price + `category • subcategory` + `Charm` + `Ammo/Husk` ISO taxonomy) + `combine/` single view (Normal/Alchemy/Treasure badge/filter + `success %` + Book order) + `[id]` with clickable combine `A x1 + B x1 = Result x1 • 90%`
- [x] `skills/` — list + `[id]` (levels, decorations, related armor/weapons)
- [x] `decorations/` — list + `[id]` (normKey search)
- [x] `charms/` — MHP3rd charms / MHW hunter charms / MHR talismans / Wilds talismans (skill search)
- [x] `comrades/` — MHP3rd + MHR buddy gear (Palico/Palamute)
- [x] `tools/` — MHW Mantles + Boosters + Palico Gadgets (`gameOnly: 'mhw'`)
- [x] `endemic/` — MHW + MH Wilds endemic creatures (`gameOnly: ['mhw', 'mhwilds']`)
- [x] `builds/` — **Armor Set Search** (ASS port)

### Armor Set Search (Athena's A.S.S. port) — NEW

- [x] `src-tauri/src/ass.rs` — Rust solver: equivalences, jewel solver (1/2/3-slot), Torso Inc multiplier, bad-skill auto-fix, `MAX_LIMIT 1000`, sort comparators
- [x] `search_armor_sets` Tauri command
- [x] UI: guided steps (Hunter Type / Skills / Details), up to 5 skills, ability selectors, HR/Elder rank gate, gender, weapon slots, piercings, Torso Inc, advanced options (collapsed)
- [x] English UI with quick "Try:" examples (Attack+Sharpness, Earplug+Wind, Critical)

### Global Per-Game Search — NEW

- [x] `global-search.svelte` in header — accent-insensitive (`strip_accents`/`norm_key`), debounced, grouped suggestions (monster/item/skill/weapon/armor/armor_set/quest/decoration)
- [x] `global_search` Tauri command + `SearchResult`

### Monster → Dedicated Armor Sets — NEW

- [x] `get_monster_dedicated_sets` (≥40% material score), rank-filtered
- [x] Monster detail: `Dedicated` (default) + `Uses 1 Material` (secondary) tabs; unified rank filter for drops and sets
- [x] Subspecies kept separate via exact `item_id` match

### Armor Sets / Gender / Hunter Type — NEW

- [x] `derive_set_name` groups D/S/U/X/Z variants into full sets (Kut-Ku Helm D → Kut-Ku D)
- [x] `/armor/sets/[id]` detail route (pieces + rank + skills)
- [x] Gender-locked armor: `gender` column, extracted from source CSVs; honors `Guardian Helm` (male), `Maiden's Hat` (female)
- [x] Gender filter simplified to `Both | Male | Female` (Both = show all, Male/Female include Both) — removed redundant All
- [x] Hunter type filter `All | Blademaster | Gunner` — for `both` heads distinguishes by higher defense (Helm vs Cap), other slots by `armor_type`

### Data Fidelity (MHP2G) — Updated

- [x] 2075 armor pieces (348 sets), **1244 items** (1083 fully sourced + 161 upstream books/ores/fish/meats/insects/seeds/herbs/mushrooms/eggs; **15,671 `item_sources` rows** across `gather/mining/bug/fish` from `maps.json`, `shop` 5 merchants consolidated, `trade` Veggie Elder + Trenya Boat + Pokke Points, `farm` Pokke Farm spots/trees, `small monsters` via `Monsters/monsters-material.json`, plus `monster_drops`/`quest_rewards`; verified vs `MHP2G` ISO `DATA.BIN` offsets), **432 combine recipes** (379 Normal + 39 Alchemy + 14 Treasure, with `combine_type`/`chance` and Book order), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests (Training 140 / Treasure 7 / Event 37 / Challenge 14), 99 skill families (214 abilities), 192 decorations
- [x] Item taxonomy re-derived from ISO `tmp_mhfu_upstream/items.json` `icon` + verb: `Consumable 128 / Material 1037 / Ammo 79` (sums to 1244) with `subcategory` (`Recovery, Buff, Food, Charm, Husk, Coating, Ore, Monster Material` etc.; `Powercharm/Powertalon` → `Consumable • Charm`, `Huskberry` → `Ammo • Husk`), 67 fixes (Power Juice, Mega Juice, Cold Meat, Gourmet Fish, Deodorant, 27 Bowgun S)
- [x] Monster order faithful to UMD Hunter's Notes (Felyne 1 … White Fatalis 83, `ORDER BY id`); Large/Small selector preserves order
- [x] Weapon order faithful to Smith tree (`Great Sword → Bow`, `ORDER BY CASE weapon_type`, `src-tauri/src/db/queries.rs`)
- [x] Quest hubs: `elder → nekoto → guild_low → guild_high → guild_g → training → treasure → event`; Other split into Training School and Treasure Hunt; Event quests (33 downloadable, not in ISO) validated against distribution file + wiki
- [x] Quest validation vs ISO `DATA.BIN` string table (`Mountain Herb Picking` at `0x13E…`, `Training`/`Treasure` labels); total 559 base quests match retail
- [x] Fixed 138 item sell prices vs game-extracted DB (Thawing Agent 100→10 etc.)
- [x] Item wings/ammo classified per ISO `icon` (Bowgun S + Coatings + Husks → `Ammo`, with `subcategory`); header artifact removed
- [x] `docs/fidelity-report.md` — single merged report: defense/rarity/slots 100%, weapons per-type 100%, monster catalog 83, quest hubs 610 (ISO string table + guild rank fix + kit Event/Challenge) vs retail UMD + game-extracted DB + item taxonomy/combine Book order

### Data — Monster Hunter Portable 3rd (MHP3rd / `mhp3rd`, DB id 4) — Seeded

- [x] **1065 items** (`Material 964 / Consumable 55 / Ammo 46`), **291 descriptions** (17 EN + 274 JP flagged with 🇯🇵 badge), **181 buy prices**; chest-order `id` remapped (0 dangling refs). **398 combines** (276 Normal in `調合リスト` book order + 122 Alchemy) with `chance` (0 Treasure by design).
- [x] **60 monsters** (40 Large + 20 Small), **2200 drops** (carve/capture/break/drop with rank/part/probability) across all droptable monsters; **monster weaknesses (213 rows) / equipment (4458 rows)** seeded.
- [x] **972 weapons**, **1106 armor pieces** (345 sets via `derive_set_name`), **8242 `armor_materials` / 4218 `weapon_materials` + 3856 craft rows** resolved.
- [x] **378 quests** (`village 96 · guild_low 88 · guild_high 100 · event 52 · drink 16 · training 10 · hot_spring 7 · challenge 6 · nyanta 3`), all 378 carry `name_original` (JP quest-board title = in-game order). Bilingual fields: `location_original`/`objective_original`/`description_original`. **1864 `quest_rewards`.**
- [x] **Skills (101) / decorations (164)** + `armor_skill_points` (4189) / `weapon_skill_points` / `decoration_materials` (523).
- [x] **Charms** 50,635 excavated generator rows (Dragon 10,552 / King 22,439 / Queen 17,644) + `charm_skills` (93,873) with a skill-search browser.
- [x] **Comrades** gear: 92 weapons / 154 armor / 52 skills.
- [x] **Trap tools** 180 rows (`monster_tools`).
- [x] **Item sources** 4099 rows from `mhp3wiki.info`; small-monster `carve/drop/capture` rows (`source_id NULL`) made visible in item detail by SDD 001 fix (`get_item_sources` exclusion now only applies with `source_id NOT NULL`). See `docs/fidelity-report.md` § _MH P3rd — Item Catalog & Acquisition_.

### Data — Monster Hunter World + Iceborne (MHW / `mhw`, DB id 1) — Seeded

- [x] **1359 items** (World+Iceborne incl. event/collab, Chest order via `sort_order` 1-1339 MHWorldData + 2000+ extras, 343 Fandom icons offline) — `Material 1082 / Consumable 129 / Misc 106 / Ammo 41 / Hidden 1`
- [x] **94 monsters** (**23 Small + 71 Large** incl. variants Azure/Seething/Blackveil/Ruiner/Fatalis/Alatreon/Safi, species corrected, MHWorldData descriptions, 94 offline icons). _`monsters.size` backfilled from the committed JSON by `patch_mhw_015_small_size` — 13 pack creatures had drifted to `Large` on pre-015 installs because `INSERT OR IGNORE` never corrects `size`._
- [x] **3544 weapons** (14 types Great Sword→Bow incl. Charge Blade/Insect Glaive, Smith tree `sort_order` DFS, 12-HEX per-rarity icons r1..r12, sharpness/slots/element/status)
- [x] **5826 monster drops** (MHWorldData `monster_rewards.csv` + Fandom extras, `rank` Low/High/Master, `probability` %, `method` carve/break/reward, `part` Horn/Wing) + **6821 `item_sources`** + **10,056 `weapon_craft` / 9,719 `weapon_materials`**
- [x] **1595 armor** (351 sets via `derive_set_name`) / 5887 `armor_materials`, **521 quests** / 3675 `quest_rewards` (ids offset `100101+` to avoid MH2G PK collision), **179 skills** / 419 levels, **186 combines** (Normal-only: MHW crafting + Melder model), **211 Melder recipes**, **20 Mantles/Boosters**, **8 Palico Gadgets** + 38 levels — see `spec/features/009-mhw-tools-melder/`
- [x] **MHW gaps closed** (spec `002b-mhw-gaps` DONE): **404 decorations** (MHWorldData, rarity + skill icons), **788 weakness rows** across 88 monsters (per-part hitzones: cut/impact/shot + elements), **~20k derived equipment links** (19885 rows via material drops), **911 gather rows** (5 maps, area + rank conditions), **179 skills** (+`Kulve Taroth Essence`) with **weapon_skill_points** for 638 special-skill weapons (bare-name fallback, points = max_level)
- [x] **010-mhw-100 follow-up DONE**: **721 ailment rows** / **465 trap rows** / **317 hunter charms** + 1247 materials + 207 previous-links (rank tree I→V browser) / set bonuses **+58 `skill2`** fills (351/351 sets verified) / descriptions 3544+1595+94 / deco acquisition **398/404** / melder sources **212** / shop **56** / Hoarfrost+Guiding gather **75**
- [x] **015-mhw-endemic DONE**: **33 curated endemic creatures** in `endemic_life` (8 bugs incl. Vigorwasp/Flashbug/Thunderbug/Godbug + 23 small-monster packs + 2 special Boaboa Snowman/Gajalaka Trap). `kind` = Slinger-ammo yield for parity with Wilds. `uq_endemic_life_game_id` UNIQUE index added.
- [x] **Sidebar** sticky (`h-screen` + `overflow-y-auto`) for long Weapons/Armors lists; **Weapons** filter no longer shows `All` (default `Great Sword`), **Items/Monsters** Chest order `COALESCE(sort_order,id)`
- [x] Data pipeline scripts: `fetch_mhp3rd_fandom.py` → `fetch_mhp3_wiki_data.py` (Playwright) → `generate_mhp3rd_*` + `reindex_mhp3rd_items.py`; MHW via `generate_mhw_*` + `scrape_mhw_*` + `download_*mhw*.py` (MHWorldData + Fandom).

### Data — Monster Hunter Wilds (`mhwilds`, DB id 3) — Seeded

- [x] **68 monsters** (34 Large via MHDB Wilds API + 34 Small) with descriptions, **324 weakness rows** (per-part multipliers ×100: slash/blunt/pierce + elements) + **1399 drops** (Low/High rank)
- [x] **773 items** (Material with descriptions, rarity/sell/carry) + **120 crafting combines**; **179 skills** / 442 levels; **361 decorations** (slot/rarity/skills)
- [x] **1188 weapons** (14 types, per-type Smith `sort_order`, sharpness/slots/element/status/skills, forge+upgrade materials) + **714 armor** (183 sets, resistances/skills/materials) + derived equipment links (3772 rows) + armor/weapon skill points
- [x] **012-014 follow-up DONE**: **174 quests** (Optional 81 / Event 60 / Assignment 33), **76 Wilds talismans**, **34 small monsters** merged into `monsters` with `size='small'`, **70 endemic creatures** in `endemic_life` (shared UI with `/mhw/endemic`)

### Data — Monster Hunter Rise (`mhr`, DB id 2) — Seeded (base + Sunbreak v16)

- [x] **112 monsters** (78 Large + 34 Small, descriptions) + **624 weakness rows** (per-part hitzones) + **6403 drops** (carve/break/reward/drop/palico/capture, Low/High/Master)
- [x] **1668 items** (`Material 1497 / Consumable 80 / Ammo 65 / Decoration 26`, taxonomy + descriptions), **946 quests** (all hubs incl. Anomaly/Follower) / **6426 rewards**, **147 skills** / 459 levels, **243 decorations** (levels/materials/prices)
- [x] **3953 weapons** (14 types, Smith sort, attack/affinity/element/status/sharpness/rarity/defense/materials/tree) + **1591 armor** (410 sets, defense/res/slots/skill-levels/materials) + derived equipment (21920 rows) + skill points
- [x] **011-mhr-100 follow-up DONE**: **1120 ailment rows** + part-break (`monster_part_break`) + **270 trap rows** + buddy gear (322 weapons / 362 armor) + rampage (247 skills / 26 decos / 6822 weapon links) + **11,849 talismans** + **178 combines** (`item-mix` → `item_combine`, chance 100)
- [x] Documented N/A: Rise has no armor set bonuses (`bonus_skill` 0/410 non-null), no weapon descriptions upstream, no deco slot-size badges, no wirebug widget, dango/switch/endemic reference lists stay inline-only.

### Backend (Rust / Tauri)

- [x] `ass.rs`, `builds.rs`, `favorites.rs`, `import.rs`, `commands/mod.rs`, `db/{mod,schema,queries,seed}.rs` — `items.subcategory`, `item_combine.combine_type/chance` migrations + `backfill_item_categories`; `schema_version` table + `get_schema_version()`; `add_idempotency_constraints()` dedupes + UNIQUE indexes (`uq_item_combine`, `uq_item_sources`, `uq_monster_equipment`, `uq_endemic_life_game_id`, `(game_id, id)` guards) so `INSERT OR IGNORE` is a real upsert; `clear_game` removed.
- [x] `db/mod.rs` — `register_functions()` exposes `norm_key` as a deterministic SQLite scalar; `get_global_search` pushes LIKE filtering into SQLite (parametrize + ESCAPE). `rusqlite` `functions` feature; `tauri-plugin-shell` removed; CSP hardened in `tauri.conf.json` (`tauri.conf.dev.json` overlay for devtools).
- [x] `ass.rs` — `allow_dummy` removed; `danger_skills`/`reorder_gems` left as documented stubs (audit A4); robust tier/hunter_type tests added.
- [x] **43 Tauri commands registered** in `lib.rs`: entities (`get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_armor_set_detail`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, `get_combinations` + `get_*_detail` variants) · tools (`get_mhw_mantles`, `get_mhw_mantle_detail`, `get_palico_gadgets`, `get_palico_gadget_detail`, `get_melder_recipes`) · charms (`get_charms`, `get_hunter_charms`) · buddies (`get_comrade_weapons`, `get_comrade_armor`, `get_comrade_skills`, `get_buddy_weapons`, `get_buddy_armor`) · endemic (`get_endemic_life`) · solver (`search_armor_sets`, `get_monster_dedicated_sets`) · builds (`list_builds`, `save_build`, `get_build`, `delete_build`, `import_build`) · favorites (`list_favorites`, `toggle_favorite`, `remove_favorite`) · import (`preview_import`, `apply_import`) · search (`global_search`). `greet`/`get_games` removed (game registry is frontend `src/lib/stores/game.ts`).

### Build

- [x] `cargo build` — clean
- [x] `cargo test` — **48 tests pass** (ASS rank-gate + robust tiers/hunter types + global_search + idempotency/migration dedup + small-monster visibility + MHW quest offset + MHW 010 phases A/B/C + set-bonus display + MHP3rd charms + MHR 011 phases A/B/C/D/E + `mhp3rd_gear_fix_fk_regression_1931524` + `mhw_015_endemic` + `mhw_015_small_size_backfill`)
- [x] `npx svelte-check` — 0 errors, 0 warnings
- [x] `cargo clippy -D warnings` + `cargo fmt --check` + `prettier --check` + `eslint` — clean
- [x] `node scripts/check-version.js` — version sync OK
- [x] `npm audit --audit-level=moderate` — 0 vulnerabilities

---

## 🎮 Implemented User Flow

```
[App opens] → Game selector (5 cards themed by game)
                 │ click game
                 ▼
         Game home (dashboard)
                 │ global search in header (type ahead)
                 ▼
       List view (monsters / weapons / armor / quests / items / skills / decorations / builds)
                 │
                 ▼
       Detail view (description, stats, materials, drop sources, related entities)
                 │
                 ▼
       Cross-navigation to related entity
```

---

## 📋 Next Steps

### Phase 2 (Multi-game)

- [x] MHP3rd dataset seeded (items, quests, monsters, armor, weapons, combines, drops, skills, decorations) — fidelity pass DONE (spec `001-mhp3rd-fidelity-pass`)
- [x] MHW dataset seeded — core DONE (spec `002-mhw-data-import` + `009-mhw-tools-melder`); gaps DONE (spec `002b-mhw-gaps`); **100% follow-up DONE (spec `010-mhw-100`)** — ailments + tools + charms + set bonuses + set-bonus display; **endemic life DONE (spec `015-mhw-endemic`)**
- [x] MHWilds dataset seeded — core DONE (spec `004-mhwilds-data-import`); quests + charms + small monsters + endemic DONE (specs `012`–`014`, shipped; no spec folders recorded)
- [x] MHR dataset seeded — DONE (spec `003-mhr-data-import`: base bulk + Sunbreak v16 Kiranico); **100% follow-up DONE (spec `011-mhr-100`)** — ailments + part-break + traps + buddy gear + rampage + talismans

### Phase 3 (Builds polish)

- [x] Save/load custom builds (app-data `{app_data_dir}/builds.json`, per-game list) — spec `005-builds-save-load-export`
- [x] Export builds to JSON / share code (base64url) + validated import

### Phase 4 (Advanced)

- [x] Favorites system (app-data `favorites.json`, per-game stars on 7 lists + 8 details with ★ filters) — spec `006-favorites-system`
- [x] Import panel for JSON/CSV (`preview_import` / `apply_import`: 6 kinds, dry-run preview, transactional idempotent apply) — spec `007-import-panel-json-csv`
- [ ] Mobile build via Tauri v2 — spec `008` written, Android init pending
- [x] Wilds quests/charms/small monsters/endemic — specs `012`–`014` done
- [x] MHW endemic life parity — spec `015` done (33 curated rows: 8 bugs + 23 smalls + 2 special)
- [ ] MH2G crown + hunt Basic/Bonus + quest HP — Round 2 (source-blocked)
- [ ] Wilds anomaly investigations / qurious crafting — Round 2 (procedural, bounded scope TBD)

---

## 🛠️ Useful Commands

```bash
# Install dependencies
npm install

# Development (frontend only)
npm run dev

# Development (full Tauri app)
npx tauri dev

# Production build
npx tauri build

# Build Rust backend only
cargo build --manifest-path src-tauri/Cargo.toml

# Run Rust unit tests (ASS solver + db/queries idempotency & global_search)
cargo test --manifest-path src-tauri/Cargo.toml

# Type/Svelte check (aliases: lint, typecheck, check)
npm run check   # or: npm run lint / npm run typecheck
```

---

## 📁 File Structure

```
mh-aio/
├── src/                              # Frontend (Svelte 5)
│   ├── lib/
│   │   ├── api.ts                    # Typed invoke() wrapper (43 Tauri commands)
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives (card, button)
│   │   │   ├── global-search.svelte  # Per-game accent-insensitive search
│   │   │   ├── header.svelte         # Themed top bar + search
│   │   │   ├── sidebar.svelte        # Themed nav (gameOnly per item)
│   │   │   ├── back-button.svelte
│   │   │   ├── detail-header.svelte
│   │   │   ├── favorite-button.svelte
│   │   │   ├── item-icon.svelte
│   │   │   ├── material-list.svelte
│   │   │   └── drop-table.svelte
│   │   ├── prefetch.ts               # Sidebar hover-warm query registry
│   │   ├── stores/game.ts            # Game registry + localStorage guard
│   │   └── utils/
│   │       ├── index.ts              # cn()
│   │       └── norm.ts               # normKey() mirrors Rust norm_key
│   └── routes/[game]/
│       ├── monsters/                 # list + [id] (dedicated sets, ailments, traps)
│       ├── weapons/                  # list + [id]
│       ├── armor/                    # list, sets/[id], [id]  (gender/rank filter)
│       ├── quests/                   # list + [id]
│       ├── items/                    # list + combine/ + [id]
│       ├── skills/                   # list + [id]
│       ├── decorations/              # list + [id]
│       ├── charms/                   # MHP3rd / MHW / MHR / Wilds
│       ├── comrades/                 # MHP3rd + MHR buddy gear
│       ├── tools/                    # MHW mantles/boosters/palico
│       ├── endemic/                  # MHW + Wilds
│       └── builds/                   # Armor Set Search (ASS port, no dummy)
├── docs/
│   ├── data-coverage.md              # GENERATED table x game_id row counts
│   └── fidelity-report.md            # Retail UMD audit + per-game catalog reports
├── spec/                             # constitution/ + features/000-015/
├── LICENSE                           # MIT
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                    # 43 Tauri commands registered (no shell plugin)
│   │   ├── ass.rs                    # Armor Set Search solver (ASS port)
│   │   ├── builds.rs                 # Saved builds (app_data_dir/builds.json)
│   │   ├── favorites.rs              # Favorites (app_data_dir/favorites.json)
│   │   ├── import.rs                 # JSON/CSV import preview + apply
│   │   ├── commands/mod.rs
│   │   └── db/{mod,schema,queries,seed}.rs  # register_functions + idempotency indexes
│   ├── data/                         # mh2g_* / mhp3rd_* / mhw_* / mhr_* / mhwilds_*.json
│   ├── Cargo.toml                    # rusqlite {bundled, functions}
│   ├── capabilities/default.json     # core:default only
│   ├── tauri.conf.json               # CSP hardened
│   └── tauri.conf.dev.json           # devtools overlay
├── scripts/                          # Dev-only scrapers + generators (gitignored)
├── AGENTS.md
├── CONTRIBUTING.md
├── README.md
├── roadmap.md
└── STATUS.md
```
