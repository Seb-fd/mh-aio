# 015 — Tasks — MHW + Iceborne Endemic Life

## Phase 0 — Audit + proofs

- [x] T01 — Confirmed `endemic_life` table is game-agnostic; MHWilds uses `game_id = 3`, MHW space `game_id = 1` is free.
- [x] T02 — Confirmed sidebar/route currently gate on `gameOnly: 'mhwilds'` / `isWilds`. Simple widening to MHW.
- [x] T03 — Spec/plan/tasks skeleton.

## Phase A — Data + JSON

- [x] TA1 — Curated 35 MHW endemic entries (IDs 90001-90035): 14 small-monster packs, 9 bugs, 8 Slinger-pickups, 2 special, 2 misc. Sourced from Fandom MHW+Iberence wiki + Kiranico (already cached in `tmp_mhw_100/`).
- [x] TA2 — Wrote `src-tauri/data/mhw_endemic_life.json` with `{id, name, type, description}` per row. `kind` = Slinger-ammo yield for parity with Wilds.
- [x] TA3 — Wrote `scripts/generate_mhw_endemic.py` (idempotent emitter that regenerates the JSON from a hand-curated inline data block).

## Phase C — Schema + seed

- [x] TC1 — Added `seed_mhw_endemic_life(conn)` in `src-tauri/src/db/seed.rs` mirroring the Wilds fn with `MHW = 1`.
- [x] TC2 — Registered the call in `seed()` orchestrator and in `apply_data_patches` dispatcher as `run(conn, "mhw_015_endemic", seed_mhw_endemic_life)?;`.
- [x] TC3 — Added `"endemic_life"` to `content_tables` in `src-tauri/src/db/schema.rs::add_idempotency_constraints` → creates `uq_endemic_life(game_id, id)`.

## Phase D — UI

- [x] TD1 — Updated `src/lib/components/sidebar.svelte` line 115: `gameOnly: ['mhw', 'mhwilds']`.
- [x] TD2 — Updated `src/routes/[game]/endemic/+page.svelte`: dropped `isWilds`, replaced with `hasData = mhw || mhwilds`; widened empty-state hint.
- [x] TD3 — Registered `/endemic` in `src/lib/prefetch.ts::SECTION_JOBS`.

## Phase E — Tests + docs

- [x] TE1 — Added `mhw_015_endemic` test in `src-tauri/src/db/queries.rs` (count ≥ 30, zero NULL descriptions).
- [x] TE2 — Updated `AGENTS.md` MHW paragraph, `roadmap.md`, `STATUS.md`, `docs/fidelity-report.md`.

## Gates

- [x] Full local CI gate green: check/eslint/prettier/fmt/clippy/test/build/audit/version.
- [ ] GitHub CI verification (post-push).
