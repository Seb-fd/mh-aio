# 010 — Tasks — MHW to 100%

- [x] T0 — Phase 0 audit: drift counts, vocab maps, combine diff, capture + phantom proof, missing-desc confirmation (`audit_mhw_100_phase0.py`, `mhw_100_phase0.log`).
- [x] TA1 — Emit `mhw_monster_ailments.json` (185) + `mhw_monster_tools.json` (279) + `seed_mhw_monster_ailments/tools` + `notes` column + UI Tolerance column.
- [x] TA2 — Emit `mhw_hunter_charms.json` (317/1247/207) + `hunter_charms`/`hunter_charm_materials` tables + `seed_mhw_hunter_charms` + `get_hunter_charms` command.
- [x] TA3 — Quest roster fill (517/521) + set bonus completion (58 skill2, new columns) + phantom delete + stat-generated weapon/armor descriptions (3544/1595) + backfills.
- [x] TA4 — Bank `mhw_010_phase_a_*` test (charms/roster/bonuses/descriptions/phantom).
- [x] TB1 — Kiranico fetch (95 pages + 3 static, cached/throttled) + ailments numerics (721/501) + trap durations/chains (465/386/186) + stagger proxy (317) + `monster_tools.notes` + Diminishing column.
- [x] TB2 — Melder sources (212) + provisions prices/shops (56) + deco acquisition (398) + DB-column passthrough in deco detail.
- [x] TB3 — Widen `uq_item_combine` → `uq_item_combine2` (DROP-old migration) so yield variants land (186/188).
- [x] TB4 — Bank `mhw_010_phase_b_*` test.
- [x] TC1 — Fandom fetch (8 pages) + Hoarfrost/Guiding gather (75) + 4 monster descriptions (94/94) + combine arbiter (wiki 98 covered, no changes) + buy cross-check (0 gaps).
- [x] TC2 — Bank `mhw_010_phase_c_*` test.
- [x] TD1 — MHW charms rank-tree browser (families, Forge/Upgrade materials, skill + rank filters) + sidebar `gameOnly` array.
- [x] TD2 — Set-bonus display everywhere: `ArmorSet`/`ArmorSetDetail`/`ArmorDetail`/`SkillDetail` bonus fields + `get_skill_set_bonuses` reverse query; set-detail section, list badges, piece strip, skill section (all hidden when NULL); `mhw_011_set_bonus_display` test.
- [x] TV1 — `patch_mhw_010` backfill for existing installs (warm-boot gate skips seed fns) + `data_patches` test extension (22 patches, regress-and-heal).
- [x] TV2 — Silent drop-loss fix: 183 NULL-prob rows skipped by NOT NULL (`INSERT OR IGNORE`); 32 Kiranico-filled, 35 xeno pruned (Uragaan misattributions + 4 bogus items' rows + rank-impossible), 116 honest NULLs land via nullable migration (table rebuild, no inbound FKs) + UI `—`/nulls-last; `uq_monster_drops2` (+qty/prob) keeps stack variants (5826 land).
- [x] TE1 — This spec + fidelity report + AGENTS.md sync.
- [x] TE2 — Full gate: `npm run check`, eslint, prettier, `cargo fmt`, clippy, `cargo test` (36/36).
