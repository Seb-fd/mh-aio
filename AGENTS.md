# AGENTS.md

## What this is

Tauri v2 desktop app (Monster Hunter encyclopedia). Rust backend + Svelte 5 frontend + SQLite. English-primary UI. **Monster Hunter Freedom Unite (MHP2G / `mh2g`, DB id 5)** is the completed MVP with detail views and per-game theming; **Monster Hunter Portable 3rd (MHP3rd / `mhp3rd`, DB id 4)** is the next game being built out (items, quests, monsters, armor, weapons, combines, drops all seeded).

## Commands

```bash
npm run dev                 # Frontend only (Vite, port 1420)
npx tauri dev               # Full app (frontend + Rust backend)
npx tauri build             # Production build
cargo build --manifest-path src-tauri/Cargo.toml  # Rust backend only
npm run check / lint / typecheck  # svelte-check (all three aliases)
cargo test --manifest-path src-tauri/Cargo.toml   # Rust unit tests (ASS + db)
```

`package.json` defines `check`, `check:watch`, `lint` and `typecheck` (all wrapping `svelte-check`). There is **no dedicated format or frontend unit-test runner**; the Rust suites in `ass.rs` and `db::queries` are exercised by `cargo test`.

## Architecture

- **SSR disabled** globally (`src/routes/+layout.ts` sets `export const ssr = false`). Routes also disable prerender. This is a pure client-side SPA served via Tauri WebView.
- **adapter-static** with `fallback: 'index.html'` — all routes are handled client-side.
- **Tailwind CSS v4** — uses `@import 'tailwindcss'` + `@theme` block in `src/app.css`. No `tailwind.config.js` exists.
- **shadcn-svelte** — primitives in `src/lib/components/ui/` (`card.svelte`, `button.svelte`). Config in `components.json` defines aliases: `$lib/components`, `$lib/utils`, `$lib/hooks`. `bits-ui` is no longer a dependency; the ui primitives are plain Svelte 5.
- **cn() helper** — `src/lib/utils/index.ts` exports `cn()` (clsx + tailwind-merge). Use it for conditional classes. `src/lib/utils/norm.ts` mirrors the Rust `norm_key` (accent/case-insensitive) for frontend list filtering — keep them in sync.

## Database

SQLite via `rusqlite` with bundled feature. Schema defined in `src-tauri/src/db/schema.rs`. WAL mode enabled. DB path: `{app_data_dir}/mh-aio.db`. Schema created via `CREATE TABLE IF NOT EXISTS`.

**Migrations**: No framework. `apply_migrations()` in `schema.rs` uses `ALTER TABLE ... ADD COLUMN` with `pragma_table_info` check. New tables (e.g. `weapon_materials`, `armor_materials`, `item_combine`) are picked up automatically on next run since `CREATE TABLE IF NOT EXISTS` is idempotent. Recent migrations: `items.subcategory TEXT` (item taxonomy), `item_combine.combine_type TEXT` + `chance INTEGER` (Normal/Alchemy/Treasure). A `schema_version` bookkeeping table (`schema_version(version)`) is created and read via `get_schema_version()`.

**Idempotency**: The seed is strictly idempotent **without** the old destructive `clear_game`. `add_idempotency_constraints()` in `schema.rs` first **dedupes** pre-existing dirty junction rows (keeps lowest `rowid`) then creates `CREATE UNIQUE INDEX IF NOT EXISTS` natural keys (e.g. `uq_item_combine(result_item_id, component_item_id, combine_type)`, `uq_item_sources(...)`, `uq_monster_equipment(game_id, monster_id, equipment_kind, equipment_id)`) and `(game_id, id)` guard indexes on content tables, so `INSERT OR IGNORE` has a real conflict target and no longer appends duplicates on re-run. `clear_game`/`clear_mh2g`/`clear_mhp3rd` were removed entirely.

**Seed**: `src-tauri/src/db/seed.rs` runs on startup, in `BEGIN IMMEDIATE` + `INSERT OR IGNORE` — fully idempotent. Current **MHP3rd (mhp3rd, game_id 4)**: 1044 items (1065→1044 after purging `"(Hunt 1 [[Zinogre]])"` + 18 typos) updated with `mikejsavage/MHP3DB` (`DATA.BIN` decrypted ULJM-05800, 813 canonical items with `value/rarity/icon` + `MHP3DB` as ISO proxy — the `Monster Hunter Portable 3rd HD` ISO is PS3 encrypted and `PSP ULJM-05800` was not available; direct extraction via `pycdlib` fails with encrypted `DATA.BIN` `0xd6e3…`). 60 monsters, 972 weapons, 1111 armor, 378 quests, 263 combines (202 Normal + 61 Alchemy). `item_sources` 2016 (`shop` 102 + `gather` 110 / `mining` 100 / `bug` 36 / `fish` 30 / `trade` 528 / `farm` 120 / `carve` 374 / `drop` 515 + `capture` 98 small-monster) with `buy_price` 150. `monster_drops` 1679 (756 base + 792 inferred +130 healed `Skypiercer/Dragongem/Mohran`→elder). `quest_rewards` 1867→1863. No `MHTri/MH3U` (0). `descriptions` 0 null (531 backfill) + `categories` 394.

