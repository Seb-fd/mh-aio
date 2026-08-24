# MH-AIO - Project Status

## Current Version: v0.2.0 (MVP — Monster Hunter Freedom Unite / MHP2G)

## ✅ Completed

### Initial Setup
- [x] Node.js v22.23.2 + npm 10.9.8
- [x] Rust 1.98.0 + Cargo
- [x] Visual Studio Build Tools 2022
- [x] Tauri v2 + Svelte 5 + Tailwind v4 stack

### Project Structure
- [x] `package.json` configured with all dependencies
- [x] `svelte.config.js` with adapter-static + fallback
- [x] `vite.config.ts` with Tailwind v4 plugin (ignores `src-tauri/`)
- [x] `tsconfig.json`
- [x] `src/app.html`
- [x] `src/app.css` with Tailwind theme + themed-bg utilities per game

### Frontend (Svelte 5)
- [x] `src/routes/+layout.ts` (SSR disabled globally)
- [x] `src/routes/+layout.svelte` (conditional layout, theme injection via CSS custom props)
- [x] `src/routes/+page.svelte` (game selector landing)
- [x] `src/lib/utils/index.ts` (`cn()` helper — clsx + tailwind-merge)
- [x] `src/lib/api.ts` (typed wrapper around `@tauri-apps/api/core` `invoke()`)
- [x] `src/lib/components/ui/button.svelte`
- [x] `src/lib/components/ui/card.svelte`

### Game System
- [x] `src/lib/stores/game.ts` — 5 games, each with `theme` object (colors, ornament, glow)
- [x] `src/lib/components/game-selector.svelte` — landing page card grid
- [x] `src/lib/components/sidebar.svelte` — themed nav (uses CSS vars)
- [x] `src/lib/components/header.svelte` — themed top bar

### Per-Game Routes — List Views
- [x] `src/routes/[game]/+page.svelte` (game dashboard)
- [x] `src/routes/[game]/monsters/+page.svelte` — list with species color tags
- [x] `src/routes/[game]/weapons/+page.svelte` — list + type filter
- [x] `src/routes/[game]/armor/+page.svelte` — list + rank filter
- [x] `src/routes/[game]/quests/+page.svelte` — list + rank filter
- [x] `src/routes/[game]/items/+page.svelte` — list + category filter + search
- [x] `src/routes/[game]/skills/+page.svelte` — list
- [x] `src/routes/[game]/builds/+page.svelte` — placeholder for future planner

### Per-Game Routes — Detail Views (v0.2.0)
- [x] `src/routes/[game]/monsters/[id]/+page.svelte` — description + weaknesses table
- [x] `src/routes/[game]/weapons/[id]/+page.svelte` — stats + description + materials
- [x] `src/routes/[game]/armor/[id]/+page.svelte` — defenses + resistances + description + materials
- [x] `src/routes/[game]/items/[id]/+page.svelte` — description + combine recipe + drop sources
- [x] `src/routes/[game]/quests/[id]/+page.svelte` — description + objective + stats
- [x] `src/routes/[game]/skills/[id]/+page.svelte` — description + max level

### Reusable Components (v0.2.0)
- [x] `back-button.svelte` — `history.back()` with fallback to game home
- [x] `detail-header.svelte` — banner with icon, title, tags
- [x] `material-list.svelte` — clickable list of crafting materials with quantities
- [x] `drop-table.svelte` — drop sources with probability bars, navigates to monster/quest

### Backend (Rust / Tauri)
- [x] `src-tauri/Cargo.toml` — Tauri v2, serde, rusqlite (bundled)
- [x] `src-tauri/tauri.conf.json`
- [x] `src-tauri/src/main.rs`
- [x] `src-tauri/src/lib.rs` — registers 14 Tauri commands, sets up DB at `{app_data_dir}/mh-aio.db`
- [x] `src-tauri/src/commands/mod.rs` — list + detail commands

### Database (SQLite via rusqlite)
- [x] `src-tauri/src/db/mod.rs` — `Database` struct with `Mutex<Connection>`, WAL mode
- [x] `src-tauri/src/db/schema.rs` — all 13 tables + ALTER TABLE migrations
- [x] `src-tauri/src/db/queries.rs` — list/detail query functions with JOINs
- [x] `src-tauri/src/db/seed.rs` — idempotent seed (INSERT OR IGNORE) for MHP2G curated data

### Schema (v0.2.0)
- `games`, `weapons`, `armor`, `armor_sets`, `monsters`, `monster_weaknesses`
- `quests`, `quest_rewards`, `items`, `item_sources`, `skills`, `decorations`
- `weapon_materials` (junction), `armor_materials` (junction), `item_combine`
- All entities have `description` column (added via migration)
- Weapons/armor have `crafting_cost`

### Tauri Commands Registered (14)
- `greet` (legacy)
- `get_games`
- `get_monsters`, `get_weapons`, `get_armor`, `get_quests`, `get_items`, `get_skills`
- `get_monster_detail`, `get_weapon_detail`, `get_armor_detail`
- `get_quest_detail`, `get_item_detail`, `get_skill_detail`

