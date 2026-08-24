# MH-AIO (Monster Hunter All-In-One Encyclopedia)

A comprehensive, cross-platform desktop encyclopedia and toolkit for Monster Hunter games, built with **Tauri v2**, **Rust**, **Svelte 5**, and **SQLite**.

**Current focus: v0.2.0 MVP — Monster Hunter Freedom Unite (MHP2G)**

---

## Features

- **Multi-game support** — switch between MHW, MHR, MHWilds, MHP3rd, MH2G
- **Complete entity browser** — monsters, weapons, armor, quests, items, skills
- **Detail views** — click any entity to see official descriptions, stats, crafting materials, drop sources with probabilities, and cross-navigation between related entities
- **Per-game theming** — each game has its own color palette and decorative ornament pattern (e.g. medieval red/gold for MHP2G, futuristic green for MHWilds)
- **Offline-first** — SQLite bundled, all data lives locally
- **Cross-navigation** — items link to the monsters/quests that drop them, materials link to item pages
- **Back button** — always returns to the previous list view

---

## Tech Stack

- **Frontend:** Svelte 5, Vite, TypeScript, Tailwind CSS v4, shadcn-svelte (bits-ui)
- **Backend:** Rust (Tauri v2 core)
- **Database:** SQLite (`rusqlite` with bundled feature, WAL mode)
- **Routing:** SvelteKit (Client-side SPA mode with static adapter and `fallback: 'index.html'`)

---

## Supported Games

The app supports multiple titles through dynamic game routing (`[game]`):

| Slug | Game | Year |
|------|------|------|
| `mhw` | Monster Hunter: World | 2018 |
| `mhr` | Monster Hunter Rise | 2021 |
| `mhwilds` | Monster Hunter Wilds | 2025 |
| `mhp3rd` | MH Portable 3rd | 2010 |
| `mh2g` | MH 2ndG (Freedom Unite) | 2008 |

Each game section covers: Monsters, Weapons, Armor, Quests, Items, Skills, Builds.

---

## Getting Started

### Prerequisites

- **Node.js** (LTS recommended) & npm
- **Rust** (stable toolchain via [rustup](https://rustup.rs/))
- **Tauri v2 Prerequisites** (WebView2 on Windows, WebKitGTK on Linux, Xcode CLT on macOS)

### Installation

```bash
git clone https://github.com/Seb-fd/mh-aio.git
cd mh-aio
npm install
```

### Running & Building by Platform

#### 🖥️ Desktop (Windows, macOS, Linux)

- **Development mode:**
  ```bash
  npx tauri dev
  ```
- **Production build (Installers / Binaries):**
  ```bash
  npx tauri build
  ```

#### 📱 Mobile (Android & iOS)

- **Android:**
  1. Ensure Android SDK, NDK, and Java JDK are installed, with `ANDROID_HOME` configured.
  2. Initialize Android (first time only):
     ```bash
     npx tauri android init
     ```
     *(Note on Windows: Ensure Developer Mode is enabled to allow symbolic links).*
  3. Run on emulator or connected device:
     ```bash
     npx tauri android dev
     ```
  4. Build production APK / AAB:
     ```bash
     npx tauri android build
     ```

- **iOS (macOS required):**
  1. Ensure Xcode and Command Line Tools are installed.
  2. Initialize iOS (first time only):
     ```bash
     npx tauri ios init
     ```
  3. Run on simulator or device:
     ```bash
     npx tauri ios dev
     ```
  4. Build production app:
     ```bash
     npx tauri ios build
     ```

---

## Project Structure

```
mh-aio/
├── src/                              # Frontend (Svelte 5)
│   ├── app.html
│   ├── app.css                       # Tailwind + themed-bg utilities per game
│   ├── lib/
│   │   ├── api.ts                    # Typed invoke() wrapper
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte        # Themed nav
│   │   │   ├── header.svelte         # Themed top bar
│   │   │   ├── back-button.svelte    # history.back()
│   │   │   ├── detail-header.svelte  # Detail page header
│   │   │   ├── material-list.svelte  # Crafting materials
│   │   │   └── drop-table.svelte     # Drop sources w/ probability bars
│   │   ├── stores/
│   │   │   └── game.ts               # 5 games + GameTheme interface
│   │   └── utils/index.ts            # cn() helper
│   └── routes/
│       ├── +layout.svelte            # Theme injection via CSS vars
│       ├── +page.svelte              # Game selector landing
│       └── [game]/
│           ├── +page.svelte          # Dashboard
│           ├── monsters/             # list + [id]
│           ├── weapons/              # list + [id]
│           ├── armor/                # list + [id]
│           ├── quests/               # list + [id]
│           ├── items/                # list + [id]
│           ├── skills/               # list + [id]
│           └── builds/               # (placeholder)
├── src-tauri/                        # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                    # 14 Tauri commands registered
│   │   ├── commands/mod.rs
│   │   └── db/
│   │       ├── mod.rs                # Database struct (Mutex<Connection>)
│   │       ├── schema.rs             # 13 tables + ALTER TABLE migrations
│   │       ├── queries.rs            # List + detail queries w/ JOINs
│   │       └── seed.rs               # Idempotent MHP2G seed
│   ├── Cargo.toml
│   └── tauri.conf.json
├── AGENTS.md
├── README.md
├── roadmap.md
└── STATUS.md
```

---

## Database Schema

13 tables with full referential integrity. Highlights:

- **Core**: `games`, `monsters`, `weapons`, `armor`, `quests`, `items`, `skills`
- **Junctions**: `weapon_materials`, `armor_materials`, `item_combine`
- **References**: `monster_weaknesses`, `item_sources`, `quest_rewards`
- **Sets**: `armor_sets`, `decorations`

All entities have a `description` column populated by the seed for MHP2G.

---

## License

This project is **open source** and released under the MIT License. All Monster Hunter data is property of Capcom.