MHW coverage: **1359 items** (World+Iceborne incl. 20 event/collab, Chest `sort_order` 1-1339 MHWorldData + 2000+ extras, 343 Fandom icons), **94 monsters** (Small 23 + Large 71 incl. variants Azure/Seething/Blackveil/Ruiner/Fatalis/Alatreon/Safi, 94 offline icons, species corrected), **3544 weapons** (14 types Great Sword→Bow incl. Charge Blade/Insect Glaive, Smith tree `sort_order` DFS, 8-color per-rarity icons White/Yellow/Green/Light Blue/Blue/Purple/Orange/Red), **5862 monster drops** (MHWorldData `monster_rewards.csv` 5680 +182 Fandom, `rank` Low/High/Master, `probability` %), `weapon_craft` 10056 / `weapon_materials` 9719. `monsters.sort_order` + `items.sort_order` give Chest/Small→Large sections; `weapons` no longer shows `All` (default `Great Sword`).

MH2G coverage: **1083 items fully sourced** (`item_sources` 12,751 rows: `gather/mining/bug/fish` from `maps.json`, `shop` consolidated 5 merchants, `trade` Veggie Elder + Trenya Boat + Pokke Points, `farm` Pokke Farm spots/trees, `small monsters` via `Monsters/monsters-material.json`, plus `monster_drops`/`quest_rewards`), **432 combine recipes** (147 Normal + 18 Alchemy + 7 Treasure) with `combine_type`/`chance` and game-book order (`ORDER BY item_combine.id`, ISO `Book of Combos` + `Alchemy Guide`). Categories re-derived from ISO `tmp_mhfu_upstream/items.json` `icon` + English verb: `Consumable 91 / Material 913 / Ammo 79` with `subcategory` (`Recovery, Buff, Food, Charm, Husk, Coating, Ore, Monster Material`, etc.; `Powercharm/Powertalon` → `Consumable • Charm`, `Huskberry/Sm Bone Husk` → `Ammo • Husk`).

## Game routing

Routes use `[game]` param with game IDs: `mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`. Game definitions live in `src/lib/stores/game.ts` (also persists selection to localStorage). Each `[game]/` sub-route covers: monsters, weapons, armor, quests, items, skills, builds. **Detail routes**: `[game]/monsters/[id]`, `[game]/weapons/[id]`, etc. **Items sub-routes**: `[game]/items` (list with `category • subcategory` + `Chest` game order) + `[game]/items/combine` (global combinations list, single view with `Normal/Alchemy/Treasure` badge/filter + `success %` + game-book order) + `[game]/items/[id]` (detail with clickable combine recipe `A x1 + B x1 = Result x1 • 90%`).

## Theming

Each game has a `GameTheme` object (in `src/lib/stores/game.ts`) with CSS custom properties: primary, accent, bg, border, glow, ornament type. Applied via inline `style` attribute on the main wrapper in `src/routes/+layout.svelte`. Components consume the variables (`var(--theme-primary)`, etc.) — never hardcode game-specific colors.

Themed components: `header.svelte`, `sidebar.svelte`, `card.svelte`, `detail-header.svelte`, list pages, detail pages, and the `.themed-bg`/`.themed-card` CSS utilities in `src/app.css`.

Ornament patterns (repeating-linear-gradient backgrounds): `medieval` (MHP2G), `japanese` (MHR, MHP3rd), `tribal` (MHW), `futuristic` (Wilds), `hunt` (generic).

## Tauri commands

Registered in `src-tauri/src/lib.rs` via `tauri::generate_handler!`. Defined in `src-tauri/src/commands/mod.rs`. All return `Result<T, String>`.

- List: `get_monsters`, `get_weapons`, `get_armor`, `get_quests`, `get_items`, `get_skills`, `get_combinations` — each takes `game_id: i32` (`get_combinations` returns `CombineView` with `combine_type`/`chance` + `components`, ordered by `item_combine.id` = Book order).
- Detail: `get_monster_detail`, `get_weapon_detail`, `get_armor_detail`, `get_quest_detail`, `get_item_detail` (now with `subcategory` + `combine_type`/`chance` in `recipes`), `get_skill_detail` — each takes `id: i32` and returns an Option<T> with joined material/source data.
- Legacy `greet` and the `get_games`/`Game` query were removed; game selection is a frontend `src/lib/stores/game.ts` registry keyed by `dbId`.
- `global_search`/`get_global_search` pushes the substring match into SQLite via the registered `norm_key` scalar function.

