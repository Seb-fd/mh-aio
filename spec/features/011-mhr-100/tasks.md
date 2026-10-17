# 011 — Tasks — MHR Rise + Sunbreak to 100%

## Phase 0 — Audit + proofs (done 2026-10-16)

- [x] T01 — Drift check: local == live v16 (112 / 3953 / 1591; items + quests supersets explained).
- [x] T02 — Payload proofs: status + part-break tables, weapon rampage lists, list-only views, talisman CSV shape, set-bonus absence (0/410).
- [x] T03 — Spec/plan/tasks skeleton.

## Phase A — Monster status + part-break

- [x] TA1 — Scrape 112 monster details → `mhr_monster_status.json` (1120 rows, 0 nulls) + `mhr_monster_partbreak.json` (541 raw → 468 stored; multi-break dupes collapsed by UNIQUE).
- [x] TA2 — Fextra trap lists → `mhr_monster_tools.json` (270 rows, 54 monsters; Effective→NULL, Ineffective→0; 58 without blocks stay absent).
- [x] TA3 — Seed (`seed_mhr_monster_status/partbreak/tools` + `monster_part_break` table) + `mhr_011a` test + monster-detail UI (Ailments/Traps flow generically; new Part Break section).

## Phase B — Buddy gear

- [x] TB1 — Scrape 4 buddy list views → `mhr_buddy_weapons.json` (322) + `mhr_buddy_armor.json` (362; upstream x2 dupes collapsed). Element/status via anchored `ELEMENT_CODES`; leading `C ` list badge stripped.
- [x] TB2 — New `buddy_weapons`/`buddy_armor` tables + seed + `mhr_011b_buddy_gear` test.
- [x] TB3 — Comrades route buddy mode for `mhr` (Palico/Palamute chips, weapons/armor tabs, EN stats) + sidebar `gameOnly: ['mhp3rd', 'mhr']`.

## Phase C — Rampage system

- [x] TC1 — Scrape 279 skill detail pages → 247 unique skills (32 shared-name dupes collapsed) + 279-skill→weapons lists → 6855 raw → 6822 unique (skill, weapon) pairs; 26 rampage decos (list view).
- [x] TC2 — Tables `rampage_skills`/`rampage_decorations`/`weapon_rampage` (+ UNIQUE) + seed + `mhr_011c_rampage` test (Rampage Bow S = 29 skills).
- [x] TC3 — Weapon-detail Rampage Skills block (name + description). Decos double as items so the generic item list shows them.

## Phase D — Talismans

- [x] TD1 — Stream-aggregate 4 base CSV methods (anima/rebirth/reincarnation/wisp_of_mystery; Sunbreak eve/haze not in upstream — documented gap) → 11,849 generator rows (max lvl per skill pair, max slot total).
- [x] TD2 — Seed into `charms` + `charm_skills` (skill names → game-2 skill ids; 0 unresolved) + `mhr_011d_talismans` test.
- [x] TD3 — Charms route already supports generic `charm_type='talisman'` (existing MHP3rd mode). Generic `get_charms_by_game` works (limited to 500 in default sort; advanced view is filter-driven).

## Phase E — Items/misc + verification

- [x] TE1 — `item-mix` → `item_combine` (178 rows, chance 100, Potion=Herb spot-verified).
- [x] TE2 — Dango/switch/endemic: documented N/A (no DB representation; player-facing side widgets outside app scope; anomaly investigations procedural — unbounded).
- [x] TE3 — `verify_mhr_100.py` integrity script (counts, FK orphans, deco icons, talisman skill resolution, monster detail smoke) + `patch_mhr_011` warm-boot backfill + docs (fidelity, AGENTS, this spec).

## Gates

- [x] All `mhr_011a/b/c/d/e` tests green before moving on; final full gate per plan.
