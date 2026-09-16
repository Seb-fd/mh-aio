# Mission — mh-aio

## What we build

Monster Hunter All-in-One (MH-AIO): a comprehensive, offline-first desktop encyclopedia and toolkit for all Monster Hunter games — monsters, weapons, armor, quests, skills, items, decorations, builds (incl. an armor set solver ported from Athena's A.S.S.), and global search.

## For whom

MH players and completionists who want fast, offline, game-faithful reference data with cross-navigation (monster → drops → armor sets → materials → quests), plus build planning.

## Current state (source: STATUS.md / roadmap.md / spec audit 2026-09-11)

| Game | Key | DB id | Status | Spec |
| ---- | --- | ----- | ------ | ---- |
| Monster Hunter Freedom Unite (MHP2G / MH2G) | `mh2g` | 5 | Fully populated and verified MVP (1083 items fully sourced, 432 combines, 83 monsters, 1500 weapons, 2075 armor, 610 quests) | `000-mh2g-baseline` (Done) |
| Monster Hunter Portable 3rd (MHP3rd) | `mhp3rd` | 4 | Fully seeded, fidelity pass DONE (1044 items, 60 monsters, 972 weapons, 1111 armor, 378 quests, 263 combines) | `001-mhp3rd-fidelity-pass` (Done) |
| Monster Hunter World: Iceborne (MHW) | `mhw` | 1 | Core seeded (1359 items / 94 monsters / 3544 weapons / 521 quests / 211 melder / 20 mantles); gaps: decorations, weaknesses/equipment, shop sources | `002-mhw-data-import` (Done) + `002b-mhw-gaps` (Next) + `009-mhw-tools-melder` (Done) |
| Monster Hunter Rise: Sunbreak (MHR) | `mhr` | 2 | Fully seeded (112 monsters, 1642 items, 3953 weapons, 1591 armor, 946 quests, 147 skills, 243 decorations; talismans pending) | `003-mhr-data-import` (Done) |
| Monster Hunter Wilds (MHWilds) | `mhwilds` | 3 | Core seeded (34 Large monsters, 773 items, 1188 weapons, 714 armor, 179 skills, 361 decorations; quests/charms pending upstream) | `004-mhwilds-data-import` (Done) |

Detail views, per-game theming with ornaments, and per-game global search exist for seeded games. MHW Tools (Mantles/Boosters/Palico) + Elder Melder are MHW-only (`gameOnly: 'mhw'`).

## Principles

1. Spec is the anchor of truth; code is a derived artifact.
2. Intent validation before code generation (human = Validator of Intent).
3. Idempotent, non-destructive seed: `INSERT OR IGNORE` backed by UNIQUE indexes; never DELETE reference rows; no count-based early returns; no `clear_game`. (Documented exception: `armor_sets` re-derive for MH2G, see `000`.)
4. Game-faithful ordering (Hunter's Notes, Smith trees, quest hubs, Book of Combos).
5. Offline-first, English-primary UI.
6. Spec-anchored: significant changes update the spec first.
