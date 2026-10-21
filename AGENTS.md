# AGENTS.md

## What this is

Tauri v2 desktop app (Monster Hunter encyclopedia). Rust backend + Svelte 5 frontend + SQLite. English-primary UI. **Monster Hunter Freedom Unite (MHP2G / `mh2g`, DB id 5)**, **MHP3rd (MHP3rd / `mhp3rd`, DB id 4)**, and **MH Rise + Sunbreak (`mhr`, DB id 2)** are the 100% complete games (specs `000`, `001`, `011`). **MHW + Iceborne (`mhw`, DB id 1)** is 100% via `010-mhw-100`.

## Commands

```bash
npm run dev                 # Frontend only (Vite, port 1420)
npx tauri dev               # Full app (frontend + Rust backend)
npx tauri build             # Production build
cargo build --manifest-path src-tauri/Cargo.toml  # Rust backend only
npm run check / lint / typecheck  # svelte-check (all three aliases)
npm run lint:eslint         # ESLint (CI runs `lint:eslint --if-present`)
npm run format              # Prettier --write (fix formatting locally)
npm run format:check        # Prettier --check (CI runs this; must pass)
cargo test --manifest-path src-tauri/Cargo.toml   # Rust unit tests (ASS + db)
```

`package.json` defines `check`, `check:watch`, `lint` and `typecheck` (all wrapping `svelte-check`), plus `format` / `format:check` (Prettier) and `lint:eslint`. There is **no frontend unit-test runner**; the Rust suites in `ass.rs` and `db::queries` are exercised by `cargo test`. Prettier config lives in `.prettierrc` (`semi: false`, `singleQuote: true`, `printWidth: 100`); CI checks **all** non-ignored files including `*.md` (`.opencode/agents/`, `docs/`, `spec/`, `roadmap.md`).

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

**Seed**: `src-tauri/src/db/seed.rs` runs on startup, in `BEGIN IMMEDIATE` + `INSERT OR IGNORE` — fully idempotent. Current **MHP3rd (mhp3rd, game_id 4)**: 1065 items (Material 964 / Consumable 55 / Ammo 46; descriptions verified real JP, not mojibake; item 10526 kept as unresolved ticket placeholder) updated with `mikejsavage/MHP3DB` (`DATA.BIN` decrypted ULJM-05800, 813 canonical items with `value/rarity/icon` + `MHP3DB` as ISO proxy — the `Monster Hunter Portable 3rd HD` ISO is PS3 encrypted and `PSP ULJM-05800` was not available; direct extraction via `pycdlib` fails with encrypted `DATA.BIN` `0xd6e3…`). 60 monsters (40 Large + 20 Small; all with drops; 200 xeno rows pruned), 972 weapons (incl. missing Ludroth Splashammer + element/status from mhp3db.github.io), 1106 armor (345 sets; 5 junk rows removed), 378 quests (44 wiki-verified key flags), 398 combines (276 Normal + 122 Alchemy). `monster_drops` 2200. `quest_rewards` 1864 (incl. 6 S-rank rows). `item_sources` 4099. Weaknesses rewritten from mhp3wiki hitzones (40/40 larges, 213 rows). New: trap tools (180 rows), 50,635 excavated charms + Charms browser (skill search), 92/154/52 comrade gear + Comrades browser. No `MHTri/MH3U` (0).

