"""Generate docs/data-coverage.md from a live mh-aio SQLite database.

This is the single source of truth for every row count quoted across
README.md, STATUS.md, roadmap.md, AGENTS.md, docs/fidelity-report.md and
spec/constitution/*. Regenerate instead of hand-editing counts.

Usage (from the repo root):

    py scripts/gen_data_coverage.py                 # reads the installed DB
    py scripts/gen_data_coverage.py <path/to.db>   # or an explicit file
    py scripts/gen_data_coverage.py --check         # CI drift check

Exit codes: 0 ok (or clean under --check), 1 drift detected under --check,
2 a required table is missing (schema rename / dropped table).

The DB is opened read-only (`file:...?mode=ro`) so this can never mutate
the user's install.
"""

from __future__ import annotations

import argparse
import os
import sqlite3
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT = REPO / "docs" / "data-coverage.md"

DEFAULT_DB = Path(
    os.environ.get("APPDATA", Path.home() / ".config")
) / "com.mhaio.app" / "mh-aio.db"

GAMES = [
    (1, "mhw"),
    (2, "mhr"),
    (3, "mhwilds"),
    (4, "mhp3rd"),
    (5, "mh2g"),
]

# table -> how to attribute rows to a game_id
#   ("direct", None)                 -> table has its own game_id column
#   ("join", <child_pk>, <table>)    -> child.<child_pk> = table.id
DIRECT = [
    "items",
    "monsters",
    "weapons",
    "armor",
    "armor_sets",
    "skills",
    "decorations",
    "quests",
    "armor_sets",
    "charms",
    "endemic_life",
    "mhw_mantles",
    "palico_gadgets",
    "hunter_charms",
    "comrade_weapons",
    "comrade_armor",
    "comrade_skills",
    "buddy_weapons",
    "buddy_armor",
    "rampage_skills",
    "rampage_decorations",
]

# table -> (child_table, child_fk)
JOINED = [
    ("quest_rewards", "quest_rewards", "quest_id", "quests"),
    ("monster_drops", "monster_drops", "monster_id", "monsters"),
    ("monster_weaknesses", "monster_weaknesses", "monster_id", "monsters"),
    ("monster_ailments", "monster_ailments", "monster_id", "monsters"),
    ("monster_tools", "monster_tools", "monster_id", "monsters"),
    ("monster_equipment", "monster_equipment", "monster_id", "monsters"),
    ("weapon_materials", "weapon_materials", "weapon_id", "weapons"),
    ("weapon_craft", "weapon_craft", "weapon_id", "weapons"),
    ("armor_materials", "armor_materials", "armor_id", "armor"),
    ("skill_levels", "skill_levels", "skill_id", "skills"),
    ("item_sources", "item_sources", "item_id", "items"),
    ("item_combine", "item_combine", "result_item_id", "items"),
    ("decoration_materials", "decoration_materials", "decoration_id", "decorations"),
    ("charm_skills", "charm_skills", "charm_id", "charms"),
    ("weapon_rampage", "weapon_rampage", "weapon_id", "weapons"),
    ("armor_skill_points", "armor_skill_points", "armor_id", "armor"),
    ("weapon_skill_points", "weapon_skill_points", "weapon_id", "weapons"),
    ("palico_gadget_levels", "palico_gadget_levels", "gadget_id", "palico_gadgets"),
]


def scalar(cur, sql: str, args: tuple = ()) -> int:
    cur.execute(sql, args)
    row = cur.fetchone()
    return int(row[0]) if row and row[0] is not None else 0


