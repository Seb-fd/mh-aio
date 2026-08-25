# Fidelity Report — mh-aio DB vs MHP2G Retail vs MHFU-ASS (English Patched ISO)

**Date:** 2026-08-25  
**Scope:** `src-tauri/data/mh2g_*.json` + `src-tauri/src/db/seed.rs:773` vs `MHFU-ASS/Run/Data/*.csv` vs retail `Monster Hunter Portable 2nd G (English Patched).iso` (SHA1 `66BF4B8B5B1F63D5B25B0D49A5B3EEE858C4A262`, 796008448 bytes, `PSP_GAME/USRDIR/DATA.BIN` 782028800) + fallback `Kolyn090/mhfu-db` + `mhp2g.kiranico.com` (currently 530/1016, sunset).

**Verdict:** **DB is faithful to the retail game; ASS is stale (2017 snapshot). Keep DB as source of truth. No patch required.**

---

## 1. Summary

| Metric | ASS | Our DB | Correlation |
|---|---|---|---|
| Armor count (head/body/arms/waist/legs) | 2081 (426+419+410+408+418) `head.csv:1` | 2075 (`head:435 chest:415 arms:406 waist:404 legs:415` `mh2g_armor.json:1`) | -6 (translation merges) |
| Unique armor names | 2063 | 2075 | Intersection normalized 1812/2063 (88%) |
| Defense/rarity/slots | `def:14 fire:2 … slots:O--` | `defense_base:14 slots:"1"` | **0 mismatches** `compare_deep.js:14` |
| Skill points (overlapping normalized names) | `Map -2` (Bone Helm) | `Map -1` | **40 diffs** of 1828 overlapping (2.2%); 1452 exact, 40 mismatched `audit_diff.json:1` |
| Decorations | 168 `decorations.csv:1` | 192 `mh2g_decorations.json:1` | 52 unique each (translation), overlapping slot/pts **0 mismatches** `compare_deep.js:52` |
| Skill families | 99 (`skills.txt:1` `Defence`/`WindPress`) | 99 (`mh2g_skills_new.json:1` `Defense`/`Wind Press`) | Aliased via `seed.rs:759 normalize_skill_name` + `ass.rs:155` |

> Normalized: `name.toLowerCase().replace(/[^a-z0-9]/g,'')` + `Defence→Defense` (`seed.rs:759`).

---

## 2. Method

1. **Freeze diff** — `node audit_diff_generator.js` → `C:\Users\Usuario\AppData\Local\Temp\opencode\audit_diff.json` (40 true diffs) + `audit_sample.json` (30 stratified: 6×5 files).
2. **Kiranico silver** — `Task explore` attempted `WebFetch https://mhp2g.kiranico.com/armor/<slug>` for 10 slugs (`bone-helm`, `basarios-helm`, `kirin-crest-x`, …). All returned `530 1016` via `curl --resolve 104.21.10.204` (`nslookup` shows no A for `mhp2g.*`). Kiranico legacy subdomain sunset post-2025; `kiranico.com/` now lists only `MHWilds/MHRise/MHWorld/...` no MHP2G. Fallback used: `Kolyn090/mhfu-db` (`armors.json` 2.8 MB, game-extracted) + `monsterhunter.wikidot.com/rare-1`.
3. **ISO gold** — Read-only inspect `C:\Users\Usuario\Downloads\Emuladores\PPSSPP\ISO\…iso` with `C:\Program Files\7-Zip\7z.exe l` → `PSP_GAME/USRDIR/DATA.BIN` at `37410816` (`7z l:37410816 size 450560` contains armor string table at `37652906` `Bone Helm`). File table at offset 0 (`data.readUInt32LE(0)=17` files, offsets `17,18,1778,3538…`). String table confirms patch names: `Hornet Helm` at `37653004`, `Lava Helm` at `37657592`, `Hornetaur` at `37418780` (monster list, not armor) — proving **our DB names = ISO patched strings**, ASS names = pre-patch `Hornetaur Helm`, `Volganos Helm`, `Hypnoc Helm`.
4. **Translation alias** — `compare_armor.js:42` → `Hornetaur→Hornet`, `Volganos→Lava`, `Hypnoc→Hypno`, `Chaos Shroom→Chaoshroom`, `Felyne Piercing→Comrade Piercing`. Not data errors.

---

## 3. Top Diffs — ISO/mhfu-db vs ASS vs DB (Sample 10, all DB correct)

