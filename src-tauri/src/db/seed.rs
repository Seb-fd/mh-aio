use rusqlite::{Connection, OptionalExtension, Result};
use serde::Deserialize;

const MHW: i32 = 1;
const MH2G: i32 = 5;
const MHP3RD: i32 = 4;
const MHWILDS: i32 = 3;
const MHR: i32 = 2;

/// Offset added to MHW quest ids (and quest_rewards.quest_id): MHWorldData canonical
/// ids (101..67841) overlap MH2G quest ids (1..610) on the single-column PK, so 67
/// rows were silently dropped by INSERT OR IGNORE (258 low_high -> 191). Offset ids
/// live in 100101+, clear of every game's ranges (MH2G 1..610, MHP3rd 10001..10382).
const MHW_QUEST_ID_OFFSET: i32 = 100_000;
/// MHW quest_reward rows are namespaced by their own PK ids (600001.. are MHP3rd's).
const MHW_QUEST_REWARD_ID_MIN: i32 = 800_001;
const MHW_QUEST_REWARD_ID_MAX: i32 = 899_999;

/// Bundled-data version. Bump ONLY when a data JSON file changes shape in a way
/// that targeted patches cannot cover (new tables, new required columns,
/// changed PK ranges). For data *backfills* (icons, categories, sort orders)
/// add a named patch to `apply_data_patches()` instead: patches run once per
/// database, cost one PK lookup per boot afterwards, and — unlike a version
/// bump — never force a full re-seed, never lose progress when the app is
/// killed mid-run, and keep warm boots fast.
pub const DATA_VERSION: i32 = 1;

pub fn seed(conn: &Connection) -> Result<()> {
    // Targeted backfills first: cheap (one PK lookup per patch once applied)
    // and independent of the full-seed gate below.
    apply_data_patches(conn)?;
    let current = crate::db::schema::get_schema_version(conn).unwrap_or(0);
    if current >= DATA_VERSION {
        // Warm boot: a previous launch already seeded this exact data.
        return Ok(());
    }
    seed_games(conn)?;
    seed_monsters(conn)?;
    backfill_monster_descriptions(conn)?;
    seed_items(conn)?;
    backfill_item_descriptions(conn)?;
    seed_monster_drops(conn)?;
    seed_item_sources_from_drops(conn)?;
    seed_extra_item_sources(conn)?;
    seed_monster_equipment(conn)?;
    seed_monster_weaknesses(conn)?;
    seed_monster_ailments(conn)?;
    seed_monster_tools(conn)?;
    seed_item_combine(conn)?;
    seed_extra_item_combine(conn)?;
    seed_weapons(conn)?;
    seed_weapon_materials(conn)?;
    seed_weapon_craft(conn)?;
    seed_armor_sets(conn)?;
    seed_armor(conn)?;
    seed_armor_materials(conn)?;
    seed_quests(conn)?;
    seed_quest_rewards(conn)?;
    seed_skills(conn)?;
    seed_skill_levels(conn)?;
    seed_decorations(conn)?;
    seed_armor_skill_points(conn)?;
    seed_weapon_skill_points(conn)?;
    // MHP3rd (ULJM-05800) â€” faithful to ISO normal (English patch), village_low/high + guild_low/high
    seed_mhp3rd_monsters(conn)?;
    backfill_mhp3rd_monster_descriptions(conn)?;
    seed_mhp3rd_items(conn)?;
    backfill_mhp3rd_item_descriptions(conn)?;
    seed_mhp3rd_monster_drops(conn)?;
    seed_mhp3rd_item_sources_from_drops(conn)?;
    seed_mhp3rd_extra_item_sources(conn)?;
    seed_mhp3rd_monster_equipment(conn)?;
    seed_mhp3rd_monster_weaknesses(conn)?;
    seed_mhp3rd_item_combine(conn)?;
    seed_mhp3rd_extra_item_combine(conn)?;
    seed_mhp3rd_weapons(conn)?;
    seed_mhp3rd_weapon_materials(conn)?;
    seed_mhp3rd_weapon_craft(conn)?;
    seed_mhp3rd_armor_sets(conn)?;
    seed_mhp3rd_armor(conn)?;
    seed_mhp3rd_armor_materials(conn)?;
    seed_mhp3rd_quests(conn)?;
    seed_mhp3rd_quest_rewards(conn)?;
    seed_mhp3rd_skills(conn)?;
    seed_mhp3rd_skill_levels(conn)?;
    seed_mhp3rd_decorations(conn)?;
    seed_mhp3rd_armor_skill_points(conn)?;
    seed_mhp3rd_weapon_skill_points(conn)?;
    // MHW + Iceborne (game_id 1) — items 100% (Fandom MHWI + MHW lists + Monster Materials), combine + melder + drops/sources + weapons
    seed_mhw_items(conn)?;
    seed_mhw_item_combine(conn)?;
    seed_mhw_extra_item_combine(conn)?;
    seed_mhw_melder_recipes(conn)?;
    seed_mhw_monsters(conn)?;
    seed_mhw_monster_drops(conn)?;
    seed_mhw_item_sources_from_drops(conn)?;
    seed_mhw_extra_item_sources(conn)?;
    seed_mhw_gather_item_sources(conn)?;
    seed_mhw_weapons(conn)?;
    seed_mhw_weapon_materials(conn)?;
    seed_mhw_weapon_craft(conn)?;
    seed_mhw_skills(conn)?;
    seed_mhw_skill_levels(conn)?;
    seed_mhw_armor_sets(conn)?;
    seed_mhw_armor(conn)?;
    seed_mhw_armor_materials(conn)?;
    seed_mhw_armor_skill_points(conn)?;
    seed_mhw_decorations(conn)?;
    seed_mhw_monster_weaknesses(conn)?;
    seed_mhw_weapon_skill_points(conn)?;
    seed_mhw_monster_equipment(conn)?;
    seed_mhw_mantles(conn)?;
    seed_palico_gadgets(conn)?;
    seed_palico_gadget_levels(conn)?;
    seed_mhw_quests(conn)?;
    seed_mhw_quest_rewards(conn)?;
    // MH Wilds (game_id 3) — MHDB Wilds API (no quests/charms upstream)
    seed_mhwilds_monsters(conn)?;
    seed_mhwilds_items(conn)?;
    seed_mhwilds_item_combine(conn)?;
    seed_mhwilds_monster_drops(conn)?;
    seed_mhwilds_item_sources_from_drops(conn)?;
    seed_mhwilds_skills(conn)?;
    seed_mhwilds_skill_levels(conn)?;
    seed_mhwilds_decorations(conn)?;
    seed_mhwilds_monster_weaknesses(conn)?;
    seed_mhwilds_weapons(conn)?;
    seed_mhwilds_weapon_materials(conn)?;
    seed_mhwilds_weapon_craft(conn)?;
    seed_mhwilds_armor_sets(conn)?;
    seed_mhwilds_armor(conn)?;
    seed_mhwilds_armor_materials(conn)?;
    seed_mhwilds_armor_skill_points(conn)?;
    seed_mhwilds_weapon_skill_points(conn)?;
    seed_mhwilds_monster_equipment(conn)?;
    // MH Rise (game_id 2) — Phase A bulk (Badge87 + CrimsonNynja, base Rise;
    // Sunbreak delta via Kiranico scrape in Phase B). No weaknesses/drops
    // (numeric) upstream yet — Phase B brings hitzones + rewards.
    seed_mhr_monsters(conn)?;
    seed_mhr_items(conn)?;
    seed_mhr_skills(conn)?;
    seed_mhr_skill_levels(conn)?;
    seed_mhr_decorations(conn)?;
    seed_mhr_weapons(conn)?;
    seed_mhr_weapon_materials(conn)?;
    seed_mhr_weapon_craft(conn)?;
    seed_mhr_armor_sets(conn)?;
    seed_mhr_armor(conn)?;
    seed_mhr_armor_materials(conn)?;
    seed_mhr_armor_skill_points(conn)?;
    seed_mhr_weapon_skill_points(conn)?;
    seed_mhr_quests(conn)?;
    // Phase B (Kiranico scrape): numeric monster data. Equipment derives
    // from drops, so it must run AFTER drops/sources (FK-order principle).
    seed_mhr_monster_weaknesses(conn)?;
    seed_mhr_monster_drops(conn)?;
    seed_mhr_item_sources_from_drops(conn)?;
    seed_mhr_monster_equipment(conn)?;
    seed_mhr_quest_rewards(conn)?;
    // Stamp the data version (inside the caller's transaction in production,
    // so a crash rolls the stamp back with the data and the next boot
    // re-seeds instead of trusting a half-written DB).
    conn.execute(
        "INSERT INTO schema_version (id, version) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET version = excluded.version",
        rusqlite::params![DATA_VERSION],
    )?;
    Ok(())
}

/// Targeted data backfills. Each patch runs ONCE per database (tracked in
/// `data_patches`) and costs a single PK lookup per boot afterwards — this is
/// the mechanism for icon/category/sort fixes that must reach existing
/// installs WITHOUT a DATA_VERSION bump (which would force a full re-seed).
/// All patches are idempotent (COALESCE/WHERE guards + INSERT OR IGNORE) and
/// run inside the caller's transaction, so a crash rolls back to
/// "not applied" and the next boot retries only the missing patch.
pub fn apply_data_patches(conn: &Connection) -> Result<()> {
    patch_item_icons_mhp3rd(conn)?;
    patch_item_icons_mhr(conn)?;
    patch_item_icons_mhwilds(conn)?;
    patch_quest_icons_mhr(conn)?;
    patch_monster_thumbs_rise_wilds(conn)?;
    patch_mh2g_monster_drops(conn)?;
    patch_mh2g_drop_corrections(conn)?;
    patch_mh2g_weaknesses(conn)?;
    patch_mh2g_equipment(conn)?;
    patch_mh2g_hunt_info(conn)?;
    patch_mh2g_items(conn)?;
    patch_mh2g_quests(conn)?;
    patch_mh2g_weapon_craft(conn)?;
    Ok(())
}

fn patch_applied(conn: &Connection, name: &str) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM data_patches WHERE name = ?1",
            rusqlite::params![name],
            |r| r.get::<_, i32>(0),
        )
        .optional()?
        .is_some())
}

fn record_patch(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO data_patches (name) VALUES (?1)",
        rusqlite::params![name],
    )?;
    Ok(())
}

/// Backfill MHFU monster drops from the extended `mh2g_monster_drops.json`
/// (Scarred Yian Garuga, Rusted Kushala Daora, thin large tables, rated
/// small monsters — see `scripts/generate_mh2g_monster_drops.py`).
/// Runs once per database WITHOUT a DATA_VERSION bump: `INSERT OR IGNORE`
/// is keyed on `uq_monster_drops`, so pre-existing rows are untouched and
/// only genuinely new (monster, item, method, part, rank) combos land.
/// The item "How to Obtain" mirror is re-run (idempotent) so new drops
/// show up item-side too.
fn patch_mh2g_monster_drops(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_monster_drops_backfill";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    // Fresh databases have no monsters yet at patch time (patches run before
    // the full seed) and FKs would reject the inserts — skip there, the full
    // `seed_monster_drops` reads the same extended JSON anyway.
    let have_monsters: i64 = conn
        .query_row("SELECT COUNT(*) FROM monsters WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_monsters == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    let json_data = include_str!("../../data/mh2g_monster_drops.json");
    let drops: Vec<DropJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in drops {
        conn.execute(
            "INSERT OR IGNORE INTO monster_drops
                (monster_id, item_id, method, part, rank, quantity, probability, condition, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
            rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
        )?;
    }
    seed_item_sources_from_drops(conn)?;
    record_patch(conn, NAME)
}

/// Correct pre-existing MHFU drop rates to MHP2G-extracted truth
/// (`scripts/correct_mh2g_drop_rates.py`, mhfu-db wins; see
/// `scripts/mh2g_drop_corrections.log`). Covers swaps (Rathalos
/// Shell/Scale), shadow-rank deletes (Gendrome High holding Low values)
/// and pure rate fixes (Garuga G Thick 64% -> 53%). Runs once per
/// database WITHOUT a DATA_VERSION bump. Matching is twin-safe
/// (key + old probability/quantity) and NULL-safe (`IS`); `condition`
/// text is never touched. Mirror `item_sources` rows for affected
/// monsters are rebuilt so the item side shows corrected rates.
fn patch_mh2g_drop_corrections(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_drop_corrections";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    // Same fresh-DB gate as the backfill above: the full seed already
    // inserts the corrected JSON, so there is nothing to fix yet.
    let have_monsters: i64 = conn
        .query_row("SELECT COUNT(*) FROM monsters WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_monsters == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    #[derive(Deserialize)]
    struct Correction {
        monster_id: i32,
        item_id: i32,
        method: String,
        part: Option<String>,
        rank: Option<String>,
        old_probability: f64,
        old_quantity: i32,
        quantity: Option<i32>,
        probability: Option<f64>,
    }
    #[derive(Deserialize)]
    struct CorrectionsFile {
        updates: Vec<Correction>,
        deletes: Vec<Correction>,
        rebuild_mirrors_for: Vec<i32>,
    }
    let json_data = include_str!("../../data/mh2g_drop_corrections.json");
    let corrections: CorrectionsFile = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for u in &corrections.updates {
        conn.execute(
            "UPDATE monster_drops SET quantity = ?1, probability = ?2
              WHERE monster_id = ?3 AND item_id = ?4 AND method = ?5
                AND part IS ?6 AND rank IS ?7
                AND ABS(probability - ?8) < 1e-9 AND quantity = ?9",
            rusqlite::params![
                u.quantity.unwrap_or(u.old_quantity),
                u.probability.unwrap_or(u.old_probability),
                u.monster_id,
                u.item_id,
                u.method,
                u.part,
                u.rank,
                u.old_probability,
                u.old_quantity
            ],
        )?;
    }
    for x in &corrections.deletes {
        conn.execute(
            "DELETE FROM monster_drops
              WHERE monster_id = ?1 AND item_id = ?2 AND method = ?3
                AND part IS ?4 AND rank IS ?5
                AND ABS(probability - ?6) < 1e-9 AND quantity = ?7",
            rusqlite::params![
                x.monster_id,
                x.item_id,
                x.method,
                x.part,
                x.rank,
                x.old_probability,
                x.old_quantity
            ],
        )?;
    }
    // Rebuild the item-side mirror for affected monsters only (mh2g owns
    // monster ids 1-83 outright; other games use offset ids). Mirror rows
    // are exactly those with NULL location + carve/capture/drop/break type.
    for mid in &corrections.rebuild_mirrors_for {
        conn.execute(
            "DELETE FROM item_sources
              WHERE source_id = ?1
                AND source_type IN ('carve', 'capture', 'drop', 'break')
                AND location IS NULL",
            rusqlite::params![mid],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability)
             SELECT item_id,
                    CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' ELSE method END,
                    monster_id, quantity, quantity, probability
             FROM monster_drops WHERE monster_id = ?1",
            rusqlite::params![mid],
        )?;
    }
    record_patch(conn, NAME)
}

/// Backfill MHFU weakness rows from the extended
/// `mh2g_monster_weaknesses.json` (Scarred/Rusted full part lists, thin
/// large expansion, smalls Body-only — see
/// `scripts/generate_mh2g_weaknesses.py`). Runs once per database WITHOUT
/// a DATA_VERSION bump: `INSERT OR IGNORE` is keyed on
/// `uq_monster_weaknesses`, so pre-existing rows are untouched.
fn patch_mh2g_weaknesses(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_weaknesses_backfill";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_monsters: i64 = conn
        .query_row("SELECT COUNT(*) FROM monsters WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_monsters == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    let json_data = include_str!("../../data/mh2g_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon],
        )?;
    }
    record_patch(conn, NAME)
}

