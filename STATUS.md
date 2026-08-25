# MH-AIO - Project Status

## Current Version: v0.3.0 (Monster Hunter Freedom Unite / MHP2G — verified dataset + ordered browsers)

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
- [x] `items/` — list (game-order chest sort + name/rarity/price/category) + `[id]`
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
   - [x] 2075 armor pieces, 1083 items, 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests (Training 140 / Treasure 7 / Event 37 / Challenge 14), 99 skill families (214 abilities), 192 decorations
- [x] Monster order faithful to UMD Hunter's Notes (Felyne 1 … White Fatalis 83, `ORDER BY id`); Large/Small selector preserves order
- [x] Weapon order faithful to Smith tree (`Great Sword → Bow`, `ORDER BY CASE weapon_type`, `src-tauri/src/db/queries.rs:698`)
- [x] Quest hubs: `elder → nekoto → guild_low → guild_high → guild_g → training → treasure → event` (`queries.rs:1157`, `quests/+page.svelte:31`); Other split into Training School and Treasure Hunt; Event quests (33 downloadable, not in ISO) validated against distribution file + wiki
- [x] Quest validation vs ISO `DATA.BIN` string table (`Mountain Herb Picking` at `0x13E…`, `Training`/`Treasure` labels); total 559 base quests match retail
- [x] Fixed 138 item sell prices vs game-extracted DB (Thawing Agent 100→10 etc.)
- [x] Wings reclassified Ammo → Material; header artifact removed
   - [x] `docs/fidelity-report.md` — single merged report: defense/rarity/slots 100%, weapons per-type 100%, monster catalog 83, quest hubs 610 (ISO string table + guild rank fix + kit Event/Challenge) vs retail UMD + game-extracted DB

### Backend (Rust / Tauri)
- [x] `ass.rs`, `commands/mod.rs`, `db/{mod,schema,queries,seed}.rs`
- [x] Tauri commands: `get_games`, `get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_armor_set_detail`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, list + detail variants, `search_armor_sets`, `global_search`

### Build
- [x] `cargo build` — clean (non-blocking warnings)
- [x] `cargo test` — ASS rank-gate unit tests pass
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
- [ ] Populate MHW, MHR, MHWilds, MHP3rd datasets (routing/theming in place)

### Phase 3 (Builds polish)
- [ ] Save/load custom builds
- [ ] Export builds to JSON / share link

### Phase 4 (Advanced)
- [ ] Favorites system
- [ ] Import panel for JSON/CSV
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

# Run Rust unit tests (ASS solver)
cargo test --manifest-path src-tauri/Cargo.toml

# Type/Svelte check
npx svelte-check
```

---

## 📁 File Structure

```
mh-aio/
├── src/                              # Frontend (Svelte 5)
│   ├── lib/
│   │   ├── api.ts                    # Typed invoke() wrapper
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives
│   │   │   ├── global-search.svelte  # Per-game accent-insensitive search
│   │   │   ├── header.svelte         # Themed top bar + search
│   │   │   ├── sidebar.svelte
│   │   │   ├── back-button.svelte
│   │   │   ├── detail-header.svelte
│   │   │   ├── material-list.svelte
│   │   │   └── drop-table.svelte
│   │   └── stores/game.ts
│   └── routes/[game]/
│       ├── monsters/                 # list + [id] (dedicated sets)
│       ├── weapons/                  # list + [id]
│       ├── armor/                    # list, sets/[id], [id]  (gender/rank filter)
│       ├── quests/                   # list + [id]
│       ├── items/                    # list + [id]
│       ├── skills/                   # list + [id]
│       ├── decorations/              # list + [id]
│       └── builds/                   # Armor Set Search (ASS port)
├── docs/fidelity-report.md
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                    # Tauri commands registered
│   │   ├── ass.rs                    # Armor Set Search solver (ASS port)
│   │   ├── commands/mod.rs
│   │   └── db/{mod,schema,queries,seed}.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── AGENTS.md
├── README.md
├── roadmap.md
└── STATUS.md
```