MHW coverage: **1359 items** (World+Iceborne incl. 20 event/collab, Chest `sort_order` 1-1339 MHWorldData

- 2000+ extras, 343 Fandom icons; `Material 1082 / Consumable 129 / Misc 106 / Ammo 41 / Hidden 1`), **94 monsters** (23 Small + 71 Large incl. variants
  Azure/Seething/Blackveil/Ruiner/Fatalis/Alatreon/Safi, 94 offline icons, species corrected; `size` backfilled from the committed JSON by `patch_mhw_015_small_size` because `INSERT OR IGNORE` never corrects it), **3544 weapons** (14
  types Great Sword→Bow incl. Charge Blade/Insect Glaive, Smith tree `sort_order` DFS, 12-HEX per-rarity icons r1..r12),
  **5826 monster drops** (MHWorldData `monster_rewards.csv` 5680 +182 Fandom −1 phantom −35 xeno, `rank` Low/High/Master,
  `probability` % with 116 honest NULL unknowns + 32 Kiranico-filled; `uq_monster_drops2` keeps stack variants), `weapon_craft` 10056 / `weapon_materials` 9719, `item_sources` 6821, **1595 armor** (351 sets) / 5887 mats, **521 quests**
  / 3675 rewards, **211 Melder** / **20 Mantles-Boosters** / **8 Palico Gadgets**. 100% pass (spec `010-mhw-100`):
  ailments **721** + traps **465** (Kiranico numbers/chains) + stagger proxy **317**, **317 hunter charms** (rank tree
  I→V + browser), quest roster **517/521**, set bonuses +58 skill2, descriptions **3544+1595+94**, deco acquisition
  **398/404** (melder + feystone; materials N/A), melder sources **212**, shop **56**, Hoarfrost/Guiding gather **75**,
  combine **186** (wiki 98 covered; `uq_item_combine2` keeps yield variants), skills **179** / 419 levels. `monsters.sort_order` +
  `items.sort_order` give Chest/Small→Large sections; `weapons` no longer shows `All` (default `Great Sword`).
  **015-mhw-endemic:** **33 curated endemic_life rows** (8 bugs incl. Vigorwasp/Flashbug/Thunderbug/Godbug + 23 small-monster packs + 2 special Boaboa Snowman/Gajalaka Trap) — `kind` is the Slinger-ammo yield for parity with Wilds. UI shared with `/mhwilds/endemic`. `uq_endemic_life_game_id` UNIQUE index added in `add_idempotency_constraints`.

MHR coverage: **1668 items** (`Material 1497 / Consumable 80 / Ammo 65 / Decoration 26`), **3953 weapons** (14 types, Smith sort), **1591 armor** (410 sets, `bonus_skill` 0/410 non-null — Rise has no set bonuses), **112 monsters** (78 Large + 34 Small), **946 quests** / 6426 rewards, **147 skills** / 459 levels, **243 decorations**, **6403 monster drops**, **624 weakness rows**. 100% pass (spec `011-mhr-100`): **1120 ailment rows** + `monster_part_break` + **270 trap rows**, buddy gear **322 Palico weapons / 362 Palamute armor**, rampage **247 skills / 26 decos / 6822 weapon links**, **11,849 talisman** generator rows + `charm_skills` 23,249, **178 combines** (`item-mix` → `item_combine`, chance 100).

MHWilds coverage: **773 items** (Material, descriptions), **1188 weapons** (14 types), **714 armor** (183 sets), **68 monsters** (34 Large + 34 Small with `size='small'`), **179 skills** / 442 levels, **361 decorations**, **1399 drops**, **324 weakness rows**, **120 combines**, **174 quests** (Optional 81 / Event 60 / Assignment 33), **76 `wilds_talisman`**, **70 endemic_life rows**. Specs `012`–`014` (shipped; no spec folders recorded). Documented gap: no `quest_rewards` (Fextra exposes reward text, not per-item counts).

MH2G coverage: **1244 items** (1083 fully sourced + 161 upstream books/ores/fish/meats/insects/seeds/herbs/mushrooms/eggs with rare/price/desc/carry; buy prices from merchants) (`item_sources` 15,671 rows: `gather/mining/bug/fish` from `maps.json`, `shop` consolidated 5 merchants, `trade` Veggie Elder + Trenya Boat + Pokke Points, `farm` Pokke Farm spots/trees, `small monsters` via `Monsters/monsters-material.json`, plus `monster_drops`/`quest_rewards`), **432 combine recipes** (379 Normal + 39 Alchemy + 14 Treasure) with `combine_type`/`chance` and game-book order (`ORDER BY item_combine.id`, ISO `Book of Combos` + `Alchemy Guide`). Categories re-derived from ISO `tmp_mhfu_upstream/items.json` `icon` + English verb: `Consumable 128 / Material 1037 / Ammo 79` with `subcategory` (`Recovery, Buff, Food, Charm, Husk, Coating, Ore, Monster Material`, etc.; `Powercharm/Powertalon` → `Consumable • Charm`, `Huskberry/Sm Bone Husk` → `Ammo • Husk`). 2075 armor (348 sets), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons, 610 quests, 99 skills / 214 abilities, 192 decorations, 3353 drops.

