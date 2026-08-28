# MH-AIO (Monster Hunter All-In-One Encyclopedia)

A comprehensive, cross-platform desktop encyclopedia and toolkit for Monster Hunter games, built with **Tauri v2**, **Rust**, **Svelte 5**, and **SQLite**.

**Current focus: MHP2G (Freedom Unite, DB id 5) is the verified-complete MVP; MHP3rd (Portable 3rd, DB id 4) is fully seeded (1065 items / 378 quests / 60 monsters / 972 weapons / 1111 armor)** — detail views, armor set solver (Athena's A.S.S. port), per-game theming, and per-game global search.

---

## Features

 - **Multi-game support** — switch between MHW, MHR, MHWilds, MHP3rd, MH2G
   - **Full MHP2G/MH2G dataset** — 2075 armor pieces, 1083 items (fully sourced: every gathering node, shop, Veggie Elder/Trenya trade, Pokke Farm, small-monster drops verified vs ISO), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests (Village/Guild/Training/Treasure/Event/Challenge), ~99 skill families (214 abilities), 192 decorations — faithful to the retail UMD (`docs/fidelity-report.md`)
   - **MHP3rd (Portable 3rd) dataset seeded** — 1065 items (Material/Consumable/Ammo), 378 quests (village/guild/event/hot_spring/drink/nyanta/training/challenge, all 378 carry JP `name_original`), 60 monsters, 972 weapons, 1111 armor, 263 combines (202 Normal + 61 Alchemy) — sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright
- **Complete entity browser** — monsters, weapons, armor, quests, items (with `category • subcategory` + `Charm` + `Ammo/Husk` ISO taxonomy, `Chest` order), skills, decorations, armor sets — all in game-faithful order and hub order
- **Combinations** — global list `Items → Combinations` (single view, `Normal/Alchemy/Treasure` badge + `success %` + Book order) and clickable recipes in item detail (`Godbug x1 + Wyvern Fang x1 = Life Crystals x1 • 90%`)
- **Armor Set Solver (Athena's A.S.S. port)** — pick up to 5 skills and find every armor set that activates them, including recommended jewels, spare slots, resists and defense. Full parity: hunter type (Blademaster/Gunner), gender, HR/Elder rank gate, weapon slots, piercings, Torso Inc, bad-skill handling, 1000-result limit and sort-by.
- **Armor sets view** — armors grouped into faithful sets (full 5/10-piece sets or singletons like Black Legs), grouped by `derive_set_name` to match the game's smith; browser filters: `Both | Male | Female` + `All | Blademaster | Gunner` (head Blademaster = higher defense) + rank
- **Monster → dedicated gear** — per-monster armor sets (≥40% of the set's materials come from that monster), rank-filtered, with a secondary "Uses 1 Material" view; subspecies (Azure/Silver Rathalos) kept separate; monster list in Hunter's Notes order (Felyne 1 … White Fatalis 83) with Large/Small/All selector
- **Weapon trees** — 11 trees in Smith order `Great Sword → Long Sword → Sword & Shield → Dual Blades → Hammer → Hunting Horn → Lance → Gunlance → Light Bowgun → Heavy Bowgun → Bow`
   - **Quests** — 610 quests across 9 hubs `Village Elder → Nekoto → Guild Low/High/G → Event → Challenge → Training School → Treasure Hunt` grouped into collapsible difficulty accordions (★ / G★ tiers); Event/Challenge downloadable quests validated vs kit `.bin` + guide
- **Global per-game search** — accent-insensitive, debounced suggestions across monsters, items, skills, weapons, armor/sets, quests and decorations
- **Gender-locked armor** — male/female-only pieces (e.g. Guardian Helm, Maiden's Hat) honored in both the browser and the solver
- **Detail views** — official descriptions, stats, crafting materials, drop sources with probabilities, and cross-navigation
- **Per-game theming** — each game has its own palette + ornament (medieval red/gold for MHP2G, futuristic green for Wilds)
- **Offline-first** — SQLite bundled, all data lives locally
- **Back button** — always returns to the previous list view

---

## Tech Stack

- **Frontend:** Svelte 5, Vite, TypeScript, Tailwind CSS v4, shadcn-svelte (plain Svelte 5 primitives)
- **Backend:** Rust (Tauri v2 core)
- **Database:** SQLite (`rusqlite` with bundled+`functions` feature, WAL mode, `norm_key` SQL scalar)
- **Routing:** SvelteKit (Client-side SPA mode with static adapter and `fallback: 'index.html'`)

---

## Armor Set Search — Credits

The **builds / armor set search** engine is a faithful Rust port of **[AthenaADP/MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS)** ("Athena's A.S.S."), an armor set search tool for Monster Hunter Freedom Unite, released under the **MIT License**.

- Original C++/CLI implementation: [AthenaADP/MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS) (MIT)
- Ported algorithm — `src-tauri/src/ass.rs` — covers equivalence grouping, the jewel/decoration solver (1/2/3-slot), Torso Inc multiplier, bad-skill auto-fix, the 1000-result cap and sort comparators.
- Solver inputs mirror the ASS UI: hunter type, gender, HR/Elder (rank gate), weapon slots, piercings, Torso Inc, allow-bad-skills.
- Note: while the **algorithm** is ported from ASS, the **data** comes from the retail game (verified against your MHP2G UMD + game-extracted DB), not ASS's 2008 CSV snapshot. See `docs/fidelity-report.md`.

Under: **MIT License** — see the upstream repository for the original license.

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

MH2G is the verified-complete game; MHP3rd is fully seeded (routing + theming + all entity browsers); the remaining titles are wired for routing/theming with data to come.

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
│   │   ├── api.ts                    # Typed invoke() wrapper (incl. CombineView, no get_games)
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives (card, button — no bits-ui)
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte        # Themed nav
│   │   │   ├── header.svelte         # Themed top bar (+ global search)
│   │   │   ├── global-search.svelte  # Per-game accent-insensitive search (normKey)
│   │   │   ├── back-button.svelte    # history.back()
│   │   │   ├── detail-header.svelte  # Detail page header
│   │   │   ├── material-list.svelte  # Crafting materials
│   │   │   └── drop-table.svelte     # Drop sources w/ probability bars (Gathering/Shop/Veggie/Trenya)
│   │   ├── stores/
│   │   │   └── game.ts               # 5 games + GameTheme + localStorage guard
│   │   └── utils/
│   │       ├── index.ts              # cn()
│   │       └── norm.ts               # normKey() mirrors Rust norm_key
│   └── routes/
│       ├── +layout.svelte            # Theme injection via CSS vars
│       ├── +page.svelte              # Game selector landing
│       └── [game]/
│           ├── +page.svelte          # Dashboard
│           ├── monsters/             # list + [id] (dedicated sets)
│           ├── weapons/              # list + [id]
│           ├── armor/                # list, sets/[id], [id]
│           ├── quests/               # list + [id]
│           ├── items/                # list + combine + [id] (category • subcategory, Chest order, clickable recipes, JP badge)
│           ├── skills/               # list + [id]
│           ├── decorations/          # list + [id] (normKey search)
│           └── builds/               # Armor Set Search (ASS port, no dummy)
├── docs/
│   └── fidelity-report.md            # Data vs retail UMD audit (MH2G) + MHP3rd catalog report
├── LICENSE                           # MIT
├── src-tauri/                        # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                    # Tauri commands registered (no shell plugin)
│   │   ├── ass.rs                    # Armor Set Search solver (ASS port)
│   │   ├── commands/mod.rs           # no greet/get_games
│   │   └── db/
│   │       ├── mod.rs                # Database struct + register_functions(norm_key)
│   │       ├── schema.rs             # Tables + ALTER TABLE migrations + schema_version + idempotency UNIQUE indexes
│   │       ├── queries.rs            # List/detail/search queries w/ JOINs; get_global_search via SQLite norm_key LIKE
│   │       └── seed.rs               # Idempotent seed (MH2G + MHP3rd, 12k+ sources, deduplication) — no clear_game
│   ├── data/                         # mh2g_*.json (MH2G) + mhp3rd_*.json (MHP3rd)
│   ├── Cargo.toml                    # rusqlite {bundled, functions}; tauri no shell
│   ├── capabilities/default.json     # core:default only
│   ├── tauri.conf.json               # CSP hardened
│   └── tauri.conf.dev.json           # devtools overlay (dev only)
├── AGENTS.md
├── README.md
├── roadmap.md
└── STATUS.md
```

---

## Data Fidelity

MH2G data is generated to match the **retail UMD** (verified against a MHP2G English-patched ISO and a game-extracted DB). `docs/fidelity-report.md` documents the audit: defense/rarity/slots match 100%, and armor skill points were validated (the ported ASS algorithm is used only for solving, never as a data source). A second section covers the **MHP3rd catalog** (1065 items / 761 drops / 378 quests) sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright.

---

## License

This project is **open source** and released under the MIT License. The **Armor Set Search** engine is a port of [AthenaADP/MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS) (MIT, by Athena AD). All Monster Hunter game data is property of Capcom.
