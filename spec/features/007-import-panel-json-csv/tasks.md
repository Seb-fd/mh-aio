# 007 — Tasks — Import panel JSON/CSV

- [x] T1 — Validate + preview JSON/CSV with per-row errors (backend `preview_import`, frontend dry-run table).
- [x] T2 — Transactional apply of valid rows only (`BEGIN IMMEDIATE`, `INSERT OR IGNORE`).
- [x] T3 — FK violations caught pre-apply with actionable messages.
- [x] T4 — UI: `[game]/import` page (kind selector, file/paste, field reference, preview, apply report) + sidebar entry.
- [x] T5 — Double-apply stability check (idempotency test).
- [x] T6 — Run `npm run check` (0 errors, 0 warnings).
- [x] T7 — Run `cargo test --manifest-path src-tauri/Cargo.toml` (27/27).
- [x] T8 — Run `cargo build --manifest-path src-tauri/Cargo.toml`.
- [x] T9 — Verify against `spec.md`; update docs.
