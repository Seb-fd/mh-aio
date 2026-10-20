# MH-AIO (Monster Hunter All-In-One Encyclopedia)

A comprehensive, cross-platform desktop encyclopedia and toolkit for Monster Hunter games, built with **Tauri v2**, **Rust**, **Svelte 5**, and **SQLite**.

**Current state: all five supported titles are fully seeded.** MH2G (Freedom Unite), MHP3rd (Portable 3rd), MHW + Iceborne, MH Rise + Sunbreak, and MH Wilds each have complete entity browsers, detail views, game-faithful ordering, per-game theming, and per-game global search. MH2G additionally has a full retail-UMD fidelity audit, and MHW / MHR each have a dedicated 100%-completeness pass.

> **All row counts in this file come from [`docs/data-coverage.md`](docs/data-coverage.md)** — a generated matrix of every table × game. Regenerate it with `py scripts/gen_data_coverage.py` rather than editing counts by hand.

---

## Features

- **Multi-game support** — switch between MHW, MHR, MHWilds, MHP3rd, MH2G via themed routing (`[game]`)
  - **MH2G / Freedom Unite** (`mh2g`, dbId 5) — 1244 items, 2075 armor (348 sets), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons (11 types), 610 quests, 99 skill families (214 abilities), 192 decorations, 432 combine recipes (379 Normal + 39 Alchemy + 14 Treasure), 15,671 `item_sources` rows — faithful to the retail UMD (`docs/fidelity-report.md`)
  - **MHP3rd / Portable 3rd** (`mhp3rd`, dbId 4) — 1065 items (Material 964 / Consumable 55 / Ammo 46), 378 quests (all bilingual, `name_original`), 60 monsters (40 Large + 20 Small), 972 weapons, 1106 armor, 398 combines (276 Normal + 122 Alchemy), 2200 drops, 50,635 charms, Comrades gear (92/154/52), 180 trap tools — sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright
  - **MHW + Iceborne** (`mhw`, dbId 1) — 1359 items (World + Iceborne incl. event/collab, Chest order via `sort_order`), 94 monsters (23 Small + 71 Large incl. variants), 3544 weapons (14 types, Smith tree order), 5826 monster drops, 521 quests, 1595 armor (351 sets), 404 decorations, 186 combines, 788 weakness rows, 721 ailment rows, 465 trap rows, 317 hunter charms, 211 Melder recipes, 20 Mantles/Boosters, 8 Palico Gadgets, **33 endemic creatures** — Fandom `MHWI` + MHWorldData
  - **MH Rise + Sunbreak** (`mhr`, dbId 2) — 1668 items, 3953 weapons, 1591 armor (410 sets), 112 monsters (78 Large + 34 Small), 946 quests, 6403 drops, 147 skills, 243 decorations, 11,849 talismans, buddy gear (322 Palico + 362 Palamute), 247 rampage skills, status ailments + part-break + traps — Kiranico DB v16.0.0
  - **MH Wilds** (`mhwilds`, dbId 3) — 773 items, 1188 weapons, 714 armor (183 sets), 68 monsters (34 Large + 34 Small), 174 quests, 1399 drops, 179 skills, 361 decorations, 76 talismans, **70 endemic creatures** — MHDB Wilds API + Fextra