## Data

`docs/data-coverage.md` is the **generated source of truth** for every row count (per-table × per-game matrix). Regenerate with `py scripts/gen_data_coverage.py`; verify in CI with `--check`. Never hand-edit a count in `README.md` / `STATUS.md` / `roadmap.md` / `AGENTS.md` / `docs/fidelity-report.md` / `spec/constitution/*` — copy it from the generated doc. Specs under `spec/features/` are historical records; add a `> **Superseded:**` banner rather than rewriting their numbers.

## Game routing

Routes use `[game]` param with game IDs: `mhw`, `mhr`, `mhwilds`, `mhp3rd`, `mh2g`. Game definitions live in `src/lib/stores/game.ts` (also persists selection to localStorage). Each `[game]/` sub-route covers: monsters, weapons, armor, quests, items, skills, builds (+ `charms`, `comrades` for `mhp3rd`; `tools` for `mhw`; `endemic` for `mhw`+`mhwilds`). **Detail routes**: `[game]/monsters/[id]`, `[game]/weapons/[id]`, etc. **Items sub-routes**: `[game]/items` (list with `category • subcategory` + `Chest` game order) + `[game]/items/combine` (global combinations list, single view with `Normal/Alchemy/Treasure` badge/filter + `success %` + game-book order) + `[game]/items/[id]` (detail with clickable combine recipe `A x1 + B x1 = Result x1 • 90%`).

## Theming

Each game has a `GameTheme` object (in `src/lib/stores/game.ts`) with CSS custom properties: primary, accent, bg, border, glow, ornament type. Applied via inline `style` attribute on the main wrapper in `src/routes/+layout.svelte`. Components consume the variables (`var(--theme-primary)`, etc.) — never hardcode game-specific colors.

Themed components: `header.svelte`, `sidebar.svelte`, `card.svelte`, `detail-header.svelte`, list pages, detail pages, and the `.themed-bg`/`.themed-card` CSS utilities in `src/app.css`.

Ornament patterns (repeating-linear-gradient backgrounds): `medieval` (MHP2G), `japanese` (MHR, MHP3rd), `tribal` (MHW), `futuristic` (Wilds), `hunt` (generic).

## Tauri commands

Registered in `src-tauri/src/lib.rs` via `tauri::generate_handler!` (**43 commands**). Defined in `src-tauri/src/commands/mod.rs`. All return `Result<T, String>`.

- **Entity list** (`game_id: i32`): `get_monsters`, `get_weapons`, `get_armor`, `get_armor_sets`, `get_quests`, `get_items`, `get_skills`, `get_decorations`, `get_combinations` (`CombineView` with `combine_type`/`chance` + `components`, ordered by `item_combine.id` = Book order), `get_charms`, `get_hunter_charms`, `get_endemic_life`.
- **Entity detail** (`id: i32` → `Option<T>`): `get_monster_detail`, `get_weapon_detail`, `get_armor_detail`, `get_armor_set_detail`, `get_quest_detail`, `get_item_detail` (with `subcategory` + `combine_type`/`chance` in `recipes`), `get_skill_detail`, `get_decoration_detail`, `get_mhw_mantle_detail`, `get_palico_gadget_detail`.
- **Game-exclusive lists**: `get_mhw_mantles`, `get_palico_gadgets`, `get_melder_recipes`, `get_comrade_weapons`, `get_comrade_armor`, `get_comrade_skills`, `get_buddy_weapons`, `get_buddy_armor`.
- **Solver + derived**: `search_armor_sets` (ASS port), `get_monster_dedicated_sets` (≥40% material score).
- **Builds** (`builds.rs`, `app_data_dir/builds.json`): `list_builds`, `save_build`, `get_build`, `delete_build`, `import_build`.
- **Favorites** (`favorites.rs`, `app_data_dir/favorites.json`): `list_favorites`, `toggle_favorite`, `remove_favorite`.
- **Import** (`import.rs`): `preview_import`, `apply_import` (dry-run preview + transactional idempotent apply).
- **Search**: `global_search` pushes the substring match into SQLite via the registered `norm_key` scalar function.
- Legacy `greet` and the `get_games`/`Game` query were removed; game selection is a frontend `src/lib/stores/game.ts` registry keyed by `dbId`.

