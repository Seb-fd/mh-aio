# MH-AIO: Monster Hunter All-in-One Encyclopedia

## Project Vision

Create a comprehensive and offline tool for all Monster Hunter information, covering multiple games with detailed data on weapons, armor, monsters, quests, skills, items, builds, and suggestions.

---

## Tech Stack

### Frontend
- **Framework**: Tauri v2 (Rust + WebView)
- **UI Framework**: Svelte 5 + TypeScript
- **Styling**: Tailwind CSS v4
- **Components**: shadcn-svelte (bits-ui)
- **Build Tool**: Vite
- **State**: Svelte Stores / Runes

### Backend (Rust)
- **Framework**: Tauri v2 Commands (IPC)
- **Database**: SQLite (via `rusqlite` or `sqlx`)
- **Scrapers**: Python scripts for data extraction
- **Serialization**: Serde + JSON

### Platforms (Phase 2+)
- **Desktop**: Windows, macOS, Linux (Tauri v2)
- **Mobile**: iOS, Android (Tauri v2 mobile)
- **Web**: Progressive Web App (PWA) as alternative

---

## Project Structure

```
mh-aio/
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/             # Tauri commands
│   │   │   ├── mod.rs
│   │   │   ├── monsters.rs
│   │   │   ├── weapons.rs
│   │   │   ├── armor.rs
│   │   │   ├── quests.rs
│   │   │   ├── items.rs
│   │   │   ├── skills.rs
│   │   │   └── builds.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs
│   │   │   ├── queries/
│   │   │   └── migrations/
│   │   └── models/
│   │       ├── monster.rs
│   │       ├── weapon.rs
│   │       ├── armor.rs
│   │       ├── quest.rs
│   │       ├── item.rs
│   │       ├── skill.rs
│   │       └── game.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                          # Frontend Svelte
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/               # shadcn-svelte
│   │   │   ├── monsters/
│   │   │   ├── weapons/
│   │   │   ├── armor/
│   │   │   ├── quests/
│   │   │   ├── items/
│   │   │   ├── skills/
│   │   │   ├── builds/
│   │   │   └── shared/
│   │   ├── stores/
│   │   ├── services/             # IPC wrappers
│   │   └── utils/
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── +page.svelte          # Dashboard/Home
│   │   ├── monsters/
│   │   ├── weapons/
│   │   ├── armor/
│   │   ├── quests/
│   │   ├── items/
│   │   ├── skills/
│   │   └── builds/
│   └── app.html
├── scrapers/                     # Python Scrapers
│   ├── mhw/
│   ├── mhrise/
│   ├── mhwilds/
│   ├── mhp3rd/
│   ├── mh2ndg/
│   └── utils/
├── data/                         # Exported data
│   └── migrations/
└── package.json
```

---

## Database (SQLite)

### Main Schema

