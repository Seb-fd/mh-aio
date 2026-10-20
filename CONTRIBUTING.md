# Contributing

Thanks for considering a contribution. MH-AIO is an offline-first Tauri v2 encyclopedia
for Monster Hunter games, currently at v0.1.1 with all five titles seeded — MH2G,
MHP3rd, MHW+Iceborne and MHR+Sunbreak at 100%, plus MH Wilds core + quests, charms,
small monsters and endemic life. The canonical agent + arch doc is `AGENTS.md` at the
repo root — read it first. **All row counts live in the generated
[`docs/data-coverage.md`](docs/data-coverage.md)** — never hand-edit a number.

## How data is organized

```
src-tauri/data/<game-prefix>_*.json     # 5 games' static seed data (committed)
src-tauri/src/db/schema.rs              # CREATE TABLE IF NOT EXISTS + idempotency UNIQUE indexes
src-tauri/src/db/seed.rs                # INSERT OR IGNORE seed fns + 27 named data_patches
src-tauri/src/db/queries.rs             # Generic queries (WHERE game_id = ?1)
src/lib/api.ts                          # Typed invoke() wrapper (43 commands) mirroring Rust #[derive(Serialize)]
src/lib/stores/game.ts                  # Game registry (5 games, per-game theme)
src/routes/[game]/...                   # List + detail views (shared by all 5 games)
spec/constitution/                       # Current-state layer (mission / roadmap / tech-stack)
spec/features/000-015/                  # Specs (audit/plan/tasks) per game or feature
docs/data-coverage.md                   # GENERATED row counts — the source of truth
scripts/                                # Dev-only scrapers + generators (gitignored)
docs/fidelity-report.md                 # Retail-UMD audit (MH2G) + per-game catalog reports
```

ID offsets are per-game (e.g. `mhwilds` monsters 20001+, items 30001+,
weapons 40001+; `mhw` monsters + 100000, quests + 100000 to avoid
MH2G PK collision). See `spec/features/004-mhwilds-data-import/spec.md`
and `010-mhw-100/spec.md` for the per-game rules.

## Adding data (the common case)

> **Important:** `DATA_VERSION` is effectively dead. `seed.rs` declares `pub const DATA_VERSION: i32 = 1` but the live `schema_version` table holds `2`, so the `seed()` gate is permanently closed on upgraded installs. **A new `seed_*` call inside `seed()` will never run for an existing DB.** New data must land as a named patch in `apply_data_patches()`.

1. **Edit or create the JSON** in `src-tauri/data/<game>_<table>.json` (the seed embeds it via `include_str!`).
2. **Add a `patch_*` fn** in `src-tauri/src/db/seed.rs` that reads the JSON and does an idempotent
   `INSERT OR IGNORE`. Guard it with `patch_applied()` at the top and `record_patch()` at
   the end — that's how every warm-boot backfill works. (Fresh installs still run `seed()`
   once and then the same patches on the next boot.)
3. **Register it** as a `run(conn, "<name>", <your_fn>)?;` line in `apply_data_patches()`.
   Bump the `data_patches` count assertion in `data_patches_backfill_without_full_reseed`
   in `queries.rs` (currently `>= 26`).
4. **Add an idempotency guard** in `schema.rs` if the table doesn't already have a UNIQUE
   index. Every content table has a `(game_id, id)` UNIQUE in `add_idempotency_constraints()`;
   new tables need to be added to that list plus a dedupe entry.
5. **If you're correcting an existing column**, remember `INSERT OR IGNORE` never overwrites
   an existing value — use a `WHERE … IS NOT ?1`-guarded backfill `UPDATE` instead (see
   `patch_mhw_015_small_size` for the pattern).
6. **Write a Rust test** in `queries.rs` asserting the new count (see `mhr_011a/b/c/d/e` for
   the canonical pattern: `let count = |sql| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };`
   plus a few spot asserts).
7. **Regenerate the coverage doc**: `py scripts/gen_data_coverage.py`.
8. **Run the gate** before opening a PR (see below).

## Updating documentation