/// Backfill MHFU monster<->gear links from the extended
/// `mh2g_monster_equipment.json` (join-derived: gear links to every monster
/// dropping one of its materials, family-trio capped — see
/// `scripts/generate_mh2g_equipment.py`). Runs once per database WITHOUT a
/// DATA_VERSION bump: `INSERT OR IGNORE` is keyed on
/// `uq_monster_equipment`, so pre-existing rows are untouched.
fn patch_mh2g_equipment(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_equipment_backfill";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_monsters: i64 = conn
        .query_row("SELECT COUNT(*) FROM monsters WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_monsters == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    let json_data = include_str!("../../data/mh2g_monster_equipment.json");
    let rows: Vec<MonsterEquipJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rows {
        conn.execute(
            "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
             VALUES (5, ?1, ?2, ?3)",
            rusqlite::params![r.monster_id, r.kind, r.equipment_id],
        )?;
    }
    record_patch(conn, NAME)
}

/// Backfill MHFU hunt-reference data: ailment tolerances + trap/tool
/// effectiveness + weakness stagger HP (see
/// `scripts/generate_mh2g_hunt_info.py`). Runs once per database WITHOUT a
/// DATA_VERSION bump (`INSERT OR IGNORE` on natural keys; stagger_hp flows
/// through the weaknesses patch path as part of the extended JSON).
fn patch_mh2g_hunt_info(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_hunt_info";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_monsters: i64 = conn
        .query_row("SELECT COUNT(*) FROM monsters WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_monsters == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    // Stagger HP rides on weakness rows: refresh matched parts, insert rest.
    let json_data = include_str!("../../data/mh2g_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in &weaknesses {
        conn.execute(
            "UPDATE monster_weaknesses SET stagger_hp = COALESCE(stagger_hp, ?3)
             WHERE monster_id = ?1 AND part_name = ?2",
            rusqlite::params![w.monster_id, w.part_name, w.stagger_hp],
        )?;
    }
    seed_monster_weaknesses(conn)?;
    seed_monster_ailments(conn)?;
    seed_monster_tools(conn)?;
    record_patch(conn, NAME)
}

/// Sync MHFU items from the extended `mh2g_items.json` (1175+ new rows +
/// NULL-fills for rarity/sell/buy/carry/description — see
/// `scripts/generate_mh2g_items_backfill.py`) plus NULL combine chances.
/// Runs once per database WITHOUT a DATA_VERSION bump. Fills are
/// NULL-guarded so ISO-fixed prices and curated descriptions are never
/// overwritten.
fn patch_mh2g_items(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_items_backfill";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_items: i64 = conn
        .query_row("SELECT COUNT(*) FROM items WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_items == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    let json_data = include_str!("../../data/mh2g_items.json");
    let items: Vec<ItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_url, icon_name, icon_color, description, language)
             VALUES (?1, 5, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 'en')",
            rusqlite::params![it.id, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_url, it.icon_name, it.icon_color, it.description],
        )?;
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2,
                icon_url = COALESCE(?3, icon_url), icon_name = COALESCE(?4, icon_name), icon_color = COALESCE(?5, icon_color),
                rarity = COALESCE(rarity, ?6), sell_price = COALESCE(sell_price, ?7), buy_price = COALESCE(buy_price, ?8),
                carry_limit = COALESCE(carry_limit, ?9), description = COALESCE(description, ?10)
             WHERE id = ?11 AND game_id = 5",
            rusqlite::params![it.category, it.subcategory, it.icon_url, it.icon_name, it.icon_color, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.description, it.id],
        )?;
    }
    // Upstream descriptions for items that still lack one (curated kept).
    let desc_data = include_str!("../../data/mh2g_item_descriptions.json");
    let descs: Vec<ItemDescJson> = serde_json::from_str(desc_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in descs {
        conn.execute(
            "UPDATE items SET description = ?1 WHERE name = ?2 AND game_id = 5 AND description IS NULL",
            rusqlite::params![d.description, d.name],
        )?;
    }
    // Combine chances filled from twins (NULL rows only).
    let comb_data = include_str!("../../data/mh2g_item_combine.json");
    let recipes: Vec<CombineJson> = serde_json::from_str(comb_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for rc in recipes {
        if rc.chance.is_none() {
            continue;
        }
        conn.execute(
            "UPDATE item_combine SET chance = ?1
             WHERE result_item_id = ?2 AND component_item_id = ?3 AND chance IS NULL",
            rusqlite::params![rc.chance, rc.result_item_id, rc.component_item_id],
        )?;
    }
    record_patch(conn, NAME)
}

/// Sync MHFU quests from the fixed `mh2g_quests.json` (Nekoht-9 hub restore,
/// urgent/key flags, requirements/descriptions fills, location typos, JUMP
/// G3 — see `scripts/fix_mh2g_quests.py`) plus ticket reward rows. Runs once
/// per database WITHOUT a DATA_VERSION bump. Flags/hub/stars/location apply
/// unconditionally by id (JSON is authority); descriptions/requirements fill
/// NULLs only so curated text survives; hub icons refresh on moved quests.
fn patch_mh2g_quests(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_quests_fix";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_quests: i64 = conn
        .query_row("SELECT COUNT(*) FROM quests WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_quests == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    let json_data = include_str!("../../data/mh2g_quests.json");
    let quests: Vec<QuestJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for q in &quests {
        let hub_icon_url = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .map(|s| format!("/icons/mhfu/quests/hubs/{}.png", s))
            .unwrap_or_else(|| "unknown".to_string());
        conn.execute(
            "UPDATE quests SET hub = COALESCE(?1, hub), stars = COALESCE(?2, stars),
                is_key_quest = ?3, is_urgent = ?4,
                requirements = COALESCE(requirements, ?5),
                description = COALESCE(description, ?6),
                location = COALESCE(?7, location),
                hub_icon_name = COALESCE(?1, hub_icon_name),
                hub_icon_url = ?8
             WHERE id = ?9 AND game_id = 5",
            rusqlite::params![
                q.hub,
                q.stars,
                q.is_key_quest.unwrap_or(false),
                q.is_urgent.unwrap_or(false),
                q.requirements,
                q.description,
                q.location,
                hub_icon_url,
                q.id
            ],
        )?;
    }
    let reward_data = include_str!("../../data/mh2g_quest_rewards.json");
    let rewards: Vec<QuestRewardJson> = serde_json::from_str(reward_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rewards {
        // EXISTS guard: legacy/partial DBs (migration tests) may not have the
        // parent quest row yet; the full seed inserts it right after patches.
        conn.execute(
            "INSERT OR IGNORE INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition)
             SELECT ?1, ?2, ?3, ?4, ?5, ?6
             WHERE EXISTS (SELECT 1 FROM quests WHERE id = ?2 AND game_id = 5)
               AND EXISTS (SELECT 1 FROM items WHERE id = ?3 AND game_id = 5)",
            rusqlite::params![r.id, r.quest_id, r.item_id, r.quantity, r.probability, r.condition],
        )?;
    }
    record_patch(conn, NAME)
}

/// Backfill MHFU weapon forge/upgrade split rows from the extended
/// `mh2g_weapon_craft.json` (425 SNS/gunner weapons — see
/// `scripts/generate_mh2g_weapon_craft.py`). Runs once per database WITHOUT
/// a DATA_VERSION bump; replays the seed resolution (name->id, misses
/// skipped) so it is idempotent.
fn patch_mh2g_weapon_craft(conn: &Connection) -> Result<()> {
    const NAME: &str = "mh2g_weapon_craft";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    let have_weapons: i64 = conn
        .query_row("SELECT COUNT(*) FROM weapons WHERE game_id = 5", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    if have_weapons == 0 {
        record_patch(conn, NAME)?;
        return Ok(());
    }
    seed_weapon_craft(conn)?;
    record_patch(conn, NAME)
}

fn patch_item_icons_mhp3rd(conn: &Connection) -> Result<()> {
    const NAME: &str = "item_icons_mhp3rd";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    #[derive(Deserialize)]
    struct Row {
        id: i32,
        category: String,
        subcategory: Option<String>,
    }
    let items: Vec<Row> = serde_json::from_str(include_str!("../../data/mhp3rd_items.json"))
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut stmt = conn.prepare(
        "UPDATE items SET category = ?1, subcategory = ?2, icon_url = COALESCE(icon_url, ?3), icon_name = COALESCE(icon_name, ?4), icon_color = COALESCE(icon_color, ?5) WHERE id = ?6 AND game_id = 4 AND (category IS NULL OR category != ?1 OR subcategory IS NULL OR subcategory != ?2 OR icon_url IS NULL OR icon_name IS NULL)",
    )?;
    for it in &items {
        let (icon_url, icon_name, icon_color) =
            item_icon_from_category(&it.category, it.subcategory.as_deref());
        stmt.execute(rusqlite::params![
            it.category,
            it.subcategory,
            icon_url,
            icon_name,
            icon_color,
            it.id
        ])?;
    }
    record_patch(conn, NAME)
}

fn patch_item_icons_game(conn: &Connection, patch: &str, json: &str, game_id: i32) -> Result<()> {
    if patch_applied(conn, patch)? {
        return Ok(());
    }
    #[derive(Deserialize)]
    struct Row {
        id: i32,
        name: String,
        category: String,
        subcategory: Option<String>,
        icon_name: Option<String>,
        icon_color: Option<String>,
        icon_url: Option<String>,
    }
    let items: Vec<Row> = serde_json::from_str(json)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut stmt = conn.prepare(
        "UPDATE items SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = ?5 AND (icon_url IS NULL OR icon_name IS NULL)",
    )?;
    for it in &items {
        let (icon_url, icon_name, icon_color) = resolve_item_icon(
            it.icon_url.as_deref(),
            it.icon_name.as_deref(),
            it.icon_color.as_deref(),
            &it.category,
            it.subcategory.as_deref(),
            &it.name,
        );
        stmt.execute(rusqlite::params![
            icon_name, icon_color, icon_url, it.id, game_id
        ])?;
    }
    record_patch(conn, patch)
}

fn patch_item_icons_mhr(conn: &Connection) -> Result<()> {
    patch_item_icons_game(
        conn,
        "item_icons_mhr",
        include_str!("../../data/mhr_items.json"),
        MHR,
    )
}

fn patch_item_icons_mhwilds(conn: &Connection) -> Result<()> {
    patch_item_icons_game(
        conn,
        "item_icons_mhwilds",
        include_str!("../../data/mhwilds_items.json"),
        MHWILDS,
    )
}

fn patch_quest_icons_mhr(conn: &Connection) -> Result<()> {
    const NAME: &str = "quest_icons_mhr";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    #[derive(Deserialize)]
    struct Row {
        id: i32,
        hub: Option<String>,
        category: Option<String>,
    }
    let quests: Vec<Row> = serde_json::from_str(include_str!("../../data/mhr_quests.json"))
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut stmt = conn.prepare(
        "UPDATE quests SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3), hub_icon_name = COALESCE(hub_icon_name, ?4), hub_icon_color = COALESCE(hub_icon_color, 'Gray'), hub_icon_url = COALESCE(hub_icon_url, ?5) WHERE id = ?6 AND game_id = 2 AND (icon_url IS NULL OR hub_icon_url IS NULL)",
    )?;
    for q in &quests {
        let qtype = q.category.as_deref().unwrap_or("?");
        let icon_url = format!("/icons/mhfu/quests/{}.png", quest_type_slug(qtype));
        let icon_color = quest_type_color(qtype);
        let hub_name = q.hub.clone().unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", quest_hub_slug(&hub_name));
        stmt.execute(rusqlite::params![
            qtype,
            icon_color,
            icon_url,
            hub_name,
            hub_icon_url,
            q.id
        ])?;
    }
    record_patch(conn, NAME)
}

fn patch_monster_thumbs_rise_wilds(conn: &Connection) -> Result<()> {
    const NAME: &str = "monster_thumbs_rise_wilds";
    if patch_applied(conn, NAME)? {
        return Ok(());
    }
    // Bring every install state to: icon_url = 96px `-sm` variant (lists),
    // icon_url_lg = master portrait (detail pages).
    // 1. Legacy mhw fallback dir -> per-game dir (pre-split installs; no-op otherwise).
    conn.execute(
        "UPDATE monsters SET icon_url = REPLACE(icon_url, '/icons/mhw/monsters/', '/icons/mhr/monsters/') WHERE game_id = 2 AND icon_url LIKE '/icons/mhw/monsters/%'",
        [],
    )?;
    conn.execute(
        "UPDATE monsters SET icon_url = REPLACE(icon_url, '/icons/mhw/monsters/', '/icons/mhwilds/monsters/') WHERE game_id = 3 AND icon_url LIKE '/icons/mhw/monsters/%'",
        [],
    )?;
    // 2. Master -> `-sm` variant for lists (skips rows already on variants).
    conn.execute(
        "UPDATE monsters SET icon_url = REPLACE(icon_url, '.png', '-sm.png') WHERE game_id IN (2, 3) AND icon_url NOT LIKE '%-sm.png'",
        [],
    )?;
    // 3. Fill the large portrait from the variant (skips rows already filled).
    conn.execute(
        "UPDATE monsters SET icon_url_lg = REPLACE(icon_url, '-sm.png', '.png') WHERE game_id IN (2, 3) AND icon_url_lg IS NULL AND icon_url LIKE '%-sm.png'",
        [],
    )?;
    record_patch(conn, NAME)
}

fn seed_games(conn: &Connection) -> Result<()> {
    let games = [
        (1, "Monster Hunter World", "MHW", 2018, "PS4 / XB1 / PC"),
        (2, "Monster Hunter Rise", "MHR", 2021, "Switch / PC"),
        (3, "Monster Hunter Wilds", "MHWilds", 2025, "PS5 / XB / PC"),
        (4, "MH Portable 3rd", "MHP3rd", 2010, "PSP / PS3"),
        (5, "MH 2ndG (Freedom Unite)", "MH2G", 2008, "PSP"),
    ];

    for (id, name, abbr, year, platform) in games {
        conn.execute(
            "INSERT OR IGNORE INTO games (id, name, abbreviation, release_year, platform) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, abbr, year, platform],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct MonsterJson {
    id: i32,
    name: String,
    species: String,
    size: String,
}

fn seed_monsters(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monsters.json");
    let monsters: Vec<MonsterJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for m in &monsters {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhfu/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, icon_name, icon_color, icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, ?7, ?8, 'en')",
            rusqlite::params![m.id, MH2G, m.name, m.species, m.size, m.name, icon_color, icon_url],
        )?;
    }
    for m in &monsters {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhfu/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        let _ = conn.execute(
            "UPDATE monsters SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 5 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![m.name, icon_color, icon_url, m.id],
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct MonsterDescJson {
    name: String,
    description: String,
}

fn backfill_monster_descriptions(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_descriptions.json");
    let descs: Vec<MonsterDescJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for d in descs {
        conn.execute(
            "UPDATE monsters SET description = ?1 WHERE name = ?2 AND game_id = 5 AND description IS NULL",
            rusqlite::params![d.description, d.name],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct ItemJson {
    id: i32,
    name: String,
    category: String,
    subcategory: Option<String>,
    rarity: Option<i32>,
    sell_price: Option<i32>,
    buy_price: Option<i32>,
    #[serde(default)]
    carry_limit: Option<i32>,
    #[serde(default)]
    description: Option<String>,
    icon_url: Option<String>,
    icon_name: Option<String>,
    icon_color: Option<String>,
}

fn seed_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_items.json");
    let items: Vec<ItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for it in &items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_url, icon_name, icon_color, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, NULL, 'en')",
            rusqlite::params![it.id, MH2G, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_url, it.icon_name, it.icon_color],
        )?;
    }

    // Backfill for existing DBs where category/subcategory/icon changed (e.g., Power Juice Material→Consumable, Huskberry→Ammo)
    for it in &items {
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2, icon_url = COALESCE(?3, icon_url), icon_name = COALESCE(?4, icon_name), icon_color = COALESCE(?5, icon_color) WHERE id = ?6 AND game_id = 5 AND (category IS NULL OR category != ?1 OR subcategory IS NULL OR subcategory != ?2 OR icon_url IS NULL)",
            rusqlite::params![it.category, it.subcategory, it.icon_url, it.icon_name, it.icon_color, it.id],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct MonsterEquipJson {
    monster_id: i32,
    kind: String,
    equipment_id: i32,
}

fn seed_monster_equipment(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_equipment.json");
    let rows: Vec<MonsterEquipJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for r in rows {
        conn.execute(
            "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
             VALUES (5, ?1, ?2, ?3)",
            rusqlite::params![r.monster_id, r.kind, r.equipment_id],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct CombineJson {
    result_item_id: i32,
    component_item_id: i32,
    quantity: i32,
    result_quantity: i32,
    combine_type: Option<String>,
    chance: Option<i32>,
}

fn seed_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_item_combine.json");
    let recipes: Vec<CombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for rc in recipes {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity, rc.combine_type.as_deref().unwrap_or("normal"), rc.chance],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct ItemDescJson {
    name: String,
    description: String,
}

fn backfill_item_descriptions(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_item_descriptions.json");
    let descs: Vec<ItemDescJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for d in descs {
        conn.execute(
            "UPDATE items SET description = ?1 WHERE name = ?2 AND game_id = 5 AND description IS NULL",
            rusqlite::params![d.description, d.name],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct DropJson {
    monster_id: i32,
    item_id: i32,
    method: String,
    part: Option<String>,
    rank: Option<String>,
    quantity: i32,
    probability: f64,
    condition: Option<String>,
}

fn seed_monster_drops(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_drops.json");
    let drops: Vec<DropJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for d in drops {
        conn.execute(
            "INSERT OR IGNORE INTO monster_drops
                (monster_id, item_id, method, part, rank, quantity, probability, condition, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
            rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
        )?;
    }

    Ok(())
}

/// Derive the item "How to Obtain" sources from the real monster_drops table so
/// the item detail view shows faithful monster sources.
fn seed_item_sources_from_drops(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability)
        SELECT item_id,
               CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' ELSE method END,
               monster_id, quantity, quantity, probability
        FROM monster_drops;
    ")?;
    Ok(())
}

#[derive(Deserialize)]
struct ExtraSourceJson {
    item_id: i32,
    source_type: String,
    source_id: Option<i32>,
    location: Option<String>,
    probability: Option<f64>,
    conditions: Option<String>,
    quantity_min: Option<i32>,
    quantity_max: Option<i32>,
}

/// Extended sources: gathering/mining/bug/fish + shop (consolidated) + trade (Veggie Elder + Trenya Boat) + Pokke Farm + small monsters
/// Data sourced from mhfu-db (Kolyn090/mhfu-db, MIT) verified against MHP2G game assets and ISO DATA.BIN offsets.
/// Includes maps.json (all gathering nodes), Merchants/*.json (all shops), veggie_elder.json + trenya.json + Farm/* + Monsters/monsters-material.json.
fn seed_extra_item_sources(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_item_sources_extra.json");
    let sources: Vec<ExtraSourceJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sources {
        conn.execute(
            "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location, conditions)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                s.item_id,
                s.source_type,
                s.source_id,
                s.quantity_min.unwrap_or(1),
                s.quantity_max.unwrap_or(1),
                s.probability,
                s.location,
                s.conditions
            ],
        )?;
    }
    Ok(())
}

#[derive(Deserialize)]
struct ExtraCombineJson {
    result_item_id: i32,
    component_item_id: i32,
    quantity: i32,
    result_quantity: i32,
}

fn seed_extra_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_item_combine_extra.json");
    let recs: Vec<ExtraCombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recs {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, 'normal', NULL)",
            rusqlite::params![r.result_item_id, r.component_item_id, r.quantity, r.result_quantity],
        )?;
    }
    Ok(())
}

#[derive(Deserialize)]
struct WeaknessJson {
    monster_id: i32,
    part_name: String,
    sever: Option<i32>,
    blunt: Option<i32>,
    projectile: Option<i32>,
    fire: Option<i32>,
    water: Option<i32>,
    thunder: Option<i32>,
    ice: Option<i32>,
    dragon: Option<i32>,
    #[serde(default)]
    stagger_hp: Option<i32>,
}

fn seed_monster_weaknesses(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon, stagger_hp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon, w.stagger_hp],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct AilmentJson {
    monster_id: i32,
    ailment: String,
    initial: Option<i32>,
    increase: Option<i32>,
    max: Option<i32>,
    decay_step: Option<i32>,
    decay_interval: Option<i32>,
    duration_sec: Option<i32>,
    damage: Option<i32>,
}

fn seed_monster_ailments(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_ailments.json");
    let rows: Vec<AilmentJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for r in rows {
        conn.execute(
            "INSERT OR IGNORE INTO monster_ailments (monster_id, ailment, initial, increase, max, decay_step, decay_interval, duration_sec, damage)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![r.monster_id, r.ailment, r.initial, r.increase, r.max, r.decay_step, r.decay_interval, r.duration_sec, r.damage],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct HuntToolJson {
    monster_id: i32,
    tool: String,
    normal: Option<i32>,
    notfound: Option<i32>,
    enraged: Option<i32>,
}

fn seed_monster_tools(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_monster_tools.json");
    let rows: Vec<HuntToolJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for r in rows {
        conn.execute(
            "INSERT OR IGNORE INTO monster_tools (monster_id, tool, normal, notfound, enraged)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![r.monster_id, r.tool, r.normal, r.notfound, r.enraged],
        )?;
    }

    Ok(())
}

fn weapon_icon_slug(weapon_type: &str) -> &'static str {
    match weapon_type {
        "Great Sword" => "great-sword",
        "Long Sword" => "long-sword",
        "Sword & Shield" | "Sword and Shield" => "sword-and-shield",
        "Dual Blades" => "dual-blades",
        "Hammer" => "hammer",
        "Hunting Horn" => "hunting-horn",
        "Lance" => "lance",
        "Gunlance" => "gunlance",
        "Switch Axe" => "switch-axe",
        "Charge Blade" => "charge-blade",
        "Insect Glaive" => "insect-glaive",
        "Light Bowgun" => "light-bowgun",
        "Heavy Bowgun" => "heavy-bowgun",
        "Bow" => "bow",
        _ => "great-sword",
    }
}

#[allow(dead_code)]
fn weapon_icon_color(rarity: i32) -> &'static str {
    match rarity {
        1..=2 => "Gray",
        3..=4 => "White",
        5..=6 => "Green",
        _ => "Gold",
    }
}

fn weapon_icon_color_mhw(rarity: i32) -> (&'static str, &'static str) {
    // MHWorld & MHWI faithful rarity colors from Help:Item_Colors (HEX per rarity 1-12)
    // R1 AAAAAA, R2 DEDEDE, R3 A1C42E, R4 48AB3F, R5 5CAEBB, R6 595CDA, R7 8D59EF, R8 C76D46, R9 B3436A, R10 0AD5FA, R11 FAC81E, R12 B4F5FF
    let (color, slug) = match rarity {
        1 => ("#AAAAAA", "r1"),
        2 => ("#DEDEDE", "r2"),
        3 => ("#A1C42E", "r3"),
        4 => ("#48AB3F", "r4"),
        5 => ("#5CAEBB", "r5"),
        6 => ("#595CDA", "r6"),
        7 => ("#8D59EF", "r7"),
        8 => ("#C76D46", "r8"),
        9 => ("#B3436A", "r9"),
        10 => ("#0AD5FA", "r10"),
        11 => ("#FAC81E", "r11"),
        12 => ("#B4F5FF", "r12"),
        _ => ("#AAAAAA", "r1"),
    };
    (color, slug)
}

fn weapon_icon_color_mhf2(rarity: i32) -> (&'static str, &'static str) {
    // MHF2 & MHFU (MH2G) faithful: R1-3 EFEFEF, R4 73CE8C, R5 EF94A5, R6 94B5FF, R7 FF9C5A, R8 FF5A5A, R9 FFD65A, R10 AC5CC0
    let (color, slug) = match rarity {
        1..=3 => ("#EFEFEF", "r1"),
        4 => ("#73CE8C", "r4"),
        5 => ("#EF94A5", "r5"),
        6 => ("#94B5FF", "r6"),
        7 => ("#FF9C5A", "r7"),
        8 => ("#FF5A5A", "r8"),
        9 => ("#FFD65A", "r9"),
        10 => ("#AC5CC0", "r10"),
        _ => ("#EFEFEF", "r1"),
    };
    (color, slug)
}

fn weapon_icon_color_mhp3(rarity: i32) -> (&'static str, &'static str) {
    // MHP3 faithful: R1 F5F5F5, R2 B192F1, R3 DED460, R4 E88E9E, R5 70C674, R6 708EF7, R7 DA565A
    let (color, slug) = match rarity {
        1 => ("#F5F5F5", "r1"),
        2 => ("#B192F1", "r2"),
        3 => ("#DED460", "r3"),
        4 => ("#E88E9E", "r4"),
        5 => ("#70C674", "r5"),
        6 => ("#708EF7", "r6"),
        7 => ("#DA565A", "r7"),
        _ => ("#F5F5F5", "r1"),
    };
    (color, slug)
}

fn armor_icon_color_mhf2(rarity: i32) -> (&'static str, &'static str) {
    weapon_icon_color_mhf2(rarity)
}

fn armor_icon_color_mhp3(rarity: i32) -> (&'static str, &'static str) {
    weapon_icon_color_mhp3(rarity)
}

#[derive(Deserialize)]
struct WeaponJson {
    id: i32,
    name: String,
    weapon_type: String,
    rarity: i32,
    attack: i32,
    affinity: i32,
    element_type: Option<String>,
    element_value: Option<i32>,
    sharpness: Option<String>,
    slots: Option<String>,
    status_type: Option<String>,
    status_value: Option<i32>,
    defense_bonus: Option<i32>,
    crafting_cost: Option<i32>,
    upgrade_path: Option<String>,
    description: Option<String>,
    skills: Option<String>,
    sort_order: Option<i32>,
}

fn seed_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhf2(w.rarity);
        let icon_url = format!("/icons/mhfu/weapons/{}-{}.png", slug, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO weapons
                (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                 sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, icon_name, icon_color, icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, 'en')",
            rusqlite::params![
                w.id,
                MH2G,
                w.name,
                w.weapon_type,
                w.rarity,
                w.attack,
                w.affinity,
                w.element_type,
                w.element_value,
                w.sharpness,
                w.slots,
                w.skills,
                w.status_type,
                w.status_value,
                w.defense_bonus,
                w.crafting_cost,
                w.upgrade_path,
                w.description,
                w.weapon_type,
                icon_color,
                icon_url
            ],
        )?;
    }
    // Backfill for existing DBs where icon was NULL or type changed - migrate generic to per-rarity
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhf2(w.rarity);
        let icon_url = format!("/icons/mhfu/weapons/{}-{}.png", slug, color_slug);
        let generic_url = format!("/icons/mhfu/weapons/{}.png", slug);
        conn.execute(
            "UPDATE weapons SET icon_name = COALESCE(icon_name, ?1), icon_color = ?2, icon_url = COALESCE(NULLIF(icon_url, ?3), ?4) WHERE id = ?5 AND game_id = 5",
            rusqlite::params![w.weapon_type, icon_color, generic_url, icon_url, w.id],
        )?;
        // Also ensure existing per-rarity rows get updated if they still have old 4-color Gray/Gold and old 8-color white/yellow
        let _ = conn.execute(
            "UPDATE weapons SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 5 AND (icon_color IN ('Gray','Gold','Green','White','Yellow','Light Blue','Blue','Purple','Orange','Red') OR icon_url = ?4 OR icon_url LIKE '%-white.png' OR icon_url LIKE '%-yellow.png' OR icon_url LIKE '%-green.png')",
            rusqlite::params![icon_color, icon_url, w.id, generic_url],
        );
    }
    // Migrate old 8-color to faithful MHF2 10-rarity HEX (r1..r10)
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhf2(w.rarity);
        let icon_url = format!("/icons/mhfu/weapons/{}-{}.png", slug, color_slug);
        let _ = conn.execute(
            "UPDATE weapons SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 5 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, w.id],
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct WeaponMatJson {
    weapon_id: i32,
    item_id: i32,
    quantity: i32,
}

fn seed_weapon_materials(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_weapon_materials.json");
    let mats: Vec<WeaponMatJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for m in mats {
        conn.execute(
            "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![m.weapon_id, m.item_id, m.quantity],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct WeaponCraftItemJson {
    item: String,
    quantity: i32,
}

#[derive(Deserialize)]
struct WeaponCraftJson {
    weapon_id: i32,
    forge: Vec<WeaponCraftItemJson>,
    upgrade: Vec<WeaponCraftItemJson>,
}

fn seed_weapon_craft(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_weapon_craft.json");
    let rows: Vec<WeaponCraftJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for r in rows {
        for m in &r.forge {
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![m.item],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(iid) = iid {
                conn.execute(
                    "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity)
                     VALUES (?1, 'forge', ?2, ?3)",
                    rusqlite::params![r.weapon_id, iid, m.quantity],
                )?;
            }
        }
        for m in &r.upgrade {
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![m.item],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(iid) = iid {
                conn.execute(
                    "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity)
                     VALUES (?1, 'upgrade', ?2, ?3)",
                    rusqlite::params![r.weapon_id, iid, m.quantity],
                )?;
            }
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct ArmorJson {
    id: i32,
    set: String,
    slot_type: String,
    name: String,
    rank: String,
    rarity: Option<i32>,
    defense_base: Option<i32>,
    defense_max: Option<i32>,
    resistance_fire: Option<i32>,
    resistance_water: Option<i32>,
    resistance_thunder: Option<i32>,
    resistance_ice: Option<i32>,
    resistance_dragon: Option<i32>,
    slots: Option<String>,
    skills: Option<String>,
    armor_type: Option<String>,
    gender: Option<String>,
    crafting_cost: Option<i32>,
    description: Option<String>,
}

fn derive_set_name(armor: &ArmorJson) -> String {
    // For armors where set == name (singleton artifact), derive correct set by stripping slot word.
    // This fixes D/S/U/X/Z variants that were incorrectly split per piece (e.g., Kut-Ku Helm D -> Kut-Ku D).
    if armor.set != armor.name {
        return armor.set.clone();
    }
    // Known slot suffixes (head/chest/arms/waist/legs pieces)
    const SLOT_WORDS: &[&str] = &[
        "Helm",
        "Cap",
        "Crown",
        "Mask",
        "Hat",
        "Hood",
        "Head",
        "Face",
        "Brain",
        "Soul",
        "Horn",
        "Crest",
        "Glare",
        "Snarl",
        "Piercing",
        "Mail",
        "Vest",
        "Jacket",
        "Armor",
        "Hide",
        "Skin",
        "Coat",
        "Plate",
        "Belt",
        "Tasset",
        "Kilt",
        "Coil",
        "Obi",
        "Vambraces",
        "Guards",
        "Guards",
        "Braces",
        "Gloves",
        "Mittens",
        "Greaves",
        "Leggings",
        "Boots",
        "Pants",
        "Legs",
        "Feet",
    ];
    let parts: Vec<&str> = armor.name.split_whitespace().collect();
    if parts.is_empty() {
        return armor.set.clone();
    }
    if parts.len() == 1 {
        // Concatenated names like BlackBeltLeggingsX (no spaces) â€” strip slot suffix
        let name = parts[0];
        for &slot in SLOT_WORDS {
            for &var in &["X", "Z", "S", "U", "D", "C"] {
                let suff = format!("{}{}", slot, var);
                if name
                    .to_ascii_lowercase()
                    .ends_with(&suff.to_ascii_lowercase())
                {
                    let base = name[..name.len() - suff.len()].trim();
                    let derived = if base.is_empty() {
                        slot.to_string()
                    } else {
                        format!("{} {}", base, var)
                    };
                    return derived;
                }
            }
            if name
                .to_ascii_lowercase()
                .ends_with(&slot.to_ascii_lowercase())
            {
                let base = name[..name.len() - slot.len()].trim();
                return if base.is_empty() {
                    slot.to_string()
                } else {
                    base.to_string()
                };
            }
        }
        return armor.set.clone();
    }
    // Check for variant suffix (single letter D/S/U/X/Z or combined like "S" after Helm)
    let last = parts.last().unwrap();
    let has_variant = matches!(*last, "D" | "S" | "U" | "X" | "Z" | "C");
    let slot_idx = if has_variant && parts.len() >= 2 {
        parts.len() - 2
    } else {
        parts.len() - 1
    };
    let slot_word = parts[slot_idx];
    let is_slot = SLOT_WORDS
        .iter()
        .any(|&w| w.eq_ignore_ascii_case(slot_word));
    if !is_slot {
        return armor.set.clone();
    }
    // Build set: all parts except slot word
    let mut out = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if i == slot_idx {
            continue;
        }
        out.push(*p);
    }
    let derived = out.join(" ");
    if derived.is_empty() {
        armor.set.clone()
    } else {
        derived
    }
}

fn seed_armor_sets(conn: &Connection) -> Result<()> {
    // Use Fandom sets for MH2G (Low/High/G × Blademaster/Gunner, separate 2 sets faithful)
    #[derive(Deserialize)]
    struct FandomSet {
        display_name: String,
    }
    let json_data = include_str!("../../data/mhfu_fandom_sets_final.json");
    let fandom_sets: Vec<FandomSet> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // If already seeded with Fandom count, skip destructive delete (idempotent)
    let existing: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM armor_sets WHERE game_id = 5",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if existing as usize != fandom_sets.len() {
        conn.execute("DELETE FROM armor_sets WHERE game_id = 5", [])?;
    }
    for (idx, s) in fandom_sets.iter().enumerate() {
        let id = (idx as i32) + 1;
        conn.execute(
            "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
            rusqlite::params![id, MH2G, s.display_name],
        )?;
        // Ensure name is up to date (for renames)
        let _ = conn.execute(
            "UPDATE armor_sets SET name = ?1 WHERE id = ?2 AND game_id = 5",
            rusqlite::params![s.display_name, id],
        );
    }

    Ok(())
}

#[allow(dead_code)]
fn armor_icon_color(rank: &str) -> &'static str {
    match rank {
        "G" => "Gold",
        "High" => "Blue",
        _ => "Gray",
    }
}

fn quest_type_slug(qtype: &str) -> &'static str {
    match qtype.to_ascii_lowercase().as_str() {
        "hunting" | "hunt" | "hub" | "village" => "hunt",
        "gathering" | "gather" | "collection" => "gather",
        "slaying" | "slay" => "slay",
        "capturing" | "capture" => "capture",
        "training" | "arena" => "training",
        "event" => "event",
        "challenge" => "challenge",
        "?" | "" | "unknown" => "unknown",
        _ => "hunt",
    }
}

fn quest_type_color(qtype: &str) -> &'static str {
    match qtype.to_ascii_lowercase().as_str() {
        "hunting" | "hunt" | "hub" | "village" => "Red",
        "gathering" | "gather" => "Green",
        "slaying" | "slay" => "Orange",
        "capturing" | "capture" => "Blue",
        "training" | "arena" => "Yellow",
        "event" | "challenge" => "Yellow",
        _ => "Gray",
    }
}

fn quest_hub_slug(hub: &str) -> String {
    // normalize hub to slug: elder, nekoto, guild_low -> guild-low etc.
    // MHR hubs (follower/hub_low/hub_high/hub_master/mystery/arena) map onto
    // the existing mhfu hub set — generic icons only, no new art.
    let h = hub.to_ascii_lowercase();
    match h.as_str() {
        "village" | "village_low" | "village_high" => "village".to_string(),
        "guild_low" | "guild_high" | "guild_g" => h.replace('_', "-"),
        "hub_low" => "guild-low".to_string(),
        "hub_high" => "guild-high".to_string(),
        "hub_master" => "guild-g".to_string(),
        "follower" => "nyanta".to_string(),
        "mystery" | "?" | "" | "unknown" => "unknown".to_string(),
        "arena" | "challenge" => "challenge".to_string(),
        "event" => "event".to_string(),
        "training" => "training".to_string(),
        _ => h.replace('_', "-"),
    }
}

fn decoration_skill_icon(skill: &str) -> (&'static str, &'static str) {
    // Returns (file, color) for the 9 ItemIcon017 hue family, per MHP3:_Decoration_List auth table
    // 017i red, 017e teal, 017b cyan, 017f ochre, 017h coral, 017c lavender, 017d mauve, 017a grey-blue, 017g olive
    match skill {
        // 017i - red (Attack/Fire)
        "Attack" | "Fire Res" | "BombStrUp" | "Stinger" | "Potential" | "Fate" | "Draw"
        | "PowerCAdd" | "Spc Attack" => ("ItemIcon017i.png", "Red"),
        // 017e - teal/green (Expert/Defense)
        "Expert" | "Defense" | "Freezer" | "Jumping" | "Steadfast" | "Stun" | "Paralysis"
        | "Transportr" | "Constitutn" => ("ItemIcon017e.png", "Green"),
        // 017b - cyan/blue (Evade/Water)
        "Evade" | "Evade Dist" | "Water Res" | "Resistor" | "Fencing" | "Reload" | "Quickload"
        | "Salvo" | "Sleep" | "Sheathing" | "Trapmaster" | "SleepCAdd" => {
            ("ItemIcon017b.png", "Cyan")
        }
        // 017f - ochre/yellow (Thunder/Stamina)
        "ThunderRes" | "Exhaust" | "Sprinter" | "Gobbler" | "Perceive" | "ParalyCAdd"
        | "SwdShrpner" | "Fatigue" => ("ItemIcon017f.png", "Yellow"),
        // 017h - coral pink (Recovery)
        "Recovery" | "Medicine" | "Rec Speed" | "Antiseptic" | "Hunger" | "Gathering"
        | "Tranquilzr" | "Health" => ("ItemIcon017h.png", "Pink"),
        // 017c - lavender (Disabler/Element)
        "Disabler" | "ElementAtk" | "Dragon Res" | "Heavy Attack" | "Friendship" | "Blessing"
        | "Protection" | "Wide Area" => ("ItemIcon017c.png", "Violet"),
        // 017d - mauve (Charger/KO)
        "Charger" | "KO" | "Metabolism" | "Gambit" | "PsychicVis" | "Carving" | "Precision"
        | "ShortCharg" => ("ItemIcon017d.png", "DarkPurple"),
        // 017a - grey-blue (Guard/Earplug)
        "Guard" | "Guard Up" | "Auto-Guard" | "HearProtct" | "WindPress" | "Quake Res"
        | "Razor" | "Chamber" | "Gunnery" | "PierceSAdd" | "PelletSAdd" => {
            ("ItemIcon017a.png", "Gray")
        }
        // 017g - olive (Antidote/Handicraft)
        "Antidote" | "Prevention" | "Handicraft" | "Footing" | "Professor" | "Sneak"
        | "Terrain" => ("ItemIcon017g.png", "Lime"),
        // fallback by keyword
        _ => {
            let s = skill.to_ascii_lowercase();
            if s.contains("attack") || s.contains("fire") || s.contains("bomb") {
                ("ItemIcon017i.png", "Red")
            } else if s.contains("expert") || s.contains("stun") || s.contains("defense") {
                ("ItemIcon017e.png", "Green")
            } else if s.contains("evade") || s.contains("water") || s.contains("sleep") {
                ("ItemIcon017b.png", "Cyan")
            } else if s.contains("thunder") || s.contains("sprinter") || s.contains("paraly") {
                ("ItemIcon017f.png", "Yellow")
            } else if s.contains("recover") || s.contains("medicine") || s.contains("hunger") {
                ("ItemIcon017h.png", "Pink")
            } else if s.contains("element") || s.contains("dragon") || s.contains("friend") {
                ("ItemIcon017c.png", "Violet")
            } else if s.contains("charger") || s.contains("psychic") || s.contains("ko") {
                ("ItemIcon017d.png", "DarkPurple")
            } else if s.contains("guard") || s.contains("earplug") || s.contains("wind") {
                ("ItemIcon017a.png", "Gray")
            } else if s.contains("handicraft") || s.contains("antidote") {
                ("ItemIcon017g.png", "Lime")
            } else {
                ("ItemIcon017a.png", "Gray")
            }
        }
    }
}

fn monster_icon_slug(name: &str) -> String {
    // "Lao-Shan Lung" -> "lao-shan-lung", "Yian Kut-Ku" -> "yian-kut-ku"
    name.to_ascii_lowercase()
        .replace(' ', "-")
        .replace(['\'', '’'], "")
}

fn monster_icon_color(species: &str) -> &'static str {
    match species {
        "Elder Dragon" => "Gold",
        "Flying Wyvern" => "Red",
        "Fanged Wyvern" => "Orange",
        "Brute Wyvern" => "DarkRed",
        "Carapaceon" => "Orange",
        "Leviathan" => "Blue",
        "Pelagus" => "Green",
        "Lynian" => "Beige",
        "Neopteron" => "Cyan",
        "Herbivore" => "Lime",
        _ => "Gray",
    }
}

/// Reuse map: (category, subcategory) -> (icon_url, icon_name, icon_color).
/// Canonical source is mh2g_items.json (1083 rows, all MH4G-* files verified
/// on disk). MHP3rd/Rise/Wilds rows without upstream icons resolve through
/// here so no new art is needed for known taxonomies. Every URL below must
/// exist under static/icons/mhfu/ — see scripts/audit_missing_icons.py.
fn item_icon_from_category(
    category: &str,
    subcategory: Option<&str>,
) -> (&'static str, &'static str, &'static str) {
    let sub = subcategory.unwrap_or("").trim();
    match (category, sub) {
        // ── Ammo ──
        ("Ammo", "Coating") => (
            "/icons/mhfu/MH4G-Coating_Icon_Purple.png",
            "Coating",
            "Purple",
        ),
        ("Ammo", "Husk") => ("/icons/mhfu/MH4G-Husk_Icon_Grey.png", "Husk", "Grey"),
        ("Ammo", "Para S") => ("/icons/mhfu/MH4G-Shot_Icon_Yellow.png", "Shot", "Yellow"),
        ("Ammo", "Poison S") | ("Ammo", "Posion S") => {
            ("/icons/mhfu/MH4G-Shot_Icon_Purple.png", "Shot", "Purple")
        }
        ("Ammo", "Recov S") => ("/icons/mhfu/MH4G-Shot_Icon_Green.png", "Shot", "Green"),
        ("Ammo", "Sleep S") => (
            "/icons/mhfu/MH4G-Shot_Icon_Light_Blue.png",
            "Shot",
            "Light Blue",
        ),
        ("Ammo", _) => ("/icons/mhfu/MH4G-Shot_Icon_White.png", "Shot", "White"),
        // ── Consumable ──
        ("Consumable", "Buff") => (
            "/icons/mhfu/MH4G-Medicine_Icon_Yellow.png",
            "Medicine",
            "Yellow",
        ),
        ("Consumable", "Charm") => ("/icons/mhfu/MH4G-Sac_Icon_Red.png", "Sac", "Red"),
        ("Consumable", "Cure") => (
            "/icons/mhfu/MH4G-Medicine_Icon_Blue.png",
            "Medicine",
            "Blue",
        ),
        ("Consumable", "Food") => ("/icons/mhfu/MH4G-Meat_Icon_Orange.png", "Meat", "Orange"),
        ("Consumable", "Recovery") => (
            "/icons/mhfu/MH4G-Medicine_Icon_Green.png",
            "Medicine",
            "Green",
        ),
        ("Consumable", "Tool") => (
            "/icons/mhfu/MH4G-Trap_Tool_Icon_Green.png",
            "Trap Tool",
            "Green",
        ),
        ("Consumable", _) => ("/icons/mhfu/MH4G-Sac_Icon_Grey.png", "Sac", "Grey"),
        // ── Material ──
        ("Material", "Bag") => ("/icons/mhfu/MH4G-Sac_Icon_Red.png", "Sac", "Red"),
        ("Material", "Bait") => ("/icons/mhfu/MH4G-Bait_Icon_Green.png", "Bait", "Green"),
        ("Material", "Ball") => ("/icons/mhfu/MH4G-Ball_Icon_White.png", "Ball", "White"),
        ("Material", "Barrel") => (
            "/icons/mhfu/MH4G-Barrel_Icon_Yellow.png",
            "Barrel",
            "Yellow",
        ),
        ("Material", "Bbq-spit") | ("Material", "BBQ") => {
            ("/icons/mhfu/MH4G-BBQ_Icon_Red.png", "BBQ", "Red")
        }
        ("Material", "Binoculars") => (
            "/icons/mhfu/MH4G-Binoculars_Icon_White.png",
            "Binoculars",
            "White",
        ),
        ("Material", "Bomb") => ("/icons/mhfu/MH4G-Bomb_Icon_Yellow.png", "Bomb", "Yellow"),
        ("Material", "Book") => ("/icons/mhfu/MH4G-Book_Icon_Yellow.png", "Book", "Yellow"),
        ("Material", "Boomerang") => (
            "/icons/mhfu/MH4G-Boomerang_Icon_Yellow.png",
            "Boomerang",
            "Yellow",
        ),
        ("Material", "Bone") => ("/icons/mhfu/MH4G-Bone_Icon_Grey.png", "Bone", "Grey"),
        ("Material", "Bottle") => ("/icons/mhfu/MH4G-Medicine_Icon_Red.png", "Medicine", "Red"),
        ("Material", "Bugnet") => ("/icons/mhfu/MH4G-Bugnet_Icon_Grey.png", "Bugnet", "Grey"),
        ("Material", "Carapaceon-shell") | ("Material", "Carapace") | ("Material", "Shell") => (
            "/icons/mhfu/FourthGen-Shell_Icon_Orange.png",
            "Shell",
            "Orange",
        ),
        ("Material", "Dung") => ("/icons/mhfu/MH4G-Dung_Icon_Brown.png", "Dung", "Brown"),
        ("Material", "Egg") => ("/icons/mhfu/MH4G-Egg_Icon_White.png", "Egg", "White"),
        ("Material", "Fish") => ("/icons/mhfu/MH4G-Fish_Icon_Yellow.png", "Fish", "Yellow"),
        ("Material", "Flute") | ("Material", "Horn") => {
            ("/icons/mhfu/MH4G-Horn_Icon_Yellow.png", "Horn", "Yellow")
        }
        ("Material", "Insect") | ("Material", "Bug") => (
            "/icons/mhfu/MH4G-Spiderweb_Icon_White.png",
            "Spiderweb",
            "White",
        ),
        ("Material", "Jewel")
        | ("Material", "Gem")
        | ("Material", "Mantle")
        | ("Material", "Streamstone") => (
            "/icons/mhfu/MH4G-Jewel_Icon_Light_Blue.png",
            "Jewel",
            "Light Blue",
        ),
        ("Material", "Knife") => ("/icons/mhfu/MH4G-Knife_Icon_Purple.png", "Knife", "Purple"),
        ("Material", "Map") => ("/icons/mhfu/MH4G-Map_Icon_White.png", "Map", "White"),
        ("Material", "Meat") | ("Material", "Animal") => {
            ("/icons/mhfu/MH4G-Meat_Icon_Red.png", "Meat", "Red")
        }
        ("Material", "Monster Material") | ("Material", "Monster") | ("Material", "Scale") => {
            ("/icons/mhfu/MH4G-Scale_Icon_Grey.png", "Scale", "Grey")
        }
        ("Material", "Ore") | ("Material", "Armor Sphere") | ("Material", "Coin") => {
            ("/icons/mhfu/MH4G-Ore_Icon_Grey.png", "Ore", "Grey")
        }
        ("Material", "Pickaxe") => ("/icons/mhfu/MH4G-Pickaxe_Icon_Grey.png", "Pickaxe", "Grey"),
        ("Material", "Plant")
        | ("Material", "Herb")
        | ("Material", "Seed")
        | ("Material", "Mushroom") => ("/icons/mhfu/MH4G-Seed_Icon_Green.png", "Seed", "Green"),
        ("Material", "Smoke") => ("/icons/mhfu/MH4G-Smoke_Icon_White.png", "Smoke", "White"),
        ("Material", "Ticket")
        | ("Material", "Account")
        | ("Material", "Voucher")
        | ("Material", "Certificate") => {
            ("/icons/mhfu/MH4G-Ticket_Icon_White.png", "Ticket", "White")
        }
        ("Material", "Tool") => (
            "/icons/mhfu/MH4G-Trap_Tool_Icon_Green.png",
            "Trap Tool",
            "Green",
        ),
        ("Material", "Trap") => ("/icons/mhfu/MH4G-Trap_Icon_Green.png", "Trap", "Green"),
        ("Material", "Whetstone") => (
            "/icons/mhfu/MH4G-Whetstone_Icon_Yellow.png",
            "Whetstone",
            "Yellow",
        ),
        ("Material", "Claw") | ("Material", "Fang") | ("Material", "Jaw") => {
            ("/icons/mhfu/MH4G-Claw_Icon_Grey.png", "Claw", "Grey")
        }
        ("Material", "Hide")
        | ("Material", "Wing")
        | ("Material", "Tail")
        | ("Material", "Body") => ("/icons/mhfu/MH4G-Hide_Icon_White.png", "Hide", "White"),
        ("Material", "Sac") => ("/icons/mhfu/MH4G-Sac_Icon_Red.png", "Sac", "Red"),
        ("Material", "Room") => ("/icons/mhfu/MH4G-Map_Icon_White.png", "Map", "White"),
        ("Material", _) => (
            "/icons/mhfu/MH4G-Pickaxe_Icon_White.png",
            "Pickaxe",
            "White",
        ),
        // ── Misc (MHW account/supply/trade buckets reuse ticket) ──
        ("Misc", _) => ("/icons/mhfu/MH4G-Ticket_Icon_White.png", "Ticket", "White"),
        _ => (
            "/icons/mhfu/MH4G-Question_Mark_Icon_Grey.png",
            "Unknown",
            "Grey",
        ),
    }
}

/// Keyword fallback for rows whose subcategory is "?" or empty (Rise/Wilds).
/// Inspects the item name and recovers the equivalent subcategory icon.
/// `category` only selects the generic fallback when no keyword hits.
fn item_icon_from_name(name: &str, category: &str) -> (&'static str, &'static str, &'static str) {
    let n = name.to_ascii_lowercase();
    let has = |words: &[&str]| words.iter().any(|w| n.contains(w));
    // Cure / recovery first (before generic medicine/buff overlap)
    if has(&[
        "antidote",
        "nulberry",
        "cleanser",
        "deodorant",
        "herbal",
        "antibody",
        "energy drink",
    ]) {
        return (
            "/icons/mhfu/MH4G-Medicine_Icon_Blue.png",
            "Medicine",
            "Blue",
        );
    }
    if has(&[
        "potion",
        "lifepowder",
        "dust of life",
        "immunizer",
        "ration",
        "herb",
        "honey",
        "bitterbug",
        "godbug",
    ]) {
        // food words are checked below; potions/herbs are recovery
        if !has(&["steak", "meat", "bbq", "ration"]) {
            return (
                "/icons/mhfu/MH4G-Medicine_Icon_Green.png",
                "Medicine",
                "Green",
            );
        }
    }
    if has(&[
        "demon",
        "armor",
        "powercharm",
        "powertalon",
        "armorcharm",
        "armortalon",
        "adamant",
        "might",
        "immunizer",
        "mega juice",
        "power juice",
        "dash juice",
        "pill",
        "seed",
        "powder",
        "drug",
    ]) {
        return (
            "/icons/mhfu/MH4G-Medicine_Icon_Yellow.png",
            "Medicine",
            "Yellow",
        );
    }
    if has(&["steak", "meat", "bbq", "ration", "egg", "milk", "cheese"]) {
        if has(&["egg"]) {
            return ("/icons/mhfu/MH4G-Egg_Icon_White.png", "Egg", "White");
        }
        return ("/icons/mhfu/MH4G-Meat_Icon_Orange.png", "Meat", "Orange");
    }
    if has(&["coating"]) {
        return (
            "/icons/mhfu/MH4G-Coating_Icon_Purple.png",
            "Coating",
            "Purple",
        );
    }
    if has(&[
        "ammo", "pellet", "pierce", "crag", "clust", "spread", "shrapnel", "sticky", "slicing",
        "wyvern", "ballista", "cannon",
    ]) {
        return ("/icons/mhfu/MH4G-Shot_Icon_White.png", "Shot", "White");
    }
    if has(&["husk"]) {
        return ("/icons/mhfu/MH4G-Husk_Icon_Grey.png", "Husk", "Grey");
    }
    if has(&["ticket", "voucher", "certificate", "coin", "pass", "coupon"]) {
        return ("/icons/mhfu/MH4G-Ticket_Icon_White.png", "Ticket", "White");
    }
    if has(&[
        "ore",
        "crystal",
        "sphere",
        "stone",
        "coal",
        "iron",
        "machalite",
        "dragonite",
        "carbalite",
        "novacrystal",
        "firestone",
        "ice crystal",
    ]) {
        return ("/icons/mhfu/MH4G-Ore_Icon_Grey.png", "Ore", "Grey");
    }
    if has(&["bone", "skull", "jaw", "marrow", "medulla"]) {
        return ("/icons/mhfu/MH4G-Bone_Icon_Grey.png", "Bone", "Grey");
    }
    if has(&["fang", "claw", "talon"]) {
        return ("/icons/mhfu/MH4G-Claw_Icon_Grey.png", "Claw", "Grey");
    }
    if has(&[
        "scale", "carapace", "shell", "cortex", "rib", "fin", "ridge",
    ]) {
        if has(&["carapace", "shell", "cortex"]) {
            return (
                "/icons/mhfu/FourthGen-Shell_Icon_Orange.png",
                "Shell",
                "Orange",
            );
        }
        return ("/icons/mhfu/MH4G-Scale_Icon_Grey.png", "Scale", "Grey");
    }
    if has(&[
        "hide", "pelt", "fur", "skin", "wing", "tail", "tentacle", "beak", "feather",
    ]) {
        return ("/icons/mhfu/MH4G-Hide_Icon_White.png", "Hide", "White");
    }
    if has(&["horn", "antler", "tusk"]) {
        return ("/icons/mhfu/MH4G-Horn_Icon_Yellow.png", "Horn", "Yellow");
    }
    if has(&["sac", "gland", "organ", "bladder"]) {
        return ("/icons/mhfu/MH4G-Sac_Icon_Red.png", "Sac", "Red");
    }
    if has(&[
        "jewel",
        "gem",
        "mantle",
        "plate",
        "ruby",
        "streamstone",
        "bloodstone",
        "dragonstone",
    ]) {
        return (
            "/icons/mhfu/MH4G-Jewel_Icon_Light_Blue.png",
            "Jewel",
            "Light Blue",
        );
    }
    if has(&[
        "mushroom", "herb", "plant", "flower", "root", "moss", "cactus", "ivy", "sap", "leaf",
        "blossom", "seed", "nut", "berry", "kelbi",
    ]) {
        return ("/icons/mhfu/MH4G-Seed_Icon_Green.png", "Seed", "Green");
    }
    if has(&[
        "bug",
        "beetle",
        "cricket",
        "locust",
        "worm",
        "insect",
        "webbing",
        "spiderweb",
        "honey",
        "stinkhopper",
    ]) {
        return (
            "/icons/mhfu/MH4G-Spiderweb_Icon_White.png",
            "Spiderweb",
            "White",
        );
    }
    if has(&[
        "fish",
        "tuna",
        "trout",
        "eel",
        "whetfish",
        "sushifish",
        "burst",
        "apex",
    ]) {
        return ("/icons/mhfu/MH4G-Fish_Icon_Yellow.png", "Fish", "Yellow");
    }
    if has(&["bomb", "barrel", "gunpowder", "blast"]) {
        return ("/icons/mhfu/MH4G-Bomb_Icon_Yellow.png", "Bomb", "Yellow");
    }
    if has(&["trap", "pitfall", "shock"]) {
        return ("/icons/mhfu/MH4G-Trap_Icon_Green.png", "Trap", "Green");
    }
    if has(&["knife", "throwing", "boomerang"]) {
        return ("/icons/mhfu/MH4G-Knife_Icon_Purple.png", "Knife", "Purple");
    }
    if has(&["book", "guide", "manual", "map", "organizer"]) {
        return ("/icons/mhfu/MH4G-Book_Icon_Yellow.png", "Book", "Yellow");
    }
    if has(&["charm", "talisman"]) {
        return ("/icons/mhfu/MH4G-Sac_Icon_Red.png", "Sac", "Red");
    }
    if has(&["smoke", "poison smoke", "sleep smoke"]) {
        return ("/icons/mhfu/MH4G-Smoke_Icon_White.png", "Smoke", "White");
    }
    if has(&["dung"]) {
        return ("/icons/mhfu/MH4G-Dung_Icon_Brown.png", "Dung", "Brown");
    }
    if has(&["ball", "paintball", "flash bomb", "sonic"]) {
        return ("/icons/mhfu/MH4G-Ball_Icon_White.png", "Ball", "White");
    }
    // Generic fallback per top-level category
    match category {
        "Consumable" => (
            "/icons/mhfu/MH4G-Medicine_Icon_Green.png",
            "Medicine",
            "Green",
        ),
        "Ammo" => ("/icons/mhfu/MH4G-Shot_Icon_White.png", "Shot", "White"),
        _ => (
            "/icons/mhfu/MH4G-Pickaxe_Icon_White.png",
            "Pickaxe",
            "White",
        ),
    }
}

/// Resolve an item icon, preferring upstream icon_* when present (MH2G/MHW),
/// else the category reuse map, else the name-keyword fallback (Rise/Wilds "?").
fn resolve_item_icon(
    upstream_url: Option<&str>,
    upstream_name: Option<&str>,
    upstream_color: Option<&str>,
    category: &str,
    subcategory: Option<&str>,
    item_name: &str,
) -> (String, String, String) {
    if let (Some(u), Some(nm), Some(c)) = (upstream_url, upstream_name, upstream_color) {
        if !u.is_empty() {
            return (u.to_string(), nm.to_string(), c.to_string());
        }
    }
    let sub = subcategory.unwrap_or("");
    if !sub.is_empty() && sub != "?" {
        let (u, nm, c) = item_icon_from_category(category, subcategory);
        return (u.to_string(), nm.to_string(), c.to_string());
    }
    let (u, nm, c) = item_icon_from_name(item_name, category);
    (u.to_string(), nm.to_string(), c.to_string())
}

fn seed_armor(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // Build Fandom set_id map for MH2G (display_name -> id, armor_id -> set_id) + hunter_type for armor_type
    #[derive(Deserialize)]
    struct FandomSet {
        hunter_type: String,
        pieces: Vec<FandomPiece>,
    }
    #[derive(Deserialize)]
    struct FandomPiece {
        armor_id: Option<i32>,
    }
    let fandom_data = include_str!("../../data/mhfu_fandom_sets_final.json");
    let fandom_sets: Vec<FandomSet> = serde_json::from_str(fandom_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut armor_to_set: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut armor_to_type: std::collections::HashMap<i32, String> =
        std::collections::HashMap::new();
    for (idx, s) in fandom_sets.iter().enumerate() {
        let set_id = (idx as i32) + 1;
        let atype = match s.hunter_type.as_str() {
            "blademaster" => "blade",
            "gunner" => "gunner",
            _ => "both",
        };
        for p in &s.pieces {
            if let Some(aid) = p.armor_id {
                armor_to_set.entry(aid).or_insert(set_id);
                armor_to_type
                    .entry(aid)
                    .or_insert_with(|| atype.to_string());
            }
        }
    }
    // Fallback for armors not in Fandom (e.g., some event armors): use derive
    let mut set_map: Vec<(String, i32)> = Vec::new();
    let mut set_id_fallback: i32 = fandom_sets.len() as i32;
    let mut derive_cache: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let get_fallback_set = |armor: &ArmorJson,
                            map: &mut Vec<(String, i32)>,
                            cache: &mut std::collections::HashMap<String, i32>,
                            fallback_id: &mut i32|
     -> i32 {
        let name = derive_set_name(armor);
        if let Some(&id) = cache.get(&name) {
            return id;
        }
        if let Some((_, id)) = map.iter().find(|(s, _)| s == &name) {
            cache.insert(name.clone(), *id);
            return *id;
        }
        *fallback_id += 1;
        map.push((name.clone(), *fallback_id));
        cache.insert(name.clone(), *fallback_id);
        *fallback_id
    };
    // For initial insert we need set_id per armor, but we will compute per armor
    for a in &armors {
        let gender = a.gender.clone().unwrap_or_else(|| "both".to_string());
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhf2(rarity);
        let icon_url = format!("/icons/mhfu/armor/{}-{}.png", a.slot_type, color_slug);
        let set_id = if let Some(&sid) = armor_to_set.get(&a.id) {
            sid
        } else {
            // fallback derive (for armors not in Fandom, e.g., some event)
            get_fallback_set(a, &mut set_map, &mut derive_cache, &mut set_id_fallback)
        };
        let armor_type = if let Some(t) = armor_to_type.get(&a.id) {
            t.clone()
        } else {
            a.armor_type.clone().unwrap_or_else(|| "both".to_string())
        };
        conn.execute(
            "INSERT OR IGNORE INTO armor
                (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                 resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                 slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, 'en')",
            rusqlite::params![
                a.id, MH2G, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, set_id, armor_type, gender, a.crafting_cost, a.description,
                a.slot_type, icon_color, icon_url
            ],
        )?;
    }
    // Reassign all MH2G armor set_id and armor_type to Fandom grouping (idempotent)
    // For Leather/Chain, keep Green/Blue Pants independent as per user preference (exclude them from Fandom assignment)
    let exclude_ids: std::collections::HashSet<i32> = [1662, 1663].iter().cloned().collect(); // Green Pants, Blue Pants
    for (idx, s) in fandom_sets.iter().enumerate() {
        let set_id = (idx as i32) + 1;
        let atype = match s.hunter_type.as_str() {
            "blademaster" => "blade",
            "gunner" => "gunner",
            _ => "both",
        };
        for p in &s.pieces {
            if let Some(aid) = p.armor_id {
                if exclude_ids.contains(&aid) {
                    continue;
                }
                let _ = conn.execute(
                    "UPDATE armor SET set_id = ?1, armor_type = ?2 WHERE id = ?3 AND game_id = 5",
                    rusqlite::params![set_id, atype, aid],
                );
            }
        }
    }
    // For any MH2G armor not in Fandom sets, ensure it is not stuck in a wrong Fandom set (e.g., Giaprey Gloves in Bone)
    for a in &armors {
        if !armor_to_set.contains_key(&a.id) && !exclude_ids.contains(&a.id) {
            let current_set: Option<i32> = conn
                .query_row(
                    "SELECT set_id FROM armor WHERE id = ?1 AND game_id = 5",
                    rusqlite::params![a.id],
                    |r| r.get(0),
                )
                .optional()?
                .flatten();
            if let Some(cs) = current_set {
                // If current set is a Fandom set (id <= fandom_sets.len()), it shouldn't contain this armor
                if cs <= fandom_sets.len() as i32 {
                    let fallback =
                        get_fallback_set(a, &mut set_map, &mut derive_cache, &mut set_id_fallback);
                    let _ = conn.execute(
                        "UPDATE armor SET set_id = ?1, armor_type = ?2 WHERE id = ?3 AND game_id = 5",
                        rusqlite::params![fallback, a.armor_type.clone().unwrap_or_else(|| "both".to_string()), a.id],
                    );
                    // Ensure fallback set exists
                    let set_name = derive_set_name(a);
                    let exists: Option<i32> = conn
                        .query_row(
                            "SELECT id FROM armor_sets WHERE name = ?1 AND game_id = 5",
                            rusqlite::params![set_name],
                            |r| r.get(0),
                        )
                        .optional()?
                        .flatten();
                    if exists.is_none() {
                        let _ = conn.execute(
                            "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
                            rusqlite::params![fallback, MH2G, set_name],
                        );
                    }
                }
            }
        }
    }
    // Ensure Green/Blue Pants remain in their own independent sets (fallback)
    for aid in exclude_ids {
        // Find armor for this id
        if let Some(armor) = armors.iter().find(|x| x.id == aid) {
            let fallback =
                get_fallback_set(armor, &mut set_map, &mut derive_cache, &mut set_id_fallback);
            let _ = conn.execute(
                "UPDATE armor SET set_id = ?1 WHERE id = ?2 AND game_id = 5",
                rusqlite::params![fallback, aid],
            );
            // Ensure the fallback set exists in armor_sets
            let set_name = derive_set_name(armor);
            let exists: Option<i32> = conn
                .query_row(
                    "SELECT id FROM armor_sets WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![set_name],
                    |r| r.get(0),
                )
                .optional()?
                .flatten();
            if exists.is_none() {
                let _ = conn.execute(
                    "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
                    rusqlite::params![fallback, MH2G, set_name],
                );
            }
        }
    }
    // Final validation: ensure no set contains pieces that don't belong (e.g., Bone set should not contain Giaprey)
    // For each MH2G armor, verify its set's name contains its prefix
    let all_armor_sets: Vec<(i32, String)> = conn
        .prepare("SELECT id, name FROM armor_sets WHERE game_id = 5")?
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .filter_map(|r| r.ok())
        .collect();
    let set_name_map: std::collections::HashMap<i32, String> = all_armor_sets.into_iter().collect();
    let mut to_fix: Vec<(i32, i32)> = Vec::new(); // (armor_id, correct_set_id)
    for a in &armors {
        let current_set: Option<i32> = conn
            .query_row(
                "SELECT set_id FROM armor WHERE id = ?1 AND game_id = 5",
                rusqlite::params![a.id],
                |r| r.get(0),
            )
            .optional()?
            .flatten();
        if let Some(cs) = current_set {
            if let Some(sn) = set_name_map.get(&cs) {
                // Check if armor name's prefix matches set name's first word
                let armor_prefix = a
                    .name
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_lowercase();
                let set_prefix = sn.split_whitespace().next().unwrap_or("").to_lowercase();
                // For sets like "Leather Armor Low", prefix is Leather, armor Leather Helm matches
                // For Bone, armor Giaprey Gloves has prefix Giaprey, set Bone has prefix Bone -> mismatch
                if !sn.to_lowercase().contains(&armor_prefix)
                    && !a.name.to_lowercase().contains(&set_prefix)
                {
                    // Need to find correct set for this armor via derive or Fandom
                    let correct = if let Some(&sid) = armor_to_set.get(&a.id) {
                        sid
                    } else {
                        get_fallback_set(a, &mut set_map, &mut derive_cache, &mut set_id_fallback)
                    };
                    if correct != cs {
                        to_fix.push((a.id, correct));
                    }
                }
            }
        }
    }
    for (aid, correct_sid) in to_fix {
        let _ = conn.execute(
            "UPDATE armor SET set_id = ?1 WHERE id = ?2 AND game_id = 5",
            rusqlite::params![correct_sid, aid],
        );
    }
    // Backfill existing DBs - migrate rank-based gray to faithful rarity HEX r1..r10
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhf2(rarity);
        let icon_url = format!("/icons/mhfu/armor/{}-{}.png", a.slot_type, color_slug);
        let generic = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        let _ = conn.execute(
            "UPDATE armor SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(NULLIF(icon_url, ?3), ?4) WHERE id = ?5 AND game_id = 5 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![a.slot_type, icon_color, generic, icon_url, a.id],
        );
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 5 AND (icon_color IN ('Gray','Gold','Blue','White','Yellow','Light Blue','Green','Purple','Orange','Red') OR icon_url = ?4 OR icon_url LIKE '%-white.png')",
            rusqlite::params![icon_color, icon_url, a.id, generic],
        );
    }
    // Faithful migration: ensure all existing rows get corrected HEX
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhf2(rarity);
        let icon_url = format!("/icons/mhfu/armor/{}-{}.png", a.slot_type, color_slug);
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 5 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, a.id],
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct ArmorMatJson {
    armor_id: i32,
    item_id: i32,
    quantity: i32,
}

fn seed_armor_materials(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_armor_materials.json");
    let mats: Vec<ArmorMatJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for m in mats {
        conn.execute(
            "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![m.armor_id, m.item_id, m.quantity],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct QuestJson {
    id: i32,
    name: String,
    name_original: Option<String>,
    #[serde(rename = "type")]
    qtype: String,
    rank: String,
    hub: Option<String>,
    category: Option<String>,
    stars: Option<i32>,
    objective: String,
    objective_original: Option<String>,
    location: String,
    location_original: Option<String>,
    time_limit: Option<i32>,
    faints_allowed: Option<i32>,
    is_key_quest: Option<bool>,
    is_urgent: Option<bool>,
    description: Option<String>,
    description_original: Option<String>,
    client: Option<String>,
    requirements: Option<String>,
    reward_money: Option<i32>,
    contract_fee: Option<i32>,
    main_monsters: Option<Vec<String>>,
}

fn seed_quests(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_quests.json");
    let quests: Vec<QuestJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for q in &quests {
        let main_monsters_json = q
            .main_monsters
            .as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhfu/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", hub_slug);
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, name_original, type, rank, hub, stars, objective, objective_original, location, location_original, time_limit, faints_allowed, is_key_quest, is_urgent, description, description_original, client, requirements, reward_money, contract_fee, main_monsters, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, 'en')",
            rusqlite::params![
                q.id,
                MH2G,
                q.name,
                q.name_original,
                q.qtype,
                q.rank,
                q.hub,
                q.stars,
                q.objective,
                q.objective_original.as_deref().unwrap_or(&q.objective),
                q.location,
                q.location_original.as_deref().unwrap_or(&q.location),
                q.time_limit.unwrap_or(50),
                q.faints_allowed.unwrap_or(3),
                q.is_key_quest.unwrap_or(false),
                q.is_urgent.unwrap_or(false),
                q.description,
                q.description_original.as_deref().unwrap_or(q.description.as_deref().unwrap_or("")),
                q.client,
                q.requirements,
                q.reward_money,
                q.contract_fee,
                main_monsters_json,
                q.qtype,
                icon_color,
                icon_url,
                q.hub.clone().unwrap_or_else(|| "unknown".to_string()),
                "Gray",
                hub_icon_url
            ],
        )?;
    }
    // Backfill for existing DBs (EN overwrite + preserve original) — reuse parsed list; log if second parse unexpectedly fails.
    let backfill: Vec<QuestJson> =
        match serde_json::from_str(include_str!("../../data/mh2g_quests.json")) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[seed] mh2g_quests backfill parse failed: {}", e);
                Vec::new()
            }
        };
    for q in backfill {
        let _ = conn.execute(
            "UPDATE quests SET objective = ?1, objective_original = COALESCE(objective_original, ?2), location = ?3, location_original = COALESCE(location_original, ?4), description = COALESCE(?, description), description_original = COALESCE(description_original, ?) WHERE id = ?5 AND game_id = 5",
            rusqlite::params![q.objective, q.objective_original.as_deref().unwrap_or(&q.objective), q.location, q.location_original.as_deref().unwrap_or(&q.location), q.description, q.description_original.as_deref().unwrap_or(q.description.as_deref().unwrap_or("")), q.id],
        );
    }
    // Backfill icons for existing DBs where icon was NULL
    for q in &quests {
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhfu/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", hub_slug);
        let _ = conn.execute(
            "UPDATE quests SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3), hub_icon_name = COALESCE(hub_icon_name, ?4), hub_icon_color = COALESCE(hub_icon_color, 'Gray'), hub_icon_url = COALESCE(hub_icon_url, ?5) WHERE id = ?6 AND game_id = 5 AND (icon_url IS NULL OR hub_icon_url IS NULL)",
            rusqlite::params![q.qtype, icon_color, icon_url, q.hub.clone().unwrap_or_else(|| "unknown".to_string()), hub_icon_url, q.id],
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct QuestRewardJson {
    id: i32,
    quest_id: i32,
    item_id: i32,
    quantity: i32,
    probability: Option<f64>,
    condition: Option<String>,
}

fn seed_quest_rewards(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_quest_rewards.json");
    let rewards: Vec<QuestRewardJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for r in rewards {
        conn.execute(
            "INSERT OR IGNORE INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![r.id, r.quest_id, r.item_id, r.quantity, r.probability, r.condition],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct SkillFamilyJson {
    id: i32,
    name: String,
    description: String,
    max_level: Option<i32>,
}

fn seed_skills(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_skills_new.json");
    let skills: Vec<SkillFamilyJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language)
             VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![s.id, MH2G, s.name, s.description, s.max_level],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct SkillLevelJson {
    id: i32,
    skill_id: i32,
    points: i32,
    ability_name: String,
    description: Option<String>,
}

fn seed_skill_levels(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_skill_levels.json");
    let levels: Vec<SkillLevelJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO skill_levels (id, skill_id, points, ability_name, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![l.id, l.skill_id, l.points, l.ability_name, l.description],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct DecoSkillPointJson {
    name: String,
    points: i32,
}

#[derive(Deserialize)]
struct DecoMatJson {
    name: String,
    amount: i32,
}

#[derive(Deserialize)]
struct DecoJson {
    id: i32,
    name: String,
    slot_size: i32,
    rarity: Option<i32>,
    price: Option<i32>,
    skill_points: Vec<DecoSkillPointJson>,
    materials: Vec<DecoMatJson>,
}

fn seed_decorations(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_decorations.json");
    let decos: Vec<DecoJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for d in decos {
        // Resolve primary and secondary skill ids
        let mut primary_id: Option<i32> = None;
        let mut primary_pts: Option<i32> = None;
        let mut secondary_id: Option<i32> = None;
        let mut secondary_pts: Option<i32> = None;

        for (idx, sp) in d.skill_points.iter().enumerate() {
            let normalized = normalize_skill_name(&sp.name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if idx == 0 {
                primary_id = sid;
                primary_pts = Some(sp.points);
            } else if idx == 1 {
                secondary_id = sid;
                secondary_pts = Some(sp.points);
            }
        }

        // Skip if primary not resolved (should not happen for faithful data)
        if primary_id.is_none() {
            continue;
        }

        let primary_skill_name = d
            .skill_points
            .first()
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        let (icon_file, icon_color) = decoration_skill_icon(primary_skill_name);
        let icon_url = format!("/icons/mhfu/decorations/{}", icon_file);
        let icon_name = primary_skill_name.to_string();
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, icon_name, icon_color, icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![
                d.id,
                MH2G,
                d.name,
                primary_id,
                primary_pts,
                primary_pts,
                secondary_id,
                secondary_pts,
                d.slot_size,
                d.price,
                icon_name,
                icon_color,
                icon_url
            ],
        )?;
        // Backfill existing DBs where icon was NULL or slot-based placeholder (migrate to skill hue)
        let _ = conn.execute(
            "UPDATE decorations SET icon_name = ?1, icon_color = ?2, icon_url = ?3 WHERE id = ?4 AND game_id = 5 AND (icon_url IS NULL OR icon_url LIKE '%slot-%' OR icon_name LIKE 'Slot %')",
            rusqlite::params![icon_name, icon_color, icon_url, d.id],
        );
        let _ = conn.execute(
            "UPDATE decorations SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 5 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![icon_name, icon_color, icon_url, d.id],
        );

        // Insert crafting materials
        for m in &d.materials {
            let normalized_mat = normalize_item_name(&m.name);
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![normalized_mat],
                    |row| row.get(0),
                )
                .optional()?;
            if iid.is_none() {
                // Never insert a NULL FK — skip unresolved rather than making an orphan.
                continue;
            }
            conn.execute(
                "INSERT OR IGNORE INTO decoration_materials (decoration_id, item_id, item_name, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![d.id, iid, m.name, m.amount],
            )?;
        }
    }

    Ok(())
}

fn normalize_item_name(name: &str) -> String {
    match name {
        "Crag S Lv3" => "Crag S Lvl3".to_string(),
        "Crag S Lv2" => "Crag S Lvl2".to_string(),
        "Crag S Lv1" => "Crag S Lvl1".to_string(),
        _ => name.to_string(),
    }
}

/// MHP3rd decoration/armor material names are used verbatim (Athena/match data), no MH2G aliases.
fn normalize_item_name_p3rd(name: &str) -> String {
    name.trim().to_string()
}

fn normalize_skill_name(name: &str) -> String {
    match name {
        "ClustS Add" | "ClustSAdd" => "ClustSAdd".to_string(),
        "Crag S Add" | "CragSAdd" => "CragSAdd".to_string(),
        "WindPress" => "Wind Press".to_string(),
        "PelletS Add" => "PelletSAdd".to_string(),
        "NormalS Add" => "NormalSAdd".to_string(),
        "PierceS Add" => "PierceSAdd".to_string(),
        "ThunderRes" => "ThunderRes".to_string(),
        _ => name.to_string(),
    }
}

/// MHP3rd skill families keep their exact Athena spelling (e.g. `WindPress`, not `Wind Press`),
/// so we must NOT apply the MH2G alias normalizations. Only trim whitespace.
fn normalize_skill_name_p3rd(name: &str) -> String {
    name.trim().to_string()
}

fn parse_skill_string(s: &str) -> Vec<(String, i32)> {
    // Input like "Attack +3, Defense -2, Hunger +5"
    let mut out = Vec::new();
    for part in s.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        // split on last space before +/- number
        if let Some(pos) = t.rfind(|c: char| c == '+' || c == '-') {
            let name = t[..pos].trim().to_string();
            let val_str = t[pos..].trim();
            if let Ok(v) = val_str.parse::<i32>() {
                if !name.is_empty() {
                    out.push((name, v));
                }
            }
        }
    }
    out
}

fn seed_armor_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, skills FROM armor WHERE game_id = 5 AND skills IS NOT NULL")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name(&name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((armor_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute(
            "INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)",
            rusqlite::params![aid, sid, pts],
        )?;
    }
    Ok(())
}

fn seed_weapon_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM weapons WHERE game_id = 5 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (weapon_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name(&name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 5",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((weapon_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (wid, sid, pts) in to_insert {
        conn.execute(
            "INSERT OR IGNORE INTO weapon_skill_points (weapon_id, skill_id, points) VALUES (?1, ?2, ?3)",
            rusqlite::params![wid, sid, pts],
        )?;
    }
    Ok(())
}

// â”€â”€ MHP3rd (ULJM-05800) â”€â”€ village_low/high + guild_low/high, faithful to ISO normal â”€â”€

fn seed_mhp3rd_monsters(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monsters.json");
    let monsters: Vec<MonsterJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in &monsters {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhp3rd/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, ?7, ?8, 'en')",
            rusqlite::params![m.id, MHP3RD, m.name, m.species, m.size, m.name, icon_color, icon_url],
        )?;
    }
    for m in &monsters {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhp3rd/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        let _ = conn.execute(
            "UPDATE monsters SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 4 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![m.name, icon_color, icon_url, m.id],
        );
    }
    Ok(())
}
fn backfill_mhp3rd_monster_descriptions(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monster_descriptions.json");
    let descs: Vec<MonsterDescJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in descs {
        conn.execute(
            "UPDATE monsters SET description = ?1 WHERE name = ?2 AND game_id = 4 AND description IS NULL",
            rusqlite::params![d.description, d.name],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_items.json");
    let items: Vec<ItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        let (icon_url, icon_name, icon_color) =
            item_icon_from_category(&it.category, it.subcategory.as_deref());
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, icon_url, icon_name, icon_color, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10, ?11, 'en')",
            rusqlite::params![it.id, MHP3RD, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, icon_url, icon_name, icon_color],
        )?;
    }
    // Existing installs are covered by the item_icons_mhp3rd data patch.
    Ok(())
}
fn backfill_mhp3rd_item_descriptions(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_item_descriptions.json");
    let descs: Vec<ItemDescJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in descs {
        conn.execute(
            "UPDATE items SET description = ?1 WHERE name = ?2 AND game_id = 4 AND description IS NULL",
            rusqlite::params![d.description, d.name],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_monster_drops(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monster_drops.json");
    let drops: Vec<DropJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in drops {
        conn.execute(
            "INSERT OR IGNORE INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
            rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_item_sources_from_drops(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability) SELECT item_id, CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' ELSE method END, monster_id, quantity, quantity, probability FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 4)",
        [],
    )?;
    Ok(())
}
fn seed_mhp3rd_extra_item_sources(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_item_sources_extra.json");
    let sources: Vec<ExtraSourceJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sources {
        conn.execute(
            "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location, conditions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![s.item_id, s.source_type, s.source_id, s.quantity_min.unwrap_or(1), s.quantity_max.unwrap_or(1), s.probability, s.location, s.conditions],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_monster_equipment(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monster_equipment.json");
    let rows: Vec<MonsterEquipJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rows {
        conn.execute(
            "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id) VALUES (4, ?1, ?2, ?3)",
            rusqlite::params![r.monster_id, r.kind, r.equipment_id],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_monster_weaknesses(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_item_combine.json");
    let recipes: Vec<CombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for rc in recipes {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity, rc.combine_type.as_deref().unwrap_or("normal"), rc.chance],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_extra_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_item_combine_extra.json");
    let recs: Vec<ExtraCombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recs {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, 'normal', NULL)",
            rusqlite::params![r.result_item_id, r.component_item_id, r.quantity, r.result_quantity],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhp3(w.rarity);
        let icon_url = format!("/icons/mhp3rd/weapons/{}-{}.png", slug, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, 'en')",
            rusqlite::params![w.id, MHP3RD, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description, w.sort_order, w.weapon_type, icon_color, icon_url],
        )?;
    }
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhp3(w.rarity);
        let icon_url = format!("/icons/mhp3rd/weapons/{}-{}.png", slug, color_slug);
        let generic_url = format!("/icons/mhp3rd/weapons/{}.png", slug);
        conn.execute(
            "UPDATE weapons SET icon_name = COALESCE(icon_name, ?1), icon_color = ?2, icon_url = COALESCE(NULLIF(icon_url, ?3), ?4) WHERE id = ?5 AND game_id = 4",
            rusqlite::params![w.weapon_type, icon_color, generic_url, icon_url, w.id],
        )?;
        let _ = conn.execute(
            "UPDATE weapons SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 4 AND (icon_color IN ('Gray','Gold','Green','White','Yellow','Light Blue','Blue','Purple','Orange','Red') OR icon_url = ?4 OR icon_url LIKE '%mhfu%')",
            rusqlite::params![icon_color, icon_url, w.id, generic_url],
        );
    }
    // Migrate old 8-color to faithful 7-rarity HEX r1..r7
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhp3(w.rarity);
        let icon_url = format!("/icons/mhp3rd/weapons/{}-{}.png", slug, color_slug);
        let _ = conn.execute(
            "UPDATE weapons SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 4 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, w.id],
        );
    }
    Ok(())
}
fn seed_mhp3rd_weapon_materials(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapon_materials.json");
    let mats: Vec<WeaponMatJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        // Skip rows whose weapon/item are not present in the seeded dataset
        // (weapon_materials references a few weapons withdrawn from the
        // `weapons` table, so a direct insert would violate the FK).
        if weapon_exists(conn, MHP3RD, m.weapon_id)? && item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.weapon_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn weapon_exists(conn: &Connection, game_id: i32, id: i32) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM weapons WHERE id = ?1 AND game_id = ?2",
            rusqlite::params![id, game_id],
            |row| row.get::<_, i32>(0),
        )
        .optional()?
        .is_some())
}

fn item_exists(conn: &Connection, id: i32) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM items WHERE id = ?1",
            rusqlite::params![id],
            |row| row.get::<_, i32>(0),
        )
        .optional()?
        .is_some())
}
fn seed_mhp3rd_weapon_craft(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapon_craft.json");
    let rows: Vec<WeaponCraftJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rows {
        // Skip weapons withdrawn from the `weapons` table to avoid an FK violation.
        if !weapon_exists(conn, MHP3RD, r.weapon_id)? {
            continue;
        }
        for m in &r.forge {
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE name = ?1 AND game_id = 4",
                    rusqlite::params![m.item],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(iid) = iid {
                conn.execute(
                    "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity) VALUES (?1, 'forge', ?2, ?3)",
                    rusqlite::params![r.weapon_id, iid, m.quantity],
                )?;
            }
        }
        for m in &r.upgrade {
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE name = ?1 AND game_id = 4",
                    rusqlite::params![m.item],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(iid) = iid {
                conn.execute(
                    "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity) VALUES (?1, 'upgrade', ?2, ?3)",
                    rusqlite::params![r.weapon_id, iid, m.quantity],
                )?;
            }
        }
    }
    Ok(())
}
fn seed_mhp3rd_armor_sets(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut set_id: i32 = 0;
    let mut seen = Vec::<String>::new();
    for a in &armors {
        let set_name = derive_set_name(a);
        if !seen.contains(&set_name) {
            seen.push(set_name.clone());
            set_id += 1;
            conn.execute(
                "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
                rusqlite::params![set_id + 10000, MHP3RD, set_name],
            )?;
        }
    }
    Ok(())
}
fn seed_mhp3rd_armor(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut set_map: Vec<(String, i32)> = Vec::new();
    let mut set_id: i32 = 0;
    for a in &armors {
        let set_name = derive_set_name(a);
        if !set_map.iter().any(|(s, _)| s == &set_name) {
            set_id += 1;
            set_map.push((set_name.clone(), set_id + 10000));
        }
    }
    let set_id_of = |set: &str| -> i32 {
        set_map
            .iter()
            .find(|(s, _)| s == set)
            .map(|(_, i)| *i)
            .unwrap_or(0)
    };
    let set_id_of_armor = |armor: &ArmorJson| -> i32 { set_id_of(&derive_set_name(armor)) };
    for a in &armors {
        let gender = a.gender.clone().unwrap_or_else(|| "both".to_string());
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhp3(rarity);
        let icon_url = format!("/icons/mhp3rd/armor/{}-{}.png", a.slot_type, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, 'en')",
            rusqlite::params![a.id, MHP3RD, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id_of_armor(a), a.armor_type, gender, a.crafting_cost, a.description, a.slot_type, icon_color, icon_url],
        )?;
    }
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhp3(rarity);
        let icon_url = format!("/icons/mhp3rd/armor/{}-{}.png", a.slot_type, color_slug);
        let generic = format!("/icons/mhp3rd/armor/{}.png", a.slot_type);
        let old_generic = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        let _ = conn.execute(
            "UPDATE armor SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(NULLIF(icon_url, ?3), COALESCE(NULLIF(icon_url, ?4), ?5)) WHERE id = ?6 AND game_id = 4 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![a.slot_type, icon_color, generic, old_generic, icon_url, a.id],
        );
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 4 AND (icon_url = ?4 OR icon_url = ?5 OR icon_url LIKE '%mhfu%')",
            rusqlite::params![icon_color, icon_url, a.id, generic, old_generic],
        );
    }
    // Migrate old rank-based gray to faithful MHP3 7-rarity HEX
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhp3(rarity);
        let icon_url = format!("/icons/mhp3rd/armor/{}-{}.png", a.slot_type, color_slug);
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 4 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, a.id],
        );
    }
    Ok(())
}
fn seed_mhp3rd_armor_materials(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_armor_materials.json");
    let mats: Vec<ArmorMatJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        conn.execute(
            "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![m.armor_id, m.item_id, m.quantity],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_quests(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_quests.json");
    let quests: Vec<QuestJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for q in &quests {
        let main_monsters_json = q
            .main_monsters
            .as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhfu/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", hub_slug);
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, name_original, type, rank, hub, stars, objective, objective_original, location, location_original, time_limit, faints_allowed, is_key_quest, is_urgent, description, description_original, client, requirements, reward_money, contract_fee, main_monsters, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, 'en')",
            rusqlite::params![q.id, MHP3RD, q.name, q.name_original, q.qtype, q.rank, q.hub, q.stars, q.objective, q.objective_original.as_deref().unwrap_or(&q.objective), q.location, q.location_original.as_deref().unwrap_or(&q.location), q.time_limit.unwrap_or(50), q.faints_allowed.unwrap_or(3), q.is_key_quest.unwrap_or(false), q.is_urgent.unwrap_or(false), q.description, q.description_original.as_deref().unwrap_or(q.description.as_deref().unwrap_or("")), q.client, q.requirements, q.reward_money, q.contract_fee, main_monsters_json, q.qtype, icon_color, icon_url, q.hub.clone().unwrap_or_else(|| "unknown".to_string()), "Gray", hub_icon_url],
        )?;
    }
    // Backfill EN for existing installs that already have JP rows (preserve original JP in *_original)
    let mhp3rd_backfill: Vec<QuestJson> =
        match serde_json::from_str(include_str!("../../data/mhp3rd_quests.json")) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[seed] mhp3rd_quests backfill parse failed: {}", e);
                Vec::new()
            }
        };
    for q in mhp3rd_backfill {
        let _ = conn.execute(
            "UPDATE quests SET objective = ?1, objective_original = COALESCE(objective_original, ?2), location = ?3, location_original = COALESCE(location_original, ?4), description = COALESCE(?, description), description_original = COALESCE(description_original, ?) WHERE id = ?5 AND game_id = 4 AND (objective != ?1 OR location != ?3 OR description IS NULL)",
            rusqlite::params![q.objective, q.objective_original.as_deref().unwrap_or(&q.objective), q.location, q.location_original.as_deref().unwrap_or(&q.location), q.description, q.description_original.as_deref().unwrap_or(q.description.as_deref().unwrap_or("")), q.id],
        );
    }
    for q in &quests {
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhfu/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", hub_slug);
        let _ = conn.execute(
            "UPDATE quests SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3), hub_icon_name = COALESCE(hub_icon_name, ?4), hub_icon_color = COALESCE(hub_icon_color, 'Gray'), hub_icon_url = COALESCE(hub_icon_url, ?5) WHERE id = ?6 AND game_id = 4 AND (icon_url IS NULL OR hub_icon_url IS NULL)",
            rusqlite::params![q.qtype, icon_color, icon_url, q.hub.clone().unwrap_or_else(|| "unknown".to_string()), hub_icon_url, q.id],
        );
    }
    Ok(())
}
fn seed_mhp3rd_quest_rewards(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_quest_rewards.json");
    let rewards: Vec<QuestRewardJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rewards {
        conn.execute(
            "INSERT OR IGNORE INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![r.id, r.quest_id, r.item_id, r.quantity, r.probability, r.condition],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_skills(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_skills_new.json");
    let skills: Vec<SkillFamilyJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![s.id, MHP3RD, s.name, s.description, s.max_level],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_skill_levels(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_skill_levels.json");
    let levels: Vec<SkillLevelJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO skill_levels (id, skill_id, points, ability_name, description, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![l.id, l.skill_id, l.points, l.ability_name, l.description],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_decorations(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_decorations.json");
    let decos: Vec<DecoJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in decos {
        let mut primary_id: Option<i32> = None;
        let mut primary_pts: Option<i32> = None;
        let mut secondary_id: Option<i32> = None;
        let mut secondary_pts: Option<i32> = None;
        for (idx, sp) in d.skill_points.iter().enumerate() {
            let normalized = normalize_skill_name_p3rd(&sp.name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 4",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if idx == 0 {
                primary_id = sid;
                primary_pts = Some(sp.points);
            } else if idx == 1 {
                secondary_id = sid;
                secondary_pts = Some(sp.points);
            }
        }
        if primary_id.is_none() {
            continue;
        }
        let primary_skill_name = d
            .skill_points
            .first()
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        let (icon_file, icon_color) = decoration_skill_icon(primary_skill_name);
        let icon_url = format!("/icons/mhfu/decorations/{}", icon_file);
        let icon_name = primary_skill_name.to_string();
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![d.id, MHP3RD, d.name, primary_id, primary_pts, primary_pts, secondary_id, secondary_pts, d.slot_size, d.price, icon_name, icon_color, icon_url],
        )?;
        let _ = conn.execute(
            "UPDATE decorations SET icon_name = ?1, icon_color = ?2, icon_url = ?3 WHERE id = ?4 AND game_id = 4 AND (icon_url IS NULL OR icon_url LIKE '%slot-%' OR icon_name LIKE 'Slot %')",
            rusqlite::params![icon_name, icon_color, icon_url, d.id],
        );
        let _ = conn.execute(
            "UPDATE decorations SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 4 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![icon_name, icon_color, icon_url, d.id],
        );
        for m in &d.materials {
            let normalized_mat = normalize_item_name_p3rd(&m.name);
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE LOWER(name) = LOWER(?1) AND game_id = 4",
                    rusqlite::params![normalized_mat],
                    |row| row.get(0),
                )
                .optional()?;
            if iid.is_none() {
                // Never insert a NULL FK — skip unresolved rather than making an orphan.
                continue;
            }
            conn.execute(
                "INSERT OR IGNORE INTO decoration_materials (decoration_id, item_id, item_name, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![d.id, iid, m.name, m.amount],
            )?;
        }
    }
    Ok(())
}
fn seed_mhp3rd_armor_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, skills FROM armor WHERE game_id = 4 AND skills IS NOT NULL")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name_p3rd(&name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 4",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((armor_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![aid, sid, pts])?;
    }
    Ok(())
}
fn seed_mhp3rd_weapon_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM weapons WHERE game_id = 4 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (weapon_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name_p3rd(&name);
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 4",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((weapon_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (wid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO weapon_skill_points (weapon_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![wid, sid, pts])?;
    }
    Ok(())
}

