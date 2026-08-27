use rusqlite::{Connection, OptionalExtension, Result};
use serde::Deserialize;

const MH2G: i32 = 5;
const MHP3RD: i32 = 4;

pub fn seed(conn: &Connection) -> Result<()> {
    seed_games(conn)?;
    clear_mh2g(conn)?;
    clear_mhp3rd(conn)?;
    seed_monsters(conn)?;
    backfill_monster_descriptions(conn)?;
    seed_items(conn)?;
    backfill_item_descriptions(conn)?;
    seed_monster_drops(conn)?;
    seed_item_sources_from_drops(conn)?;
    seed_extra_item_sources(conn)?;
    seed_monster_equipment(conn)?;
    seed_monster_weaknesses(conn)?;
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
    // MHP3rd (ULJM-05800) — faithful to ISO normal (English patch), village_low/high + guild_low/high
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
    Ok(())
}

fn clear_game(conn: &Connection, game_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM weapon_materials WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM weapon_craft WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM weapon_skill_points WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM armor_materials WHERE armor_id IN (SELECT id FROM armor WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM armor_skill_points WHERE armor_id IN (SELECT id FROM armor WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM quest_rewards WHERE quest_id IN (SELECT id FROM quests WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM item_sources WHERE item_id IN (SELECT id FROM items WHERE game_id = ?1)
            OR (source_type = 'carve' AND source_id IN (SELECT id FROM monsters WHERE game_id = ?1))
            OR (source_type = 'quest_reward' AND source_id IN (SELECT id FROM quests WHERE game_id = ?1))",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE game_id = ?1)
            OR component_item_id IN (SELECT id FROM items WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM monster_equipment WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = ?1) OR game_id = ?1",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM skill_levels WHERE skill_id IN (SELECT id FROM skills WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute(
        "DELETE FROM decoration_materials WHERE decoration_id IN (SELECT id FROM decorations WHERE game_id = ?1)",
        rusqlite::params![game_id],
    )?;
    conn.execute("DELETE FROM decorations WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM monsters WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM items WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM weapons WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM armor WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM armor_sets WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM quests WHERE game_id = ?1", rusqlite::params![game_id])?;
    conn.execute("DELETE FROM skills WHERE game_id = ?1", rusqlite::params![game_id])?;
    Ok(())
}

/// Wipe MH2G-related rows in FK-safe order so the real dataset can be rebuilt.
fn clear_mh2g(conn: &Connection) -> Result<()> {
    clear_game(conn, MH2G)
}

fn clear_mhp3rd(conn: &Connection) -> Result<()> {
    clear_game(conn, MHP3RD)
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

    for m in monsters {
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, NULL, 'en')",
            rusqlite::params![m.id, MH2G, m.name, m.species, m.size],
        )?;
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
}

fn seed_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_items.json");
    let items: Vec<ItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for it in &items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 'en')",
            rusqlite::params![it.id, MH2G, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price],
        )?;
    }

    // Backfill for existing DBs where category/subcategory changed (e.g., Power Juice Material→Consumable, Huskberry→Ammo)
    for it in &items {
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2 WHERE id = ?3 AND game_id = 5 AND (category IS NULL OR category != ?1 OR subcategory IS NULL OR subcategory != ?2)",
            rusqlite::params![it.category, it.subcategory, it.id],
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
struct DropJson {    monster_id: i32,
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
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![r.result_item_id, r.component_item_id, r.quantity, r.result_quantity],
        )?;
    }
    Ok(())
}

#[derive(Deserialize)]
struct WeaknessJson {
    monster_id: i32,
    part_name: String,
    sever: i32,
    blunt: i32,
    projectile: i32,
    fire: i32,
    water: i32,
    thunder: i32,
    ice: i32,
    dragon: i32,
}

fn seed_monster_weaknesses(conn: &Connection) -> Result<()> {
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

    Ok(())
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
}