def gather(db: Path) -> dict:
    con = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    cur = con.cursor()

    # --- table inventory -----------------------------------------------------
    cur.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
    tables = {r[0] for r in cur.fetchall()}

    out: dict = {"tables": sorted(tables), "direct": {}, "joined": {}, "extra": {}}

    # --- direct game_id tables ----------------------------------------------
    for t in dict.fromkeys(DIRECT):
        if t not in tables:
            raise SystemExit(f"required table missing: {t} (schema rename?)")
        cur.execute(f"PRAGMA table_info({t})")
        cols = {r[1] for r in cur.fetchall()}
        if "game_id" not in cols:
            raise SystemExit(
                f"{t} has no game_id column — move it to the JOINED list "
                f"(columns: {sorted(cols)})"
            )
        out["direct"][t] = {
            g: scalar(cur, f"SELECT COUNT(*) FROM {t} WHERE game_id = ?", (gid,))
            for gid, g in GAMES
        }
        out["direct"][t]["_total"] = scalar(cur, f"SELECT COUNT(*) FROM {t}")

    # --- joined tables -------------------------------------------------------
    for name, child, fk, parent in JOINED:
        if child not in tables or parent not in tables:
            raise SystemExit(f"required table missing: {child} or {parent}")
        out["joined"][name] = {
            g: scalar(
                cur,
                f"SELECT COUNT(*) FROM {child} c JOIN {parent} p ON p.id = c.{fk}"
                f" WHERE p.game_id = ?",
                (gid,),
            )
            for gid, g in GAMES
        }
        out["joined"][name]["_total"] = scalar(cur, f"SELECT COUNT(*) FROM {child}")

    # --- single-table extras (no per-game attribution needed) ---------------
    for t in ("melder_recipes",):
        out["extra"][t] = scalar(cur, f"SELECT COUNT(*) FROM {t}") if t in tables else None

    # --- monsters.size breakdown (exact stored value) -----------------------
    out["sizes"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT COALESCE(size,'<null>'), COUNT(*) FROM monsters WHERE game_id = ?"
            " GROUP BY size ORDER BY COUNT(*) DESC",
            (gid,),
        )
        out["sizes"][g] = cur.fetchall()

    # --- item category split -------------------------------------------------
    out["item_categories"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT category, COUNT(*) FROM items WHERE game_id = ?"
            " GROUP BY category ORDER BY COUNT(*) DESC",
            (gid,),
        )
        out["item_categories"][g] = cur.fetchall()

    # --- quest hub / category split -----------------------------------------
    out["quest_hubs"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT hub, COUNT(*) FROM quests WHERE game_id = ?"
            " GROUP BY hub ORDER BY COUNT(*) DESC",
            (gid,),
        )
        rows = cur.fetchall()
        out["quest_hubs"][g] = rows if any(h is not None for h, _ in rows) else []
    out["quest_categories"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT category, COUNT(*) FROM quests WHERE game_id = ?"
            " GROUP BY category ORDER BY COUNT(*) DESC",
            (gid,),
        )
        out["quest_categories"][g] = cur.fetchall()

    # --- item_combine combine_type split -------------------------------------
    out["combine_types"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT COALESCE(combine_type,'<null>'), COUNT(*) FROM item_combine ic"
            " JOIN items i ON i.id = ic.result_item_id WHERE i.game_id = ?"
            " GROUP BY combine_type ORDER BY COUNT(*) DESC",
            (gid,),
        )
        out["combine_types"][g] = cur.fetchall()

    # --- charms split --------------------------------------------------------
    out["charms"] = {}
    for gid, g in GAMES:
        cur.execute(
            "SELECT charm_type, COUNT(*) FROM charms WHERE game_id = ? GROUP BY charm_type",
            (gid,),
        )
        out["charms"][g] = cur.fetchall()

    # --- schema_version / data_patches --------------------------------------
    out["schema_version"] = None
    if "schema_version" in tables:
        cur.execute("SELECT version FROM schema_version ORDER BY version DESC LIMIT 1")
        row = cur.fetchone()
        out["schema_version"] = int(row[0]) if row else None
    out["data_patches"] = (
        scalar(cur, "SELECT COUNT(*) FROM data_patches") if "data_patches" in tables else None
    )
    if "data_patches" in tables:
        cur.execute("SELECT name FROM data_patches ORDER BY name")
        out["patch_names"] = [r[0] for r in cur.fetchall()]

    con.close()
    return out