### Per-Game Theming (v0.2.0)
- [x] Extended `Game` interface with `GameTheme` (14 CSS variables + ornament type)
- [x] 5 distinct themes: MHP2G (medieval red+gold), MHW (tribal blue), MHR (japanese orange), MHWilds (futuristic green), MHP3rd (japanese purple)
- [x] Ornament patterns via CSS (repeating-linear-gradient): medieval, japanese, tribal, futuristic, hunt
- [x] Theme applied via CSS custom properties on the layout wrapper
- [x] All list/detail pages consume the themed CSS vars

### Curated Seed Data (MHP2G)
- 28 monsters with descriptions and species/size tags
- 30 weapons with descriptions and crafting costs
- 25 armor pieces with descriptions, costs, resistances
- 12 quests with descriptions and key-quest flags
- 31 items with descriptions and categories
- 20 skills with descriptions and max levels
- ~38 weapon material recipes
- ~42 armor material recipes
- ~50 item drop sources (carve/quest_reward/mining/gather with probabilities)
- 4 combine recipes (Mega Potion = Potion + Honey, Max Potion = Nutrients + Mega Potion)

### Build
- [x] Placeholder icons (32x32, 128x128, 128x128@2x, icon.ico)
- [x] `cargo build` — clean
- [x] `npm run build` — clean
- [x] `npx svelte-check` — 0 errors, 0 warnings
- [x] Tauri app launches and runs without runtime errors (FK issue fixed in seed idempotency)

---

## 🎮 Implemented User Flow

```
[App opens] → Game selector (5 cards themed by game)
                 │ click game
                 ▼
         Game home (dashboard with 8 sections)
                 │
                 ▼
         List view (monsters / weapons / armor / quests / items / skills)
                 │ click card
                 ▼
         Detail view (description, stats, materials, drop sources, related entities)
                 │ click material/source
                 ▼
         Cross-navigation to related entity (item/monster/quest)
                 │
                 ▼
         Back button → returns to previous list
```

---

## 📋 Next Steps

### Immediate (v0.3.0)
- [ ] Expand curated seed data — cover remaining monsters/weapons/armor
- [x] Add monster_weaknesses data for all monsters
- [ ] Add quest_rewards data
- [ ] Per-weapon sharpness data
- [ ] Weapon upgrade paths / evolution trees

### Phase 2 (Build planner)
- [ ] Skill list with "select skills" UI
- [ ] Armor pieces filtered by which skills they provide
- [ ] Optimal set calculator (max points per skill)
- [ ] Save/load custom builds
- [ ] Export builds to JSON

### Phase 3 (Other games)
- [ ] MHW data via scraper or curated import
- [ ] MHR data
- [ ] MHWilds data
- [ ] MHP3rd data

### Phase 4 (Polish)
- [ ] Global search across all entities
- [ ] Favorites system
- [ ] Offline mode verification
- [ ] Auto-update mechanism
- [ ] Mobile build via Tauri v2

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

# Type/Svelte check
npx svelte-check
```

---

## 📁 File Structure

```
mh-aio/
├── src/                              # Frontend (Svelte 5)
│   ├── app.html
│   ├── app.css                       # Tailwind + theme utilities
│   ├── lib/
│   │   ├── api.ts                    # Typed invoke() wrapper
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives (card, button)
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte        # Themed nav
│   │   │   ├── header.svelte         # Themed top bar
│   │   │   ├── back-button.svelte    # v0.2.0
│   │   │   ├── detail-header.svelte  # v0.2.0
│   │   │   ├── material-list.svelte  # v0.2.0
│   │   │   └── drop-table.svelte     # v0.2.0
│   │   ├── stores/
│   │   │   └── game.ts               # 5 games + GameTheme interface
│   │   └── utils/index.ts
│   └── routes/
│       ├── +layout.ts                # SSR disabled
│       ├── +layout.svelte            # Theme injection
│       ├── +page.svelte              # Game selector
│       └── [game]/
│           ├── +layout.ts
│           ├── +page.svelte          # Dashboard
│           ├── monsters/             # list + [id] detail
│           ├── weapons/              # list + [id] detail
│           ├── armor/                # list + [id] detail
│           ├── quests/               # list + [id] detail
│           ├── items/                # list + [id] detail
│           ├── skills/               # list + [id] detail
│           └── builds/               # placeholder
├── src-tauri/                        # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                    # 14 commands registered
│   │   ├── commands/mod.rs
│   │   └── db/
│   │       ├── mod.rs                # Database struct
│   │       ├── schema.rs             # 13 tables + ALTER TABLE migrations
│   │       ├── queries.rs            # List + detail queries
│   │       └── seed.rs               # Idempotent MHP2G seed
│   ├── Cargo.toml
│   └── tauri.conf.json
├── scrapers/                         # Future Python scrapers
├── static/                           # Static assets
├── AGENTS.md
├── README.md
├── roadmap.md
└── STATUS.md
```
