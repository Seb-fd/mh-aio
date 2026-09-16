# Tech Stack — mh-aio

Source of truth for agent behavior: root `AGENTS.md`. This file is an SDD summary — do not duplicate, only reference.

## Frontend

- Tauri v2 (Rust + WebView), Svelte 5 + TypeScript, SvelteKit client-side only (`src/routes/+layout.ts`: `ssr = false`), `adapter-static` with `fallback: 'index.html'`.
- Tailwind CSS v4 (`@import 'tailwindcss'` + `@theme` in `src/app.css`, no config file). Never use `@apply` with custom config.
- shadcn-svelte primitives in `src/lib/components/ui/` (plain Svelte 5). `cn()` in `src/lib/utils/index.ts`. `normKey()` in `src/lib/utils/norm.ts` mirrors Rust `norm_key` — keep in sync.
- Routes: `[game]` param (`mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`). Game registry + `GameTheme` in `src/lib/stores/game.ts` (localStorage persisted). Sub-routes: monsters, weapons, armor (+ `sets/[id]`), quests, items (+ `combine`, `[id]`), skills, decorations, builds, `tools/` (MHW-only: `mantles`, `boosters`, `palico` — `gameOnly: 'mhw'`).
- Theming via CSS vars (`var(--theme-*)`) + `.themed-bg` / `.themed-card`. Ornaments: `medieval` (MH2G), `japanese` (MHR, MHP3rd), `tribal` (MHW), `futuristic` (Wilds), `hunt` (generic). Never hardcode game colors.
- API wrapper `src/lib/api.ts` (typed `invoke()`). Types duplicated between Rust `Serialize` and TS.

## Backend (Rust)

- Tauri v2 commands in `src-tauri/src/commands/mod.rs`, registered in `src-tauri/src/lib.rs` via `generate_handler!`. All return `Result<T, String>`.
- List: `get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, `get_combinations` (take `game_id: i32`) + MHW tools `get_melder_recipes`, `get_mhw_mantles`, `get_palico_gadgets`. Detail: `get_monster_detail` (+ `get_monster_dedicated_sets`), `get_weapon_detail`, `get_armor_detail` (+ `get_armor_set_detail`), `get_quest_detail`, `get_item_detail`, `get_skill_detail`, `get_decoration_detail`, `get_mhw_mantle_detail`, `get_palico_gadget_detail` (take `id: i32`). `search_armor_sets` (ASS) + `global_search` (pushes substring match into SQLite via `norm_key`).
- SQLite via `rusqlite` (bundled + functions, WAL, `PRAGMA foreign_keys = ON`). Schema in `src-tauri/src/db/schema.rs` (`CREATE TABLE IF NOT EXISTS` + `apply_migrations()` via `ALTER TABLE ... ADD COLUMN` + `schema_version` table). Idempotency via `add_idempotency_constraints()` (dedup + UNIQUE indexes) + `INSERT OR IGNORE` in `src-tauri/src/db/seed.rs` (`BEGIN IMMEDIATE`).
- Seed order matters (parents before children). `crate-type = ["lib", "cdylib", "staticlib"]`.
- Solver: `src-tauri/src/ass.rs` (Athena's A.S.S. port).

## Commands

```bash
npm run dev
npx tauri dev
npx tauri build
cargo build --manifest-path src-tauri/Cargo.toml
npm run check   # also lint / typecheck (svelte-check aliases)
cargo test --manifest-path src-tauri/Cargo.toml
```

No dedicated format or frontend unit-test runner. Vite watch ignores `src-tauri/`.

## Conventions for SDD plans

- New tables: rely on `CREATE TABLE IF NOT EXISTS` idempotency; new columns via `apply_migrations()` + `pragma_table_info` check.
- Junction rows: ensure UNIQUE index exists so `INSERT OR IGNORE` has a conflict target.
- `.gitignore` already covers `build/`, `src-tauri/target/`, `tmp_*`, `scripts/*` (except `check-version.js`). Never commit generated files.