fn seed_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for w in weapons {
        conn.execute(
            "INSERT OR IGNORE INTO weapons
                (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                 sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, 'en')",
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
                w.description
            ],
        )?;
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
                .query_row("SELECT id FROM items WHERE name = ?1 AND game_id = 5", rusqlite::params![m.item], |row| row.get(0))
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
                .query_row("SELECT id FROM items WHERE name = ?1 AND game_id = 5", rusqlite::params![m.item], |row| row.get(0))
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
        "Helm", "Cap", "Crown", "Mask", "Hat", "Hood", "Head", "Face", "Brain", "Soul", "Horn", "Crest", "Glare", "Snarl", "Piercing",
        "Mail", "Vest", "Jacket", "Armor", "Hide", "Skin", "Coat", "Plate", "Belt", "Tasset", "Kilt", "Coil", "Obi",
        "Vambraces", "Guards", "Guards", "Braces", "Gloves", "Mittens",
        "Greaves", "Leggings", "Boots", "Pants", "Legs", "Feet",
    ];
    let parts: Vec<&str> = armor.name.split_whitespace().collect();
    if parts.is_empty() {
        return armor.set.clone();
    }
    if parts.len() == 1 {
        // Concatenated names like BlackBeltLeggingsX (no spaces) — strip slot suffix
        let name = parts[0];
        for &slot in SLOT_WORDS {
            for &var in &["X", "Z", "S", "U", "D", "C"] {
                let suff = format!("{}{}", slot, var);
                if name.to_ascii_lowercase().ends_with(&suff.to_ascii_lowercase()) {
                    let base = name[..name.len() - suff.len()].trim();
                    let derived = if base.is_empty() { slot.to_string() } else { format!("{} {}", base, var) };
                    return derived;
                }
            }
            if name.to_ascii_lowercase().ends_with(&slot.to_ascii_lowercase()) {
                let base = name[..name.len() - slot.len()].trim();
                return if base.is_empty() { slot.to_string() } else { base.to_string() };
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
    let is_slot = SLOT_WORDS.iter().any(|&w| w.eq_ignore_ascii_case(slot_word));
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
    let json_data = include_str!("../../data/mh2g_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // Assign set ids in order of first appearance using derived set name (faithful to game, fixes singleton D variants)
    let mut set_id: i32 = 0;
    let mut seen = Vec::<String>::new();
    for a in &armors {
        let set_name = derive_set_name(a);
        if !seen.contains(&set_name) {
            seen.push(set_name.clone());
            set_id += 1;
            conn.execute(
                "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language)
                 VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
                rusqlite::params![set_id, MH2G, set_name],
            )?;
        }
    }

    Ok(())
}

fn seed_armor(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    let mut set_map: Vec<(String, i32)> = Vec::new();
    let mut set_id: i32 = 0;
    for a in &armors {
        let set_name = derive_set_name(a);
        if !set_map.iter().any(|(s, _)| s == &set_name) {
            set_id += 1;
            set_map.push((set_name.clone(), set_id));
        }
    }
    let set_id_of = |set: &str| -> i32 { set_map.iter().find(|(s, _)| s == set).map(|(_, i)| *i).unwrap_or(0) };
    let set_id_of_armor = |armor: &ArmorJson| -> i32 { set_id_of(&derive_set_name(armor)) };

    for a in armors {
        let gender = a.gender.clone().unwrap_or_else(|| "both".to_string());
        conn.execute(
            "INSERT OR IGNORE INTO armor
                (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                 resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                 slots, skills, set_id, armor_type, gender, crafting_cost, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, 'en')",
            rusqlite::params![
                a.id, MH2G, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, set_id_of_armor(&a), a.armor_type, gender, a.crafting_cost, a.description
            ],
        )?;
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
    stars: Option<i32>,
    objective: String,
    location: String,
    time_limit: Option<i32>,
    faints_allowed: Option<i32>,
    is_key_quest: Option<bool>,
    description: Option<String>,
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

    for q in quests {
        let main_monsters_json = q
            .main_monsters
            .as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, name_original, type, rank, hub, stars, objective, location, time_limit, faints_allowed, is_key_quest, description, client, requirements, reward_money, contract_fee, main_monsters, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, 'en')",
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
                q.location,
                q.time_limit.unwrap_or(50),
                q.faints_allowed.unwrap_or(3),
                q.is_key_quest.unwrap_or(false),
                q.description,
                q.client,
                q.requirements,
                q.reward_money,
                q.contract_fee,
                main_monsters_json
            ],
        )?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct QuestRewardJson {
    id: i32,
    quest_id: i32,
    item_id: i32,
    quantity: i32,
    probability: f64,
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

        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10, 'en')",
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
                d.price
            ],
        )?;

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
    let mut stmt = conn.prepare("SELECT id, skills FROM armor WHERE game_id = 5 AND skills IS NOT NULL")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))?;

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
    let mut stmt = conn.prepare("SELECT id, skills FROM weapons WHERE game_id = 5 AND skills IS NOT NULL AND skills != ''")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))?;

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

// ── MHP3rd (ULJM-05800) ── village_low/high + guild_low/high, faithful to ISO normal ──