def render(d: dict) -> str:
    L: list[str] = []
    w = L.append

    w("# Data Coverage")
    w("")
    w("**Generated file — do not hand-edit.** Regenerate with:")
    w("")
    w("```bash")
    w("py scripts/gen_data_coverage.py            # from the installed DB")
    w("py scripts/gen_data_coverage.py --check    # CI drift check")
    w("```")
    w("")
    w(
        "Every row count quoted in `README.md`, `STATUS.md`, `roadmap.md`, "
        "`AGENTS.md`, `docs/fidelity-report.md` and `spec/constitution/*` must "
        "match this file. If you add data, regenerate rather than editing "
        "counts by hand."
    )
    w("")
    w("## Game IDs")
    w("")
    w("| `game_id` | slug      | game                     |")
    w("| --------- | --------- | ------------------------ |")
    w("| 1         | `mhw`     | Monster Hunter: World    |")
    w("| 2         | `mhr`     | Monster Hunter Rise      |")
    w("| 3         | `mhwilds` | Monster Hunter: Wilds    |")
    w("| 4         | `mhp3rd`  | Monster Hunter P3rd      |")
    w("| 5         | `mh2g`    | Monster Hunter Freedom Unite |")
    w("")

    # ---- entity matrix
    w("## Entity counts by game")
    w("")
    hdr = "| Table | mhw | mhr | mhwilds | mhp3rd | mh2g | Total |"
    w(hdr)
    w("| --- | ---: | ---: | ---: | ---: | ---: | ---: |")
    for t in (
        "monsters",
        "items",
        "weapons",
        "armor",
        "armor_sets",
        "skills",
        "decorations",
        "quests",
        "charms",
        "endemic_life",
    ):
        r = d["direct"][t]
        w(
            f"| `{t}` | {r['mhw']} | {r['mhr']} | {r['mhwilds']} | {r['mhp3rd']} | "
            f"{r['mh2g']} | {r['_total']} |"
        )
    w("")
    w("### Junction + derived tables")
    w("")
    w(hdr)
    w("| --- | ---: | ---: | ---: | ---: | ---: | ---: |")
    for t in (
        "quest_rewards",
        "monster_drops",
        "monster_weaknesses",
        "monster_ailments",
        "monster_tools",
        "monster_equipment",
        "weapon_materials",
        "weapon_craft",
        "armor_materials",
        "skill_levels",
        "item_sources",
        "item_combine",
        "decoration_materials",
        "charm_skills",
        "weapon_rampage",
        "armor_skill_points",
        "weapon_skill_points",
    ):
        r = d["joined"][t]
        w(
            f"| `{t}` | {r['mhw']} | {r['mhr']} | {r['mhwilds']} | {r['mhp3rd']} | "
            f"{r['mh2g']} | {r['_total']} |"
        )
    w("")
    w("### Game-exclusive tables")
    w("")
    w("| Table | Game | Rows |")
    w("| --- | --- | ---: |")
    for t, g in (
        ("mhw_mantles", "mhw"),
        ("palico_gadgets", "mhw"),
        ("hunter_charms", "mhw"),
        ("buddy_weapons", "mhr"),
        ("buddy_armor", "mhr"),
        ("rampage_skills", "mhr"),
        ("rampage_decorations", "mhr"),
        ("comrade_weapons", "mhp3rd"),
        ("comrade_armor", "mhp3rd"),
        ("comrade_skills", "mhp3rd"),
    ):
        w(f"| `{t}` | {g} | {d['direct'][t][g]} |")
    w("")
    w(
        f"`palico_gadget_levels` (child of `palico_gadgets`): "
        f"{d['joined']['palico_gadget_levels']['mhw']} rows."
    )
    w("")

    # ---- monsters.size
    w("## `monsters.size` breakdown (exact stored value)")
    w("")
    w("Case-sensitive — `Small` and `small` are stored differently per game.")
    w("")
    w("| Game | size values |")
    w("| --- | --- |")
    for _, g in GAMES:
        parts = ", ".join(f"`{s}` = {n}" for s, n in d["sizes"][g])
        w(f"| {g} | {parts} |")
    w("")

    # ---- item categories
    w("## `items.category` split")
    w("")
    w("| Game | Breakdown |")
    w("| --- | --- |")
    for _, g in GAMES:
        parts = ", ".join(f"`{c}` = {n}" for c, n in d["item_categories"][g])
        w(f"| {g} | {parts} |")
    w("")

    # ---- combines
    w("## `item_combine.combine_type` split")
    w("")
    w("| Game | Breakdown |")
    w("| --- | --- |")
    for _, g in GAMES:
        rows = d["combine_types"][g]
        parts = ", ".join(f"`{t}` = {n}" for t, n in rows) if rows else "_none_"
        w(f"| {g} | {parts} |")
    w("")

    # ---- charms
    w("## `charms.charm_type` split")
    w("")
    w("| Game | Breakdown |")
    w("| --- | --- |")
    for _, g in GAMES:
        rows = d["charms"][g]
        parts = ", ".join(f"`{t}` = {n}" for t, n in rows) if rows else "_none_"
        w(f"| {g} | {parts} |")
    w("")

    # ---- quests
    w("## Quest hub / category split")
    w("")
    w("| Game | Key | Breakdown |")
    w("| --- | --- | --- |")
    for _, g in GAMES:
        hubs = d["quest_hubs"][g]
        cats = d["quest_categories"][g]
        if hubs:
            parts = ", ".join(f"`{h}` = {n}" for h, n in hubs)
            w(f"| {g} | `hub` | {parts} |")
        if cats:
            parts = ", ".join(f"`{c}` = {n}" for c, n in cats)
            w(f"| {g} | `category` | {parts} |")
    w("")

    # ---- meta
    w("## Seed bookkeeping")
    w("")
    w("| Field | Value |")
    w("| --- | --- |")
    w(f"| `schema_version` table | `{d['schema_version']}` |")
    w(f"| `data_patches` rows | `{d['data_patches']}` |")
    w("")
    w(
        "> **Note:** `src-tauri/src/db/seed.rs` declares "
        "`pub const DATA_VERSION: i32 = 1` while the live `schema_version` "
        "table holds a higher number. The `seed()` boot gate "
        "(`if current >= DATA_VERSION { return Ok(()) }`) is therefore "
        "permanently closed for upgraded installs — **all** post-v1 data "
        "lands through `apply_data_patches()` (tracked in `data_patches`), "
        "never through a version bump. See `AGENTS.md` Gotchas."
    )
    w("")
    w("### Registered `data_patches` names")
    w("")
    for n in d.get("patch_names", []):
        w(f"- `{n}`")
    w("")

    w("## Tables present in the schema")
    w("")
    w(
        "Full list for reference when adding a table (new tables need a "
        "`(game_id, id)` UNIQUE index in `add_idempotency_constraints()`):"
    )
    w("")
    w(", ".join(f"`{t}`" for t in d["tables"]))
    w("")

    return "\n".join(L)


