# 004 — MHWilds data import — DONE (core)

## Context

- Status: DONE (core). Source turned out to be the MHDB Wilds API (`https://wilds.mhdb.io/en`, browser UA required — it 403s the default python UA) via `scripts/generate_mhwilds_from_mhdb.py`, not Kiranico/Game8 scraping.
- Current state: `src-tauri/src/db/seed.rs` Wilds section (`const MHWILDS: i32 = 3`), `src-tauri/data/mhwilds_*.json` (14 files), test `mhwilds_004_coverage_lists_details_and_junctions` (19/19 green).
- Related docs: `002-mhw-data-import` (pattern), `009-mhw-tools-melder` (pattern for future Wilds tools), `000-mh2g-baseline`.

## User stories

- As a player, I want Wilds data browsable offline, including Focus Mode–relevant info where data supports it.

## Scope

### In scope (completed)

- 34 Large monsters (+340 per-part weakness rows from part multipliers ×100, +1775 drops with Low/High rank), 773 items (Material, descriptions, carry limits), 121 crafting combines, 179 skills / 442 levels, 361 decorations, 1188 weapons (14 types, Smith-tree `sort_order`, sharpness/slots/elements/status/skills/materials), 714 armor (183 sets, resistances/skills/materials), derived equipment links (~1273 armor + ~2499 weapon), armor/weapon skill points.
- Ordering: Chest `sort_order` (alphabetical proxy — no retail box order published), Smith per-type DFS for weapons, monsters alphabetical (no Hunter's Notes equivalent published).

### Out of scope (upstream gaps, documented)

- Quests (no quest endpoint upstream) — quest list stays empty for `mhwilds`; needs Kiranico/Game8 follow-up (open a new spec if scoped).
- Charms (64 upstream, no `charms` table in schema) — open a new spec if scoped (see 009 pattern).
- Small monsters (API is large-only), Focus Mode–specific UI (separate feature if needed), Seikret/Palico equivalents.

## Acceptance criteria

- [x] All entity lists + details work for `mhwilds` (monsters/weapons/armor/items/skills/decorations/combine), no orphan FKs (name→id resolution at generation; `item_exists`/`weapon_exists` guards at seed; no PK collisions — dedicated offsets).
- [x] Detail views show materials/drop sources/weaknesses/equipment (covered by test smoke).
- [x] Idempotent re-seed (stable counts; `seed_is_idempotent_and_non_destructive` green).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (19/19).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Files: `mhwilds_{monsters,monster_weaknesses,monster_drops,items,item_combine,skills_new,skill_levels,decorations,weapons,weapon_materials,weapon_craft,armor,armor_sets,armor_materials}.json`.
- ID offsets: monsters 20001+, items 30001+, skills/levels/decos 30001+ (own spaces), weapons 40001+, armor 50001+, sets 30000+, combine 900000+.
- Tables: content + junctions + `monster_equipment` (derived like 002b); no new tables.
- Seed idempotency: `(game_id, id)` guards + UNIQUE junction indexes; `INSERT OR IGNORE` only.
- Icons: no Wilds sets offline — monsters reuse mhw slug paths (fallback in UI), weapons/armor reuse mhw rarity scheme, items NULL, decos mhfu path.

## Constraints

- Non-destructive idempotent seed; FK parents first; English-primary UI.