```sql
-- Supported games
CREATE TABLE games (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    abbreviation TEXT NOT NULL,  -- MHW, MHR, MHWilds, MHP3rd, MH2ndG
    release_year INTEGER,
    platform TEXT
);

-- Weapons
CREATE TABLE weapons (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    weapon_type TEXT NOT NULL,  -- GS, LS, DB, SA, CB, etc.
    rarity INTEGER,
    attack INTEGER,
    affinity INTEGER,
    element_type TEXT,
    element_value INTEGER,
    sharpness JSON,  -- {red, orange, yellow, green, blue, white, purple}
    slots JSON,      -- [{size: 1}, {size: 2}]
    skills JSON,     -- [{id, level}]
    crafting_cost INTEGER,
    upgrade_path JSON,
    language TEXT DEFAULT 'en'
);

-- Armor
CREATE TABLE armor (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    slot_type TEXT NOT NULL,  -- head, chest, arms, waist, legs
    rank TEXT NOT NULL,       -- low, high, master
    rarity INTEGER,
    defense_base INTEGER,
    defense_max INTEGER,
    resistance_fire INTEGER,
    resistance_water INTEGER,
    resistance_thunder INTEGER,
    resistance_ice INTEGER,
    resistance_dragon INTEGER,
    slots JSON,
    skills JSON,  -- [{skill_id, level}]
    set_id INTEGER,
    crafting_cost INTEGER,
    materials JSON,
    language TEXT DEFAULT 'en'
);

-- Armor sets
CREATE TABLE armor_sets (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    bonus_skill TEXT,
    bonus_required INTEGER,
    language TEXT DEFAULT 'en'
);

-- Monsters
CREATE TABLE monsters (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    species TEXT,  -- flying wyvern, brute wyvern, etc.
    size TEXT,     -- small, large, elder
    breakable_parts JSON,  -- [{name, sever, blunt, projectile, elements}]
    ailments JSON,
    language TEXT DEFAULT 'en'
);

-- Monster weaknesses
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
    type TEXT,  -- village, hub, event
    rank TEXT,  -- 1-8 stars, or LR/HR/MR
    objective TEXT,
    location TEXT,
    time_limit INTEGER,
    faints_allowed INTEGER,
    player_limit INTEGER,
    is_key_quest BOOLEAN DEFAULT FALSE,
    unlocks TEXT,
    language TEXT DEFAULT 'en'
);

-- Quest rewards
CREATE TABLE quest_rewards (
    id INTEGER PRIMARY KEY,
    quest_id INTEGER REFERENCES quests(id),
    item_id INTEGER REFERENCES items(id),
    quantity INTEGER,
    probability REAL,  -- 0.0 - 1.0
    condition TEXT     -- main, capture, break_part, etc.
);

-- Items
CREATE TABLE items (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    category TEXT,  -- material, consumable, ammo, etc.
    rarity INTEGER,
    sell_price INTEGER,
    description TEXT,
    language TEXT DEFAULT 'en'
);

-- Item sources
CREATE TABLE item_sources (
    id INTEGER PRIMARY KEY,
    item_id INTEGER REFERENCES items(id),
    source_type TEXT,  -- monster, quest, gathering, shop, crafting
    source_id INTEGER,
    quantity_min INTEGER,
    quantity_max INTEGER,
    probability REAL,
    location TEXT,
    conditions TEXT
);

-- Skills
CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    name TEXT NOT NULL,
    description TEXT,
    max_level INTEGER,
    effects JSON,  -- [{level, description}]
    language TEXT DEFAULT 'en'
);

-- Decorations/Gems
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

-- Indexes
CREATE INDEX idx_weapons_game ON weapons(game_id);
CREATE INDEX idx_weapons_type ON weapons(weapon_type);
CREATE INDEX idx_armor_game ON armor(game_id);
CREATE INDEX idx_armor_slot ON armor(slot_type);
CREATE INDEX idx_monsters_game ON monsters(game_id);
CREATE INDEX idx_quests_game ON quests(game_id);
CREATE INDEX idx_items_game ON items(game_id);
CREATE INDEX idx_skills_game ON skills(game_id);
```

---

## Priority Games

### Priority 1 (MVP)
1. **Monster Hunter World: Iceborne** (2018/2019)
   - Sources: mhw-db.com API, Kiranico, MHWorldData (GitHub)
   - Status: Most accessible and complete data

2. **Monster Hunter Rise: Sunbreak** (2021/2022)
   - Sources: Kiranico, Game8, Monster Hunter Wiki
   - Status: Good data available

3. **Monster Hunter Wilds** (2025)
   - Sources: Kiranico (mhwilds.kiranico.com), Game8
   - Status: Constantly updated

### Priority 2
4. **Monster Hunter Portable 3rd** (2010)
   - Sources: MHP3rd Database (GitHub), Monster Hunter Wiki
   - Status: Limited but available data

5. **Monster Hunter 2ndG / Freedom Unite** (2008/2009)
   - Sources: Monster Hunter Wiki (fandom)
   - Status: More limited data,OldData

---

## Features by Phase

### Phase 1: MVP Core (6-8 weeks)
**Goal**: Functional app with MH World data

#### 1.1 Project Setup
- [ ] Initialize Tauri v2 + Svelte 5 + shadcn-svelte
- [ ] Configure SQLite with migrations
- [ ] Configure Tailwind CSS
- [ ] Base component structure

#### 1.2 Data Model
- [ ] Implement SQL schema
- [ ] Create initial migrations
- [ ] Rust models (serde)

#### 1.3 Initial Scrapers
- [ ] Scraper for MHW (mhw-db.com API)
- [ ] JSON data importer
- [ ] Data validation

#### 1.4 Core UI
- [ ] Main layout with navigation
- [ ] Game selector (tabs or dropdown)
- [ ] Home/dashboard page
- [ ] Global search

