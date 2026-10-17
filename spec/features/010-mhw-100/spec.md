# 010 — MHW to 100% (ailments, traps, charms, roster, Kiranico, Fandom) — DONE

## Context

- Problem: MHW core was DONE (002/002b/009) but twelve gaps kept it below mh2g/mhp3rd parity: (1) zero `monster_ailments` rows; (2) zero `monster_tools` rows; (3) `stagger_hp` all NULL; (4) 4 missing monster descriptions; (5) no hunter charms table; (6) 50 empty quest rosters; (7) set bonuses incomplete (no skill2); (8) weapons/armor descriptions 0; (9) deco acquisition empty (no craft mats exist); (10) melder sources thin (10); (11) no Hoarfrost/Guiding Lands gather; (12) combine halves collapsed by the UNIQUE index. All closed below.
- Current state: `mhw_monster_ailments.json` (721), `mhw_monster_tools.json` (465), `mhw_hunter_charms.json` (317), `mhw_item_sources_melder.json` (212), `mhw_item_sources_gather_fandom.json` (75), `hunter_charms` + `hunter_charm_materials` tables, `get_hunter_charms` command, MHW charms rank-tree browser; tests `mhw_010_phase_{a,b,c}` (36/36 green with the suite).
- Related docs: `002-mhw-data-import` (core), `002b-mhw-gaps` (prior gaps), `009-mhw-tools-melder` (tools/melder), `000-mh2g-baseline` (parity target).

## User stories

- As a World player, I want ailment tolerances, trap effectiveness, charm trees, full quest rosters, and deco acquisition browsable offline so that World+Iceborne reaches mh2g parity.
- As a maintainer, I want every gap sourced from a pinned upstream (MHWorldData frozen master, Kiranico static HTML, Fandom api.php) with logs, so future audits can re-verify.

## Scope

### In scope (completed)

- Ailments 721 rows / 93 monsters (MHWorldData tolerances + Kiranico buildup/decay/damage/duration; `notes` column for qualitative levels).
- Traps 465 rows / 5 tools (Pitfall/Shock/Vine booleans + Kiranico durations + diminishing chains in `notes`; Flash Bomb + Dung added; Sonic/Meat have no source — documented).
- Stagger proxy 317/788 (Kiranico Part-Break values, exact part-name matches, COALESCE so real values win; documented proxy caveat).
- Hunter charms 317 + 1247 materials + 207 previous-links (new tables, rank-tree browser with family chains, forge/upgrade materials, sidebar `gameOnly: ['mhp3rd','mhw']`).
- Quest roster 517/521 (4 gathering quests correctly empty; targets-only local rosters kept over upstream locale lists).
- Set bonuses completed + 58 skill2 fills (new `bonus_skill2/required2` columns; 1:1 positional join verified 351/351).
- Descriptions 3544/3544 weapons + 1595/1595 armor (deterministic stat summaries — World has no gear flavor text upstream) + 94/94 monsters (4 from Fandom).
- Deco acquisition 398/404 (24 meldable jewels with RP/MP costs + per-feystone odds; materials stay empty — none exist upstream).
- Melder sources 212 (Kiranico meld costs in conditions); shop 56 (provisions prices + rows); Hoarfrost/Guiding gather 75 (Fandom How-To-Get with region/node conditions).
- Combine: wiki 98/98 covered locally; `uq_item_combine2` (+quantity, result_quantity) so yield variants land (186/188; 2 identical halves collapse with zero info loss).
- Phantom delete: Uragaan + Fulgurbug `capture` row (Fandom-merge artifact, 0 upstream evidence).
- Drop-loss fix (found in verification): 183 NULL-rate rows silently skipped by `probability NOT NULL` (`INSERT OR IGNORE`); 32 Kiranico-filled, 35 xeno pruned (wrong-monster attributions + rank-impossible), 116 honest NULLs land via nullable migration (table rebuild); `uq_monster_drops2` (+qty/prob) keeps stack variants — 5826 land. `patch_mhw_010` backfills all of the above onto existing installs (warm-boot gate skips seed fns).

### Out of scope

- Sonic/Meat trap seconds (no numeric source; Fandom ★ display deferred), weapon augments (no upstream CSV), endemic life / canteen / layered armor, MHW solver tuning, urgent flags (no urgent system in World — matches mhp3rd precedent).

## Acceptance criteria

- [x] Monster `[id]` shows Ailments (with Tolerance) + Traps & Items (with Diminishing) for MHW.
- [x] `/mhw/charms` rank-tree browser (family chains, forge/upgrade materials, skill + rank filters); sidebar shows Charms for mhw.
- [x] Deco `[id]` shows melder/feystone acquisition for MHW (stored text wins over smith fallback).
- [x] Item `[id]` shows melder/shop/Hoarfrost/Guiding sources for MHW.
- [x] Double-seed stable (UNIQUE indexes; `seed_is_idempotent_and_non_destructive` green).
- [x] Generators re-run converge (verified per phase).
- [x] `npm run check` 0 errors, 0 warnings; eslint 0 errors; prettier clean.
- [x] `cargo test` passes (36/36 incl. three new `mhw_010_*` tests); clippy + fmt clean.

## Data contracts

- New files: `mhw_monster_ailments.json`, `mhw_monster_tools.json`, `mhw_hunter_charms.json`, `mhw_item_sources_melder.json`, `mhw_item_sources_gather_fandom.json`; generators `generate_mhw_ailments_tools.py`, `generate_mhw_hunter_charms.py`, `generate_mhw_kiranico.py` (+ `fetch_mhw_kiranico.py`), `generate_mhw_fandom.py` (+ `fetch_mhw_fandom.py`), `fix_mhw_100_phaseA.py`, `generate_mhw_descriptions.py`; logs `mhw_100_phase*.log`.
- Tables: `hunter_charms`, `hunter_charm_materials` (+`uq_hunter_charms`, materials PK); columns `monster_ailments.notes`, `monster_tools.notes`, `armor_sets.bonus_skill2/required2`, `decorations.acquisition`; index migration `uq_item_combine` → `uq_item_combine2` (DROP old first).
- Seed idempotency: `INSERT OR IGNORE` + JSON-truth UPDATE backfills (never DELETE except the documented phantom row + index migration); no `clear_game`, no count early-returns.
- Ordering: ailments/tools `ORDER BY id`; charms by name; families chain via `previous_id`.

## Constraints

- Idempotent non-destructive seed; FK parents first (charms after items+skills); English-primary UI; theming via `var(--theme-*)` only; no hallucinated flavor (generated descriptions are stat summaries, documented as derived).
