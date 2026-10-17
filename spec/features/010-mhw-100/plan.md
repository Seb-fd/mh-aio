# 010 — Plan — MHW to 100% — DONE (as built)

## Approach (as built)

Phase 0 audited drift + shapes (zero drift; vocab maps; phantom proven). Phase A
emitted zero-scrape CSV wins (ailments/traps/charms/roster/bonuses/descriptions)
as curated JSON + seed fns with UPDATE backfills. Phase B crawled Kiranico
static HTML (95 monster pages + melder/provisions/decorations, cached,
throttled) for numerics/durations/costs/odds/proxy. Phase C parsed Fandom
api.php wikitext (How-To-Get gather, 4 descriptions, combine arbiter). Phase D
built the MHW rank-tree charms browser (deco acquisition reused the existing
block). Phase E banked three `mhw_010_*` tests + this spec + fidelity sync.

## Files touched

- Backend (`src-tauri/src/...`): `db/schema.rs` (2 tables, 4 columns, 2 UNIQUEs,
  1 index migration), `db/seed.rs` (5 new fns: ailments/tools/hunter-charms/
  melder-sources/fandom-gather + backfills in ailments/tools/weaknesses/decos/
  armor/quests/items + phantom DELETE + combine index note), `db/queries.rs`
  (notes fields, `HunterCharmView` + query, deco acquisition passthrough, 3 new
  tests), `commands/mod.rs` (`get_hunter_charms`), `lib.rs` (registration).
- Frontend (`src/...`): `lib/api.ts` (HunterCharm types, notes, command),
  `routes/[game]/charms/+page.svelte` (MHW rank-tree branch), monster detail
  (Tolerance + Diminishing columns), sidebar (`gameOnly` string|string[]).
- Data (`src-tauri/data/...`): 5 new JSONs + updates to ailments/tools/
  weaknesses/quests/sets/weapons/armor/monsters/items/decorations/extra-shops.
- Docs: this spec, `docs/fidelity-report.md` §MHW, `AGENTS.md` coverage line.

## Data / migrations

- `schema.rs`: new tables via `CREATE TABLE IF NOT EXISTS`; new columns via
  `add_column_if_missing`; `uq_item_combine` dropped + recreated as
  `uq_item_combine2` (wider key) so yield variants land on existing installs;
  seed re-inserts collapsed rows every boot.
- `seed.rs`: new fns called from the MHW orchestrator after their parents;
  `BEGIN IMMEDIATE` + `INSERT OR IGNORE` + JSON-truth UPDATE backfills.
- Ordering: no new ordering; families chain client-side via `previous_id`.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria: `SELECT COUNT(*) FROM
hunter_charms WHERE game_id = 1` (= 317); ailments/tools non-empty for MHW
  larges; `/mhw/charms` family chains with materials; deco detail shows
  melder/feystone text; item detail shows melder/shop/Guiding sources;
  double-seed diff (stable); generators re-run (converge).