// ── MHW + Iceborne (game_id 1) ── World + Iceborne 100% (MHWorldData + Kiranico)

#[derive(Deserialize)]
struct MhwItemJson {
    id: i32,
    name: String,
    category: String,
    subcategory: Option<String>,
    rarity: Option<i32>,
    sell_price: Option<i32>,
    buy_price: Option<i32>,
    carry_limit: Option<i32>,
    icon_name: Option<String>,
    icon_color: Option<String>,
    icon_url: Option<String>,
    description: Option<String>,
    sort_order: Option<i32>,
}

fn seed_mhw_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_items.json");
    let items: Vec<MhwItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_name, icon_color, icon_url, description, sort_order, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, 'en')",
            rusqlite::params![it.id, MHW, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_name, it.icon_color, it.icon_url, it.description, it.sort_order],
        )?;
    }
    // Backfill for existing DBs where category/subcategory/icon/sort changed
    for it in &items {
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2, rarity = COALESCE(?3, rarity), sell_price = COALESCE(?4, sell_price), buy_price = COALESCE(?5, buy_price), carry_limit = COALESCE(?6, carry_limit), icon_name = COALESCE(?7, icon_name), icon_color = COALESCE(?8, icon_color), icon_url = COALESCE(?9, icon_url), description = COALESCE(NULLIF(description,''), ?10), sort_order = COALESCE(?11, sort_order) WHERE id = ?12 AND game_id = 1",
            rusqlite::params![it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_name, it.icon_color, it.icon_url, it.description, it.sort_order, it.id],
        )?;
    }
    // Ensure every MHW item has sort_order (fallback to id offset for legacy DBs)
    let _ = conn.execute(
        "UPDATE items SET sort_order = id - 20000 WHERE game_id = 1 AND sort_order IS NULL",
        [],
    );
    Ok(())
}

