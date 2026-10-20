# MH-AIO: Monster Hunter All-in-One Encyclopedia

## Project Vision

A comprehensive, offline-first encyclopedia and toolkit for all Monster Hunter games, covering multiple titles with detailed data on weapons, armor, monsters, quests, skills, items, builds (incl. an armor set solver ported from Athena's A.S.S.), and suggestions.

Current state: **all five titles are fully seeded.** MH2G (Freedom Unite), MHP3rd (Portable 3rd), MHW + Iceborne, MH Rise + Sunbreak and MH Wilds each have complete entity browsers, detail views, game-faithful ordering (monsters Hunter's Notes order, weapons Smith order, items Chest order, quest hubs), filtered browsers (Large/Small, Blademaster/Gunner, rank), the armor set solver, charms/comrades/tools/endemic section views, per-game theming, and per-game global search. MH2G additionally has a full retail-UMD fidelity audit; MHW and MHR each have a dedicated 100%-completeness pass.

**Authoritative row counts: [`docs/data-coverage.md`](docs/data-coverage.md)** (generated per-table × per-game matrix; regenerate with `py scripts/gen_data_coverage.py`).

---

## Tech Stack

### Frontend

- **Framework:** Tauri v2 (Rust + WebView)
- **UI:** Svelte 5 + TypeScript
- **Styling:** Tailwind CSS v4 (`@theme` block, no config file)
- **Components:** shadcn-svelte (plain Svelte 5 primitives — `bits-ui` removed)
- **Build:** Vite (with `server.watch.ignored: ['src-tauri/**']`)
- **State:** Svelte Stores / Runes (`$state`, `$derived`, `$effect`)
- **Routing:** SvelteKit client-side (SSR disabled, adapter-static with `fallback: 'index.html'`)

### Backend (Rust)

- **Framework:** Tauri v2 Commands (IPC) — `tauri-plugin-shell` removed, CSP hardened
- **Database:** SQLite via `rusqlite` (`bundled` + `functions` feature, WAL mode, `PRAGMA foreign_keys=ON`, `norm_key` SQL scalar via `register_functions`)
- **Serialization:** Serde + JSON
- **Migrations:** Hand-rolled (`ALTER TABLE ... ADD COLUMN` with `pragma_table_info` check) + `schema_version` table + `add_idempotency_constraints()` dedup/UNIQUE indexes; `INSERT OR IGNORE` idempotent seed (no `clear_game`)

### Future Platforms

- **Desktop:** Windows / macOS / Linux (Tauri v2) — current
- **Mobile:** iOS / Android (Tauri v2 mobile) — planned (lib already supports `cdylib` + `staticlib`)
- **Web:** PWA — not planned (Tauri-focused)

---

## Project Structure

```
mh-aio/
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                # 43 Tauri commands registered (no shell, no greet/get_games)
│   │   ├── ass.rs                # Armor Set Search solver (ASS port)
│   │   ├── builds.rs             # Saved builds (app_data_dir/builds.json)
│   │   ├── favorites.rs          # Favorites (app_data_dir/favorites.json)
│   │   ├── import.rs             # JSON/CSV import preview + transactional apply
│   │   ├── commands/
│   │   │   └── mod.rs            # Command handlers
│   │   └── db/
│   │       ├── mod.rs            # Database (Mutex<Connection> + register_functions(norm_key))
│   │       ├── schema.rs         # Tables + ALTER TABLE migrations + schema_version + idempotency UNIQUE indexes
│   │       ├── queries.rs        # List/detail/search (CombineView, game order, SQLite LIKE via norm_key)
│   │       └── seed.rs           # Idempotent 5-game seed + 26 named data_patches (no clear_game, dedup)
│   ├── data/                     # mh2g_* / mhp3rd_* / mhw_* / mhr_* / mhwilds_*.json
│   ├── Cargo.toml                # crate-type = ["lib", "cdylib", "staticlib"]; rusqlite {bundled, functions}
│   ├── capabilities/default.json # core:default only
│   ├── tauri.conf.json           # CSP hardened
│   └── tauri.conf.dev.json       # devtools overlay
├── src/                          # Frontend Svelte 5
│   ├── app.html
│   ├── app.css                   # Tailwind + themed-bg per game ornament
│   ├── lib/
│   │   ├── api.ts                # Typed invoke() wrapper (43 commands, no Game/greet)
│   │   ├── components/
│   │   │   ├── ui/               # shadcn-svelte primitives (card, button — plain Svelte 5)
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte    # Themed nav (per-item `gameOnly`)
│   │   │   ├── header.svelte     # Themed top bar
│   │   │   ├── global-search.svelte
│   │   │   ├── back-button.svelte
│   │   │   ├── detail-header.svelte
│   │   │   ├── favorite-button.svelte
│   │   │   ├── item-icon.svelte
│   │   │   ├── material-list.svelte
│   │   │   └── drop-table.svelte
│   │   ├── prefetch.ts           # Sidebar hover-warm query registry
│   │   ├── stores/
│   │   │   └── game.ts           # 5 games + GameTheme + localStorage guard (parseStoredGame)
│   │   └── utils/
│   │       ├── index.ts          # cn()
│   │       └── norm.ts           # normKey() mirrors Rust norm_key
│   └── routes/
│       ├── +layout.ts            # ssr=false, prerender=false
│       ├── +layout.svelte        # theme injection (no path-sync effect)
│       ├── +page.svelte          # Landing = Game Selector
│       └── [game]/
│           ├── +layout.ts
│           ├── +page.svelte      # Dashboard
│           ├── monsters/         # list + [id]
│           ├── weapons/          # list + [id]
│           ├── armor/            # list, sets/[id], [id]
│           ├── quests/           # list + [id]
│           ├── items/            # list + combine + [id] (JP badge, normKey search)
│           ├── skills/           # list + [id]
│           ├── decorations/      # list + [id]
│           ├── charms/           # MHP3rd charms / MHW hunter charms / MHR talismans / Wilds talismans
│           ├── comrades/         # MHP3rd + MHR buddy gear
│           ├── tools/            # MHW mantles/boosters/palico gadgets
│           ├── endemic/          # MHW + MH Wilds endemic creatures
│           └── builds/           # Armor Set Search (ASS port, no dummy)
├── docs/
│   ├── data-coverage.md          # GENERATED per-table x game_id row counts (source of truth)
│   └── fidelity-report.md        # MH2G audit + MHP3rd/MHW/MHR catalog reports
├── spec/                         # constitution/ + features/000-015/
├── scripts/                      # Dev-only scrapers + generators (gitignored)
├── LICENSE                       # MIT
├── AGENTS.md
├── CONTRIBUTING.md
├── README.md
├── roadmap.md
└── STATUS.md
```

---

## Database Schema (current tables)

```sql
-- Games
CREATE TABLE games (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    abbreviation TEXT NOT NULL,
    release_year INTEGER,
    platform TEXT
);

-- Weapons
CREATE TABLE weapons (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    weapon_type TEXT NOT NULL,
    rarity INTEGER,
    attack INTEGER,
    affinity INTEGER,
    element_type TEXT,
    element_value INTEGER,
    sharpness TEXT,   -- JSON
    slots TEXT,       -- JSON
    skills TEXT,      -- JSON
    crafting_cost INTEGER,
    upgrade_path TEXT,-- JSON
    description TEXT,
    language TEXT DEFAULT 'en'
);

-- Armor
CREATE TABLE armor (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    slot_type TEXT NOT NULL,
    rank TEXT NOT NULL,
    rarity INTEGER,
    defense_base INTEGER,
    defense_max INTEGER,
    resistance_fire INTEGER,
    resistance_water INTEGER,
    resistance_thunder INTEGER,
    resistance_ice INTEGER,
    resistance_dragon INTEGER,
    slots TEXT,       -- JSON
    skills TEXT,      -- JSON
    set_id INTEGER,
    crafting_cost INTEGER,
    materials TEXT,   -- JSON
    description TEXT,
    language TEXT DEFAULT 'en'
);

-- Monsters
CREATE TABLE monsters (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    species TEXT,
    size TEXT,
    breakable_parts TEXT, -- JSON
    ailments TEXT,        -- JSON
    description TEXT,
    language TEXT DEFAULT 'en'
);

CREATE TABLE monster_weaknesses (
    id INTEGER PRIMARY KEY,
    monster_id INTEGER REFERENCES monsters(id),
    part_name TEXT NOT NULL,
    sever INTEGER,
    blunt INTEGER,
    projectile INTEGER,
    fire INTEGER,
    water INTEGER,
    thunder INTEGER,
    ice INTEGER,
    dragon INTEGER
);

-- Quests
CREATE TABLE quests (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    type TEXT,
    rank TEXT,
    objective TEXT,
    location TEXT,
    time_limit INTEGER,
    faints_allowed INTEGER,
    player_limit INTEGER,
    is_key_quest BOOLEAN DEFAULT FALSE,
    unlocks TEXT,
    description TEXT,
    language TEXT DEFAULT 'en'
);

CREATE TABLE quest_rewards (
    id INTEGER PRIMARY KEY,
    quest_id INTEGER REFERENCES quests(id),
    item_id INTEGER REFERENCES items(id),
    quantity INTEGER,
    probability REAL,
    condition TEXT
);

-- Items (category + subcategory per ISO item-taxonomy)
CREATE TABLE items (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    category TEXT,        -- Consumable / Material / Ammo (ISO-derived)
    subcategory TEXT,     -- Recovery / Buff / Food / Charm / Husk / Coating / Ore / Monster Material ...
    rarity INTEGER,
    sell_price INTEGER,
    buy_price INTEGER,
    description TEXT,
    language TEXT DEFAULT 'en'
);

CREATE TABLE item_sources (
    id INTEGER PRIMARY KEY,
    item_id INTEGER REFERENCES items(id),
    source_type TEXT,   -- gather / mining / bug / fish / shop / trade / farm / carve / capture / drop / break / quest_reward
    source_id INTEGER,
    quantity_min INTEGER,
    quantity_max INTEGER,
    probability REAL,
    location TEXT,
    conditions TEXT
);

-- Item combination recipes (Book order; combine_type = normal/alchemy/treasure)
CREATE TABLE item_combine (
    id INTEGER PRIMARY KEY,
    result_item_id INTEGER REFERENCES items(id),
    component_item_id INTEGER REFERENCES items(id),
    quantity INTEGER NOT NULL,
    result_quantity INTEGER NOT NULL DEFAULT 1,
    combine_type TEXT DEFAULT 'normal',
    chance INTEGER
);

-- Crafting materials junction tables (weapon_materials, weapon_craft, armor_materials, decoration_materials, monster_equipment)

-- Skills
CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    description TEXT,
    max_level INTEGER,
    effects TEXT,        -- JSON
    language TEXT DEFAULT 'en'
);

CREATE TABLE decorations (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    skill_id INTEGER REFERENCES skills(id),
    skill_level INTEGER,
    slot_size INTEGER,
    rarity INTEGER,
    language TEXT DEFAULT 'en'
);

-- Junction tables (v0.2.0)
CREATE TABLE weapon_materials (
    id INTEGER PRIMARY KEY,
    weapon_id INTEGER REFERENCES weapons(id),
    item_id INTEGER REFERENCES items(id),
    quantity INTEGER NOT NULL
);

CREATE TABLE armor_materials (
    id INTEGER PRIMARY KEY,
    armor_id INTEGER REFERENCES armor(id),
    item_id INTEGER REFERENCES items(id),
    quantity INTEGER NOT NULL
);
```

---

## Priority Games

> All counts below are sourced from [`docs/data-coverage.md`](docs/data-coverage.md) (generated). Regenerate with `py scripts/gen_data_coverage.py`.

### At 100% (content complete + verified)

1. **Monster Hunter 2ndG / Freedom Unite** (2008) — `mh2g`, DB id 5 — spec `000-mh2g-baseline`
   - 1244 items (1083 fully sourced + 161 upstream books/ores/fish/meats/insects/seeds/herbs/mushrooms/eggs), **15,671 `item_sources` rows**, 432 combine recipes (379 Normal + 39 Alchemy + 14 Treasure), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 2075 armor (348 sets), 610 quests, 99 skill families (214 abilities), 192 decorations, 3353 drops
   - Item taxonomy ISO-derived (`Consumable 128 / Material 1037 / Ammo 79` + `subcategory` Charm/Husk/Coating); ordering faithful to UMD (Hunter's Notes, Smith trees, quest hubs, Book of Combos)
   - Data verified against retail UMD and event distribution file (see `docs/fidelity-report.md`)
2. **Monster Hunter Portable 3rd** (2010) — `mhp3rd`, DB id 4 — spec `001-mhp3rd-fidelity-pass`
   - 1065 items (Material 964 / Consumable 55 / Ammo 46; 291 descriptions — 17 EN + 274 JP), 378 quests (all carrying JP `name_original`), 60 monsters (40 Large + 20 Small), 972 weapons, 1106 armor (345 sets), 398 combines (276 Normal + 122 Alchemy), 2200 drops, 4099 item_sources, 1864 quest rewards, 50,635 charms, Comrades 92/154/52, 180 trap tools
   - Sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright; SDD 001 fixed small-monster visibility (`get_item_sources` `source_id IS NOT NULL` guard)
3. **Monster Hunter World: Iceborne** (2018/2019) — `mhw`, DB id 1 — specs `002` + `002b` + `009` + **`010-mhw-100`** + **`015-mhw-endemic`**
   - 1359 items, 94 monsters (23 Small + 71 Large), 3544 weapons, 1595 armor (351 sets), 521 quests / 3675 rewards, 179 skills, 404 decorations, 5826 drops, 788 weakness rows, 721 ailment rows, 465 trap rows, 186 combines, 211 Melder recipes, 20 Mantles/Boosters, 8 Palico Gadgets + 38 levels, 317 hunter charms, 33 endemic creatures. Sources: MHWorldData + Fandom
4. **Monster Hunter Rise: Sunbreak** (2023) — `mhr`, DB id 2 — specs `003` + **`011-mhr-100`**
   - 112 monsters (78 Large + 34 Small), 1668 items, 3953 weapons, 1591 armor (410 sets), 946 quests / 6426 rewards, 147 skills, 243 decorations, 6403 drops, 178 combines, 11,849 talismans, buddy gear 322/362, rampage 247 skills / 6822 links, 1120 ailment rows + part-break + 270 trap rows. Sources: Badge87 + CrimsonNynja bulk + Kiranico v16 scrape

### Seeded beyond core

5. **Monster Hunter Wilds** (2025) — `mhwilds`, DB id 3 — spec `004` + `012`–`014` (shipped)
   - 68 monsters (34 Large + 34 Small), 773 items, 1188 weapons, 714 armor (183 sets), 179 skills, 361 decorations, 1399 drops, 120 combines, 174 quests, 76 talismans, 70 endemic creatures, 324 weakness rows. Sources: MHDB Wilds API + Fextra
   - Documented gap: no `quest_rewards` (upstream exposes reward text, not per-item counts/probabilities)

### Planned

6. _(none — all five games seeded; see Phase 6 for distribution)_

---

## Features by Phase

### ✅ Phase 1: MVP Core — DONE

- [x] Tauri v2 + Svelte 5 + shadcn-svelte stack
- [x] SQLite with migrations and idempotent seed
- [x] Tailwind CSS v4 with themed-bg utilities
- [x] Base component structure
- [x] SQL schema (tables + ALTER migrations)
- [x] Rust models (serde)
- [x] List + detail queries
- [x] Full MHP2G seed data
- [x] Game selector UI + per-game dashboard
- [x] Per-game theming with ornaments
- [x] Detail views for all entity types
- [x] Back button + cross-navigation
- [x] Build verification (cargo, vite, svelte-check)

### ✅ Phase 2: Data Expansion (MH2G) — DONE

- [x] Full MHP2G monster/weapon/armor/quest/item/skill/decor set
- [x] monster_weaknesses + monster_drops + quest_rewards + item_combine
- [x] **Items fully sourced** (**15,671 `item_sources` rows**: every gathering node/mining/bug/fish from `maps.json`, 5 merchants consolidated, Veggie Elder + Trenya Boat trades, Pokke Farm spots/trees, small-monster drops) — 1083/1083 covered
- [x] **Combine recipes** 432 (379 Normal + 39 Alchemy + 14 Treasure) with `chance` + Book order (`get_combinations`, `/items/combine`, clickable `A x1 + B x1 = Result x1 • 90%`)
- [x] **Item taxonomy ISO-derived** (`Consumable 128 / Material 1037 / Ammo 79`, `subcategory` Recovery/Buff/Food/**Charm**/Husk/Coating/Ore/Monster Material)
- [x] Weapon upgrade paths / evolution trees
- [x] Armor sets (grouped via `derive_set_name`) + set detail route
- [x] Data fidelity audit vs retail UMD (`docs/fidelity-report.md`)
- [x] Gender-locked armor (male/female column)
- [x] Monster → dedicated armor sets (≥40% material score) + "Uses 1 Material"

### ✅ Phase 3: Build System — Armor Set Search (DONE)

- [x] ASS solver port (`src-tauri/src/ass.rs`) — equivalences, jewel solver, Torso Inc, bad-skill fix, 1000 limit, sort
- [x] Skill picker UI (up to 5 skills) with ability selectors
- [x] Optimal set calculator (HR/Elder rank gate, gender, weapon slots, piercings)
- [x] English, guided UX with quick "Try:" examples
- [x] Save/load custom builds (app-data `builds.json`, per-game) — spec `005`
- [x] Export builds to JSON / share code + validated import — spec `005`

### ✅ Phase 4 (partial): Global Search (DONE)

- [x] Global per-game search across all entities (accent-insensitive, debounced)
- [x] Favorites system (app-data, per-game stars + ★ list filters) — spec `006-favorites-system`
- [x] Import panel for JSON/CSV (`preview_import` / `apply_import` Tauri commands: 6 entity kinds, dry-run preview, transactional idempotent apply) — spec `007-import-panel-json-csv`
- [ ] Offline mode verification (future)
- [ ] Auto-update mechanism (future)

### ✅ Phase 5: Multi-Game — ALL FIVE TITLES DONE

- [x] MHP3rd data import + fidelity pass — 1065 items, 378 quests, 60 monsters, 972 weapons, 1106 armor, 398 combines, 2200 drops, 4099 item_sources, skills/decorations, **charms (50,635)** + comrade gear (92/154/52) + trap tools (180), weapon_skill_points — spec `001-mhp3rd-fidelity-pass` DONE
- [x] MHW data import (core) — 1359 items, 94 monsters, 3544 weapons, 1595 armor, 521 quests, 179 skills, 5826 drops, 211 Melder, 20 Mantles/Boosters, 8 Palico Gadgets via MHWorldData + Fandom pipeline (`scripts/generate_mhw_*`) — specs `002-mhw-data-import` + `009-mhw-tools-melder` DONE
- [x] MHW gaps DONE — 404 decorations, 788 weakness rows (88 monsters), ~20k derived equipment links, 911 gather rows, 179 skills + weapon_skill_points — spec `002b-mhw-gaps`
- [x] **MHW 100% follow-up** — ailments (721) + traps (465) + set-bonus display (everywhere) + charms (317) + set-bonus stats on armor — spec `010-mhw-100` DONE
- [x] **MHW endemic life** — 33 curated rows (8 bugs + 23 small-monster packs + 2 special), Slinger-ammo `kind` taxonomy, shared UI with Wilds — spec `015-mhw-endemic` DONE
- [x] MHWilds data import (core) — 68 monsters (34 Large + 34 Small), 773 items, 1188 weapons, 714 armor, 179 skills, 361 decorations, 1399 drops, 120 combines via MHDB Wilds API — spec `004-mhwilds-data-import` DONE
- [x] **Wilds quests + charms + small monsters + endemic** — 174 quests, 76 talismans, 34 small monsters, 70 endemic creatures — specs `012`–`014` DONE (shipped; no spec folders recorded)
- [x] **MHR 100%** (Sunbreak v16 Kiranico + 3geek14/mhr-charms datamine) — 112 monsters + status/part-break/traps, 1668 items, 178 combines, 3953 weapons, 1591 armor, 946 quests, 147 skills, 243 decos, **buddy gear (322 Palico + 362 Palamute)**, **rampage (247 skills + 6822 weapon links + 26 decos)**, **talismans (11,849 generator rows)** + char browser — spec `011-mhr-100` DONE
- [ ] Game-specific UI adaptations (Focus Mode, Wirebugs, etc. — own spec each if scoped, see `009` pattern)

### 📋 Phase 6: Mobile & Distribution

- [x] Spec written — `008-mobile-distribution` (acceptance criteria pending Android init; `src-tauri/gen/` currently contains only `schemas/`)
- [ ] Android init via `npx tauri android init` + build smoke (Round 2)
- [ ] Auto-updater
- [ ] System tray
- [ ] Cross-platform packaging (Windows / macOS / Linux / iOS / Android)

### ✅ Phase 7: Round 1 complete

Wilds completeness (matching the MHR 100% and MHW 100% spec shape) + MHW endemic parity:

- [x] **012 — Wilds quests** (Fextra Quests/Optional+Event; 174 rows: Optional 81 / Event 60 / Assignment 33) — Round 1 DONE
- [x] **013 — Wilds charms** (76 `wilds_talisman` rows + talisman viewer enabled for `mhwilds`) — Round 1 DONE
- [x] **014 — Wilds small monsters + endemic life** (34 small monsters merged into `monsters` with `size='small'` + 70 `endemic_life` rows) — Round 1 DONE
- [x] **015 — MHW endemic life** (parity with Wilds; curated 33 rows: 8 bugs + 23 small-monster packs + 2 special — Slinger-ammo `kind` taxonomy) — Round 1 DONE

Remaining gaps (Round 2 / source-blocked):

- [ ] **MH2G hunt Basic/Bonus %, crown sizes, quest HP** (GameFAQs 59556 bot-blocked; alt sources being probed) — Round 2
- [ ] **Wilds anomaly investigations / qurious crafting** (spec it, mark anomaly N/A — procedural, unbounded) — Round 2
- [ ] **Mobile distribution (Android init)** — Round 2 (see Phase 6)

---

## Theming

Each game has a `GameTheme` object with CSS custom properties applied at the layout level. Themes share a consistent structure but differ in palette, ornament, and accent. Five themes shipped:

| Game    | Primary          | Accent           | Ornament   |
| ------- | ---------------- | ---------------- | ---------- |
| MHW     | `#3b82f6` blue   | `#fbbf24` gold   | tribal     |
| MHR     | `#f97316` orange | `#fde047` yellow | japanese   |
| MHWilds | `#22c55e` green  | `#facc15` gold   | futuristic |
| MHP3rd  | `#a855f7` purple | `#fbbf24` gold   | japanese   |
| MH2G    | `#b91c1c` red    | `#d4a017` gold   | medieval   |

Ornaments are CSS `repeating-linear-gradient` patterns defined in `src/app.css`, scoped via `[data-ornament="..."]` attribute on the root wrapper.

---

## Internationalization

### Supported Languages

- **English** (primary)

### Strategy

- Each record has `language` field (default 'en')
- All seed data is in English
- Future: JP names alongside EN

---

## Key Technical Decisions

### Why Tauri + Svelte?

- **Tauri v2:** Small binaries (5-15MB vs 100MB+ Electron), mobile support, security
- **Svelte 5:** Compiled, no runtime overhead, runes for reactivity
- **shadcn-svelte:** Modern components, customizable, good DX

### Why SQLite?

- Single multi-game DB (with `game_id` field)
- Offline-first, no network dependencies
- Fast queries with indexes
- Easy to export/backup

### Why Python scrapers (future)?

- Better ecosystem for web scraping
- Easy to maintain independent from core
- JSON as exchange format

### Why not Firebase/Supabase?

- Requires internet connection
- Costs scale with usage
- Capcom data is static (doesn't change frequently)

---

## Time Estimation (Cumulative)

| Phase                                 | Weeks         | Status                                    |
| ------------------------------------- | ------------- | ----------------------------------------- |
| Phase 1: MVP Core                     | 6-8           | ✅ Done                                   |
| Phase 2: Data Expansion (MH2G)        | 2-3           | ✅ Done                                   |
| Phase 3: Build System (ASS)           | 3-4           | ✅ Done                                   |
| Phase 4: Global Search                | 1-2           | ✅ Done                                   |
| Phase 5: Multi-Game (4 titles @ 100%) | 3-4           | ✅ Done                                   |
| Phase 5b: MHW 100% + MHR 100%         | 2-3           | ✅ Done (010/011)                         |
| Phase 6: Mobile & Distribution        | 4-6           | 📋 Planned (spec 008; 017 = Android init) |
| Phase 7: Remaining gaps               | 2-3           | 📋 Planned (012–017)                      |
| **Total so far**                      | **~25 weeks** |                                           |

_Estimation based on part-time work (15-20h/week)_

---

## Resources

- **Tauri v2 Docs:** https://v2.tauri.app
- **Svelte 5:** https://svelte.dev/docs
- **shadcn-svelte:** https://next.shadcn-svelte.com
- **Tailwind CSS v4:** https://tailwindcss.com
- **SQLite:** https://www.sqlite.org
- **MHW API:** https://mhw-db.com
- **MHWilds API:** https://wilds.mhdb.io
- **MHP3rd DB:** https://github.com/Johnx199x/MHP3rd-DataBase

## Credits — Armor Set Search

The **builds / armor set search** engine is a Rust port of **[AthenaADP/MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS)** ("Athena's A.S.S.", MIT). The solver in `src-tauri/src/ass.rs` follows the original C++/CLI algorithm (equivalence grouping, jewel solver, Torso Inc, bad-skill fix, 1000 cap, sort). Data comes from the retail game (not ASS's CSVs); see `docs/fidelity-report.md`.

---

## License

Personal/educational project, released under MIT. Armor Set Search port credits Athena AD ([MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS), MIT). All Monster Hunter game data is property of Capcom.
