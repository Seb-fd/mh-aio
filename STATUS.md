# MH-AIO - Project Status

## Current Version: v0.1.0 (MHP2G 1083 items fully sourced + corrected categories + 432 combines · MHP3rd 1044 items / 378 quests / 60 monsters / 972 weapons / 1111 armor — all seeded · MHW+Iceborne 1359 items / 94 monsters / 3544 weapons / 5862 drops — Chest/Smith order + per-rarity icons)

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

- [x] `monsters/` — list + `[id]` (weaknesses, drops, dedicated armor sets) — 83 monsters in game order (Felyne 1 … White Fatalis 83), Large/Small/All selector (Large default, order-preserving)
- [x] `weapons/` — list + `[id]` (tree, stats, materials) — 1500 weapons in Smith order `Great Sword → Bow` (11 types)
- [x] `armor/` — list (`Sets`/`Pieces` toggle + Both/Male/Female + Blademaster/Gunner + rank filters), `sets/[id]`, `[id]`
  - [x] `quests/` — list + `[id]` — 610 quests (95 Village Elder, 35 Nekoto, 89 Guild Low, 77 Guild High, 116 Guild G, 140 Training School, 7 Treasure Hunt, 37 Event, 14 Challenge) in hub order, grouped into collapsible difficulty accordions (★/G★, first expanded)
- [x] `items/` — list (game-order chest sort + name/rarity/price `category • subcategory` + `Charm` + `Ammo/Husk` ISO taxonomy) — 91 Consumable / 913 Material / 79 Ammo (67 fixes: `Power Juice` etc.), `combine` single view (Normal/Alchemy/Treasure badge/filter + `success %` + Book order), `[id]` with clickable combine `A x1 + B x1 = Result x1 • 90%`
- [x] `skills/` — list + `[id]` (levels, decorations, related armor/weapons)
- [x] `decorations/` — list + `[id]`
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

- [x] 2075 armor pieces, **1083 items fully sourced** (12,751 `item_sources` rows: `gather/mining/bug/fish` from `maps.json`, `shop` 5 merchants consolidated, `trade` Veggie Elder + Trenya Boat + Pokke Points, `farm` Pokke Farm spots/trees, `small monsters` via `Monsters/monsters-material.json`, plus `monster_drops`/`quest_rewards`; verified vs `MHP2G` ISO `DATA.BIN` offsets), **432 combine recipes** (147 Normal + 18 Alchemy + 7 Treasure, with `combine_type`/`chance` and Book order), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests (Training 140 / Treasure 7 / Event 37 / Challenge 14), 99 skill families (214 abilities), 192 decorations
- [x] Item taxonomy re-derived from ISO `tmp_mhfu_upstream/items.json` `icon` + verb: `Consumable 91 / Material 913 / Ammo 79` with `subcategory` (`Recovery, Buff, Food, Charm, Husk, Coating, Ore, Monster Material` etc.; `Powercharm/Powertalon` → `Consumable • Charm`, `Huskberry` → `Ammo • Husk`), 67 fixes (Power Juice, Mega Juice, Cold Meat, Gourmet Fish, Deodorant, 27 Bowgun S)
- [x] Monster order faithful to UMD Hunter's Notes (Felyne 1 … White Fatalis 83, `ORDER BY id`); Large/Small selector preserves order
- [x] Weapon order faithful to Smith tree (`Great Sword → Bow`, `ORDER BY CASE weapon_type`, `src-tauri/src/db/queries.rs:698`)
- [x] Quest hubs: `elder → nekoto → guild_low → guild_high → guild_g → training → treasure → event` (`queries.rs:1157`, `quests/+page.svelte:31`); Other split into Training School and Treasure Hunt; Event quests (33 downloadable, not in ISO) validated against distribution file + wiki
- [x] Quest validation vs ISO `DATA.BIN` string table (`Mountain Herb Picking` at `0x13E…`, `Training`/`Treasure` labels); total 559 base quests match retail
- [x] Fixed 138 item sell prices vs game-extracted DB (Thawing Agent 100→10 etc.)
- [x] Item wings/ammo classified per ISO `icon` (Bowgun S + Coatings + Husks → `Ammo`, with `subcategory`); header artifact removed
  - [x] `docs/fidelity-report.md` — single merged report: defense/rarity/slots 100%, weapons per-type 100%, monster catalog 83, quest hubs 610 (ISO string table + guild rank fix + kit Event/Challenge) vs retail UMD + game-extracted DB + item taxonomy/combine Book order

