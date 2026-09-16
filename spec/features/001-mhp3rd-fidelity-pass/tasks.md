# 001 — Tasks — MHP3rd fidelity pass

- [x] T1 — Audit current MHP3rd `item_sources` coverage by `source_type` (query counts).
- [x] T2 — Extend wiki pipeline output for shop/trade/farm into `mhp3rd_*.json`.
- [x] T3 — Seed shop sources with `buy_price` (idempotent).
- [x] T4 — Seed trade + farm sources with `location`/`conditions`.
- [x] T5 — Numeric audit: rarity, prices, probabilities, chance.
- [x] T6 — Backfill descriptions/categories (no null regressions).
- [x] T7 — Re-run seed twice, confirm stable counts (no duplicates).
- [x] T8 — Run `npm run check` (0 errors, 0 warnings).
- [x] T9 — Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] T10 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T11 — Verify against `spec.md` acceptance criteria; update `docs/fidelity-report.md` + `STATUS.md`.
