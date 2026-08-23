# AGENTS.md

## What this is

Tauri v2 desktop app (Monster Hunter encyclopedia). Rust backend + Svelte 5 frontend + SQLite. English-primary UI.

## Commands

```bash
npm run dev          # Frontend only (Vite, port 1420)
npx tauri dev        # Full app (frontend + Rust backend)
npx tauri build      # Production build
cargo build --manifest-path src-tauri/Cargo.toml  # Rust backend only
```

There are **no lint, typecheck, format, or test scripts** defined. `svelte-check` is installed but has no npm script.

## Architecture

- **SSR disabled** globally (`src/routes/+layout.ts` sets `export const ssr = false`). Routes also disable prerender. This is a pure client-side SPA served via Tauri WebView.
- **adapter-static** with `fallback: 'index.html'` — all routes are handled client-side.
- **Tailwind CSS v4** — uses `@import 'tailwindcss'` + `@theme` block in `src/app.css`. No `tailwind.config.js` exists.
- **shadcn-svelte** (bits-ui) — components in `src/lib/components/ui/`. Config in `components.json` defines aliases: `$lib/components`, `$lib/utils`, `$lib/hooks`.
- **cn() helper** — `src/lib/utils/index.ts` exports `cn()` (clsx + tailwind-merge). Use it for conditional classes.

## Database

SQLite via `rusqlite` with bundled feature. Schema defined in `src-tauri/src/db/schema.rs`. WAL mode enabled. DB path: `{app_data_dir}/mh-aio.db`. No migrations framework — schema is created directly via `CREATE TABLE IF NOT EXISTS`.

New tables go in `schema.rs`. New queries go in `db/queries.rs`. New Tauri commands go in `commands/mod.rs` and must be registered in `lib.rs`'s `invoke_handler`.

## Game routing

Routes use `[game]` param with game IDs: `mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`. Game definitions live in `src/lib/stores/game.ts` (also persists selection to localStorage). Each `[game]/` sub-route covers: monsters, weapons, armor, quests, items, skills, builds.

## Gotchas

- **No .gitignore** — `build/` (compiled frontend) and `src-tauri/target/` (Rust build artifacts) are tracked. Be careful not to commit generated files unintentionally.
- **Vite watch ignores `src-tauri/`** — configured in `vite.config.ts`. Rust changes won't trigger frontend HMR.
- **Language default is English** — all DB tables use `language TEXT DEFAULT 'en'`.
- **No scrapers implemented yet** — `scrapers/` directory is empty. Data import is planned but not built.
- **Tailwind v4 syntax** — no `@apply` with custom config; uses `@theme` CSS variables. Don't look for a tailwind config file.
- **Rust lib crate** is compiled as `["lib", "cdylib", "staticlib"]` to support both desktop and future mobile targets.