### Data — Monster Hunter Portable 3rd (MHP3rd / `mhp3rd`, DB id 4) — Seeded

- [x] **1044 items** (`Material 943 / Consumable 55 / Ammo 46`), **291 descriptions** (17 EN + 274 JP flagged with 🇯🇵 badge), **150 buy prices**; chest-order `id` remapped (0 dangling refs). **263 combines** (202 Normal in `調合リスト` book order + 61 Alchemy) with `chance`.
- [x] **60 monsters**, **1679 drops** (carve 556 / capture 517 / break 329 / drop 277 with rank/part/probability) across all 40 droptable monsters; **monster weaknesses / equipment** seeded.
- [x] **972 weapons**, **1111 armor pieces** (sets derived via `derive_set_name`), **weapon/armor materials + craft** resolved.
- [x] **378 quests** (`village 96 · guild_low 88 · guild_high 100 · event 52 · hot_spring 7 · drink 16 · nyanta 3 · training 10 · challenge 6`), all 378 carry `name_original` (JP quest-board title = in-game order). Bilingual fields: `location_original`/`objective_original`/`description_original`
- [x] **Skills / decorations** + `armor_skill_points` / `weapon_skill_points` / `decoration_materials`.
- [x] **Item sources** 2016 rows (`shop` 102 / `trade` 528 / `farm` 120 / `gather` 110 / `mining` 100 / `bug` 36 / `fish` 30 / `carve` 374 / `drop` 515 / `capture` 98 small-monster / `other` 3) from `mhp3wiki.info`; small-monster `carve/drop/capture` rows (`source_id NULL`) made visible in item detail by SDD 001 fix (`get_item_sources` exclusion now only applies with `source_id NOT NULL`). See `docs/fidelity-report.md` § _MH P3rd — Item Catalog & Acquisition_.

### Data — Monster Hunter World + Iceborne (MHW / `mhw`, DB id 1) — Seeded

- [x] **1359 items** (World+Iceborne incl. event/collab, Chest order via `sort_order` 1-1339 MHWorldData + 2000+ extras, 343 Fandom icons offline)
- [x] **94 monsters** (Small 23 + Large 71 incl. variants Azure/Seething/Blackveil/Ruiner/Fatalis/Alatreon/Safi, species corrected, MHWorldData descriptions, 94 offline icons)
- [x] **3544 weapons** (14 types Great Sword→Bow incl. Charge Blade/Insect Glaive, Smith tree `sort_order` DFS, 12-HEX per-rarity icons r1..r12, sharpness/slots/element/status)
- [x] **5862 monster drops** (MHWorldData `monster_rewards.csv` 5680 + 182 Fandom extras, `rank` Low/High/Master, `probability` %, `method` carve/break/reward, `part` Horn/Wing) + `item_sources` 107 (+ rows derived from drops) + `weapon_craft` 10056 / `weapon_materials` 9719
- [x] **1595 armor** (351 sets via `derive_set_name`) / 5887 `armor_materials`, **521 quests** / 4176 `quest_rewards` (ids offset `100101+` to avoid MH2G PK collision), **178 skills** / 418 levels, **188 combines** (Normal-only: MHW crafting + Melder model), **211 Melder recipes**, **20 Mantles/Boosters**, **8 Palico Gadgets** + 38 levels — see `spec/features/009-mhw-tools-melder/`
- [x] **MHW gaps closed** (spec `002b-mhw-gaps` DONE): **404 decorations** (MHWorldData, rarity + skill icons), **788 weakness rows** across 88 monsters (per-part hitzones: cut/impact/shot + elements), **~20k derived equipment links** (6461 armor + 13923 weapon via material drops), **911 gather rows** (5 maps, area + rank conditions), **179 skills** (+`Kulve Taroth Essence`) with **weapon_skill_points** for 638 special-skill weapons (bare-name fallback, points = max_level)
- [x] **Sidebar** sticky (`h-screen` + `overflow-y-auto`) for long Weapons/Armors lists; **Weapons** filter no longer shows `All` (default `Great Sword`), **Items/Monsters** Chest order `COALESCE(sort_order,id)`
- [x] Data pipeline scripts: `fetch_mhp3rd_fandom.py` → `fetch_mhp3_wiki_data.py` (Playwright) → `generate_mhp3rd_*` + `reindex_mhp3rd_items.py`; MHW via `generate_mhw_*` + `scrape_mhw_*` + `download_*mhw*.py` (MHWorldData + Fandom).