- **Complete entity browser** — monsters (Small→Large sections, Chest order), weapons (Smith tree `Great Sword → Bow` with per-rarity icons, Charge Blade/Insect Glaive included), armor, quests, items (with `category • subcategory` + `Charm` + `Ammo/Husk` ISO taxonomy, Chest order via `sort_order`), skills, decorations, armor sets — all in game-faithful order and hub order
- **Combinations** — global list `Items → Combinations` (single view, `Normal/Alchemy/Treasure` badge + `success %` + Book order) and clickable recipes in item detail (`Godbug x1 + Wyvern Fang x1 = Life Crystals x1 • 90%`)
- **Armor Set Solver (Athena's A.S.S. port)** — pick up to 5 skills and find every armor set that activates them, including recommended jewels, spare slots, resists and defense. Full parity: hunter type (Blademaster/Gunner), gender, HR/Elder rank gate, weapon slots, piercings, Torso Inc, bad-skill handling, 1000-result limit and sort-by.
- **Armor sets view** — armors grouped into faithful sets (full 5/10-piece sets or singletons like Black Legs), grouped by `derive_set_name` to match the game's smith; browser filters: `Both | Male | Female` + `All | Blademaster | Gunner` (head Blademaster = higher defense) + rank
- **Monster → dedicated gear** — per-monster armor sets (≥40% of the set's materials come from that monster), rank-filtered, with a secondary "Uses 1 Material" view; subspecies (Azure/Silver Rathalos) kept separate; monster list in Hunter's Notes order (Felyne 1 … White Fatalis 83) with Large/Small/All selector
- **Charms & talismans** — three charm families in one browser: MHP3rd excavated charms (50,635 generator rows: Dragon / King / Queen), MHW hunter charms (317, rank tree I→V with skill search), MHR talismans (11,849 aggregated generator rows) and MH Wilds talismans (76)
- **Comrades** — Palico/Palamute buddy gear for MHR and MHP3rd, with skill tables
- **Hunter tools** — MHW Mantles/Boosters (20) and Palico Gadgets (8 + 38 proficiency levels), plus Melder recipes
- **Endemic life** — dedicated browser for MHW (33 curated creatures) and MH Wilds (70)
- **Builds, favorites & import** — save/load/export armor-set builds, star favorites across 8 entity kinds, and a transactional JSON/CSV import panel with dry-run preview
- **Weapon trees** — up to 14 trees in Smith order `Great Sword → Long Sword → Sword & Shield → Dual Blades → Hammer → Hunting Horn → Lance → Gunlance → Light Bowgun → Heavy Bowgun → Bow → Charge Blade → Insect Glaive`
  - **Quests** — MH2G's 610 quests across 9 hubs `Village Elder → Nekoto → Guild Low/High/G → Event → Challenge → Training School → Treasure Hunt` grouped into collapsible difficulty accordions (★ / G★ tiers); Event/Challenge downloadable quests validated vs kit `.bin` + guide
- **Global per-game search** — accent-insensitive, debounced suggestions across monsters, items, skills, weapons, armor/sets, quests and decorations
- **Gender-locked armor** — male/female-only pieces (e.g. Guardian Helm, Maiden's Hat) honored in both the browser and the solver
- **Detail views** — official descriptions, stats, crafting materials, drop sources with probabilities, and cross-navigation
- **Per-game theming** — each game has its own palette + ornament (medieval red/gold for MHP2G, futuristic green for Wilds, tribal for MHW)
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

| Slug      | Game                           | Year | `game_id` | Completeness                           |
| --------- | ------------------------------ | ---- | --------- | -------------------------------------- |
| `mhw`     | Monster Hunter: World          | 2018 | 1         | 100% (spec `010-mhw-100`)              |
| `mhr`     | Monster Hunter Rise + Sunbreak | 2023 | 2         | 100% (spec `011-mhr-100`)              |
| `mhwilds` | Monster Hunter: Wilds          | 2025 | 3         | Core + quests/charms/small/endemic     |
| `mhp3rd`  | MH Portable 3rd                | 2010 | 4         | 100% (spec `001-mhp3rd-fidelity-pass`) |
| `mh2g`    | MH 2ndG (Freedom Unite)        | 2008 | 5         | 100% (spec `000-mh2g-baseline`)        |

All five titles are fully seeded with complete entity browsers, detail views and theming. See [`docs/data-coverage.md`](docs/data-coverage.md) for the exact per-game row counts and [`roadmap.md`](roadmap.md) for remaining gaps.

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
     _(Note on Windows: Ensure Developer Mode is enabled to allow symbolic links)._
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
│   │   ├── api.ts                    # Typed invoke() wrapper (43 Tauri commands)
│   │   ├── components/
│   │   │   ├── ui/                   # shadcn-svelte primitives (card, button — no bits-ui)
│   │   │   ├── game-selector.svelte
│   │   │   ├── sidebar.svelte        # Themed nav (per-game visibility via gameOnly)
│   │   │   ├── header.svelte         # Themed top bar (+ global search)
│   │   │   ├── global-search.svelte  # Per-game accent-insensitive search (normKey)
│   │   │   ├── back-button.svelte    # history.back()
│   │   │   ├── detail-header.svelte  # Detail page header
│   │   │   ├── favorite-button.svelte
│   │   │   ├── item-icon.svelte      # Icon + text-label fallback
│   │   │   ├── material-list.svelte  # Crafting materials
│   │   │   └── drop-table.svelte     # Drop sources w/ probability bars
│   │   ├── prefetch.ts               # Sidebar hover-warm query registry
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
│           ├── items/                # list + combine + [id]
│           ├── skills/               # list + [id]
│           ├── decorations/          # list + [id]
│           ├── charms/               # MHP3rd charms / MHW hunter charms / MHR talismans / Wilds talismans
│           ├── comrades/             # MHP3rd + MHR buddy gear
│           ├── tools/                # MHW Mantles/Boosters + Palico Gadgets
│           ├── endemic/              # MHW + MH Wilds endemic creatures
│           └── builds/               # Armor Set Search (ASS port, no dummy)
├── docs/
│   ├── data-coverage.md              # GENERATED table x game_id row counts (source of truth)
│   └── fidelity-report.md            # Data vs retail UMD audit (MH2G) + MHP3rd/MHW/MHR catalogs
├── spec/                             # SDD specs (constitution/ + features/000-015/)
├── LICENSE                           # MIT
├── src-tauri/                        # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                    # 43 Tauri commands registered (no shell plugin)
│   │   ├── ass.rs                    # Armor Set Search solver (ASS port)
│   │   ├── builds.rs                 # Saved builds (app_data_dir/builds.json)
│   │   ├── favorites.rs              # Favorites (app_data_dir/favorites.json)
│   │   ├── import.rs                 # JSON/CSV import preview + transactional apply
│   │   ├── commands/mod.rs           # Command handlers
│   │   └── db/
│   │       ├── mod.rs                # Database struct + register_functions(norm_key)
│   │       ├── schema.rs             # Tables + ALTER TABLE migrations + schema_version + idempotency UNIQUE indexes
│   │       ├── queries.rs            # List/detail/search queries w/ JOINs; get_global_search via SQLite norm_key LIKE
│   │       └── seed.rs               # Idempotent seed (5 games, 27 data_patches) — no clear_game
│   ├── data/                         # mh2g_*.json / mhp3rd_*.json / mhw_*.json / mhr_*.json / mhwilds_*.json
│   ├── Cargo.toml                    # rusqlite {bundled, functions}; tauri no shell
│   ├── capabilities/default.json     # core:default only
│   ├── tauri.conf.json               # CSP hardened
│   └── tauri.conf.dev.json           # devtools overlay (dev only)
├── AGENTS.md
├── CONTRIBUTING.md
├── README.md
├── roadmap.md
└── STATUS.md
```

---

## Data Fidelity

**Authoritative row counts live in [`docs/data-coverage.md`](docs/data-coverage.md)** — a generated per-table × per-game matrix. Regenerate with `py scripts/gen_data_coverage.py` (or verify with `--check`).

MH2G data is generated to match the **retail UMD** (verified against a MHP2G English-patched ISO and a game-extracted DB). `docs/fidelity-report.md` documents the audit: defense/rarity/slots match 100%, and armor skill points were validated (the ported ASS algorithm is used only for solving, never as a data source). Further sections cover the **MHP3rd catalog** (1065 items / 378 quests, sourced from `MHP3: Item List` + `www.mhp3wiki.info` via Playwright), **MHW + Iceborne** (spec `010`), and **MH Rise + Sunbreak** (spec `011`).

---

## Documentation

| Doc                                                  | Purpose                                                                                                        |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| [`docs/data-coverage.md`](docs/data-coverage.md)     | **Generated** per-table × per-game row counts — the source of truth for every number quoted anywhere           |
| [`docs/fidelity-report.md`](docs/fidelity-report.md) | Retail-UMD audit for MH2G + catalog reports for MHP3rd / MHW / MHR                                             |
| [`AGENTS.md`](AGENTS.md)                             | Canonical architecture + agent/automation contract (read this before changing code)                            |
| [`CONTRIBUTING.md`](CONTRIBUTING.md)                 | Data-adding playbook, CI gate, commit/push policy                                                              |
| [`STATUS.md`](STATUS.md)                             | Per-spec completion checklist and current phase status                                                         |
| [`roadmap.md`](roadmap.md)                           | Phased plan, shipped features, remaining gaps                                                                  |
| [`spec/`](spec/)                                     | Spec-driven design docs — `constitution/` is the current-state layer, `features/NNN-*` are per-feature records |

---

## License

This project is **open source** and released under the MIT License. The **Armor Set Search** engine is a port of [AthenaADP/MHFU-ASS](https://github.com/AthenaADP/MHFU-ASS) (MIT, by Athena AD). All Monster Hunter game data is property of Capcom.