## Frontend API wrapper

`src/lib/api.ts` exports an `api` object with typed methods matching each Tauri command. Uses `@tauri-apps/api/core` `invoke()`. Types are duplicated between Rust (`#[derive(Serialize)]`) and TS. `src/lib/utils/norm.ts` mirrors the Rust `norm_key` for frontend list filtering (accent/case-insensitive).

## Gotchas

- **`.gitignore` exists and is correct** — it ignores `build/` (compiled frontend) and `src-tauri/target/` (Rust build artifacts). Don't commit generated files unintentionally.
- **Vite watch ignores `src-tauri/`** — configured in `vite.config.ts`. Rust changes won't trigger frontend HMR.
- **Language default is English** — all DB tables use `language TEXT DEFAULT 'en'`.
- **Seed is idempotent and NON-destructive** — uses `INSERT OR IGNORE` (now backed by UNIQUE indexes in `schema.rs`) + backfill UPDATEs. Runs every boot; **never DELETE** reference rows. Do NOT add count-based early-returns; rely on PK/UNIQUE conflicts for skip behavior. `clear_game` was removed.
- **`DATA_VERSION` is effectively dead — use `apply_data_patches()` for all new data.** `seed.rs` declares `pub const DATA_VERSION: i32 = 1`, but the live `schema_version` table holds `2`, so the `seed()` gate (`if current >= DATA_VERSION { return Ok(()) }`) is permanently closed on upgraded installs. **A new `seed_*` call inside `seed()` will never run for anyone who already has a DB.** Add a named `patch_*` fn + a `run(conn, "<name>", …)` line in `apply_data_patches()` instead; it self-guards via `patch_applied()`/`record_patch()` on the `data_patches` table (currently 26 rows). Fresh installs still run `seed()` once and then the same patches on the next boot.
- **INSERT OR IGNORE never corrects an existing column** — it only skips PK/UNIQUE conflicts. If a JSON field changed after users seeded, add a `WHERE … IS NOT ?1`-guarded backfill `UPDATE` in a patch (see `patch_mhw_015_small_size` for the canonical pattern: 13 MHW monsters had `size='Large'` because an older seed path inserted them first).
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
>   npm run lint:eslint --if-present  # ESLint (CI job `ESLint`)
>   npm run format:check --if-present # Prettier (CI job `Prettier check`; fix with `npm run format`)
>   cargo fmt --manifest-path src-tauri/Cargo.toml --check  # Rustfmt (CI job `Rust fmt check`)
>   cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings  # Clippy (CI job `Cargo clippy`)
>   cargo test --manifest-path src-tauri/Cargo.toml   # Rust unit tests (ASS + db)
>   cargo build --manifest-path src-tauri/Cargo.toml  # if the workflow builds
>   node scripts/check-version.js  # version sync (CI job `Version sync check`)
>   npm audit --audit-level=moderate  # CI job `npm audit` (needs npm 11 bulk endpoint + lockfile in sync; if `Invalid package tree`, run `npm install --package-lock-only` and commit `package-lock.json`)
>   # + check .github/workflows/*.yml for extra jobs (release, tauri build) and run them if applicable
>   ```
>   If something fails, **fix it first**, re-run until green, and only then `commit/push`. If push already happened and CI fails, fix immediately with a new commit. Do not ask the user to act as CI. **Markdown files are checked too** — always run `npm run format` after editing `.opencode/agents/*.md`, `docs/**`, `spec/**` or `roadmap.md`.

## Release & Versioning

- **Version lives in FOUR places** — all must agree before tagging:
  1. `package.json` → `"version"`
  2. `src-tauri/tauri.conf.json` → `"version"`
  3. `src-tauri/Cargo.toml` → `version = "..."`
  4. `src-tauri/Cargo.lock` → `name = "mh-aio"` / `version = "..."` (regenerated by `cargo build`)

  `node scripts/check-version.js` enforces 1–3 (the `Version sync check` CI job). It does **not**
  check `Cargo.lock` — verify that manually after bumping.

- **Git Tags**: releases are triggered by pushing a semver git tag **with the `v` prefix**
  (`git tag -a v0.2.0 -m "MH-AIO v0.2.0" && git push origin v0.2.0`) — **only when explicitly
  requested**. `release.yml` triggers on `'v*'`; a bare `0.2.0` tag will **not** build anything.
- **Release notes**: `CHANGELOG.md` is the canonical long-form record. `release.yml` inlines a
  curated `releaseBody` (duplicated highlights — keep them in sync) and sets
  `generateReleaseNotes: true`, which **prepends** `releaseBody` to GitHub's auto-generated commit
  notes.
- **CI/CD**: `.github/workflows/release.yml` builds 5 targets and publishes to GitHub Releases:
  Windows x64 (`.exe` + `.msi`), macOS arm64 + macOS x64 (`.dmg` + `.app.tar.gz`), Linux x64 and
  Linux arm64 (`.deb` + `.rpm` + `.AppImage`). `fail-fast: false` — **inspect every matrix job**,
  not just the run conclusion.
- **Tag only on green CI.** Push the version-bump commit, wait for the `master` CI run to succeed,
  _then_ tag. A red build should never get a tag.

### Signing status

- **macOS — ad-hoc signed.** `src-tauri/tauri.conf.json` sets
  `bundle.macOS.signingIdentity = "-"`. This is free (no Apple account) and stops Gatekeeper
  reporting downloaded Apple Silicon builds as "damaged", which 0.1.0 did. It is **not notarized**:
  users must allow the app once in System Settings → Privacy & Security.
  Upgrading to a real Developer ID requires a paid Apple Developer account (only the Account
  Holder can create a Developer ID Application cert) plus these secrets: `APPLE_ID`,
  `APPLE_PASSWORD`, `APPLE_CERTIFICATE` (base64 `.p12`), `APPLE_CERTIFICATE_PASSWORD`,
  `KEYCHAIN_PASSWORD`. **Notarization is mandatory once you use a Developer ID certificate.**
- **Windows — unsigned.** `bundle.windows` has no `certificateThumbprint` / `digestAlgorithm` /
  `timestampUrl`, and no `signCommand`. SmartScreen will warn on download. Note that since
  Microsoft's 2024 Trusted Root change, EV certificates no longer bypass the warning on a first
  release — signing mainly builds long-term reputation. To sign: add the three `bundle.windows`
  keys, provide `WINDOWS_CERTIFICATE` (base64 `.pfx`) + `WINDOWS_CERTIFICATE_PASSWORD` secrets, and
  add the `certutil -decode` + `Import-PfxCertificate` step to the workflow.
- **No auto-updater.** The Tauri updater plugin is **not** configured, so no `latest.json` manifest
  is published and signing keys are only relevant if that ever changes. `tauri signer sign` is not
  used in this project.
- `docs/data-coverage.md` is generated and excluded from Prettier in `.prettierignore` — its gate
  is `py scripts/gen_data_coverage.py --check`, not `prettier --check`.

## Mobile Support (Android & iOS)

- **Tauri v2 Mobile**: Supported via Tauri mobile targets (`src-tauri/gen/`).
- **Android**:
  - Initialize: `npx tauri android init` (installs Rust NDK targets automatically).
  - Dev mode (emulator/device): `npx tauri android dev -t x86_64` (match emulator CPU architecture such as x86_64 or aarch64 to avoid SIGILL crashes).
  - Production build (APK/AAB): `npx tauri android build`.
  - Requirements: Android SDK/NDK, `ANDROID_HOME` env var, and Java JDK.
  - Vite HMR: Configured in `vite.config.ts` with `hmr` using `TAURI_DEV_HOST`.
- **iOS**: Initialize with `npx tauri ios init`, dev mode with `npx tauri ios dev`, build with `npx tauri ios build`. Requires macOS and Xcode.