### Phase 2: Complete Encyclopedia (4-6 weeks)
**Goal**: All data sections

#### 2.1 Monsters
- [ ] Monster list with filters
- [ ] Detailed view per monster
- [ ] Weakness table (parts x elements)
- [ ] Drop materials
- [ ] Hunting tips

#### 2.2 Weapons
- [ ] List by weapon type
- [ ] Filters by element, rarity, rank
- [ ] Evolution tree
- [ ] Weapon comparator
- [ ] Detailed stats

#### 2.3 Armor
- [ ] List by slot and rank
- [ ] Filters by skills
- [ ] Armor sets and bonuses
- [ ] Armor comparator
- [ ] Set builder (see Phase 4)

#### 2.4 Quests
- [ ] Quest list by type/rank
- [ ] Highlighted key quests
- [ ] Rewards with probabilities
- [ ] Filters by objectives

#### 2.5 Items
- [ ] Complete item list
- [ ] Acquisition sources
- [ ] Gathering locations
- [ ] Crafting recipes

#### 2.6 Skills
- [ ] List per game
- [ ] Description per level
- [ ] Which armor/decorations provide it
- [ ] Build guides

### Phase 3: Multi-Game (3-4 weeks)
**Goal**: Support for all priority games

#### 3.1 Additional Scrapers
- [ ] Scraper for MHRise/Sunbreak
- [ ] Scraper for MHWilds
- [ ] Scraper for MHP3rd
- [ ] Scraper for MH2ndG

#### 3.2 Per-Game Adaptation
- [ ] Translation system per game
- [ ] Adapt UI to game mechanics
  - Wirebugs (Rise)
  - Switch Skills (Rise)
  - Focus Mode (Wilds)
  - Old skill system (2ndG, P3rd)
- [ ] Different mechanics data

### Phase 4: Build System (3-4 weeks)
**Goal**: Planning tools

#### 4.1 Set Builder
- [ ] Select desired skills
- [ ] Filter armor by skills
- [ ] Show optimal combinations
- [ ] Calculate available slots
- [ ] Export/share builds

#### 4.2 Build Suggestions
- [ ] Builds per weapon type
- [ ] Elemental vs raw builds
- [ ] Builds per player rank
- [ ] Meta/endgame builds

#### 4.3 Calculator
- [ ] Calculate total damage
- [ ] Compare configurations
- [ ] Show skill efficiency

### Phase 5: Advanced Features (2-3 weeks)
**Goal**: Extra features

#### 5.1 Import Panel
- [ ] UI to import JSON/CSV
- [ ] Data validation
- [ ] Update merging
- [ ] Change log

#### 5.2 Advanced Search
- [ ] Fuzzy search
- [ ] Combined filters
- [ ] Save searches

#### 5.3 Favorites and History
- [ ] Mark favorite monsters/weapons
- [ ] View history
- [ ] Personal notes

### Phase 6: Desktop & Mobile (4-6 weeks)
**Goal**: Native apps

#### 6.1 Desktop (Tauri v2)
- [ ] Build for Windows
- [ ] Build for macOS
- [ ] Build for Linux
- [ ] Auto-updater
- [ ] System tray

#### 6.2 Mobile (Tauri v2)
- [ ] Adapt UI for mobile
- [ ] Touch gestures
- [ ] Build for iOS
- [ ] Build for Android
- [ ] Offline-first

---

## Detailed Data Sources

### Monster Hunter World / Iceborne
- **API**: https://mhw-db.com (RESTful, JSON)
- **GitHub**: https://github.com/gatheringhallstudios/MHWorldData
- **Kiranico**: https://mhworld.kiranico.com
- **Game8**: https://game8.co/games/Monster-Hunter-World

### Monster Hunter Rise / Sunbreak
- **Kiranico**: https://mhrise.kiranico.com
- **Game8**: https://game8.co/games/Monster-Hunter-Rise
- **Wiki**: https://monsterhunterwiki.org/wiki/MHRS
- **GitHub**: https://github.com/Johnx199x/MHP3rd-DataBase (reference)

### Monster Hunter Wilds
- **Kiranico**: https://mhwilds.kiranico.com
- **Game8**: https://game8.co/games/Monster-Hunter-Wilds
- **Wiki**: https://monsterhunterwiki.org/wiki/Monster_Hunter_Wilds
- **API**: https://wilds.mhdb.io (new)

