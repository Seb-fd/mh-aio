# 001 — MHP3rd fidelity pass — DONE

## Context

- Status: DONE (T1–T11 all `[x]`). What was planned as "populate shop/trade/farm + numeric audit" turned out to be mostly a **visibility fix + audit**: the 2016 `item_sources` rows (shop 102 / trade 528 / farm 120 included) were already seeded; the real gap was `get_item_sources` (`queries.rs`) excluding all `carve/capture/drop/break/quest_reward` junction rows, which hid 987 small-monster rows (`source_id NULL`). Fixed: exclusion now applies only when `source_id IS NOT NULL` (mirrored `monster_drops` dupes stay excluded; same fix unhid 3772 MH2G rows).
- Current state: `src-tauri/src/db/seed.rs` MHP3rd section (`game_id 4`), `src-tauri/data/mhp3rd_*.json` (20 files), `docs/fidelity-report.md` § MHP3rd catalog.
- Related docs: `STATUS.md`, `roadmap.md` Phase 5, `000-mh2g-baseline` (pattern reference).
- Supersedes the old Context line claiming "`item_sources` covers gather-only (26 rows) plus shop/trade/farm gaps" — that line was stale and contradicted the audit outcome below; do not reintroduce it.

## User stories

- As a player, I want every MHP3rd item to show where to obtain it (shop / gather / trade / farm / carve / drop) so that farming is actionable.
- As a maintainer, I want numeric fields audited so stats match the game.

## Scope

### In scope (completed)

- Shop / trade / farm `item_sources` verified seeded (102 / 528 / 120).
- Visibility fix for small-monster `carve/capture/drop` rows (`source_id NULL` now rendered by `drop-table.svelte` as non-clickable location + condition).
- Numeric struct audit (prices, rarity, probabilities, `chance`).
- Backfill descriptions/categories without breaking idempotency.

### Out of scope

- MHW / MHR / Wilds imports. UI redesign. `carry_limit` / `icon_*` / `sort_order` seeding for MHP3rd (never seeded; accepted gap). Treasure combines (do not exist in MHP3rd — 0 rows by design).

## Acceptance criteria

- [x] No MHP3rd item lacks an acquisition source unless genuinely unobtainable (documented: 753 items with NULL description are genuinely missing source text — JP-only game — not a seed bug).
- [x] `item_sources` rows carry correct `source_type`, `location`, `conditions`; `buy_price` lives in `items.buy_price` (150 rows), not in the 102 `shop` source rows (`buy_price: None` by design).
- [x] Re-running seed does not duplicate rows (UNIQUE indexes hold; verified by double-seed + `seed_is_idempotent_and_non_destructive` + `small_monster_sources_are_visible_without_duplicates`).
- [x] `npm run check` passes with 0 errors, 0 warnings.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes (13/13 at audit time).
- [x] `cargo build --manifest-path src-tauri/Cargo.toml` passes.

## Data contracts

- Tables: `items`, `item_sources`, `monster_drops`, `quest_rewards`, `item_combine`.
- Canonical counts: items **1044** (`Material 943 / Consumable 55 / Ammo 46`; chest-order reindex, 0 dangling refs; item 10526 kept as unresolved ticket placeholder for 6 Ludroth U armor rows) — the **1065 / Material 964 / 181 buy prices / 761 drops** figures in older root docs are pre-purge era (`-21` = `"(Hunt 1 [[Zinogre]])"` + 18 typos), do not use them. `buy_price>0` **150**; `sell_price==0` 23 (guides/tickets); `rarity` null 2 (`Courage Scraps`). Combines **263** (202 Normal in book order + 61 Alchemy, 0 Treasure) with `chance`. Sources **2016** (trade 528 / drop 515 / carve 374 / farm 120 / gather 110 / shop 102 / mining 100 / capture 98 / bug 36 / fish 30 / other 3; farm strings fixed Pokke→Yukumo). Drops **1949** (60/60 monsters; 200 xeno rows pruned). Monsters 60 / weaknesses **213** (rewritten hitzones) / equipment **4481** (Akantor + Green Nargacuga linked). Weapons **973** (incl. Ludroth Splashammer; element/status filled; all with craft rows) / craft **988** / materials complete (139 backfilled). Armor **1106** (5 junk removed) / armor_materials 4130. Quests 378 / rewards 1867 (44 key flags; incl. 6 S-rank rows). Skills 101 / levels 215 (182 with descriptions) / decos 164 (158 with rarity). Charms **50,635** + charm_skills. Comrade weapons/armor/skills **92/154/52**. Trap tools **180**. Monster ailments/tools tables exist (MHFU-filled; MHP3rd tools filled).
- Seed idempotency: existing `uq_item_sources(...)`, `uq_item_combine(...)` cover all rows.
- Ordering: Book order (`ORDER BY item_combine.id`), Chest order for items.

## Audit outcome (T1–T7 done)

- T1: `mhp3rd_item_sources_extra.json` = 2016 rows (see contracts). Shop/trade/farm were **already seeded** — the real gap was visibility, not data.
- T2–T4: no JSON/seed extension needed. Root cause + fix as in Context above. UI (`drop-table.svelte`) already renders `source_id NULL` rows.
- T5: 1044 items — `buy_price>0` 150, `sell_price==0` 23, `rarity` null 2. Categories: Material 943 / Consumable 55 / Ammo 46.
- T6: descriptions from `mhp3rd_item_descriptions.json` (291 rows: 274 JP / 17 EN, CJK-detected); 753 NULL = genuinely missing source text. `carry_limit`/`icon_*`/`sort_order` never seeded for MHP3rd — accepted, out of scope.
- T7: `cargo test` green (13/13) incl. new `small_monster_sources_are_visible_without_duplicates` + existing `seed_is_idempotent_and_non_destructive`.

## UI notes (locked to avoid doc drift)

- JP badge renders as text **`JP`** (`items/[id]/+page.svelte` CJK regex), not the 🇯🇵 emoji cited in older docs.
- Bilingual fields exist **only on quests** (`name/location/objective_original`; `description_original` key exists but is empty 0/378 in source → falls back to `description`). Items/monsters/weapons/armor have no `*_original`.
- `builds` (ASS) works technically for `game_id 4` (all queries filter by `game_id`) but is tuned for MH2G (HR/Elder gate, Torso Inc, piercings); no MHP3rd-specific gate documented.
- `tools/*` does NOT apply to MHP3rd (`gameOnly: 'mhw'`); MHP3rd instead has `charms/*` (skill search) and `comrades/*` (weapons/armor/skills tabs, JP names — no EN source exists) with `gameOnly: 'mhp3rd'` sidebar entries.

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: parents before children.
- English-primary UI (JP badge where EN missing).