#[derive(Deserialize)]
struct MhwCombineJson {
    result_item_id: i32,
    component_item_id: i32,
    quantity: i32,
    result_quantity: i32,
}

fn seed_mhw_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_item_combine.json");
    let recipes: Vec<MhwCombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for rc in recipes {
        // Ensure both result and component exist (FK guard)
        if item_exists(conn, rc.result_item_id)? && item_exists(conn, rc.component_item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, 'normal', NULL)",
                rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity],
            )?;
        }
    }
    Ok(())
}

#[derive(Deserialize)]
struct MhwMelderJson {
    result_item_id: i32,
    research_cost: i32,
    melding_cost: i32,
    unlock_condition: Option<String>,
    melder_type: Option<String>,
}

fn seed_mhw_melder_recipes(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_melder_recipes.json");
    let recipes: Vec<MhwMelderJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recipes {
        if item_exists(conn, r.result_item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO melder_recipes (game_id, result_item_id, research_cost, melding_cost, unlock_condition, melder_type) VALUES (1, ?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![r.result_item_id, r.research_cost, r.melding_cost, r.unlock_condition, r.melder_type.as_deref().unwrap_or("normal")],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_monsters(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwMonJson {
        id: i32,
        name: String,
        species: String,
        size: String,
        description: Option<String>,
        sort_order: Option<i32>,
    }
    let json_data = include_str!("../../data/mhw_monsters.json");
    let mons: Vec<MhwMonJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in &mons {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhw/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'en')",
            rusqlite::params![m.id, MHW, m.name, m.species, m.size, m.description, m.sort_order, m.name, icon_color, icon_url],
        )?;
    }
    for m in &mons {
        let slug = monster_icon_slug(&m.name);
        let icon_url = format!("/icons/mhw/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        let _ = conn.execute(
            "UPDATE monsters SET species = COALESCE(NULLIF(species,'Unknown'), ?1), description = COALESCE(NULLIF(description,''), ?2), sort_order = COALESCE(sort_order, ?3), icon_name = COALESCE(icon_name, ?4), icon_color = COALESCE(icon_color, ?5), icon_url = COALESCE(icon_url, ?6) WHERE id = ?7 AND game_id = 1",
            rusqlite::params![m.species, m.description, m.sort_order, m.name, icon_color, icon_url, m.id],
        );
    }
    // Fallback for legacy DBs without sort_order: small then large, alphabetical within section
    let _ = conn.execute(
        "UPDATE monsters SET sort_order = CASE WHEN size='Small' THEN id ELSE 10000+id END WHERE game_id=1 AND sort_order IS NULL",
        [],
    );
    Ok(())
}

fn seed_mhw_monster_drops(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwDropJson {
        monster_id: i32,
        item_id: i32,
        method: String,
        part: Option<String>,
        rank: Option<String>,
        quantity: i32,
        probability: Option<f64>,
        condition: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_monster_drops.json");
    let drops: Vec<MhwDropJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in drops {
        if item_exists(conn, d.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
                rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_item_sources_from_drops(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability) SELECT item_id, CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' WHEN 'reward' THEN 'reward' ELSE method END, monster_id, quantity, quantity, probability FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1)",
        [],
    )?;
    Ok(())
}

fn seed_mhw_extra_item_sources(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwExtraSrc {
        item_id: i32,
        source_type: String,
        source_id: Option<i32>,
        location: Option<String>,
        probability: Option<f64>,
        conditions: Option<String>,
        quantity_min: Option<i32>,
        quantity_max: Option<i32>,
    }
    let json_data = include_str!("../../data/mhw_item_sources_extra.json");
    let sources: Vec<MhwExtraSrc> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sources {
        if item_exists(conn, s.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location, conditions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![s.item_id, s.source_type, s.source_id, s.quantity_min.unwrap_or(1), s.quantity_max.unwrap_or(1), s.probability, s.location, s.conditions],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_gather_item_sources(conn: &Connection) -> Result<()> {
    // 002b: gathering nodes from MHWorldData location_items.csv
    // (scripts/generate_mhw_decorations_weaknesses.py). Same row shape as the
    // extra-sources file; rank folds into `conditions` (no rank column).
    #[derive(Deserialize)]
    struct MhwGatherSrc {
        item_id: i32,
        source_type: String,
        source_id: Option<i32>,
        location: Option<String>,
        probability: Option<f64>,
        conditions: Option<String>,
        quantity_min: Option<i32>,
        quantity_max: Option<i32>,
    }
    let json_data = include_str!("../../data/mhw_item_sources_gather.json");
    let sources: Vec<MhwGatherSrc> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sources {
        if item_exists(conn, s.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location, conditions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![s.item_id, s.source_type, s.source_id, s.quantity_min.unwrap_or(1), s.quantity_max.unwrap_or(1), s.probability, s.location, s.conditions],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_extra_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_item_combine_extra.json");
    let recs: Vec<MhwCombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for rc in recs {
        if item_exists(conn, rc.result_item_id)? && item_exists(conn, rc.component_item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, 'normal', NULL)",
                rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_weapons(conn: &Connection) -> Result<()> {
    // MHWorldData weapon_base.csv -> mhw_weapons.json (MHWorldData order, per-type sort_order)
    let json_data = include_str!("../../data/mhw_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, 'en')",
            rusqlite::params![w.id, MHW, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description, w.sort_order, w.weapon_type, icon_color, icon_url],
        )?;
    }
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        let _ = conn.execute(
            "UPDATE weapons SET sort_order = COALESCE(sort_order, ?1), icon_name = COALESCE(icon_name, ?2), icon_color = COALESCE(icon_color, ?3), icon_url = COALESCE(icon_url, ?4), description = COALESCE(NULLIF(description,''), ?5) WHERE id = ?6 AND game_id = 1",
            rusqlite::params![w.sort_order, w.weapon_type, icon_color, icon_url, w.description, w.id],
        );
    }
    // Backfill existing MHW weapons that still have generic icon (without color suffix) to per-rarity variant
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        let _ = conn.execute(
            "UPDATE weapons SET icon_url = ?1, icon_color = ?2 WHERE id = ?3 AND game_id = 1 AND (icon_url = ?4 OR icon_url LIKE '%/mhw/weapons/'||?5||'.png')",
            rusqlite::params![icon_url, icon_color, w.id, format!("/icons/mhw/weapons/{}.png", slug), slug],
        );
    }
    // Migrate old 8-color (white/yellow etc) to faithful 12-HEX r1..r12 for existing DBs
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        let _ = conn.execute(
            "UPDATE weapons SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 1 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, w.id],
        );
    }
    Ok(())
}

fn seed_mhw_weapon_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WMat {
        weapon_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhw_weapon_materials.json");
    let mats: Vec<WMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if weapon_exists(conn, MHW, m.weapon_id)? && item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.weapon_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhw_weapon_craft(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WCraft {
        weapon_id: i32,
        craft_kind: String,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhw_weapon_craft.json");
    let recs: Vec<WCraft> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recs {
        if weapon_exists(conn, MHW, r.weapon_id)? && item_exists(conn, r.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![r.weapon_id, r.craft_kind, r.item_id, r.quantity],
            )?;
        }
    }
    Ok(())
}

fn armor_icon_color_mhw(rarity: i32) -> (&'static str, &'static str) {
    // MHWorld & MHWI faithful 12-hex rarity colors from Help:Item_Colors
    match rarity {
        1 => ("#AAAAAA", "r1"),
        2 => ("#DEDEDE", "r2"),
        3 => ("#A1C42E", "r3"),
        4 => ("#48AB3F", "r4"),
        5 => ("#5CAEBB", "r5"),
        6 => ("#595CDA", "r6"),
        7 => ("#8D59EF", "r7"),
        8 => ("#C76D46", "r8"),
        9 => ("#B3436A", "r9"),
        10 => ("#0AD5FA", "r10"),
        11 => ("#FAC81E", "r11"),
        12 => ("#B4F5FF", "r12"),
        _ => ("#AAAAAA", "r1"),
    }
}

fn armor_slot_slug(slot: &str) -> &'static str {
    match slot {
        "head" => "head",
        "chest" => "chest",
        "arms" => "arms",
        "waist" => "waist",
        "legs" => "legs",
        _ => "head",
    }
}

fn seed_mhw_skills(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwSkillJson {
        id: i32,
        name: String,
        description: Option<String>,
        max_level: Option<i32>,
    }
    let json_data = include_str!("../../data/mhw_skills.json");
    let skills: Vec<MhwSkillJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![s.id, MHW, s.name, s.description, s.max_level],
        )?;
    }
    Ok(())
}

fn seed_mhw_skill_levels(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwLevelJson {
        id: i32,
        skill_id: i32,
        points: i32,
        ability_name: String,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_skill_levels.json");
    let levels: Vec<MhwLevelJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO skill_levels (id, skill_id, points, ability_name, description, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![l.id, l.skill_id, l.points, l.ability_name, l.description],
        )?;
    }
    Ok(())
}

fn seed_mhw_armor_sets(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwSetJson {
        id: i32,
        name: String,
        bonus_skill: Option<String>,
        bonus_required: Option<i32>,
    }
    let json_data = include_str!("../../data/mhw_armor_sets.json");
    let sets: Vec<MhwSetJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sets {
        conn.execute(
            "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![s.id, MHW, s.name, s.bonus_skill, s.bonus_required],
        )?;
    }
    Ok(())
}

fn seed_mhw_armor(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MhwArmorJson {
        id: i32,
        set: String,
        slot_type: String,
        name: String,
        rank: String,
        rarity: Option<i32>,
        defense_base: Option<i32>,
        defense_max: Option<i32>,
        resistance_fire: Option<i32>,
        resistance_water: Option<i32>,
        resistance_thunder: Option<i32>,
        resistance_ice: Option<i32>,
        resistance_dragon: Option<i32>,
        slots: Option<String>,
        skills: Option<String>,
        armor_type: Option<String>,
        gender: Option<String>,
        crafting_cost: Option<i32>,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_armor.json");
    let armors: Vec<MhwArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    // Map set name -> id via mhw_armor_sets.json (authoritative)
    let sets_data = include_str!("../../data/mhw_armor_sets.json");
    #[derive(Deserialize)]
    struct SetMap {
        id: i32,
        name: String,
    }
    let sets: Vec<SetMap> = serde_json::from_str(sets_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut set_map = std::collections::HashMap::new();
    for s in &sets {
        set_map.insert(s.name.clone(), s.id);
    }
    for a in &armors {
        let set_id = set_map.get(&a.set).copied().unwrap_or(0);
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhw(rarity);
        let slot = armor_slot_slug(&a.slot_type);
        let icon_url = format!("/icons/mhw/armor/{}-{}.png", slot, color_slug);
        let icon_name = a.slot_type.clone();
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, 'en')",
            rusqlite::params![a.id, MHW, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id, a.armor_type, a.gender.clone().unwrap_or_else(|| "both".to_string()), a.crafting_cost, a.description, icon_name, icon_color, icon_url],
        )?;
    }
    // Backfill per-rarity icons for existing DBs
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhw(rarity);
        let slot = armor_slot_slug(&a.slot_type);
        let icon_url = format!("/icons/mhw/armor/{}-{}.png", slot, color_slug);
        let generic = format!("/icons/mhw/armor/{}.png", slot);
        let _ = conn.execute(
            "UPDATE armor SET icon_name = COALESCE(icon_name, ?1), icon_color = ?2, icon_url = COALESCE(NULLIF(icon_url, ?3), ?4) WHERE id = ?5 AND game_id = 1",
            rusqlite::params![a.slot_type, icon_color, generic, icon_url, a.id],
        );
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 1 AND (icon_color IN ('Gray','Gold','Green','White','Yellow','Light Blue','Blue','Purple','Orange','Red') OR icon_url = ?4)",
            rusqlite::params![icon_color, icon_url, a.id, generic],
        );
    }
    // Migrate old 8-color white/yellow... to faithful 12-HEX r1..r12
    for a in &armors {
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhw(rarity);
        let slot = armor_slot_slug(&a.slot_type);
        let icon_url = format!("/icons/mhw/armor/{}-{}.png", slot, color_slug);
        let _ = conn.execute(
            "UPDATE armor SET icon_color = ?1, icon_url = ?2 WHERE id = ?3 AND game_id = 1 AND (icon_color != ?1 OR icon_url != ?2)",
            rusqlite::params![icon_color, icon_url, a.id],
        );
    }
    Ok(())
}

fn seed_mhw_armor_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct AMat {
        armor_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhw_armor_materials.json");
    let mats: Vec<AMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if item_exists(conn, m.item_id)? {
            // FK guard: armor must exist
            let exists: Option<i32> = conn
                .query_row(
                    "SELECT id FROM armor WHERE id = ?1 AND game_id = 1",
                    rusqlite::params![m.armor_id],
                    |r| r.get(0),
                )
                .optional()?;
            if exists.is_some() {
                conn.execute(
                    "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                    rusqlite::params![m.armor_id, m.item_id, m.quantity],
                )?;
            }
        }
    }
    Ok(())
}

fn seed_mhw_armor_skill_points(conn: &Connection) -> Result<()> {
    // Reuse parse_skill_string logic but for game_id 1 with MHW skill names (trim only, no MH2G aliases)
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM armor WHERE game_id = 1 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 1",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((armor_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![aid, sid, pts])?;
    }
    Ok(())
}

fn seed_mhw_decorations(conn: &Connection) -> Result<()> {
    // 002b: decorations from MHWorldData decoration_base.csv
    // (scripts/generate_mhw_decorations_weaknesses.py). MHW jewels have no
    // crafting materials and no buy price upstream — materials stays empty.
    let json_data = include_str!("../../data/mhw_decorations.json");
    let decos: Vec<DecoJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in decos {
        let mut primary_id: Option<i32> = None;
        let mut primary_pts: Option<i32> = None;
        let mut secondary_id: Option<i32> = None;
        let mut secondary_pts: Option<i32> = None;
        for (idx, sp) in d.skill_points.iter().enumerate() {
            let normalized = sp.name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 1",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if idx == 0 {
                primary_id = sid;
                primary_pts = Some(sp.points);
            } else if idx == 1 {
                secondary_id = sid;
                secondary_pts = Some(sp.points);
            }
        }
        if primary_id.is_none() {
            // Never insert a NULL skill FK — skip unresolved rather than orphaning.
            continue;
        }
        let primary_skill_name = d
            .skill_points
            .first()
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        let (icon_file, icon_color) = decoration_skill_icon(primary_skill_name);
        let icon_url = format!("/icons/mhfu/decorations/{}", icon_file);
        let icon_name = primary_skill_name.to_string();
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, icon_name, icon_color, icon_url, language) VALUES (?1, 1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![d.id, d.name, primary_id, primary_pts, primary_pts, secondary_id, secondary_pts, d.slot_size, d.rarity, d.price, icon_name, icon_color, icon_url],
        )?;
    }
    Ok(())
}

