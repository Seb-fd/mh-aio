# Fidelity Report — mh-aio MH2G Data

**Verdict:** The MH2G dataset in `src-tauri/data/mh2g_*.json` is faithful to the retail UMD. The MHFU-ASS 2017 CSV snapshot is stale and is used only as algorithm reference, never as a data source.

## Summary

| Metric | Result |
|---|---|
| Armor count | 2075 pieces (head/chest/arms/waist/legs) |
| Items | 1083 |
| Skill families / abilities | 99 / 214 |
| Decorations | 192 |
| Defense / rarity / slots | **100% match** vs retail (0 mismatches) |
| Skill points (normalized names) | 40 diffs of 1828 overlapping — **all favor our DB** (ASS has ±1 and sign flips, e.g. Kirin Crest X Protection -2 vs +2) |
| Decorations | overlapping slot/pts **0 mismatches** |
| Skill families | aliased between `Defence`/`Defense`, `WindPress`/`Wind Press` via `seed.rs:759` |

## Method

- Compared `mh2g_armor.json` against a game-extracted DB (`Kolyn090/mhfu-db`) and the `Monster Hunter Portable 2nd G (English Patched).iso` string table.
- Normalized names (lowercase, strip non-alphanumerics) to handle translation aliases (`Hornetaur→Hornet`, `Volganos→Lava`, `Hypnoc→Hypno`) and skill-name differences.
- `mhp2g.kiranico.com` is sunset (HTTP 530), so Kiranico was not usable as an arbiter; `mhfu-db` was used as the fallback.

## Notes

- `src-tauri/src/ass.rs` isolates the ASS port to the **algorithm only** (equivalence grouping, jewel solver, Torso Inc, bad-skill fix). All gameplay data comes from the retail game.
- Armor names use the ISO patched strings (e.g. `Hornet Helm`, `Lava Helm`), which our DB matches.

## Recommendation

Keep `mh2g_*.json` as the retail-faithful source. No DB patch required.
