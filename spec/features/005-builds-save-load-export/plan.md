# 005 — Plan — Builds save/load + export — DONE (as built)

## Approach (as built)

Versioned `SavedBuild` DTO shared Rust↔TS (`ass.rs` DTOs gained `Serialize`+`Deserialize`; no solver logic touched). Backend `builds.rs` owns file persistence in app-data with atomic writes + strict validation; five thin Tauri commands; frontend `builds/` gained a collapsible Saved-builds card (step 4) + per-result save buttons + JSON download + base64url share codes + import box.

## Files to touch

- Backend (`src-tauri/src/...`): `builds.rs` (new), `ass.rs` (serde derives only), `commands/mod.rs` + `lib.rs` (5 commands registered).
- Frontend (`src/...`): `lib/api.ts` (`SavedBuild` + 5 methods), `routes/[game]/builds/+page.svelte` (saved section + save buttons + share/import).
- Data (`src-tauri/data/...`): none.
- Docs to update (`STATUS.md`, `docs/fidelity-report.md`, SDD spec itself): this spec + STATUS Phase 3 ticks.

## Data / migrations

- No DB migration (app-data JSON file, not SQLite, not localStorage — explicit decision, see spec).
- Store: `{app_data_dir}/builds.json`; atomic temp+rename; validation rejects bad version/empty name/unknown game|gender|hunter_type/0-or-6+ skills/non-5-piece solutions.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

- Queries / UI steps to confirm acceptance criteria: save → restart app → list shows it; load restores query + solution; export file → import on fresh profile → identical content; garbage import rejected with message.