- Every row count in `README.md`, `STATUS.md`, `roadmap.md`, `AGENTS.md`,
  `docs/fidelity-report.md` and `spec/constitution/*` must match the generated
  `docs/data-coverage.md`. Regenerate it, then copy figures from it — don't retype them.
- `py scripts/gen_data_coverage.py --check` fails if the committed doc is stale.
- Specs under `spec/features/` are **historical records**. When a number in one is now
  wrong, add a `> **Superseded:**` banner pointing at the corrected value and the spec
  that owns it — don't rewrite the body.
- `spec/constitution/*` IS the current-state layer: rewrite it freely.
- `npm run format` must be run after editing any `.md` (Prettier checks all of them,
  including `docs/`, `spec/`, `.opencode/agents/`, `README.md`, `AGENTS.md`).

## Adding a new game

That's its own spec. See `spec/features/000-mh2g-baseline/spec.md` (the
retro-anchor) and `010-mhw-100/spec.md` / `011-mhr-100/spec.md` (the
two 100% completeness passes) for the pattern: `audit + proofs →
phases A-E → verify_mh*.py → patch_mh* → docs + full gate → commit`.

## Adding a new feature

Same idea, scoped smaller. See `005-builds-save-load-export`,
`006-favorites-system`, `007-import-panel-json-csv` for the
builds/favorites/import patterns. Each has its own spec, its own Tauri
commands, and (for builds/favorites) an `app_data_dir` storage choice
documented inline.

## Pull requests

- The CI gate is the source of truth: `npm run check`, `npm run
lint:eslint --if-present`, `npm run format:check --if-present`,
  `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo
clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`, `cargo
test --manifest-path src-tauri/Cargo.toml --locked`, `cargo build
--manifest-path src-tauri/Cargo.toml`, `node scripts/check-version.js`,
  `npm audit --audit-level=moderate`.
- Open a spec under `spec/features/<NNN>-<slug>/{spec,plan,tasks}.md`
  for any non-trivial change. Spec is the design review.
- A single phase per commit keeps the history reviewable.
- Number your spec from the next free integer. Specs `012`–`014` shipped
  with no `spec/features/` folder (Wilds quests / charms / small+endemic) —
  if you touch that work, add the retro-spec before extending it.

## Commit / push / release policy (strict)

- **No auto-commit, no auto-push, no auto-tag, no auto-PR.** Maintainer
  explicitly types `commit`, `commit and push`, `push`, or `tag` for
  each. This is intentional: the project is under active refactor and
  the spec is the contract, not the diff.
- Pre-push CI gate is mandatory — run the gate locally (see below) and
  fix findings _before_ requesting `commit`/`push`.
- `scripts/*` is gitignored. Runtime JSONs in `src-tauri/data/` are
  committed (they're the seed).
- `tmp_*/` and `src-tauri/gen/` are gitignored (regenerable caches;
  Tauri mobile generated).
- Markdown files are checked by Prettier — always run `npm run format`
  after editing `*.md` under `docs/`, `spec/`, `roadmap.md`, or
  `.opencode/agents/`.

## Local gate (mirrors CI)

```bash
# Frontend
npm run check
npm run lint:eslint --if-present
npm run format:check --if-present

# Rust
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --locked
cargo build --manifest-path src-tauri/Cargo.toml

# Version + audit
node scripts/check-version.js
npm audit --audit-level=moderate

# Docs (row counts must match the generated coverage doc)
py scripts/gen_data_coverage.py --check
```

If any of these fail, fix the findings, re-run, and only then ask for a commit.
`npm audit` currently reports **0 vulnerabilities** — the four transitive advisories
(`brace-expansion`, `devalue`, `postcss-selector-parser`, `source-map-js`) were
resolved with pinned `overrides` in `package.json`. If a new advisory appears, fix it
in the same PR rather than documenting it as accepted; do not fix unrelated
pre-existing warnings inside a feature PR.

## License

MIT. Code is original unless noted (Armor Set Search is a Rust port of
[Athena's A.S.S.](https://github.com/AthenaADP/MHFU-ASS) under MIT, see
`src-tauri/src/ass.rs` and the Credits section of `roadmap.md`). All
Monster Hunter game data is property of Capcom.