### Data — Monster Hunter Wilds (`mhwilds`, DB id 3) — Seeded (core)

- [x] **34 Large monsters** (MHDB Wilds API, descriptions) + **340 weakness rows** (per-part multipliers ×100: slash/blunt/pierce + elements) + **1775 drops** (carve 583 / break 431 / reward 761, Low 477 / High 1298)
- [x] **773 items** (Material with descriptions, rarity/sell/carry) + **121 crafting combines**; **179 skills** / 442 levels; **361 decorations** (slot/rarity/skills)
- [x] **1188 weapons** (14 types, per-type Smith `sort_order`, sharpness/slots/element/status/skills, forge+upgrade materials) + **714 armor** (183 sets, resistances/skills/materials) + derived equipment links (~1273 armor + ~2499 weapon) + armor/weapon skill points
- [x] Gaps (no upstream data): quests (list empty), charms (no table), small monsters — see spec `004-mhwilds-data-import`

### Data — Monster Hunter Rise (`mhr`, DB id 2) — Seeded (base + Sunbreak v16)

- [x] **112 monsters** (78 Large + 34 Small, descriptions) + **629 weakness rows** (per-part hitzones) + **7092 drops** (carve/break/reward/drop/palico/capture, Low/High/Master)
- [x] **1642 items** (taxonomy + descriptions + 74 gather rows), **946 quests** (all hubs incl. Anomaly/Follower, 7508 rewards), **147 skills** / 459 levels, **243 decorations** (levels/materials/prices)
- [x] **3953 weapons** (14 types, Smith sort, attack/affinity/element/status/sharpness/rarity/defense/materials/tree) + **1591 armor** (410 sets, defense/res/slots/skill-levels/materials) + derived equipment + skill points
- [x] Gaps (documented): talismans (no table), deco slot sizes on weapons (badges carry no numbers), weapon descriptions — see spec `003-mhr-data-import`

### Backend (Rust / Tauri)

- [x] `ass.rs`, `commands/mod.rs`, `db/{mod,schema,queries,seed}.rs` — `items.subcategory`, `item_combine.combine_type/chance` migrations + `backfill_item_categories`; `schema_version` table + `get_schema_version()`; `add_idempotency_constraints()` dedupes + UNIQUE indexes (`uq_item_combine`, `uq_item_sources`, `uq_monster_equipment`, `(game_id, id)` guards) so `INSERT OR IGNORE` is a real upsert; `clear_game` removed.
- [x] `db/mod.rs` — `register_functions()` exposes `norm_key` as a deterministic SQLite scalar; `get_global_search` now pushes LIKE filtering into SQLite (parametrize + ESCAPE). `rusqlite` `functions` feature; `tauri-plugin-shell` removed; CSP hardened in `tauri.conf.json` (`tauri.conf.dev.json` overlay for devtools).
- [x] `ass.rs` — `allow_dummy` removed; `danger_skills`/`reorder_gems` left as documented stubs (audit A4); robust tier/hunter_type tests added.
- [x] Tauri commands: `get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_armor_set_detail`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, `get_combinations` (Book order), list + detail variants (`get_item_detail` with `subcategory` + `recipes` with `combine_type`/`chance`), `search_armor_sets`, `global_search`. `greet`/`get_games` removed (game registry is frontend `src/lib/stores/game.ts`).

