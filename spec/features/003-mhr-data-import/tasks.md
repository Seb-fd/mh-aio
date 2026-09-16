# 003 — Tasks — MHR data import

## Phase A — bulk base Rise (done)

- [x] T-A1 — Fetch Badge87 (items/skills/decos/armor/weapons) + CrimsonNynja (monsters/quests); assess shapes.
- [x] T-A2 — `scripts/generate_mhrise_bulk.py` → 12 `mhr_*.json` (dedicated PK offsets).
- [x] T-A3 — `seed_mhr_*` section (`MHR = 2`) + derived equipment/skill-points + `mhr_003a_coverage_*` test.
- [x] T-A4 — Docs (STATUS/roadmap/fidelity-report/spec) mark Phase A with approximations.

## Phase B — Sunbreak v16 Kiranico scrape (done)

- [x] T-B1 — Scraper `scripts/fetch_mhrise_kiranico.py` (monsters 112 → quests 698 → items 1764 → weapons 3953 → armor 1591 → skills 147 → decos 243; throttled, `tmp/` cache + resume).
- [x] T-B2 — Merge `scripts/merge_mhrise_phaseb.py` (element codes anchored, stem-derived sets, HTML-entity cleanup, orphan-level self-check).
- [x] T-B3 — Seed + coverage tests (`mhr_003b_*` incl. weapons/armor/skills/decos/quests/rewards assertions).
- [x] T-B4 — Docs mark 003 Done.

## Gates

- [x] `npm run check` (0/0) · `cargo test` (29/29) · `cargo build` (clean).
