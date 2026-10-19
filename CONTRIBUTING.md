# Contributing

Thanks for considering a contribution. MH-AIO is an offline-first Tauri v2 encyclopedia
for Monster Hunter games, currently at v0.1.1 with four games at 100% (MH2G,
MHP3rd, MHW+Iceborne, MHR+Sunbreak) and one baseline (MH Wilds). The
canonical agent + arch doc is `AGENTS.md` at the repo root — read it first.

## How data is organized

```
src-tauri/data/<game-prefix>_*.json     # 8 games' static seed data (committed)
src-tauri/src/db/schema.rs              # CREATE TABLE IF NOT EXISTS + idempotency UNIQUE indexes
src-tauri/src/db/seed.rs                # INSERT OR IGNORE seed fns, one per table per game
src-tauri/src/db/queries.rs             # Generic queries (WHERE game_id = ?1)
src/lib/api.ts                          # Typed invoke() wrapper mirroring Rust #[derive(Serialize)]
src/lib/stores/game.ts                  # Game registry (5 games, per-game theme)
src/routes/[game]/...                   # List + detail views (shared by all 5 games)
spec/features/000-011/                  # Specs (audit/plan/tasks) per game or feature
scripts/                                # Dev-only scrapers + generators (gitignored, runtime JSONs committed)
```

ID offsets are per-game (e.g. `mhwilds` monsters 20001+, items 30001+,
weapons 40001+; `mhw` monsters + 100000, quests + 100000 to avoid
MH2G PK collision). See `spec/features/004-mhwilds-data-import/spec.md`
and `010-mhw-100/spec.md` for the per-game rules.

## Adding data (the common case)

1. **Pick the game's seed function** in `src-tauri/src/db/seed.rs`
   (e.g. `seed_mhp3rd_items`).
2. **Edit the matching JSON** in `src-tauri/data/<game>_<table>.json`
   (runtime files; the seed embeds them via `include_str!`).
3. **Add an idempotency guard** in `schema.rs` if the table doesn't
   already have a UNIQUE index. Existing tables all do; new tables need
   `uq_<table> <cols>` + a dedupe entry in `add_idempotency_constraints()`.
4. **Write a Rust test** in `queries.rs` asserting the new count
   (see `mhr_011a/b/c/d/e` for the canonical pattern: `let count = |sql|
-> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };` + a few
   spot asserts).
5. **Run the gate** before opening a PR (see below).

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
```

If any of these fail, fix the findings, re-run, and only then ask for
a commit. If a finding is pre-existing (e.g. the 4 transitive `npm
audit` brace-expansion/devalue/postcss/source-map-js warnings), mark
it as accepted and document in the PR body — don't fix unrelated
warnings in a feature PR.

## License

MIT. Code is original unless noted (Armor Set Search is a Rust port of
[Athena's A.S.S.](https://github.com/AthenaADP/MHFU-ASS) under MIT, see
`src-tauri/src/ass.rs` and the Credits section of `roadmap.md`). All
Monster Hunter game data is property of Capcom.
