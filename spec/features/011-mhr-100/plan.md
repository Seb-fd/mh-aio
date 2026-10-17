# 011 — Plan — MHR Rise + Sunbreak to 100%

## Approach

Reuse the 003 scraper pattern (`scripts/fetch_mhrise_kiranico.py`: throttled ~0.4s, disk cache `tmp/mhrise_kiranico/`, resume-safe) extended to new views; small list-only views scraped inline (no detail pages exist). Generators aggregate to committed JSON; seed stays idempotent (`INSERT OR IGNORE` + backfills); each phase lands behind a `mhr_011*` test; finish with `verify_mhr_100.py` + `patch_mhr_011` + docs + full gate.

## Files to touch

- Backend: `src-tauri/src/db/seed.rs` (`seed_mhr_*` additions + `patch_mhr_011`), `schema.rs` (5 new tables only), `queries.rs` (list/detail queries + reverse lookups + tests), `commands/mod.rs` + `lib.rs` (only if new commands needed — prefer reusing existing `get_*` with game_id).
- Data: `src-tauri/data/mhr_*.json` (new files listed in spec).
- Scripts (gitignored): extend/add `scripts/fetch_mhrise_011_*.py`, generators, `scripts/verify_mhr_100.py`.
- Frontend: `charms` route (talisman mode), `comrades` route (buddy mode), weapon detail (rampage block), reference lists (dango/switch/endemic), `api.ts` mirrors.
- Docs: `docs/fidelity-report.md`, `AGENTS.md` (MHR paragraph), `STATUS.md`/`roadmap.md` if they track phases, this spec's `tasks.md`.

## Data / migrations

- No ALTERs: new tables via `CREATE TABLE IF NOT EXISTS`; talismans reuse `charms`/`charm_skills`; combine reuses `item_combine` (+ widened UNIQUE index only if (result, component, type) collides across games — game_id is in the rows? verify: `uq_item_combine*` shape in Phase E).
- FK order: parents (monsters/items/skills/weapons) before junctions; buddy tables standalone.

## Verification

```bash
py scripts/verify_mhr_100.py
cargo test --manifest-path src-tauri/Cargo.toml
npm run check ; npm run format:check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo build --manifest-path src-tauri/Cargo.toml
node scripts/check-version.js
```

- Counts per table for game 2, orphan-FK check, double-seed stability, UI smoke per new surface.