fn seed_mhp3rd_monsters(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_monsters.json");
    let monsters: Vec<MonsterJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for m in monsters {
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, language) VALUES (?1, ?2, ?3, ?4, ?5, NULL, 'en')",
            rusqlite::params![m.id, MHP3RD, m.name, m.species, m.size],
        )?;
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
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 'en')",
            rusqlite::params![it.id, MHP3RD, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price],
        )?;
    }
    for it in &items {
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2 WHERE id = ?3 AND game_id = 4 AND (category IS NULL OR category != ?1 OR subcategory IS NULL OR subcategory != ?2)",
            rusqlite::params![it.category, it.subcategory, it.id],
        )?;
    }
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
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![r.result_item_id, r.component_item_id, r.quantity, r.result_quantity],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_weapons(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapons.json");
    let weapons: Vec<WeaponJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for w in weapons {
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, 'en')",
            rusqlite::params![w.id, MHP3RD, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description],
        )?;
    }
    Ok(())
}
fn seed_mhp3rd_weapon_materials(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapon_materials.json");
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
fn seed_mhp3rd_weapon_craft(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhp3rd_weapon_craft.json");
    let rows: Vec<WeaponCraftJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for r in rows {
        for m in &r.forge {
            let iid: Option<i32> = conn
                .query_row("SELECT id FROM items WHERE name = ?1 AND game_id = 4", rusqlite::params![m.item], |row| row.get(0))
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
                .query_row("SELECT id FROM items WHERE name = ?1 AND game_id = 4", rusqlite::params![m.item], |row| row.get(0))
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
    let set_id_of = |set: &str| -> i32 { set_map.iter().find(|(s, _)| s == set).map(|(_, i)| *i).unwrap_or(0) };
    let set_id_of_armor = |armor: &ArmorJson| -> i32 { set_id_of(&derive_set_name(armor)) };
    for a in armors {
        let gender = a.gender.clone().unwrap_or_else(|| "both".to_string());
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, 'en')",
            rusqlite::params![a.id, MHP3RD, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id_of_armor(&a), a.armor_type, gender, a.crafting_cost, a.description],
        )?;
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
    for q in quests {
        let main_monsters_json = q.main_monsters.as_ref().map(|v| serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string()));
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, name_original, type, rank, hub, stars, objective, location, time_limit, faints_allowed, is_key_quest, description, client, requirements, reward_money, contract_fee, main_monsters, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, 'en')",
            rusqlite::params![q.id, MHP3RD, q.name, q.name_original, q.qtype, q.rank, q.hub, q.stars, q.objective, q.location, q.time_limit.unwrap_or(50), q.faints_allowed.unwrap_or(3), q.is_key_quest.unwrap_or(false), q.description, q.client, q.requirements, q.reward_money, q.contract_fee, main_monsters_json],
        )?;
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
            let sid: Option<i32> = conn.query_row("SELECT id FROM skills WHERE name = ?1 AND game_id = 4", rusqlite::params![normalized], |row| row.get(0)).optional()?;
            if idx == 0 { primary_id = sid; primary_pts = Some(sp.points); } else if idx == 1 { secondary_id = sid; secondary_pts = Some(sp.points); }
        }
        if primary_id.is_none() { continue; }
        conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_level, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10, 'en')",
            rusqlite::params![d.id, MHP3RD, d.name, primary_id, primary_pts, primary_pts, secondary_id, secondary_pts, d.slot_size, d.price],
        )?;
        for m in &d.materials {
            let normalized_mat = normalize_item_name_p3rd(&m.name);
            let iid: Option<i32> = conn.query_row("SELECT id FROM items WHERE LOWER(name) = LOWER(?1) AND game_id = 4", rusqlite::params![normalized_mat], |row| row.get(0)).optional()?;
            conn.execute(
                "INSERT OR IGNORE INTO decoration_materials (decoration_id, item_id, item_name, quantity) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![d.id, iid, m.name, m.amount],
            )?;
        }
    }
    Ok(())
}
fn seed_mhp3rd_armor_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, skills FROM armor WHERE game_id = 4 AND skills IS NOT NULL")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (armor_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name_p3rd(&name);
            let sid: Option<i32> = conn.query_row("SELECT id FROM skills WHERE name = ?1 AND game_id = 4", rusqlite::params![normalized], |row| row.get(0)).optional()?;
            if let Some(sid) = sid { to_insert.push((armor_id, sid, pts)); }
        }
    }
    drop(stmt);
    for (aid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO armor_skill_points (armor_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![aid, sid, pts])?;
    }
    Ok(())
}
fn seed_mhp3rd_weapon_skill_points(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, skills FROM weapons WHERE game_id = 4 AND skills IS NOT NULL AND skills != ''")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))?;
    let mut to_insert: Vec<(i32, i32, i32)> = Vec::new();
    for r in rows {
        let (weapon_id, skills_str) = r?;
        for (name, pts) in parse_skill_string(&skills_str) {
            let normalized = normalize_skill_name_p3rd(&name);
            let sid: Option<i32> = conn.query_row("SELECT id FROM skills WHERE name = ?1 AND game_id = 4", rusqlite::params![normalized], |row| row.get(0)).optional()?;
            if let Some(sid) = sid { to_insert.push((weapon_id, sid, pts)); }
        }
    }
    drop(stmt);
    for (wid, sid, pts) in to_insert {
        conn.execute("INSERT OR IGNORE INTO weapon_skill_points (weapon_id, skill_id, points) VALUES (?1, ?2, ?3)", rusqlite::params![wid, sid, pts])?;
    }
    Ok(())
}