fn seed_mhw_monster_weaknesses(conn: &Connection) -> Result<()> {
    // 002b: per-part hitzones from MHWorldData monster_hitzones.csv
    // (scripts/generate_mhw_decorations_weaknesses.py). Real cut/impact/shot +
    // element numbers, same scale as mh2g. Status ailments have no column
    // and are intentionally dropped.
    let json_data = include_str!("../../data/mhw_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon],
        )?;
    }
    Ok(())
}

fn seed_mhw_weapon_skill_points(conn: &Connection) -> Result<()> {
    // 002b: 638 MHW weapons carry a `skills` string (e.g. 'Guts');
    // normalize by trim-only lookup against game_id 1 skills.
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM weapons WHERE game_id = 1 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (weapon_id, skills_str) = r?;
        let mut resolved = false;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 1",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((weapon_id, sid, pts));
                resolved = true;
            }
        }
        if !resolved {
            // MHW single-rank specials are stored bare (e.g. 'Guts',
            // 'Kulve Taroth Essence') with no +N suffix — the weapon alone
            // grants the full rank, so points = max_level.
            let name = skills_str.trim().to_string();
            let hit: Option<(i32, Option<i32>)> = conn
                .query_row(
                    "SELECT id, max_level FROM skills WHERE name = ?1 AND game_id = 1",
                    rusqlite::params![name],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .optional()?;
            if let Some((sid, max_lv)) = hit {
                to_insert.push((weapon_id, sid, max_lv.unwrap_or(1)));
            }
        }
    }
    drop(stmt);
    for (wid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO weapon_skill_points (weapon_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![wid, sid, pts])?;
    }
    Ok(())
}

