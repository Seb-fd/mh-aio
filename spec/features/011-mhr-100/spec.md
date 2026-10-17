# 011 — MHR Rise + Sunbreak to 100% — DONE

## Context

- MHR baseline DONE (`003`: 112 monsters, 1642 items, 3953 weapons, 1591 armor/410 sets, 946 quests/7508 rewards, 147 skills, 243 decos; tests `mhr_003a/003b` green).
- Upstream: Kiranico DB v16.0.0 (`mhrise.kiranico.com`, static HTML, same `/data/<plural>?view=X` + `/data/<id>` scheme as the 003/mhw scrapers) + datamined talisman CSVs (`3geek14/mhr-charms` GitLab).
- This spec closes every documented 003 gap + never-scraped Kiranico views, mirroring `010-mhw-100` phase structure (audit → A–E → verify). **All five phases DONE.**

## Phase 0 findings (proven 2026-10-16, read-only probes)

- Drift: none. Live v16 = local counts exactly (monsters 112, weapons 3953, armor 1591). Homepage "items 1764" vs local 1642 = Deliveries + Room-Deco views excluded by 003 design (verify in Phase E). Homepage "quests 698" vs local 946 = local is a superset (+180 event).
- Monster detail pages carry: per-part hitzones (scraped), **part-break table** (part HP + break thresholds + extract color — NEW), **status buildup table** (Poison/Para/Sleep/Stun/Blast/Exhaust: initial +inc →max, decay step/interval, duration, damage — NEW, maps 1:1 onto `monster_ailments`), quest-HP table (optional), drops (scraped), JP motion values (skip). **No trap text anywhere** → traps via Fextra probe, else N/A.
- Weapon detail pages carry: Slots badges (parse `<img alt>` for sizes), Rampage Slots, Defense Bonus, **per-weapon rampage skill list** (NEW). Rampage-skill detail pages list weapons (298 rows for Attack Boost I) — scrape skill→weapons direction (280 pages ≈ 2 min) instead of 3953 weapon pages.
- List-only views (full data inline, no detail pages): buddy weapons/armor (stats only, **no materials upstream**), `item-mix` crafting list (numbered, → `item_combine`), rampage decorations, switch skills, dango skills, canteen menu, endemic life, requests.
- Talisman CSVs are full probability enumerations (40 MB / 778k lines each) — aggregate to distinct (method × skill1/lv × skill2/lv × slots) generator rows in existing `charms` + `charm_skills` tables (MHP3rd pattern). Sunbreak methods (Moonbow/Haze/Aurora): check GitLab tree in Phase D.
- `mhr_armor_sets.json`: `bonus_skill` key exists, **0/410 non-null** → Rise has no armor set bonuses (rampage skills are the weapon-side equivalent). `bonus_skill2` N/A by design.
- Weapons: `slots` non-null 2508/3953 (sizes TBD via alt-text parse), `skills`/`description` 0 (stay null unless found).

## Scope

### In scope

- A. Monsters: status buildup + durations + part-break (+ trap effectiveness iff Fextra probe lands).
- B. Buddy gear: NEW `buddy_weapons` / `buddy_armor` tables (kind palico|palamute, EN columns; no materials upstream — documented) + Comrades browser for `mhr` + item/monster material links where FKs exist (none expected — stats-only).
- C. Rampage: `rampage_skills` + `rampage_decorations` + `weapon_rampage` junction; weapon-detail Rampage block; rampage deco costs/materials.
- D. Talismans: aggregated generator rows per melding method + Talisman mode in `charms` route (third branch) with skill search.
- E. Items/misc + verify: `item-mix` → `item_combine` (chance 100 — Rise crafting always succeeds), Market/Argosy shop + expedition gather sources, dango + switch + endemic reference tables (+ minimal UI), event/delivery/room-deco item verdict, quest-HP verdict, `verify_mhr_100.py`, `patch_mhr_011`, docs, full gate.

### Out of scope (documented N/A)

- Anomaly Investigations procedural mapping (unbounded; fixed quests covered). Qurious crafting/augments (procedural rolls). Armor `bonus_skill2` (no such sets). Weapon descriptions (absent upstream). JP motion values. Wirebug widgets. Buddy gear materials (absent upstream).

## Acceptance criteria

- [ ] All entity lists + details work for `mhr`, no orphan FKs, idempotent re-seed (stable counts).
- [ ] `mhr_011a/b/c/d/e` tests green (one per phase, counts + spot asserts).
- [ ] `verify_mhr_100.py` PASSED (counts, orphan checks, generator convergence where applicable).
- [ ] Pre-existing installs heal via `patch_mhr_011` (warm-boot backfill test).
- [ ] Full gate green (check/eslint/prettier/fmt/clippy/38+ tests/build/version).

## Data contracts

- New JSON (committed): `mhr_monster_status.json`, `mhr_monster_partbreak.json`, `mhr_buddy_weapons.json`, `mhr_buddy_armor.json`, `mhr_rampage_skills.json`, `mhr_rampage_decorations.json`, `mhr_weapon_rampage.json`, `mhr_talismans.json`, `mhr_combine.json`, `mhr_item_sources_extra.json`, `mhr_reference.json` (dango/switch/endemic).
- New tables: `buddy_weapons`, `buddy_armor`, `rampage_skills`, `rampage_decorations`, `weapon_rampage`. Reused: `monster_ailments`, `monster_tools` (traps iff sourced), `charms` + `charm_skills` (talismans), `item_combine`.
- ID offsets (game_id 2 space, no collisions): TBD in each phase implementation; recorded here when fixed.
- Seed: `INSERT OR IGNORE` + JSON-truth backfills; `patch_mhr_011` for warm-boot installs. Never DELETE reference rows.