### Monster Hunter Portable 3rd
- **GitHub**: https://github.com/Johnx199x/MHP3rd-DataBase
- **Database**: https://mhp3db.github.io
- **Wiki**: https://monsterhunter.fandom.com/wiki/Monster_Hunter_Portable_3rd

### Monster Hunter 2ndG / Freedom Unite
- **Wiki**: https://monsterhunter.fandom.com/wiki/Monster_Hunter_Freedom_Unite
- **MH-AIO**: https://mh-api.com (multi-game)

---

## Scrapers

### Technology
- **Language**: Python 3.10+
- **HTTP**: httpx (async) or requests
- **Parsing**: BeautifulSoup4 / parsel
- **Storage**: JSON export → SQLite import

### Scraper Structure
```
scrapers/
├── mhw/
│   ├── weapons.py      # mhw-db.com API
│   ├── armor.py
│   ├── monsters.py
│   ├── quests.py
│   ├── items.py
│   └── run_all.py
├── mhrise/
│   ├── kiranico.py
│   └── game8.py
├── mhwilds/
│   └── kiranico.py
├── mhp3rd/
│   └── wiki.py
├── mh2ndg/
│   └── wiki.py
└── utils/
    ├── database.py     # SQLite helpers
    ├── parsers.py      # Common parsers
    └── exporters.py    # JSON export
```

### Data Flow
1. Scraper runs → extracts data from web/API
2. Exports to normalized JSON
3. Tauri importer reads JSON
4. Inserts into SQLite with validation
5. UI queries SQLite via commands

---

## Internationalization

### Supported Languages
- **English** (primary)
- **Spanish**
- **Japanese** (original names)

### Strategy
- Each record has `language` field
- UI allows language switching
- JP names always available as reference

---

## Key Technical Decisions

### Why Tauri + Svelte?
- **Tauri v2**: Small binaries (5-15MB vs 100MB+ Electron), mobile support, security
- **Svelte 5**: Compiled, no runtime overhead, runes for reactivity
- **shadcn-svelte**: Modern components, customizable, good DX

### Why SQLite?
- Single multi-game DB (with `game_id` field)
- Offline-first, no network dependencies
- Fast queries with indexes
- Easy to export/backup

### Why Python scrapers?
- Better ecosystem for web scraping
- Easy to maintain independent from core
- JSON as exchange format

### Why not Firebase/Supabase?
- Requires internet connection
- Costs scale with usage
- Capcom data is static (doesn't change frequently)

---

## Time Estimation

| Phase | Weeks | Dependencies |
|-------|-------|--------------|
| Phase 1: MVP Core | 6-8 | Setup, MHW Scraper |
| Phase 2: Encyclopedia | 4-6 | Phase 1 |
| Phase 3: Multi-Game | 3-4 | Phase 2 |
| Phase 4: Builds | 3-4 | Phase 2 |
| Phase 5: Advanced | 2-3 | Phase 4 |
| Phase 6: Desktop/Mobile | 4-6 | Phase 5 |
| **Total** | **22-31** | |

*Estimation based on part-time work (15-20h/week)*

---

## Immediate Next Steps

1. **Project setup**
   ```bash
   npm create tauri-app@latest mh-aio -- --template svelte-ts
   cd mh-aio
   npm install
   npx shadcn-svelte@next init
   ```

2. **Install core dependencies**
   ```bash
   # Frontend
   npx shadcn-svelte@next add button card dialog input tabs table badge
   
   # Backend (Cargo.toml)
   # rusqlite = { version = "0.31", features = ["bundled"] }
   # serde = { version = "1", features = ["derive"] }
   # serde_json = "1"
   ```

3. **Create first scraper** (MHW via mhw-db.com API)
4. **Design SQL schema**
5. **Implement first Tauri command**
6. **Create basic UI with shadcn**

---

## Resources and Links

- **Tauri v2 Docs**: https://v2.tauri.app
- **Svelte 5**: https://svelte.dev/docs
- **shadcn-svelte**: https://next.shadcn-svelte.com
- **Tailwind CSS**: https://tailwindcss.com
- **SQLite**: https://www.sqlite.org

---

## License

Personal/educational project. All Monster Hunter data is property of Capcom.
