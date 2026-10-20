# Tech Stack — mh-aio

Source of truth for agent behavior: root `AGENTS.md`. This file is an SDD summary — do not duplicate, only reference.

## Frontend

- Tauri v2 (Rust + WebView), Svelte 5 + TypeScript, SvelteKit client-side only (`src/routes/+layout.ts`: `ssr = false`), `adapter-static` with `fallback: 'index.html'`.
- Tailwind CSS v4 (`@import 'tailwindcss'` + `@theme` in `src/app.css`, no config file). Never use `@apply` with custom config.
- shadcn-svelte primitives in `src/lib/components/ui/` (plain Svelte 5). `cn()` in `src/lib/utils/index.ts`. `normKey()` in `src/lib/utils/norm.ts` mirrors Rust `norm_key` — keep in sync.
- Routes: `[game]` param (`mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`). Game registry + `GameTheme` in `src/lib/stores/game.ts` (localStorage persisted). Sub-routes: monsters, weapons, armor (+ `sets/[id]`), quests, items (+ `combine`, `[id]`), skills, decorations, builds, `charms/`, `comrades/`, `tools/` (MHW-only: `mantles`, `boosters`, `palico` — `gameOnly: 'mhw'`), `endemic/` (MHW + Wilds — `gameOnly: ['mhw', 'mhwilds']`).
- Theming via CSS vars (`var(--theme-*)`) + `.themed-bg` / `.themed-card`. Ornaments: `medieval` (MH2G), `japanese` (MHR, MHP3rd), `tribal` (MHW), `futuristic` (Wilds), `hunt` (generic). Never hardcode game colors.
- API wrapper `src/lib/api.ts` (typed `invoke()`). Types duplicated between Rust `Serialize` and TS.
- Sidebar hover-warm query registry in `src/lib/prefetch.ts` (`SECTION_JOBS`).

## Backend (Rust)

- Tauri v2 commands in `src-tauri/src/commands/mod.rs`, registered in `src-tauri/src/lib.rs` via `generate_handler!` (**43 commands**). All return `Result<T, String>`.
- Entity list (`game_id: i32`): `get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, `get_combinations`, `get_charms`, `get_hunter_charms`, `get_endemic_life`.
- Entity detail (`id: i32` → `Option<T>`): `get_monster_detail`, `get_weapon_detail`, `get_armor_detail`, `get_armor_set_detail`, `get_quest_detail`, `get_item_detail`, `get_skill_detail`, `get_decoration_detail`, `get_mhw_mantle_detail`, `get_palico_gadget_detail`.
- Game-exclusive lists: `get_mhw_mantles`, `get_palico_gadgets`, `get_melder_recipes`, `get_comrade_weapons`, `get_comrade_armor`, `get_comrade_skills`, `get_buddy_weapons`, `get_buddy_armor`.
- Solver + derived: `search_armor_sets` (ASS port), `get_monster_dedicated_sets` (≥40% material score).
- App-data families: `builds.rs` (`list_builds`, `save_build`, `get_build`, `delete_build`, `import_build`), `favorites.rs` (`list_favorites`, `toggle_favorite`, `remove_favorite`), `import.rs` (`preview_import`, `apply_import`).
- Search: `global_search` pushes the substring match into SQLite via the registered `norm_key` scalar.
- SQLite via `rusqlite` (bundled + functions, WAL, `PRAGMA foreign_keys = ON`). Schema in `src-tauri/src/db/schema.rs` (`CREATE TABLE IF NOT EXISTS` + `apply_migrations()` via `ALTER TABLE ... ADD COLUMN` + `schema_version` table). Idempotency via `add_idempotency_constraints()` (dedup + UNIQUE indexes) + `INSERT OR IGNORE` in `src-tauri/src/db/seed.rs` (`BEGIN IMMEDIATE`).
- **`DATA_VERSION` is dead** (`pub const DATA_VERSION: i32 = 1` vs live `schema_version = 2`) — the `seed()` boot gate is permanently closed on upgraded installs. All new data lands through named `patch_*` fns registered in `apply_data_patches()` and recorded in the `data_patches` table (27 rows).
- Seed order matters (parents before children). `crate-type = ["lib", "cdylib", "staticlib"]`.
- Solver: `src-tauri/src/ass.rs` (Athena's A.S.S. port).

## Commands

```bash
npm run dev                  # frontend only (Vite)
npx tauri dev                # full app
npx tauri build              # production bundle
cargo build --manifest-path src-tauri/Cargo.toml

npm run check                # svelte-check (aliases: lint, typecheck)
npm run lint:eslint          # ESLint
npm run format               # Prettier --write
npm run format:check         # Prettier --check (CI gate)
npm run version:check        # package.json / tauri.conf.json version sync
npm audit --audit-level=moderate

cargo test  --manifest-path src-tauri/Cargo.toml   # 48 Rust unit tests (ASS + db)
cargo fmt    --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

py scripts/gen_data_coverage.py            # regenerate docs/data-coverage.md
py scripts/gen_data_coverage.py --check    # CI drift gate for row counts
```

**There is no frontend unit-test runner** — frontend correctness is covered by `svelte-check` + ESLint; data correctness is covered by the Rust suite in `ass.rs` and `db::queries`. Vite watch ignores `src-tauri/`.

## Conventions for SDD plans

- New tables: rely on `CREATE TABLE IF NOT EXISTS` idempotency; new columns via `apply_migrations()` + `pragma_table_info` check.
- Junction rows: ensure UNIQUE index exists so `INSERT OR IGNORE` has a conflict target.
- Correcting an existing column needs an explicit `WHERE … IS NOT ?1`-guarded `UPDATE` in a patch — `INSERT OR IGNORE` never overwrites.
- Row counts in any doc must be copied from the generated `docs/data-coverage.md`, never typed by hand.
- `.gitignore` already covers `build/`, `src-tauri/target/`, `src-tauri/gen/`, `tmp_*`, `scripts/*` (except `check-version.js`). Never commit generated files.