def _rel(p: Path) -> str:
    """Path relative to the repo root when possible, else absolute."""
    try:
        return str(p.relative_to(REPO))
    except ValueError:
        return str(p)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("db", nargs="?", help="path to mh-aio.db")
    ap.add_argument(
        "--check",
        action="store_true",
        help="exit 1 if the output file is stale (CI drift gate)",
    )
    ap.add_argument("--out", help=f"output path (default {OUT})")
    args = ap.parse_args()

    db = Path(args.db) if args.db else DEFAULT_DB
    if not db.exists():
        print(
            f"error: DB not found at {db}\n"
            f"       run the app once to seed it, or pass an explicit path.",
            file=sys.stderr,
        )
        return 2

    out = Path(args.out) if args.out else OUT
    body = render(gather(db))

    if args.check:
        if not out.exists():
            print(f"FAIL: {_rel(out)} does not exist — run the generator", file=sys.stderr)
            return 1
        if out.read_text(encoding="utf-8") != body:
            print(
                f"FAIL: {_rel(out)} is stale — run `py scripts/gen_data_coverage.py`",
                file=sys.stderr,
            )
            return 1
        print(f"OK: {_rel(out)} is up to date")
        return 0

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")
    print(f"wrote {_rel(out)} ({len(body.splitlines())} lines)")
    return 0


if __name__ == "__main__":
    sys.exit(main())