# Roadmap (SDD index) — mh-aio

Root `roadmap.md` remains the canonical phase history. This file is the ordered SDD feature index. Each row links to its `spec/features/` folder, which holds `spec.md` (what + acceptance), `plan.md` (how), `tasks.md` (checklist).

| # | Feature | Folder | Status | Source |
| - | ------- | ------ | ------ | ------ |
| 000 | MH2G baseline (verified MVP retro-anchor) | `features/000-mh2g-baseline/` | Done | roadmap Phase 1/2, fidelity-report |
| 001 | MHP3rd fidelity pass (visibility fix + numeric audit) | `features/001-mhp3rd-fidelity-pass/` | Done | STATUS.md, fidelity-report |
| 002 | MHW data import — core (items/monsters/weapons/armor/quests/skills/melder) | `features/002-mhw-data-import/` | Done | roadmap Phase 5 |
| 002b | MHW gaps (decorations, weaknesses/equipment, shop sources, 12-HEX docs) | `features/002b-mhw-gaps/` | Done | audit 2026-09-11 |
| 003 | MHR data import (bulk base Rise + Sunbreak v16 Kiranico scrape) | `features/003-mhr-data-import/` | Done | roadmap Phase 5 |
| 004 | MHWilds data import (MHDB Wilds API, core; quests/charms deferred) | `features/004-mhwilds-data-import/` | Done | roadmap Phase 5 |
| 005 | Builds save/load + export JSON / share link (app-data) | `features/005-builds-save-load-export/` | Done | roadmap Phase 3 follow-up |
| 006 | Favorites system (app-data, per-game stars + ★ filters) | `features/006-favorites-system/` | Done | roadmap Phase 4 follow-up |
| 007 | Import panel JSON/CSV (validate + transactional apply) | `features/007-import-panel-json-csv/` | Done | roadmap Phase 4 follow-up |
| 008 | Mobile & distribution (build, updater, tray, packaging) | `features/008-mobile-distribution/` | Planned | roadmap Phase 6 |
| 009 | MHW tools (Mantles/Boosters/Palico) + Elder Melder (retro-anchor) | `features/009-mhw-tools-melder/` | Done | audit 2026-09-11 |

Game-specific UI adaptations (Focus Mode, Wirebugs, etc.) get their own spec if scoped — they no longer "ride along" implicitly with 002–004 (see 009 pattern).
