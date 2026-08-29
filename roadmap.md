# MH-AIO: Monster Hunter All-in-One Encyclopedia

## Project Vision

A comprehensive, offline-first encyclopedia and toolkit for all Monster Hunter games, covering multiple titles with detailed data on weapons, armor, monsters, quests, skills, items, builds (incl. an armor set solver ported from Athena's A.S.S.), and suggestions.

Current state: **MH2G / Freedom Unite is fully populated and verified** (2075 armor, 1083 items, 83 monsters, 1500 weapons, 99 skill families, 192 decorations) with detail views, game-faithful ordering (monsters Hunter's Notes order, weapons Smith order), filtered browsers (Large/Small, Blademaster/Gunner), armor set search, and per-game global search. **MHP3rd / Portable 3rd (DB id 4) is fully seeded** (1065 items, 378 quests — all 378 bilingual, 60 monsters, 972 weapons, 1111 armor, 263 combines, 761 drops, 1867 quest rewards). Remaining titles (MHW/MHR/MHWilds) are wired for routing/theming with data pending.

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
│   │   ├── lib.rs                # Tauri commands registered (no shell, no greet/get_games)
│   │   ├── commands/
│   │   │   └── mod.rs            # List + detail commands
│   │   └── db/
│   │       ├── mod.rs            # Database (Mutex<Connection> + register_functions(norm_key))
│   │       ├── schema.rs         # Tables + ALTER TABLE migrations + schema_version + idempotency UNIQUE indexes
│   │       ├── queries.rs        # List/detail/search (CombineView, game order, SQLite LIKE via norm_key)
│   │       └── seed.rs           # Idempotent MH2G+MHP3rd seed (no clear_game, dedup) + backfill
│   ├── data/                     # mh2g_*.json + mhp3rd_*.json
│   ├── Cargo.toml                # crate-type = ["lib", "cdylib", "staticlib"]; rusqlite {bundled, functions}
│   ├── capabilities/default.json # core:default only
│   ├── tauri.conf.json           # CSP hardened
│   └── tauri.conf.dev.json       # devtools overlay
├── src/                          # Frontend Svelte 5
│   ├── app.html
│   ├── app.css                   # Tailwind + themed-bg per game ornament
│   ├── lib/
│   │   ├── api.ts                # Typed invoke() wrapper (no Game/greet)
│   │   ├── components/
│   │   │   ├── ui/               # shadcn-svelte primitives (card, button — plain Svelte 5)
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte    # Themed nav (v0.1.0)
│   │   │   ├── header.svelte     # Themed top bar
│   │   │   ├── back-button.svelte
│   │   │   ├── detail-header.svelte
│   │   │   ├── material-list.svelte
│   │   │   └── drop-table.svelte
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
│           └── builds/           # Armor Set Search (ASS port, no dummy)
├── docs/
│   └── fidelity-report.md        # MH2G audit + MHP3rd catalog report
├── LICENSE                       # MIT
├── AGENTS.md
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

### Current Focus

1. **Monster Hunter 2ndG / Freedom Unite** (2008) — MHP2G, **fully populated and verified**
   - 2075 armor, **1083 items fully sourced** (12,751 `item_sources` rows + 432 combine recipes), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests (Village/Guild/Training/Treasure/Event/Challenge), 99 skill families (214 abilities), 192 decorations
   - Materials, drop sources, combine recipes populated; item taxonomy ISO-derived (`Consumable 91 / Material 913 / Ammo 79` + `subcategory` Charm/Husk/Coating); ordering faithful to UMD (Hunter's Notes, Smith trees, quest hubs, Book of Combos)
   - Data verified against retail UMD and event distribution file (see `docs/fidelity-report.md`)
   - Armor Set Search (Athena's A.S.S. port) + per-game global search + ordered, filtered browsers (Large/Small, Blademaster/Gunner, Training/Treasure/Event) + combinations view (Normal/Alchemy/Treasure, `success %`) + clickable recipes
2. **Monster Hunter Portable 3rd** (2010) — MHP3rd, **fully seeded (mhp3rd, DB id 4)**
   - 1065 items (Material 964 / Consumable 55 / Ammo 46, 291 descriptions, 181 buy prices), 378 quests (`village 96 · guild_low 88 · guild_high 100 · event 52 · hot_spring 7 · drink 16 · nyanta 3 · training 10 · challenge 6`, all 378 carry JP `name_original`), 60 monsters, 972 weapons, 1111 armor, 263 combines (202 Normal + 61 Alchemy), 761 drops, 1867 quest rewards
   - Sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright; gaps: gathering-only `item_sources` (shop/trade/farm pending), numeric struct audit pending

### Planned

3. **Monster Hunter World: Iceborne** (2018/2019)
   - Sources: mhw-db.com API, Kiranico
4. **Monster Hunter Rise: Sunbreak** (2021/2022)
   - Sources: Kiranico, Game8
5. **Monster Hunter Wilds** (2025)
   - Sources: Kiranico (mhwilds.kiranico.com), Game8

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
- [x] **Items fully sourced** (12,751 `item_sources` rows: every gathering node/mining/bug/fish from `maps.json`, 5 merchants consolidated, Veggie Elder + Trenya Boat trades, Pokke Farm spots/trees, small-monster drops) — 1083/1083 covered
- [x] **Combine recipes** 432 (147 Normal + 18 Alchemy + 7 Treasure) with `chance` + Book order (`get_combinations`, `/items/combine`, clickable `A x1 + B x1 = Result x1 • 90%`)
- [x] **Item taxonomy ISO-derived** (`Consumable 91 / Material 913 / Ammo 79`, `subcategory` Recovery/Buff/Food/**Charm**/Husk/Coating/Ore/Monster Material)
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
- [ ] Save/load custom builds (future)
- [ ] Export builds to JSON / share link (future)

### ✅ Phase 4 (partial): Global Search (DONE)

- [x] Global per-game search across all entities (accent-insensitive, debounced)
- [ ] Favorites system (future)
- [ ] Import panel for JSON/CSV (future)
- [ ] Offline mode verification (future)
- [ ] Auto-update mechanism (future)

### ✅ Phase 5: Multi-Game — MHP3rd seeded (partial)

- [x] MHP3rd data import — 1065 items, 378 quests, 60 monsters, 972 weapons, 1111 armor, 263 combines, 761 drops, skills/decorations seeded via `src-tauri/data/mhp3rd_*.json` + `db/seed.rs` (idempotent)
- [ ] MHW scraper (mhw-db.com API)
- [ ] MHR scraper (Kiranico / Game8)
- [ ] MHWilds scraper
- [ ] Game-specific UI adaptations (Focus Mode, Wirebugs, etc.)

### 📋 Phase 6: Mobile & Distribution

- [ ] Mobile build via Tauri v2 (already supported via `cdylib`)
- [ ] Auto-updater
- [ ] System tray
- [ ] Cross-platform packaging (Windows / macOS / Linux / iOS / Android)

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

| Phase                          | Weeks         | Status     |
| ------------------------------ | ------------- | ---------- |
| Phase 1: MVP Core              | 6-8           | ✅ Done    |
| Phase 2: Data Expansion (MH2G) | 2-3           | ✅ Done    |
| Phase 3: Build System (ASS)    | 3-4           | ✅ Done    |
| Phase 4: Global Search         | 1-2           | ✅ Done    |
| Phase 5: Multi-Game            | 3-4           | 📋 Planned |
| Phase 6: Mobile & Distribution | 4-6           | 📋 Planned |
| **Total**                      | **~25 weeks** |            |

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
