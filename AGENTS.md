# AGENTS.md

## What this is

Tauri v2 desktop app (Monster Hunter encyclopedia). Rust backend + Svelte 5 frontend + SQLite. English-primary UI. Current focus: **Monster Hunter Freedom Unite (MHP2G / mh2g)** MVP with detail views and per-game theming.

## Commands

```bash
npm run dev          # Frontend only (Vite, port 1420)
npx tauri dev        # Full app (frontend + Rust backend)
npx tauri build      # Production build
cargo build --manifest-path src-tauri/Cargo.toml  # Rust backend only
npx svelte-check     # Type/Svelte validation
```

There are **no lint, typecheck, format, or test scripts** defined in `package.json`. `svelte-check` is available via npx.

## Architecture

- **SSR disabled** globally (`src/routes/+layout.ts` sets `export const ssr = false`). Routes also disable prerender. This is a pure client-side SPA served via Tauri WebView.
- **adapter-static** with `fallback: 'index.html'` — all routes are handled client-side.
- **Tailwind CSS v4** — uses `@import 'tailwindcss'` + `@theme` block in `src/app.css`. No `tailwind.config.js` exists.
- **shadcn-svelte** (bits-ui) — components in `src/lib/components/ui/`. Config in `components.json` defines aliases: `$lib/components`, `$lib/utils`, `$lib/hooks`.
- **cn() helper** — `src/lib/utils/index.ts` exports `cn()` (clsx + tailwind-merge). Use it for conditional classes.

## Database

SQLite via `rusqlite` with bundled feature. Schema defined in `src-tauri/src/db/schema.rs`. WAL mode enabled. DB path: `{app_data_dir}/mh-aio.db`. Schema created via `CREATE TABLE IF NOT EXISTS`.

**Migrations**: No framework. `apply_migrations()` in `schema.rs` uses `ALTER TABLE ... ADD COLUMN` with `pragma_table_info` check. New tables (e.g. `weapon_materials`, `armor_materials`, `item_combine`) are picked up automatically on next run since `CREATE TABLE IF NOT EXISTS` is idempotent.

**Seed**: `src-tauri/src/db/seed.rs` runs on startup. Uses `INSERT OR IGNORE` everywhere — fully idempotent for both fresh installs and upgrades from older schemas. `backfill_descriptions` and `backfill_costs` UPDATE existing rows that have NULL fields.

## Game routing

Routes use `[game]` param with game IDs: `mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`. Game definitions live in `src/lib/stores/game.ts` (also persists selection to localStorage). Each `[game]/` sub-route covers: monsters, weapons, armor, quests, items, skills, builds. **Detail routes**: `[game]/monsters/[id]`, `[game]/weapons/[id]`, etc.

## Theming

Each game has a `GameTheme` object (in `src/lib/stores/game.ts`) with CSS custom properties: primary, accent, bg, border, glow, ornament type. Applied via inline `style` attribute on the main wrapper in `src/routes/+layout.svelte`. Components consume the variables (`var(--theme-primary)`, etc.) — never hardcode game-specific colors.

Themed components: `header.svelte`, `sidebar.svelte`, `card.svelte`, `detail-header.svelte`, list pages, detail pages, and the `.themed-bg`/`.themed-card` CSS utilities in `src/app.css`.

Ornament patterns (repeating-linear-gradient backgrounds): `medieval` (MHP2G), `japanese` (MHR, MHP3rd), `tribal` (MHW), `futuristic` (Wilds), `hunt` (generic).

## Tauri commands

Registered in `src-tauri/src/lib.rs` via `tauri::generate_handler!`. Defined in `src-tauri/src/commands/mod.rs`. All return `Result<T, String>`.

- List: `get_games`, `get_monsters`, `get_weapons`, `get_armor`, `get_quests`, `get_items`, `get_skills` — each takes `game_id: i32`.
- Detail: `get_monster_detail`, `get_weapon_detail`, `get_armor_detail`, `get_quest_detail`, `get_item_detail`, `get_skill_detail` — each takes `id: i32` and returns an Option<T> with joined material/source data.
- Legacy: `greet(name: &str) -> String`.

## Frontend API wrapper

`src/lib/api.ts` exports an `api` object with typed methods matching each Tauri command. Uses `@tauri-apps/api/core` `invoke()`. Types are duplicated between Rust (`#[derive(Serialize)]`) and TS.

## Gotchas

- **No .gitignore** — `build/` (compiled frontend) and `src-tauri/target/` (Rust build artifacts) are tracked. Be careful not to commit generated files unintentionally.
- **Vite watch ignores `src-tauri/`** — configured in `vite.config.ts`. Rust changes won't trigger frontend HMR.
- **Language default is English** — all DB tables use `language TEXT DEFAULT 'en'`.
- **Seed is idempotent** — uses `INSERT OR IGNORE` + backfill UPDATEs. Safe to run on every app start. Do NOT add count-based early-returns; rely on PK conflicts for skip behavior.
- **Tailwind v4 syntax** — no `@apply` with custom config; uses `@theme` CSS variables. Don't look for a tailwind config file.
- **Rust lib crate** is compiled as `["lib", "cdylib", "staticlib"]` to support both desktop and future mobile targets.
- **FK constraints** — SQLite has FK enforcement on by default in some contexts. When inserting into junction tables, ensure parent rows exist first (seed order matters).
- **Themed components** must use `var(--theme-*)` not hardcoded colors. Reuse `.themed-card` and `.themed-bg` utility classes where possible.

## Release & Versioning

- **Version Sync**: When preparing a release, update the version string in both `package.json` and `src-tauri/tauri.conf.json`.
- **Git Tags**: Releases are triggered by pushing a semver git tag (e.g., `git tag v0.1.1 && git push origin v0.1.1`).
- **CI/CD**: GitHub Actions workflow (`.github/workflows/release.yml`) builds binaries for Windows, macOS, and Linux and publishes them to GitHub Releases automatically.