## Frontend API wrapper

`src/lib/api.ts` exports an `api` object with typed methods matching each Tauri command. Uses `@tauri-apps/api/core` `invoke()`. Types are duplicated between Rust (`#[derive(Serialize)]`) and TS. `src/lib/utils/norm.ts` mirrors the Rust `norm_key` for frontend list filtering (accent/case-insensitive).

## Gotchas

- **`.gitignore` exists and is correct** — it ignores `build/` (compiled frontend) and `src-tauri/target/` (Rust build artifacts). Don't commit generated files unintentionally.
- **Vite watch ignores `src-tauri/`** — configured in `vite.config.ts`. Rust changes won't trigger frontend HMR.
- **Language default is English** — all DB tables use `language TEXT DEFAULT 'en'`.
- **Seed is idempotent and NON-destructive** — uses `INSERT OR IGNORE` (now backed by UNIQUE indexes in `schema.rs`) + backfill UPDATEs. Runs every boot; **never DELETE** reference rows. Do NOT add count-based early-returns; rely on PK/UNIQUE conflicts for skip behavior. `clear_game` was removed.
- **FK constraints are ON** (`PRAGMA foreign_keys = ON` in `db/mod.rs`) — seed order matters (parents before children); don't insert orphan FKs. A `norm_key` SQL scalar function is registered for the global search.
- **Tailwind v4 syntax** — no `@apply` with custom config; uses `@theme` CSS variables. Don't look for a tailwind config file.
- **Rust lib crate** is compiled as `["lib", "cdylib", "staticlib"]` to support both desktop and future mobile targets.
- **FK constraints** — SQLite has FK enforcement on by default in some contexts. When inserting into junction tables, ensure parent rows exist first (seed order matters).
- **Themed components** must use `var(--theme-*)` not hardcoded colors. Reuse `.themed-card` and `.themed-bg` utility classes where possible.

## Git Workflow — STRICT: No auto-commit/push

> **⚠️ DO NOT commit, tag, push, or create PRs unless the user explicitly says so.**
>
> - **Default is NO git writes.** Just leave changes unstaged / uncommitted in the working tree. The user will explicitly say `commit`, `push`, `commit and push`, etc. when they want it.
> - This rule overrides any generic "commit when done" habits. When in doubt, **ask** instead of pushing.
> - `git status` / `git diff` / `git log` are always allowed (read-only). `git add` / `commit` / `push` / `tag` / `gh pr create` are **forbidden** without explicit user instruction.
> - Even when the user says "update the documentation" or "proceed", that does **not** imply commit/push unless they literally write `commit` or `push`.
> - **Pre-commit/push CI gate — MANDATORY when the user requests commit/push:** Before `git add/commit/push/tag`, **ensure GitHub CI will pass**. Run locally everything CI runs and fix findings before committing:
>   ```bash
>   npm run check   # svelte-check (also lint/typecheck — they are aliases)
>   cargo test --manifest-path src-tauri/Cargo.toml
>   cargo build --manifest-path src-tauri/Cargo.toml  # if the workflow builds
>   # + check .github/workflows/*.yml for extra jobs (release, tauri build) and run them if applicable
>   ```
>   If something fails, **fix it first**, re-run until green, and only then `commit/push`. If push already happened and CI fails, fix immediately with a new commit. Do not ask the user to act as CI.

## Release & Versioning

- **Version Sync**: When preparing a release, update the version string in both `package.json` and `src-tauri/tauri.conf.json`.
- **Git Tags**: Releases are triggered by pushing a semver git tag (e.g., `git tag v0.1.1 && git push origin v0.1.1`) — **only when explicitly requested**.
- **CI/CD**: GitHub Actions workflow (`.github/workflows/release.yml`) builds binaries for Windows, macOS, and Linux and publishes them to GitHub Releases automatically.

## Mobile Support (Android & iOS)

- **Tauri v2 Mobile**: Supported via Tauri mobile targets (`src-tauri/gen/`).
- **Android**:
  - Initialize: `npx tauri android init` (installs Rust NDK targets automatically).
  - Dev mode (emulator/device): `npx tauri android dev -t x86_64` (match emulator CPU architecture such as x86_64 or aarch64 to avoid SIGILL crashes).
  - Production build (APK/AAB): `npx tauri android build`.
  - Requirements: Android SDK/NDK, `ANDROID_HOME` env var, and Java JDK.
  - Vite HMR: Configured in `vite.config.ts` with `hmr` using `TAURI_DEV_HOST`.
- **iOS**: Initialize with `npx tauri ios init`, dev mode with `npx tauri ios dev`, build with `npx tauri ios build`. Requires macOS and Xcode.
