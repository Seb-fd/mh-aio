# 015 — MHW + Iceborne Endemic Life — DONE

## Context

- MHW + Iceborne baseline DONE (`010-mhw-100`: 1359 items, 94 monsters, 3544 weapons, 1595 armor, 521 quests, etc.; tests `010-mhw-100*` green).
- MHWilds got an Endemic Life section as part of `014-mhwilds-small-endemic` (70 entries in `endemic_life`, list-only `/mhwilds/endemic` route, sidebar entry `gameOnly: 'mhwilds'`).
- MHW has **no in-game "Endemic Life" category** in the original release. To provide parity with Wilds, this spec curates 33 small creatures + bug/ammo-yield pickups from Fandom MHW+Iberence + Kiranico. `kind` field uses **Slinger-ammo yield** for parity with Wilds (e.g. Vigorwasp → `"Recovery"`, Godbug → `"Mega Potion"`).

## Scope

### In scope

- A. **Data**: `src-tauri/data/mhw_endemic_life.json` (33 entries, IDs 90001-90033). Sources:
  - 14 small-monster pack creatures from `tmp/mhw_mat.json` / Fandom (Barnos, Gajau, Gastodon, Girros, Hornetaur, Jagras, Kelbi, Kestodon, Mernos, Mosswine, Noios, Raphinos, Shamos, Vespoid).
  - Bug family: Vigorwasp, Godbug, Bitterbug, Flashbug, Thunderbug, Insect, King Scarab, Scarab, Pillopod.
  - Slinger ammo origins: Scatternut, Screamer Pod, Bomb Pod, Thorn Pod, Stone, Redpit Vine, Woven Vine, Shiny Beetle.
  - Special: Boaboa Snowman, Gajalaka Trap.
- B. **Seed**: `seed_mhw_endemic_life` mirrors `seed_mhwilds_endemic_life` with `game_id = 1` (`MHW` constant). Registered in `seed()` orchestrator. `apply_data_patches` includes `run(conn, "mhw_015_endemic", seed_mhw_endemic_life)?;`.
- C. **Schema**: `add_idempotency_constraints` gains `"endemic_life"` in `content_tables` array → `CREATE UNIQUE INDEX IF NOT EXISTS uq_endemic_life ON endemic_life(game_id, id)`.
- D. **UI**: sidebar `gameOnly: ['mhw', 'mhwilds']`; route gate drops `isWilds`, replaces with `game?.id === 'mhw' || game?.id === 'mhwilds'`; empty-state hint widens to "Endemic Life data isn't seeded for this game yet."
- E. **Prefetch**: register `/endemic` in `SECTION_JOBS` so hover-warm works.
- F. **Test**: `mhw_015_endemic` in `queries.rs` asserts count ≥ 30 and zero rows with NULL description.
- G. **Docs**: AGENTS.md, roadmap.md, STATUS.md, docs/fidelity-report.md all updated.

### Out of scope (explicit)

- **Global search**: `endemic_life` not added to `get_global_search.TABLES` — keeps this change tight.
- **Favorites**: `FavoriteKind` does not get `'endemic'` — endemic remains list-only (parity with Wilds).
- **Detail page**: not added.
- **MHP3RD / MHRise / MH2G endemic**: not in scope; each would be its own spec.

## Acceptance criteria

- [x] `cargo test` reports 47/47 pass (new `mhw_015_endemic` test added).
- [x] Fresh DB shows 30+ rows in `endemic_life WHERE game_id = 1`; `/mhw/endemic` renders them with kind filter chips.
- [x] `/mhwilds/endemic` still shows 70 rows (no regression).
- [x] Sidebar shows Endemic Life for both MHW and MHWilds; absent for MHP3RD/MHR/MH2G.
- [x] Warm-boot (run `seed()` twice on same DB): second run is no-op, no schema delta.
- [x] Full CI gate green: `npm run check` 0/0, ESLint 0 errors, Prettier clean, `cargo fmt --check` clean, `cargo clippy -D warnings` clean, `cargo build` clean, `node scripts/check-version.js` OK, `npm audit` 0 vulns.

## Data contracts

- New JSON (committed): `mhw_endemic_life.json` — shape `{id, name, type, description}` per row (matches Wilds).
- New seed fn: `seed_mhw_endemic_life(conn)` — uses `include_str!("../../data/mhw_endemic_life.json")` + `INSERT OR IGNORE`.
- New patch: `mhw_015_endemic` (in `apply_data_patches` dispatcher).
- New UNIQUE index: `uq_endemic_life` on `endemic_life(game_id, id)`.
- ID offset: 90001-90033.
