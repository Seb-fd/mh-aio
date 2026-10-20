# Mission — mh-aio

## What we build

Monster Hunter All-in-One (MH-AIO): a comprehensive, offline-first desktop encyclopedia and toolkit for all Monster Hunter games — monsters, weapons, armor, quests, skills, items, decorations, builds (incl. an armor set solver ported from Athena's A.S.S.), and global search.

## For whom

MH players and completionists who want fast, offline, game-faithful reference data with cross-navigation (monster → drops → armor sets → materials → quests), plus build planning.

## Current state

> **Source of truth for counts:** [`docs/data-coverage.md`](../../docs/data-coverage.md) — a generated per-table × per-game matrix. Regenerate with `py scripts/gen_data_coverage.py`; verify in CI with `--check`. Never hand-type a row count into this table.

All five titles are seeded with complete entity browsers, detail views, per-game theming and per-game global search. Per-game numbers below are the headline counts; the generated matrix is exhaustive.

| Game                                        | Key       | DB id | Status                                                                                                                                                                                                                                                                   | Specs                                                                       |
| ------------------------------------------- | --------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| Monster Hunter Freedom Unite (MHP2G / MH2G) | `mh2g`    | 5     | 100% — 1244 items (1083 fully sourced + 161 upstream), 432 combines (379/39/14), 83 monsters (54 Large + 25 Small + 4 Giant), 1500 weapons, 2075 armor (348 sets), 610 quests, 15,671 `item_sources`. Retail-UMD audited.                                                | `000-mh2g-baseline`                                                         |
| Monster Hunter Portable 3rd (MHP3rd)        | `mhp3rd`  | 4     | 100% — 1065 items, 60 monsters (40 Large + 20 Small), 972 weapons, 1106 armor (345 sets), 378 quests, 398 combines (276/122), 2200 drops, 50,635 charms, Comrades 92/154/52, 180 trap tools.                                                                             | `001-mhp3rd-fidelity-pass`                                                  |
| Monster Hunter World: Iceborne (MHW)        | `mhw`     | 1     | 100% — 1359 items, 94 monsters (23 Small + 71 Large), 3544 weapons, 1595 armor (351 sets), 521 quests / 3675 rewards, 404 decos, 5826 drops, 788 weaknesses, 721 ailments, 465 traps, 317 hunter charms, 33 endemic creatures.                                           | `002` · `002b` · `009` · `010-mhw-100` · `015-mhw-endemic`                  |
| Monster Hunter Rise: Sunbreak (MHR)         | `mhr`     | 2     | 100% — 1668 items, 112 monsters (78 Large + 34 Small), 3953 weapons, 1591 armor (410 sets), 946 quests / 6426 rewards, 147 skills, 243 decos, 11,849 talismans, buddy gear 322/362, rampage 247 skills / 6822 links.                                                     | `003-mhr-data-import` · `011-mhr-100`                                       |
| Monster Hunter Wilds (MHWilds)              | `mhwilds` | 3     | Core + follow-ups — 773 items, 68 monsters (34 Large + 34 Small), 1188 weapons, 714 armor (183 sets), 179 skills, 361 decos, 174 quests, 76 talismans, 70 endemic creatures. **Documented gap: no `quest_rewards`** (upstream exposes reward text, not per-item counts). | `004-mhwilds-data-import` · `012`–`014` (shipped; no spec folders recorded) |

Game-exclusive surfaces are gated per game in `src/lib/components/sidebar.svelte`:

- `tools/` — MHW Mantles/Boosters/Palico + Melder, `gameOnly: 'mhw'`.
- `endemic/` — MHW + MH Wilds, `gameOnly: ['mhw', 'mhwilds']`.
- `charms/` — MHP3rd excavated charms, MHW hunter charms, MHR talismans, Wilds talismans.
- `comrades/` — MHP3rd + MHR buddy gear.

## Principles

1. Spec is the anchor of truth; code is a derived artifact.
2. Intent validation before code generation (human = Validator of Intent).
3. Idempotent, non-destructive seed: `INSERT OR IGNORE` backed by UNIQUE indexes; never DELETE reference rows; no count-based early returns; no `clear_game`. (Documented exception: `armor_sets` re-derive for MH2G, see `000`.)
4. Game-faithful ordering (Hunter's Notes, Smith trees, quest hubs, Book of Combos).
5. Offline-first, English-primary UI.
6. Spec-anchored: significant changes update the spec first.
7. **Generated counts are never hand-edited.** `docs/data-coverage.md` is the source of truth; `spec/features/` are historical records and take additive `> **Superseded:**` banners rather than rewrites.
8. **`DATA_VERSION` is dead** — new data lands via a named `patch_*` in `apply_data_patches()`, never via a version bump. See `AGENTS.md` Gotchas.