### Build

- [x] `cargo build` — clean
- [x] `cargo test` — 13 tests pass (ASS rank-gate + robust tiers/hunter types + global_search + idempotency/migration dedup + small-monster visibility + MHW quest offset)
- [x] `npx svelte-check` — 0 errors, 0 warnings

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
- [x] MHW dataset seeded — core DONE (spec `002-mhw-data-import` + `009-mhw-tools-melder`); gaps DONE (spec `002b-mhw-gaps`)
- [x] MHWilds dataset seeded — core DONE (spec `004-mhwilds-data-import`; quests/charms deferred)
- [x] MHR dataset seeded — DONE (spec `003-mhr-data-import`: base bulk + Sunbreak v16 Kiranico; talismans deferred)

### Phase 3 (Builds polish)

- [x] Save/load custom builds (app-data `{app_data_dir}/builds.json`, per-game list) — spec `005-builds-save-load-export`
- [x] Export builds to JSON / share code (base64url) + validated import

### Phase 4 (Advanced)

- [x] Favorites system (app-data `favorites.json`, per-game stars on 7 lists + 8 details with ★ filters) — spec `006-favorites-system`
- [x] Import panel for JSON/CSV (`[game]/import`: 6 kinds, dry-run preview, transactional idempotent apply) — spec `007-import-panel-json-csv`
- [ ] Mobile build via Tauri v2 (already supported via `cdylib`)

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
│   │   ├── api.ts                    # Typed invoke() wrapper (get_games removed)
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives (card, button)
│   │   │   ├── global-search.svelte  # Per-game accent-insensitive search
│   │   │   ├── header.svelte         # Themed top bar + search
│   │   │   ├── sidebar.svelte
│   │   │   ├── back-button.svelte
│   │   │   ├── detail-header.svelte
│   │   │   ├── material-list.svelte
│   │   │   └── drop-table.svelte
│   │   ├── stores/game.ts            # Game registry + localStorage guard
│   │   └── utils/
│   │       ├── index.ts              # cn()
│   │       └── norm.ts               # normKey() mirrors Rust norm_key
│   └── routes/[game]/
│       ├── monsters/                 # list + [id] (dedicated sets)
│       ├── weapons/                  # list + [id]
│       ├── armor/                    # list, sets/[id], [id]  (gender/rank filter)
│       ├── quests/                   # list + [id]
│       ├── items/                    # list + [id] (CJK badge) + combine/
│       ├── skills/                   # list + [id]
│       ├── decorations/              # list + [id]
│       └── builds/                   # Armor Set Search (ASS port, no dummy)
├── docs/fidelity-report.md
├── LICENSE                           # MIT
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                    # Tauri commands registered (no shell plugin)
│   │   ├── ass.rs                    # Armor Set Search solver (ASS port)
│   │   ├── commands/mod.rs
│   │   └── db/{mod,schema,queries,seed}.rs  # register_functions + idempotency indexes
│   ├── data/                         # mh2g_*.json + mhp3rd_*.json
│   ├── Cargo.toml                    # rusqlite {bundled, functions}
│   ├── capabilities/default.json     # core:default only
│   ├── tauri.conf.json               # CSP hardened
│   └── tauri.conf.dev.json           # devtools overlay
├── AGENTS.md
├── README.md
├── roadmap.md
└── STATUS.md
```