fn seed_mhw_monster_equipment(conn: &Connection) -> Result<()> {
    // 002b: no upstream equipment list for MHW — derive links from already
    // seeded junctions: an armor piece / weapon links to every monster that
    // drops one of its crafting materials. Powers monster detail armor/weapon
    // tabs + the dynamic dedicated-sets query, mirroring mh2g coverage.
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 1, md.monster_id, 'armor', am.armor_id
         FROM armor_materials am
         JOIN armor a ON a.id = am.armor_id AND a.game_id = 1
         JOIN items i ON i.id = am.item_id AND i.game_id = 1
         JOIN monster_drops md ON md.item_id = am.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 1",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 1, md.monster_id, 'weapon', wm.weapon_id
         FROM weapon_materials wm
         JOIN weapons w ON w.id = wm.weapon_id AND w.game_id = 1
         JOIN items i ON i.id = wm.item_id AND i.game_id = 1
         JOIN monster_drops md ON md.item_id = wm.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 1",
        [],
    )?;
    Ok(())
}

fn seed_mhw_mantles(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct MantleJson {
        id: i32,
        name: String,
        tool_type: String,
        rarity: Option<i32>,
        description: Option<String>,
        effect: String,
        duration_sec: Option<i32>,
        cooldown_sec: Option<i32>,
        cooldown_upgraded_sec: Option<i32>,
        slots: Option<String>,
        acquisition: Option<String>,
        upgrade_quest: Option<String>,
        upgrade_effect: Option<String>,
        sort_order: Option<i32>,
        icon_url: Option<String>,
        icon_url_plus: Option<String>,
        icon_name: Option<String>,
        icon_color: Option<String>,
        icon_name_plus: Option<String>,
        icon_color_plus: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_mantles.json");
    let mantles: Vec<MantleJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in &mantles {
        let icon_url = m.icon_url.clone().unwrap_or_else(|| {
            if m.tool_type == "booster" {
                "/icons/mhw/boosters/health-booster.png".to_string()
            } else {
                "/icons/mhw/mantles/ghillie-mantle.png".to_string()
            }
        });
        let icon_url_plus = m
            .icon_url_plus
            .clone()
            .unwrap_or_else(|| icon_url.replace(".png", "-plus.png"));
        let icon_name = m.icon_name.clone().unwrap_or_else(|| m.name.clone());
        let icon_color = m
            .icon_color
            .clone()
            .unwrap_or_else(|| "#595CDA".to_string());
        let icon_name_plus = m
            .icon_name_plus
            .clone()
            .unwrap_or_else(|| format!("{} +", m.name));
        let icon_color_plus = m
            .icon_color_plus
            .clone()
            .unwrap_or_else(|| "#FAC81E".to_string());
        conn.execute(
            "INSERT OR IGNORE INTO mhw_mantles (id, game_id, name, tool_type, rarity, description, effect, duration_sec, cooldown_sec, cooldown_upgraded_sec, slots, acquisition, upgrade_quest, upgrade_effect, sort_order, icon_name, icon_color, icon_url, icon_name_plus, icon_color_plus, icon_url_plus, language) VALUES (?1,1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,'en')",
            rusqlite::params![m.id, m.name, m.tool_type, m.rarity, m.description, m.effect, m.duration_sec, m.cooldown_sec, m.cooldown_upgraded_sec, m.slots, m.acquisition, m.upgrade_quest, m.upgrade_effect, m.sort_order, icon_name, icon_color, icon_url, icon_name_plus, icon_color_plus, icon_url_plus],
        )?;
    }
    for m in &mantles {
        let icon_url = m
            .icon_url
            .clone()
            .unwrap_or_else(|| "/icons/mhw/mantles/ghillie-mantle.png".to_string());
        let icon_url_plus = m
            .icon_url_plus
            .clone()
            .unwrap_or_else(|| icon_url.replace(".png", "-plus.png"));
        let icon_name = m.icon_name.clone().unwrap_or_else(|| m.name.clone());
        let icon_color = m
            .icon_color
            .clone()
            .unwrap_or_else(|| "#595CDA".to_string());
        let icon_name_plus = m
            .icon_name_plus
            .clone()
            .unwrap_or_else(|| format!("{} +", m.name));
        let icon_color_plus = m
            .icon_color_plus
            .clone()
            .unwrap_or_else(|| "#FAC81E".to_string());
        conn.execute(
            "UPDATE mhw_mantles SET description=COALESCE(description,?1), effect=COALESCE(effect,?2), acquisition=COALESCE(acquisition,?3), upgrade_quest=COALESCE(upgrade_quest,?4), upgrade_effect=COALESCE(upgrade_effect,?5), icon_name=COALESCE(icon_name,?6), icon_color=COALESCE(icon_color,?7), icon_url=COALESCE(icon_url,?8), icon_name_plus=COALESCE(icon_name_plus,?9), icon_color_plus=COALESCE(icon_color_plus,?10), icon_url_plus=COALESCE(icon_url_plus,?11) WHERE id=?12 AND game_id=1",
            rusqlite::params![m.description, m.effect, m.acquisition, m.upgrade_quest, m.upgrade_effect, icon_name, icon_color, icon_url, icon_name_plus, icon_color_plus, icon_url_plus, m.id],
        )?;
        // Migrate legacy generic icons to per-item
        let _ = conn.execute(
            "UPDATE mhw_mantles SET icon_url=?1, icon_color=?2, icon_name=?3, icon_url_plus=?4, icon_color_plus=?5, icon_name_plus=?6 WHERE id=?7 AND game_id=1 AND (icon_url IN ('/icons/mhw/tools/mantle.png','/icons/mhw/tools/booster.png') OR icon_url IS NULL)",
            rusqlite::params![icon_url, icon_color, icon_name, icon_url_plus, icon_color_plus, icon_name_plus, m.id],
        );
    }
    Ok(())
}

fn seed_palico_gadgets(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct GadgetJson {
        id: i32,
        name: String,
        gadget_type: String,
        tribe: Option<String>,
        description: Option<String>,
        effect: Option<String>,
        acquisition: Option<String>,
        sort_order: Option<i32>,
        icon_url: Option<String>,
        icon_name: Option<String>,
        icon_color: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_palico_gadgets.json");
    let gadgets: Vec<GadgetJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for g in &gadgets {
        let fallback_url = format!(
            "/icons/mhw/palico/{}.png",
            g.name.to_lowercase().replace(' ', "-").replace('\'', "")
        );
        let icon_url = g.icon_url.clone().unwrap_or(fallback_url.clone());
        let icon_name = g.icon_name.clone().unwrap_or_else(|| g.name.clone());
        let icon_color = g
            .icon_color
            .clone()
            .unwrap_or_else(|| match g.gadget_type.as_str() {
                "tailraider" => "#C76D46".to_string(),
                "safari" => "#48AB3F".to_string(),
                _ => "#8D59EF".to_string(),
            });
        conn.execute(
            "INSERT OR IGNORE INTO palico_gadgets (id, game_id, name, gadget_type, tribe, description, effect, acquisition, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1,1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,'en')",
            rusqlite::params![g.id, g.name, g.gadget_type, g.tribe, g.description, g.effect, g.acquisition, g.sort_order, icon_name, icon_color, icon_url],
        )?;
    }
    for g in &gadgets {
        let fallback_url = format!(
            "/icons/mhw/palico/{}.png",
            g.name.to_lowercase().replace(' ', "-").replace('\'', "")
        );
        let icon_url = g.icon_url.clone().unwrap_or(fallback_url);
        let icon_name = g.icon_name.clone().unwrap_or_else(|| g.name.clone());
        let icon_color = g
            .icon_color
            .clone()
            .unwrap_or_else(|| "#8D59EF".to_string());
        conn.execute(
            "UPDATE palico_gadgets SET description=COALESCE(description,?1), effect=COALESCE(effect,?2), acquisition=COALESCE(acquisition,?3), icon_name=COALESCE(icon_name,?4), icon_color=COALESCE(icon_color,?5), icon_url=COALESCE(icon_url,?6) WHERE id=?7 AND game_id=1",
            rusqlite::params![g.description, g.effect, g.acquisition, icon_name, icon_color, icon_url, g.id],
        )?;
        let _ = conn.execute(
            "UPDATE palico_gadgets SET icon_url=?1, icon_color=?2, icon_name=?3 WHERE id=?4 AND game_id=1 AND icon_url IN ('/icons/mhw/palico/gadget.png','/icons/mhw/palico/tailraider.png','/icons/mhw/palico/safari.png')",
            rusqlite::params![icon_url, icon_color, icon_name, g.id],
        );
    }
    Ok(())
}

fn seed_palico_gadget_levels(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct LevelJson {
        id: i32,
        gadget_id: i32,
        proficiency: i32,
        ability_name: String,
        description: Option<String>,
        unlock_condition: Option<String>,
    }
    let json_data = include_str!("../../data/mhw_palico_gadget_levels.json");
    let levels: Vec<LevelJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO palico_gadget_levels (id, gadget_id, proficiency, ability_name, description, unlock_condition) VALUES (?1,?2,?3,?4,?5,?6)",
            rusqlite::params![l.id, l.gadget_id, l.proficiency, l.ability_name, l.description, l.unlock_condition],
        )?;
    }
    Ok(())
}