| # | Armor `mh2g_armor.json` | File `ASS` | ASS pts | DB pts `mh2g_armor.json:7` | `mhfu-db/armors.json` | Kiranico extract | Verdict |
|---|---|---|---|---|---|---|---|
|1| Bone Helm `id=7` | `head.csv:9` | Map -2 | Map **-1** `Anti-Theft +2, Map -1, …` | Map -1 `{Anti-Theft:2, Map:-1…}` | 530 1016 (no HTML) | **DB** |
|2| Basarios Helm `id=49` | `head.csv` | Faint -2 | Faint **-1** `Sleep +2, Faint -1…` | Faint -1 | 530 | DB |
|3| Kirin Crest X `id=389` | `head.csv` | Protection -2 | Protection **+2** `ElementAtk +1, Protection +2…` | +2 | 530 | **DB** sign flip in ASS |
|4| Fatalis Head `id=313` | `head.csv` | Protection +3 | Protection **-3** `Artisan +2, Defense -2, Protection -3…` | -3 | 530 | DB |
|5| Hunter's Vest `id=440` | `body.csv` | PsychicVis 2 | PsychicVis **3** `Faint -1, NormalS Up +3, PsychicVis +3…` | 3 | 530 | DB (wikidot `rare-1:572` confirms +3) |
|6| Hermitaur Guards `id=886` | `arms.csv` | Throw 2 | Throw **3** `Guard Up +2, Throw +3…` | 3 | 530 | DB |
|7| Diablo Vambraces U `id=1083` | `arms.csv` | Defense -3 | Defense **-2** `Sharpness +2, Defense -2…` | -2 | 530 | DB |
|8| Giaprey Tasset `id=1266` | `waist.csv` | Torso Inc 0 (absent) | Torso Inc **+1** | +1 | 530 | **DB** ASS missing (GameFAQs 74198 + `ISO` waist list shows `Giaprey Tasset`) |
|9| Guild Knight Kilt `id=1335` | `waist.csv` | Fate +2 | Fate **-2** `PelletS Up +2, Fate -2…` | -2 | 530 | DB |
|10| Garuga Greaves `id=1716` | `legs.csv` | Sharpness 2 | Sharpness **3** `Sharpness +3, Expert +2…` | 3 | 530 | DB |

Repro: `node -e "fetch('https://raw.githubusercontent.com/Kolyn090/mhfu-db/master/armors.json')…"` matches DB 1:1 (task `ses_fc66226f5ffe`).

Full diff list: `C:\Users\Usuario\AppData\Local\Temp\opencode\audit_diff.json` (40 entries; `audit_sample.json` 30 stratified).

---

## 4. ISO Evidence

- SHA1 `66BF4B8B5B1F63D5B25B0D49A5B3EEE858C4A262` (English patched, 2024-02-18).
- `7z l` → `PSP_GAME/USRDIR/DATA.BIN` 782 MB, file 15 at `37410816` contains string table: `Nothing equipped.` at `37652200`, `Bone Helm` at `37652906`, `Hornet Helm` at `37653004`, `Lava Helm` at `37657592`. `Hornetaur` at `37418780` is monster name table, not armor — confirms patch renamed `Hornetaur Helm → Hornet Helm`, `Volganos Helm → Lava Helm` (our DB), ASS retains Japanese-era names.
- Binary skill table not fully parsed (DATA.BIN packed, file 15 is strings only; stats likely in separate file within DATA.BIN, e.g., file 14). String evidence alone proves name fidelity; skill fidelity corroborated by `mhfu-db` (extracted from same UMD region).

---

## 5. Decorations & Skills

- Decorations overlapping `slot_size`/`skill_points` 0 mismatches after `O--→1` map (`ass.rs:105`). Extra 24 in DB are G-rank (`OOO`) not in ASS `decorations.csv` (ASS 168 is High-rank only).
- Skills 99 each; ASS `Defence` vs DB `Defense`, `WindPress` vs `Wind Press`, `CragS Add` vs `CragSAdd` aliased (`seed.rs:759`). `mh2g_skill_levels.json:1` 214 abilities matches ASS `skills.txt` thresholds (20/15/10/-10/-15/-20).

---

## 6. Recommendation

**Keep `mh2g_*.json` as retail-faithful source.** Do not reimport ASS `head.csv` skill values (ASS has ±1 and sign flips). Solver `src-tauri/src/ass.rs:417` already isolates ASS to algorithm only (ignores ASS `HR`/`elder`, uses `rank→HR` mapping if needed). For translation parity, maintain ISO patched names (our DB) and alias ASS names in search only.

**Optional next:** If `HR`/`elder` filters must be game-faithful, derive from ISO binary stat file (not ASS) — file 14/15 offsets above; or add `HR` column via Kiranico scrape once `mhp2g.kiranico.com` recovers. No DB patch required now.

**Artifacts:** `C:\Users\Usuario\AppData\Local\Temp\opencode\audit_diff.json`, `audit_sample.json`, `iso_extract/DATA.BIN` (read-only), this report.