fn seed_mhw_quests(conn: &Connection) -> Result<()> {
    // MHW + Iceborne v3: 521 quests — hub = low_high (LR/HR ★1-9) / master (MR ★1-6) / siege (3) flat
    // category = assigned/optional/event/arena/challenge/special/siege (sub-section inside hub, category before stars)
    // Source: MHWorldData quest_base.csv + quest_base_translations.csv (EN objective/description)
    // Ids are MHWorldData canonical (101..67841) PLUS MHW_QUEST_ID_OFFSET (stored 100101+)
    // so they never collide with MH2G ids (1..610) on the single-column PK.
    migrate_mhw_quest_ids(conn)?;
    let json_data = include_str!("../../data/mhw_quests.json");
    let quests: Vec<QuestJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for q in &quests {
        let main_monsters_json = q
            .main_monsters
            .as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhw/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhw/quests/hubs/{}.png", hub_slug);
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, name_original, type, rank, hub, category, stars, objective, objective_original, location, location_original, time_limit, faints_allowed, is_key_quest, is_urgent, description, description_original, client, requirements, reward_money, contract_fee, main_monsters, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, 'en')",
            rusqlite::params![
                q.id + MHW_QUEST_ID_OFFSET,
                MHW,
                q.name,
                q.name_original,
                q.qtype,
                q.rank,
                q.hub,
                q.category,
                q.stars,
                q.objective,
                q.objective_original.as_deref().unwrap_or(&q.objective),
                q.location,
                q.location_original.as_deref().unwrap_or(&q.location),
                q.time_limit.unwrap_or(50),
                q.faints_allowed.unwrap_or(3),
                q.is_key_quest.unwrap_or(false),
                q.is_urgent.unwrap_or(false),
                q.description,
                q.description_original.as_deref().unwrap_or(q.description.as_deref().unwrap_or("")),
                q.client,
                q.requirements,
                q.reward_money,
                q.contract_fee,
                main_monsters_json,
                q.qtype,
                icon_color,
                icon_url,
                q.hub.clone().unwrap_or_else(|| "unknown".to_string()),
                "Gray",
                hub_icon_url
            ],
        )?;
    }
    // Backfill existing DBs — migrate hubs (event/arena/challenge/special -> low_high/master) + category
    let backfill: Vec<QuestJson> =
        match serde_json::from_str(include_str!("../../data/mhw_quests.json")) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[seed] mhw_quests backfill parse failed: {}", e);
                Vec::new()
            }
        };
    for q in backfill {
        let _ = conn.execute(
            "UPDATE quests SET objective = ?1, location = ?2, description = COALESCE(?, description), client = COALESCE(?, client), requirements = COALESCE(?, requirements), type = COALESCE(?, type), rank = ?3, hub = ?4, category = COALESCE(?, category), stars = COALESCE(?, stars) WHERE id = ?5 AND game_id = 1",
            rusqlite::params![q.objective, q.location, q.description, q.client, q.requirements, q.qtype, q.rank, q.hub, q.category, q.stars, q.id + MHW_QUEST_ID_OFFSET],
        );
    }
    // Backfill icons
    for q in &quests {
        let type_slug = quest_type_slug(&q.qtype);
        let icon_url = format!("/icons/mhw/quests/{}.png", type_slug);
        let icon_color = quest_type_color(&q.qtype);
        let hub_slug = q
            .hub
            .as_deref()
            .map(quest_hub_slug)
            .unwrap_or_else(|| "unknown".to_string());
        let hub_icon_url = format!("/icons/mhw/quests/hubs/{}.png", hub_slug);
        let _ = conn.execute(
            "UPDATE quests SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3), hub_icon_name = COALESCE(hub_icon_name, ?4), hub_icon_color = COALESCE(hub_icon_color, 'Gray'), hub_icon_url = COALESCE(hub_icon_url, ?5), category = COALESCE(category, ?6) WHERE id = ?7 AND game_id = 1",
            rusqlite::params![q.qtype, icon_color, icon_url, q.hub.clone().unwrap_or_else(|| "unknown".to_string()), hub_icon_url, q.category, q.id + MHW_QUEST_ID_OFFSET],
        );
    }
    // Migrate legacy 6-hub schema to 3-hub (for DBs created before v3) — technically best: single source of truth hub=low_high/master/siege, category preserves sub-section
    let _ = conn.execute(
        "UPDATE quests SET hub = CASE WHEN rank IN ('Low','High') THEN 'low_high' WHEN rank='Master' THEN 'master' ELSE hub END WHERE game_id=1 AND hub IN ('event','arena','challenge','special')",
        [],
    );
    // Ensure every MHW quest has a category (old DBs had NULL) — derive from original hub if still NULL
    let _ = conn.execute(
        "UPDATE quests SET category = COALESCE(category,
            CASE hub
                WHEN 'siege' THEN 'siege'
                WHEN 'low_high' THEN 'optional'
                WHEN 'master' THEN 'optional'
                ELSE 'event' END)
         WHERE game_id=1 AND category IS NULL",
        [],
    );
    // Force hub/category to canonical JSON values for all MHW quests (idempotent).
    // NOTE: this UPDATE cannot resurrect rows dropped by the pre-offset PK collision;
    // that is what migrate_mhw_quest_ids + the offset INSERT above are for.
    // This overwrites any stale hub (event->low_high etc.) to match generated JSON, without touching mhp3rd/mh2g (game_id 4/5)
    let json_for_canonical: Vec<QuestJson> =
        serde_json::from_str(include_str!("../../data/mhw_quests.json"))
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for q in json_for_canonical {
        let _ = conn.execute(
            "UPDATE quests SET hub = ?1, category = ?2, rank = ?3, stars = ?4 WHERE id = ?5 AND game_id=1 AND (hub IS NULL OR hub != ?1 OR category IS NULL OR category != ?2)",
            rusqlite::params![q.hub, q.category, q.rank, q.stars, q.id + MHW_QUEST_ID_OFFSET],
        );
    }
    Ok(())
}

/// One-time, idempotent migration for DBs seeded before the quest-id offset:
/// re-ids existing MHW quests (+OFFSET) and repoints their rewards — including the
/// rewards of the 67 dropped quests, which FK rules had attached to MH2G quests
/// with the same id. Guards skip already-migrated rows on every boot.
/// FK handling must work in BOTH contexts, because each pragma is inert in one.
/// Production runs inside BEGIN IMMEDIATE (see db/mod.rs), where
/// `PRAGMA foreign_keys = OFF` is a silent no-op, so defer_foreign_keys (checked
/// at COMMIT, when everything is consistent again) does the job. Tests and tools
/// run in autocommit, where defer_foreign_keys is the no-op and the OFF/ON
/// toggle covers the intermediate dangling state.
/// Enforcement is restored before any further write and before errors propagate.
fn migrate_mhw_quest_ids(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA defer_foreign_keys = ON;")?;
    conn.execute_batch("PRAGMA foreign_keys = OFF;")?;
    let result = (|| -> Result<()> {
        conn.execute(
            "UPDATE quests SET id = id + ?1 WHERE game_id = ?2 AND id < ?1",
            rusqlite::params![MHW_QUEST_ID_OFFSET, MHW],
        )?;
        conn.execute(
            "UPDATE quest_rewards SET quest_id = quest_id + ?1 WHERE id BETWEEN ?2 AND ?3 AND quest_id < ?1",
            rusqlite::params![
                MHW_QUEST_ID_OFFSET,
                MHW_QUEST_REWARD_ID_MIN,
                MHW_QUEST_REWARD_ID_MAX
            ],
        )?;
        Ok(())
    })();
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    result?;
    Ok(())
}

fn seed_mhw_quest_rewards(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_quest_rewards.json");
    let rewards: Vec<QuestRewardJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rewards {
        // FK guards: quest must exist (already inserted) and item must exist
        if item_exists(conn, r.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![r.id, r.quest_id + MHW_QUEST_ID_OFFSET, r.item_id, r.quantity, r.probability, r.condition],
            )?;
        }
    }
    Ok(())
}

// ── MH Wilds (game_id 3) ── MHDB Wilds API (scripts/generate_mhwilds_from_mhdb.py)
// Upstream gaps (documented in spec 004): no small monsters, no quests,
// no charms table, no deco materials/buy prices. Wilds portraits live in
// static/icons/mhwilds/monsters/ (scripts/download_mhr_wilds_monster_icons.py,
// Fandom MHWilds:_Monsters); weapons/armor reuse the mhw rarity scheme and
// items resolve through the mhfu reuse map (see item_icon_from_category).

fn seed_mhwilds_monsters(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsMon {
        id: i32,
        name: String,
        species: String,
        size: String,
        description: Option<String>,
        sort_order: Option<i32>,
    }
    let json_data = include_str!("../../data/mhwilds_monsters.json");
    let mons: Vec<WildsMon> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in &mons {
        let slug = monster_icon_slug(&m.name);
        // Lists render the 96px `-sm` variant; detail pages use the master.
        let icon_url = format!("/icons/mhwilds/monsters/{}-sm.png", slug);
        let icon_url_lg = format!("/icons/mhwilds/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, sort_order, icon_name, icon_color, icon_url, icon_url_lg, language) VALUES (?1, 3, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'en')",
            rusqlite::params![m.id, m.name, m.species, m.size, m.description, m.sort_order, m.name, icon_color, icon_url, icon_url_lg],
        )?;
    }
    // Pre-split installs are migrated by the monster_thumbs_rise_wilds data patch.
    Ok(())
}

fn seed_mhwilds_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhwilds_items.json");
    let items: Vec<MhwItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        let (icon_url, icon_name, icon_color) = resolve_item_icon(
            it.icon_url.as_deref(),
            it.icon_name.as_deref(),
            it.icon_color.as_deref(),
            &it.category,
            it.subcategory.as_deref(),
            &it.name,
        );
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_name, icon_color, icon_url, description, sort_order, language) VALUES (?1, 3, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![it.id, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, icon_name, icon_color, icon_url, it.description, it.sort_order],
        )?;
    }
    // Existing installs are covered by the item_icons_mhwilds data patch.
    Ok(())
}

fn seed_mhwilds_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhwilds_item_combine.json");
    let recs: Vec<MhwCombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for rc in recs {
        if item_exists(conn, rc.result_item_id)? && item_exists(conn, rc.component_item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity, combine_type, chance) VALUES (?1, ?2, ?3, ?4, 'normal', NULL)",
                rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhwilds_monster_drops(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsDrop {
        monster_id: i32,
        item_id: i32,
        method: String,
        part: Option<String>,
        rank: Option<String>,
        quantity: i32,
        probability: Option<f64>,
        condition: Option<String>,
    }
    let json_data = include_str!("../../data/mhwilds_monster_drops.json");
    let drops: Vec<WildsDrop> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in drops {
        if item_exists(conn, d.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
                rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
            )?;
        }
    }
    Ok(())
}

fn seed_mhwilds_item_sources_from_drops(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability) SELECT item_id, CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' WHEN 'reward' THEN 'reward' ELSE method END, monster_id, quantity, quantity, probability FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 3)",
        [],
    )?;
    Ok(())
}

fn seed_mhwilds_skills(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsSkill {
        id: i32,
        name: String,
        description: Option<String>,
        max_level: Option<i32>,
    }
    let json_data = include_str!("../../data/mhwilds_skills_new.json");
    let skills: Vec<WildsSkill> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language) VALUES (?1, 3, ?2, ?3, ?4, 'en')",
            rusqlite::params![s.id, s.name, s.description, s.max_level],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_skill_levels(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsLevel {
        id: i32,
        skill_id: i32,
        points: i32,
        ability_name: String,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhwilds_skill_levels.json");
    let levels: Vec<WildsLevel> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO skill_levels (id, skill_id, points, ability_name, description, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![l.id, l.skill_id, l.points, l.ability_name, l.description],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_decorations(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhwilds_decorations.json");
    let decos: Vec<DecoJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in decos {
        let mut primary_id: Option<i32> = None;
        let mut primary_pts: Option<i32> = None;
        let mut secondary_id: Option<i32> = None;
        let mut secondary_pts: Option<i32> = None;
        for (idx, sp) in d.skill_points.iter().enumerate() {
            let normalized = sp.name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 3",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if idx == 0 {
                primary_id = sid;
                primary_pts = Some(sp.points);
            } else if idx == 1 {
                secondary_id = sid;
                secondary_pts = Some(sp.points);
            }
        }
        if primary_id.is_none() {
            continue;
        }
        let primary_skill_name = d
            .skill_points
            .first()
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        let (icon_file, icon_color) = decoration_skill_icon(primary_skill_name);
        let icon_url = format!("/icons/mhfu/decorations/{}", icon_file);
        let icon_name = primary_skill_name.to_string();
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, icon_name, icon_color, icon_url, language) VALUES (?1, 3, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![d.id, d.name, primary_id, primary_pts, primary_pts, secondary_id, secondary_pts, d.slot_size, d.rarity, d.price, icon_name, icon_color, icon_url],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_monster_weaknesses(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhwilds_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhwilds_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, 3, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, 'en')",
            rusqlite::params![w.id, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description, w.sort_order, w.weapon_type, icon_color, icon_url],
        )?;
    }
    // Smith-order fix: sort_order is now the Kiranico Smith DFS sequence
    // (scripts/reorder_mhwilds_weapons.py), not rarity+alphabetical.
    // Backfill existing installs (INSERT OR IGNORE never updates). The
    // statement is prepared once: ~1200 executions per boot otherwise
    // each pay a full prepare cycle.
    let mut backfill = conn.prepare(
        "UPDATE weapons SET sort_order = ?1 WHERE id = ?2 AND game_id = 3 AND COALESCE(sort_order, -1) != COALESCE(?1, -1)",
    )?;
    for w in &weapons {
        backfill.execute(rusqlite::params![w.sort_order, w.id])?;
    }
    Ok(())
}

fn seed_mhwilds_weapon_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WMat {
        weapon_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhwilds_weapon_materials.json");
    let mats: Vec<WMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if weapon_exists(conn, MHWILDS, m.weapon_id)? && item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.weapon_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhwilds_weapon_craft(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WCraft {
        weapon_id: i32,
        craft_kind: String,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhwilds_weapon_craft.json");
    let recs: Vec<WCraft> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recs {
        if weapon_exists(conn, MHWILDS, r.weapon_id)? && item_exists(conn, r.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![r.weapon_id, r.craft_kind, r.item_id, r.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhwilds_armor_sets(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsSet {
        id: i32,
        name: String,
        bonus_skill: Option<String>,
        bonus_required: Option<i32>,
    }
    let json_data = include_str!("../../data/mhwilds_armor_sets.json");
    let sets: Vec<WildsSet> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sets {
        conn.execute(
            "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, 3, ?2, ?3, ?4, 'en')",
            rusqlite::params![s.id, s.name, s.bonus_skill, s.bonus_required],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_armor(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WildsArmor {
        id: i32,
        set: String,
        slot_type: String,
        name: String,
        rank: String,
        rarity: Option<i32>,
        defense_base: Option<i32>,
        defense_max: Option<i32>,
        resistance_fire: Option<i32>,
        resistance_water: Option<i32>,
        resistance_thunder: Option<i32>,
        resistance_ice: Option<i32>,
        resistance_dragon: Option<i32>,
        slots: Option<String>,
        skills: Option<String>,
        armor_type: Option<String>,
        gender: Option<String>,
        crafting_cost: Option<i32>,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhwilds_armor.json");
    let armors: Vec<WildsArmor> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let sets_data = include_str!("../../data/mhwilds_armor_sets.json");
    #[derive(Deserialize)]
    struct SetMap {
        id: i32,
        name: String,
    }
    let sets: Vec<SetMap> = serde_json::from_str(sets_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut set_map = std::collections::HashMap::new();
    for s in &sets {
        set_map.insert(s.name.clone(), s.id);
    }
    for a in &armors {
        let set_id = set_map.get(&a.set).copied().unwrap_or(0);
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhw(rarity);
        let slot = armor_slot_slug(&a.slot_type);
        let icon_url = format!("/icons/mhw/armor/{}-{}.png", slot, color_slug);
        let icon_name = a.slot_type.clone();
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language) VALUES (?1, 3, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, 'en')",
            rusqlite::params![a.id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id, a.armor_type, a.gender.clone().unwrap_or_else(|| "both".to_string()), a.crafting_cost, a.description, icon_name, icon_color, icon_url],
        )?;
    }
    Ok(())
}

fn seed_mhwilds_armor_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct AMat {
        armor_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhwilds_armor_materials.json");
    let mats: Vec<AMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.armor_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhwilds_armor_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM armor WHERE game_id = 3 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 3",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((armor_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![aid, sid, pts])?;
    }
    Ok(())
}

fn seed_mhwilds_weapon_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM weapons WHERE game_id = 3 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (weapon_id, skills_str) = r?;
        let mut resolved = false;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 3",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((weapon_id, sid, pts));
                resolved = true;
            }
        }
        if !resolved {
            // Single-rank specials stored bare (e.g. 'Ballistics') — points = max_level.
            let name = skills_str.trim().to_string();
            let hit: Option<(i32, Option<i32>)> = conn
                .query_row(
                    "SELECT id, max_level FROM skills WHERE name = ?1 AND game_id = 3",
                    rusqlite::params![name],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .optional()?;
            if let Some((sid, max_lv)) = hit {
                to_insert.push((weapon_id, sid, max_lv.unwrap_or(1)));
            }
        }
    }
    drop(stmt);
    for (wid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO weapon_skill_points (weapon_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![wid, sid, pts])?;
    }
    Ok(())
}

fn seed_mhwilds_monster_equipment(conn: &Connection) -> Result<()> {
    // Same derivation as MHW (spec 002b): link equipment via material drops.
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 3, md.monster_id, 'armor', am.armor_id
         FROM armor_materials am
         JOIN armor a ON a.id = am.armor_id AND a.game_id = 3
         JOIN items i ON i.id = am.item_id AND i.game_id = 3
         JOIN monster_drops md ON md.item_id = am.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 3",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 3, md.monster_id, 'weapon', wm.weapon_id
         FROM weapon_materials wm
         JOIN weapons w ON w.id = wm.weapon_id AND w.game_id = 3
         JOIN items i ON i.id = wm.item_id AND i.game_id = 3
         JOIN monster_drops md ON md.item_id = wm.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 3",
        [],
    )?;
    Ok(())
}

// ── MH Rise (game_id 2) ── Phase A bulk (scripts/generate_mhrise_bulk.py)
// Approximations documented in spec 003: armor/deco skill points = 1
// (source has names, no levels); weapons carry no element/sharpness/slots/
// skills/tree yet; no numeric weaknesses/drops/sources (Phase B Kiranico).
// Portraits live in static/icons/mhr/monsters/
// (scripts/download_mhr_wilds_monster_icons.py, Fandom MHR:_Monsters);
// items resolve through the mhfu reuse map, quests through generic mhfu icons.

fn seed_mhr_monsters(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseMon {
        id: i32,
        name: String,
        species: String,
        size: String,
        description: Option<String>,
        sort_order: Option<i32>,
    }
    let json_data = include_str!("../../data/mhr_monsters.json");
    let mons: Vec<RiseMon> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in &mons {
        let slug = monster_icon_slug(&m.name);
        // Lists render the 96px `-sm` variant; detail pages use the master.
        let icon_url = format!("/icons/mhr/monsters/{}-sm.png", slug);
        let icon_url_lg = format!("/icons/mhr/monsters/{}.png", slug);
        let icon_color = monster_icon_color(&m.species);
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, sort_order, icon_name, icon_color, icon_url, icon_url_lg, language) VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'en')",
            rusqlite::params![m.id, m.name, m.species, m.size, m.description, m.sort_order, m.name, icon_color, icon_url, icon_url_lg],
        )?;
    }
    // Pre-split installs are migrated by the monster_thumbs_rise_wilds data patch.
    Ok(())
}

fn seed_mhr_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhr_items.json");
    let items: Vec<MhwItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        // Upstream carries no icons; resolve via category map + name fallback
        // so every row gets an offline mhfu icon instead of NULL.
        let (icon_url, icon_name, icon_color) = resolve_item_icon(
            it.icon_url.as_deref(),
            it.icon_name.as_deref(),
            it.icon_color.as_deref(),
            &it.category,
            it.subcategory.as_deref(),
            &it.name,
        );
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_name, icon_color, icon_url, description, sort_order, language) VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![it.id, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, icon_name, icon_color, icon_url, it.description, it.sort_order],
        )?;
    }
    // Existing installs are covered by the item_icons_mhr data patch.
    Ok(())
}

fn seed_mhr_skills(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseSkill {
        id: i32,
        name: String,
        description: Option<String>,
        max_level: Option<i32>,
    }
    let json_data = include_str!("../../data/mhr_skills_new.json");
    let skills: Vec<RiseSkill> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language) VALUES (?1, 2, ?2, ?3, ?4, 'en')",
            rusqlite::params![s.id, s.name, s.description, s.max_level],
        )?;
    }
    Ok(())
}

fn seed_mhr_skill_levels(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseLevel {
        id: i32,
        skill_id: i32,
        points: i32,
        ability_name: String,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhr_skill_levels.json");
    let levels: Vec<RiseLevel> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for l in levels {
        conn.execute(
            "INSERT OR IGNORE INTO skill_levels (id, skill_id, points, ability_name, description, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![l.id, l.skill_id, l.points, l.ability_name, l.description],
        )?;
    }
    Ok(())
}

fn seed_mhr_decorations(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhr_decorations.json");
    let decos: Vec<DecoJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in decos {
        let mut primary_id: Option<i32> = None;
        let mut primary_pts: Option<i32> = None;
        let mut secondary_id: Option<i32> = None;
        let mut secondary_pts: Option<i32> = None;
        for (idx, sp) in d.skill_points.iter().enumerate() {
            let normalized = sp.name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 2",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if idx == 0 {
                primary_id = sid;
                primary_pts = Some(sp.points);
            } else if idx == 1 {
                secondary_id = sid;
                secondary_pts = Some(sp.points);
            }
        }
        if primary_id.is_none() {
            continue;
        }
        let primary_skill_name = d
            .skill_points
            .first()
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        let (icon_file, icon_color) = decoration_skill_icon(primary_skill_name);
        let icon_url = format!("/icons/mhfu/decorations/{}", icon_file);
        let icon_name = primary_skill_name.to_string();
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, icon_name, icon_color, icon_url, language) VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![d.id, d.name, primary_id, primary_pts, primary_pts, secondary_id, secondary_pts, d.slot_size, d.rarity, d.price, icon_name, icon_color, icon_url],
        )?;
        for m in &d.materials {
            let iid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM items WHERE LOWER(name) = LOWER(?1) AND game_id = 2",
                    rusqlite::params![m.name],
                    |row| row.get(0),
                )
                .optional()?;
            if iid.is_none() {
                continue;
            }
            conn.execute(
                "INSERT OR IGNORE INTO decoration_materials (decoration_id, item_id, item_name, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![d.id, iid, m.name, m.amount],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhr_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, 'en')",
            rusqlite::params![w.id, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description, w.sort_order, w.weapon_type, icon_color, icon_url],
        )?;
    }
    // Weapon-tree rebuild (Game8 edges, Bow/LBG relabel, Smith DFS order,
    // slot backfills): overwrite stale rows on existing installs, since
    // INSERT OR IGNORE never updates. Icons ride along because the Bow/LBG
    // relabel changes the per-type icon slug. Idempotent. The statement is
    // prepared once: ~3950 executions per boot otherwise each pay a full
    // prepare cycle.
    let mut backfill = conn.prepare(
        "UPDATE weapons SET weapon_type = ?1, upgrade_path = ?2, sort_order = ?3, slots = ?4,
                            icon_name = ?5, icon_color = ?6, icon_url = ?7
         WHERE id = ?8 AND game_id = 2
         AND (COALESCE(weapon_type, '') != COALESCE(?1, '')
              OR COALESCE(upgrade_path, '') != COALESCE(?2, '')
              OR COALESCE(sort_order, -1) != COALESCE(?3, -1)
              OR COALESCE(slots, '') != COALESCE(?4, '')
              OR COALESCE(icon_name, '') != COALESCE(?5, '')
              OR COALESCE(icon_url, '') != COALESCE(?7, ''))",
    )?;
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let (icon_color, color_slug) = weapon_icon_color_mhw(w.rarity);
        let icon_url = format!("/icons/mhw/weapons/{}-{}.png", slug, color_slug);
        backfill.execute(rusqlite::params![
            w.weapon_type,
            w.upgrade_path,
            w.sort_order,
            w.slots,
            w.weapon_type,
            icon_color,
            icon_url,
            w.id
        ])?;
    }
    Ok(())
}

fn seed_mhr_weapon_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WMat {
        weapon_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhr_weapon_materials.json");
    let mats: Vec<WMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if weapon_exists(conn, MHR, m.weapon_id)? && item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.weapon_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_weapon_craft(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct WCraft {
        weapon_id: i32,
        craft_kind: String,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhr_weapon_craft.json");
    let recs: Vec<WCraft> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in recs {
        if weapon_exists(conn, MHR, r.weapon_id)? && item_exists(conn, r.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO weapon_craft (weapon_id, craft_kind, item_id, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![r.weapon_id, r.craft_kind, r.item_id, r.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_armor_sets(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseSet {
        id: i32,
        name: String,
        bonus_skill: Option<String>,
        bonus_required: Option<i32>,
    }
    let json_data = include_str!("../../data/mhr_armor_sets.json");
    let sets: Vec<RiseSet> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for s in sets {
        conn.execute(
            "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language) VALUES (?1, 2, ?2, ?3, ?4, 'en')",
            rusqlite::params![s.id, s.name, s.bonus_skill, s.bonus_required],
        )?;
    }
    Ok(())
}

fn seed_mhr_armor(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseArmor {
        id: i32,
        set: String,
        slot_type: String,
        name: String,
        rank: String,
        rarity: Option<i32>,
        defense_base: Option<i32>,
        defense_max: Option<i32>,
        resistance_fire: Option<i32>,
        resistance_water: Option<i32>,
        resistance_thunder: Option<i32>,
        resistance_ice: Option<i32>,
        resistance_dragon: Option<i32>,
        slots: Option<String>,
        skills: Option<String>,
        armor_type: Option<String>,
        gender: Option<String>,
        crafting_cost: Option<i32>,
        description: Option<String>,
    }
    let json_data = include_str!("../../data/mhr_armor.json");
    let armors: Vec<RiseArmor> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let sets_data = include_str!("../../data/mhr_armor_sets.json");
    #[derive(Deserialize)]
    struct SetMap {
        id: i32,
        name: String,
    }
    let sets: Vec<SetMap> = serde_json::from_str(sets_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let mut set_map = std::collections::HashMap::new();
    for s in &sets {
        set_map.insert(s.name.clone(), s.id);
    }
    for a in &armors {
        let set_id = set_map.get(&a.set).copied().unwrap_or(0);
        let rarity = a.rarity.unwrap_or(1);
        let (icon_color, color_slug) = armor_icon_color_mhw(rarity);
        let slot = armor_slot_slug(&a.slot_type);
        let icon_url = format!("/icons/mhw/armor/{}-{}.png", slot, color_slug);
        let icon_name = a.slot_type.clone();
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language) VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, 'en')",
            rusqlite::params![a.id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id, a.armor_type, a.gender.clone().unwrap_or_else(|| "both".to_string()), a.crafting_cost, a.description, icon_name, icon_color, icon_url],
        )?;
    }
    Ok(())
}

fn seed_mhr_armor_materials(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct AMat {
        armor_id: i32,
        item_id: i32,
        quantity: i32,
    }
    let json_data = include_str!("../../data/mhr_armor_materials.json");
    let mats: Vec<AMat> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in mats {
        if item_exists(conn, m.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![m.armor_id, m.item_id, m.quantity],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_armor_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, skills FROM armor WHERE game_id = 2 AND skills IS NOT NULL AND skills != ''",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = name.trim().to_string();
            let sid: Option<i32> = conn
                .query_row(
                    "SELECT id FROM skills WHERE name = ?1 AND game_id = 2",
                    rusqlite::params![normalized],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(sid) = sid {
                to_insert.push((armor_id, sid, pts));
            }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![aid, sid, pts])?;
    }
    Ok(())
}

fn seed_mhr_weapon_skill_points(_conn: &Connection) -> Result<()> {
    // Phase A: bulk weapons carry no skills — no-op that keeps the
    // orchestrator symmetric for the Phase B backfill.
    Ok(())
}

fn seed_mhr_monster_equipment(conn: &Connection) -> Result<()> {
    // No monster drops upstream in Phase A, so derivation yields nothing yet;
    // kept for symmetry — Phase B drops will flow through here.
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 2, md.monster_id, 'armor', am.armor_id
         FROM armor_materials am
         JOIN armor a ON a.id = am.armor_id AND a.game_id = 2
         JOIN items i ON i.id = am.item_id AND i.game_id = 2
         JOIN monster_drops md ON md.item_id = am.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 2",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id)
         SELECT DISTINCT 2, md.monster_id, 'weapon', wm.weapon_id
         FROM weapon_materials wm
         JOIN weapons w ON w.id = wm.weapon_id AND w.game_id = 2
         JOIN items i ON i.id = wm.item_id AND i.game_id = 2
         JOIN monster_drops md ON md.item_id = wm.item_id
         JOIN monsters m ON m.id = md.monster_id AND m.game_id = 2",
        [],
    )?;
    Ok(())
}

fn seed_mhr_monster_weaknesses(conn: &Connection) -> Result<()> {
    // Phase B: state-0 hitzones from Kiranico (scripts/fetch_mhrise_kiranico.py
    // + merge_mhrise_kiranico.py). Same column scale as mh2g.
    let json_data = include_str!("../../data/mhr_monster_weaknesses.json");
    let weaknesses: Vec<WeaknessJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![w.monster_id, w.part_name, w.sever, w.blunt, w.projectile, w.fire, w.water, w.thunder, w.ice, w.dragon],
        )?;
    }
    Ok(())
}

fn seed_mhr_monster_drops(conn: &Connection) -> Result<()> {
    // Phase B: drop tables from Kiranico monster pages (rank/method mapped,
    // rates parsed by the merge script).
    #[derive(Deserialize)]
    struct RiseDrop {
        monster_id: i32,
        item_id: i32,
        method: String,
        part: Option<String>,
        rank: Option<String>,
        quantity: i32,
        probability: Option<f64>,
        condition: Option<String>,
    }
    let json_data = include_str!("../../data/mhr_monster_drops.json");
    let drops: Vec<RiseDrop> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for d in drops {
        if item_exists(conn, d.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
                rusqlite::params![d.monster_id, d.item_id, d.method, d.part, d.rank, d.quantity, d.probability, d.condition],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_item_sources_from_drops(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability) SELECT item_id, CASE method WHEN 'carve' THEN 'carve' WHEN 'capture' THEN 'capture' WHEN 'drop' THEN 'drop' WHEN 'break' THEN 'break' WHEN 'reward' THEN 'reward' ELSE method END, monster_id, quantity, quantity, probability FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 2)",
        [],
    )?;
    Ok(())
}

fn seed_mhr_quest_rewards(conn: &Connection) -> Result<()> {
    // Phase B: Kiranico quest reward tables (condition = raw method:
    // common / additional / add_target).
    #[derive(Deserialize)]
    struct RiseReward {
        id: i32,
        quest_id: i32,
        item_id: i32,
        quantity: i32,
        probability: Option<f64>,
        condition: Option<String>,
    }
    let json_data = include_str!("../../data/mhr_quest_rewards.json");
    let rewards: Vec<RiseReward> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rewards {
        if item_exists(conn, r.item_id)? {
            conn.execute(
                "INSERT OR IGNORE INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![r.id, r.quest_id, r.item_id, r.quantity, r.probability, r.condition],
            )?;
        }
    }
    Ok(())
}

fn seed_mhr_quests(conn: &Connection) -> Result<()> {
    #[derive(Deserialize)]
    struct RiseQuest {
        id: i32,
        name: String,
        hub: Option<String>,
        category: Option<String>,
        stars: Option<i32>,
        rank: Option<String>,
        objective: Option<String>,
        location: Option<String>,
        client: Option<String>,
        description: Option<String>,
        is_key_quest: Option<bool>,
        main_monsters: Option<Vec<String>>,
    }
    let json_data = include_str!("../../data/mhr_quests.json");
    let quests: Vec<RiseQuest> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for q in &quests {
        let main_json = q
            .main_monsters
            .as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        // Generic mhfu icons: category "Hub/Village/?" -> hunt/unknown,
        // hubs follower/hub_low/... -> nyanta/guild-low/... (see quest_hub_slug).
        let qtype = q.category.as_deref().unwrap_or("?");
        let type_slug = quest_type_slug(qtype);
        let icon_url = format!("/icons/mhfu/quests/{}.png", type_slug);
        let icon_color = quest_type_color(qtype);
        let hub_name = q.hub.clone().unwrap_or_else(|| "unknown".to_string());
        let hub_slug = quest_hub_slug(&hub_name);
        let hub_icon_url = format!("/icons/mhfu/quests/hubs/{}.png", hub_slug);
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, hub, category, stars, rank, objective, location, client, description, is_key_quest, main_monsters, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language)
             VALUES (?1, 2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, 'Gray', ?17, 'en')",
            rusqlite::params![q.id, q.name, q.hub, q.category, q.stars, q.rank, q.objective, q.location, q.client, q.description, q.is_key_quest.unwrap_or(false), main_json, qtype, icon_color, icon_url, hub_name, hub_icon_url],
        )?;
    }
    // Existing installs are covered by the quest_icons_mhr data patch.
    Ok(())
}
