use rusqlite::{Connection, OptionalExtension, Result};
use serde::Deserialize;

const MH2G: i32 = 5;

pub fn seed(conn: &Connection) -> Result<()> {
    seed_games(conn)?;
    clear_mh2g(conn)?;
    seed_monsters(conn)?;
    backfill_monster_descriptions(conn)?;
    seed_items(conn)?;
    backfill_item_descriptions(conn)?;
    seed_monster_drops(conn)?;
    seed_item_sources_from_drops(conn)?;
    seed_monster_equipment(conn)?;
    seed_item_combine(conn)?;
    seed_weapons(conn)?;
    seed_weapon_materials(conn)?;
    seed_weapon_craft(conn)?;
    seed_armor_sets(conn)?;
    seed_armor(conn)?;
    seed_armor_materials(conn)?;
    seed_quests(conn)?;
    seed_skills(conn)?;
    Ok(())
}

/// Wipe MH2G-related rows in FK-safe order so the real dataset can be rebuilt.
fn clear_mh2g(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        DELETE FROM weapon_materials WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 5);
        DELETE FROM weapon_craft WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 5);
        DELETE FROM armor_materials WHERE armor_id IN (SELECT id FROM armor WHERE game_id = 5);
        DELETE FROM quest_rewards WHERE quest_id IN (SELECT id FROM quests WHERE game_id = 5);
        DELETE FROM item_sources WHERE item_id IN (SELECT id FROM items WHERE game_id = 5)
            OR (source_type = 'carve' AND source_id IN (SELECT id FROM monsters WHERE game_id = 5))
            OR (source_type = 'quest_reward' AND source_id IN (SELECT id FROM quests WHERE game_id = 5));
        DELETE FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE game_id = 5)
            OR component_item_id IN (SELECT id FROM items WHERE game_id = 5);
        DELETE FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 5);
        DELETE FROM monster_equipment WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 5) OR game_id = 5;
        DELETE FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 5);
        DELETE FROM monsters WHERE game_id = 5;
        DELETE FROM items WHERE game_id = 5;
        DELETE FROM weapons WHERE game_id = 5;
        DELETE FROM armor WHERE game_id = 5;
        DELETE FROM armor_sets WHERE game_id = 5;
        DELETE FROM quests WHERE game_id = 5;
        DELETE FROM skills WHERE game_id = 5;
    ")?;
    Ok(())
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
    rarity: Option<i32>,
    sell_price: Option<i32>,
}

fn seed_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_items.json");
    let items: Vec<ItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for it in items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, rarity, sell_price, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, 'en')",
            rusqlite::params![it.id, MH2G, it.name, it.category, it.rarity, it.sell_price],
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
}

fn seed_item_combine(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_item_combine.json");
    let recipes: Vec<CombineJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for rc in recipes {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![rc.result_item_id, rc.component_item_id, rc.quantity, rc.result_quantity],
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

#[allow(dead_code)]
fn seed_monster_weaknesses(conn: &Connection) -> Result<()> {
    // Placeholder weakness/hitzone data — disabled in short term until real
    // MHFU hitzone data is sourced. Monster detail handles an empty list.
    let weaknesses: &[(i32, &str, i32, i32, i32, i32, i32, i32, i32, i32)] = &[
        (1, "Head", 55, 60, 40, 0, 5, 10, 15, 20),
        (1, "Wings", 25, 25, 30, 0, 5, 10, 15, 15),
        (1, "Torso", 25, 20, 20, 0, 5, 10, 15, 10),
        (1, "Legs", 25, 25, 20, 0, 5, 10, 15, 10),
        (1, "Tail", 30, 30, 25, 0, 5, 10, 15, 10),
        (2, "Head", 55, 55, 40, 0, 10, 15, 20, 25),
        (2, "Wings", 30, 30, 30, 0, 10, 15, 20, 15),
        (2, "Torso", 30, 25, 20, 0, 10, 15, 20, 15),
        (2, "Legs", 25, 25, 20, 0, 10, 15, 20, 15),
        (2, "Tail", 40, 40, 30, 0, 10, 15, 20, 15),
        (3, "Head", 55, 65, 45, 15, 10, 20, 10, 5),
        (3, "Body", 25, 25, 20, 10, 5, 15, 5, 5),
        (3, "Wings", 35, 35, 30, 15, 5, 15, 5, 5),
        (3, "Tail", 30, 30, 20, 10, 5, 15, 5, 5),
        (4, "Head", 60, 55, 45, 15, 20, 25, 15, 10),
        (4, "Neck", 50, 45, 35, 10, 15, 20, 10, 5),
        (4, "Body", 25, 25, 20, 10, 10, 15, 10, 5),
        (4, "Tail", 35, 30, 25, 10, 15, 20, 10, 5),
        (5, "Head", 60, 60, 50, 0, 20, 15, 25, 10),
        (5, "Torso", 40, 40, 30, 0, 15, 10, 20, 5),
        (5, "Wings", 35, 35, 30, 0, 15, 10, 20, 5),
        (6, "Head", 55, 55, 45, 0, 20, 15, 25, 10),
        (6, "Torso", 35, 35, 25, 0, 15, 10, 20, 5),
        (6, "Wings", 30, 30, 25, 0, 15, 10, 20, 5),
        (7, "Head", 45, 45, 30, 25, 15, 0, 5, 10),
        (7, "Torso", 35, 40, 25, 20, 10, 0, 5, 5),
        (7, "Legs", 40, 45, 30, 25, 15, 0, 5, 10),
        (8, "Head", 45, 45, 30, 0, 20, 10, 15, 10),
        (8, "Torso", 35, 40, 25, 0, 15, 5, 10, 5),
        (8, "Legs", 40, 45, 30, 0, 20, 10, 15, 10),
        (9, "Head", 50, 50, 40, 25, 15, 10, 5, 10),
        (9, "Torso", 30, 30, 25, 20, 10, 10, 5, 5),
        (9, "Tail", 35, 35, 30, 20, 10, 10, 5, 5),
        (10, "Head", 50, 50, 40, 25, 15, 10, 5, 10),
        (10, "Torso", 30, 30, 25, 20, 10, 10, 5, 5),
        (10, "Tail", 35, 35, 30, 20, 10, 10, 5, 5),
        (11, "Head", 45, 45, 40, 10, 0, 25, 15, 10),
        (11, "Torso", 25, 25, 20, 5, 0, 20, 10, 5),
        (11, "Legs", 30, 30, 25, 5, 0, 20, 10, 5),
        (12, "Head", 45, 45, 40, 10, 0, 25, 15, 10),
        (12, "Torso", 25, 25, 20, 5, 0, 20, 10, 5),
        (12, "Legs", 30, 30, 25, 5, 0, 20, 10, 5),
        (13, "Head", 50, 50, 40, 15, 5, 25, 15, 10),
        (13, "Torso", 30, 30, 25, 10, 0, 20, 10, 5),
        (14, "Head", 65, 60, 50, 15, 15, 25, 15, 10),
        (14, "Torso", 40, 40, 30, 10, 10, 20, 10, 5),
        (15, "Head", 65, 60, 50, 5, 25, 20, 15, 10),
        (15, "Torso", 40, 40, 30, 0, 20, 15, 10, 5),
        (16, "Head", 65, 60, 50, 15, 15, 25, 20, 10),
        (16, "Torso", 40, 40, 30, 10, 10, 20, 15, 5),
        (17, "Head", 50, 60, 40, 25, 10, 20, 15, 10),
        (17, "Shell", 15, 10, 10, 10, 5, 10, 10, 5),
        (17, "Claws", 35, 30, 25, 20, 10, 15, 10, 5),
        (18, "Head", 55, 65, 45, 25, 10, 20, 15, 10),
        (18, "Shell", 20, 15, 15, 10, 5, 10, 10, 5),
        (18, "Claws", 40, 35, 30, 20, 10, 15, 10, 5),
        (19, "Head", 55, 60, 50, 10, 15, 25, 20, 10),
        (19, "Belly", 45, 50, 40, 10, 15, 20, 15, 5),
        (19, "Tail", 40, 40, 35, 10, 15, 20, 15, 5),
        (20, "Head", 55, 60, 50, 10, 15, 25, 20, 10),
        (20, "Belly", 45, 50, 40, 10, 15, 20, 15, 5),
        (20, "Tail", 40, 40, 35, 10, 15, 20, 15, 5),
        (21, "Head", 55, 60, 45, 25, 10, 5, 0, 10),
        (21, "Body", 30, 30, 25, 20, 5, 0, 0, 5),
        (21, "Arms", 40, 40, 30, 20, 5, 0, 0, 5),
        (22, "Head", 55, 60, 45, 0, 25, 10, 15, 10),
        (22, "Body", 30, 30, 25, 0, 20, 5, 10, 5),
        (22, "Arms", 40, 40, 30, 0, 20, 5, 10, 5),
        (23, "Head", 55, 50, 45, 5, 10, 0, 15, 5),
        (23, "Arms", 50, 45, 40, 5, 10, 0, 15, 5),
        (23, "Torso", 30, 30, 25, 5, 5, 0, 10, 5),
        (25, "Head", 50, 45, 40, 10, 20, 20, 15, 15),
        (25, "Torso", 25, 25, 20, 5, 10, 10, 10, 5),
        (25, "Wings", 30, 30, 25, 5, 15, 15, 10, 10),
        (26, "Head", 35, 30, 25, 5, 25, 15, 15, 10),
        (26, "Torso", 15, 10, 10, 0, 15, 10, 10, 5),
        (26, "Tail", 25, 20, 20, 5, 20, 10, 10, 5),
        (27, "Head", 35, 30, 25, 0, 25, 15, 15, 10),
        (27, "Torso", 15, 10, 10, 0, 15, 10, 10, 5),
        (27, "Legs", 25, 20, 20, 0, 20, 10, 10, 5),
        (28, "Head", 35, 30, 25, 0, 25, 15, 15, 10),
        (28, "Torso", 15, 10, 10, 0, 15, 10, 10, 5),
        (28, "Legs", 25, 20, 20, 0, 20, 10, 10, 5),
        (29, "Head", 50, 50, 40, 10, 20, 20, 15, 15),
        (29, "Torso", 30, 30, 25, 5, 15, 15, 10, 10),
        (29, "Wings", 35, 35, 30, 5, 15, 15, 10, 10),
        (30, "Head", 50, 50, 40, 10, 20, 20, 15, 15),
        (30, "Torso", 30, 30, 25, 5, 15, 15, 10, 10),
        (30, "Wings", 35, 35, 30, 5, 15, 15, 10, 10),
        (31, "Head", 50, 50, 40, 10, 20, 20, 15, 15),
        (31, "Torso", 30, 30, 25, 5, 15, 15, 10, 10),
        (31, "Wings", 35, 35, 30, 5, 15, 15, 10, 10),
        (37, "Head", 55, 55, 45, 15, 15, 0, 15, 20),
        (37, "Body", 30, 30, 25, 10, 10, 0, 10, 15),
        (38, "Head", 50, 50, 40, 25, 10, 15, 10, 15),
        (38, "Torso", 25, 25, 20, 15, 5, 10, 5, 10),
        (38, "Wings", 30, 30, 25, 20, 5, 10, 5, 10),
        (40, "Head", 45, 45, 35, 0, 25, 15, 15, 15),
        (40, "Torso", 25, 25, 20, 0, 15, 10, 10, 10),
        (40, "Wings", 30, 30, 25, 0, 20, 10, 10, 10),
        (39, "Head", 50, 50, 45, 20, 25, 15, 10, 15),
        (39, "Torso", 30, 30, 25, 15, 20, 10, 5, 10),
        (39, "Tail", 35, 35, 30, 15, 20, 10, 5, 10),
        (46, "Head", 45, 50, 40, 20, 15, 25, 15, 25),
        (46, "Shoulder", 35, 40, 30, 15, 10, 20, 10, 20),
        (46, "Back", 30, 35, 25, 15, 10, 20, 10, 20),
        (47, "Head", 45, 50, 40, 20, 15, 25, 15, 25),
        (47, "Shoulder", 35, 40, 30, 15, 10, 20, 10, 20),
        (47, "Back", 30, 35, 25, 15, 10, 20, 10, 20),
        (48, "Legs", 50, 60, 45, 25, 10, 20, 15, 20),
        (48, "Face", 60, 70, 50, 30, 15, 25, 20, 25),
        (35, "Head", 50, 55, 40, 0, 15, 20, 25, 15),
        (35, "Torso", 25, 25, 20, 0, 10, 15, 20, 10),
        (35, "Tail", 35, 35, 30, 0, 10, 15, 20, 10),
        (36, "Head", 50, 55, 40, 25, 15, 20, 0, 15),
        (36, "Torso", 25, 25, 20, 20, 10, 15, 0, 10),
        (36, "Tail", 35, 35, 30, 20, 10, 15, 0, 10),
        (49, "Mouth", 60, 65, 50, 25, 15, 25, 20, 30),
        (49, "Tentacles", 45, 50, 40, 20, 10, 20, 15, 25),
        (34, "Head", 60, 60, 50, 15, 25, 20, 15, 10),
        (34, "Torso", 40, 40, 30, 10, 20, 15, 10, 5),
        (33, "Head", 45, 45, 40, 0, 25, 20, 15, 10),
        (33, "Torso", 25, 25, 20, 0, 20, 15, 10, 5),
        (50, "Head", 60, 60, 50, 15, 15, 20, 20, 30),
        (50, "Torso", 35, 35, 30, 10, 10, 15, 15, 25),
        (50, "Wings", 40, 40, 35, 10, 10, 15, 15, 25),
        (51, "Head", 60, 60, 50, 0, 25, 20, 20, 30),
        (51, "Torso", 35, 35, 30, 0, 20, 15, 15, 25),
        (51, "Wings", 40, 40, 35, 0, 20, 15, 15, 25),
        (52, "Head", 60, 60, 50, 20, 20, 0, 0, 35),
        (52, "Torso", 35, 35, 30, 15, 15, 0, 0, 30),
        (52, "Wings", 40, 40, 35, 15, 15, 0, 0, 30),
    ];

    for (monster_id, part, sever, blunt, proj, fire, water, thunder, ice, dragon) in weaknesses {
        conn.execute(
            "INSERT OR IGNORE INTO monster_weaknesses (monster_id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![monster_id, part, sever, blunt, proj, fire, water, thunder, ice, dragon],
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
    crafting_cost: Option<i32>,
    description: Option<String>,
}

fn seed_armor_sets(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mh2g_armor.json");
    let armors: Vec<ArmorJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // Assign set ids in order of first appearance.
    let mut set_id: i32 = 0;
    let mut seen = Vec::<String>::new();
    for a in &armors {
        if !seen.contains(&a.set) {
            seen.push(a.set.clone());
            set_id += 1;
            conn.execute(
                "INSERT OR IGNORE INTO armor_sets (id, game_id, name, bonus_skill, bonus_required, language)
                 VALUES (?1, ?2, ?3, NULL, NULL, 'en')",
                rusqlite::params![set_id, MH2G, a.set],
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
        if !set_map.iter().any(|(s, _)| s == &a.set) {
            set_id += 1;
            set_map.push((a.set.clone(), set_id));
        }
    }
    let set_id_of = |set: &str| -> i32 { set_map.iter().find(|(s, _)| s == set).map(|(_, i)| *i).unwrap_or(0) };

    for a in armors {
        conn.execute(
            "INSERT OR IGNORE INTO armor
                (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                 resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                 slots, skills, set_id, armor_type, crafting_cost, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, 'en')",
            rusqlite::params![
                a.id, MH2G, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, set_id_of(&a.set), a.armor_type, a.crafting_cost, a.description
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

fn seed_quests(conn: &Connection) -> Result<()> {
    let quests: &[(i32, &str, &str, &str, &str, i32, i32, bool)] = &[
        (1, "Gathering Road", "Gathering", "Low", "Mezeporta", 50, 3, false),
        (2, "The Birth of a Hunter", "Hunting", "Low", "Mezeporta", 50, 3, true),
        (3, "Rathalos, King of the Sky", "Hunting", "High", "Mezeporta", 50, 2, true),
        (4, "Tigrex of the Sand Sea", "Hunting", "High", "Mezeporta", 50, 2, true),
        (5, "Nargacuga, the Shadow", "Hunting", "High", "Mezeporta", 50, 2, true),
        (6, "Slay Rajang!", "Hunting", "G", "Mezeporta", 50, 1, true),
        (7, "Kirin, the Lightning", "Hunting", "G", "Mezeporta", 50, 1, true),
        (8, "Fatalis", "Hunting", "G", "Castle Schrade", 50, 1, true),
        (9, "White Fatalis", "Hunting", "G", "Castle Schrade", 50, 1, true),
        (10, "A Gathering in the Forest", "Gathering", "Low", "Forest and Hills", 50, 3, false),
        (11, "Swimmin' in the Desert", "Gathering", "Low", "Desert", 50, 3, false),
        (12, "The Piscine Wyvern", "Hunting", "Low", "Desert", 50, 3, true),
    ];

    for (id, name, qtype, rank, location, time_limit, faints, is_key) in quests {
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, type, rank, objective, location, time_limit, faints_allowed, is_key_quest, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL, 'en')",
            rusqlite::params![id, MH2G, name, qtype, rank, qtype, location, time_limit, faints, is_key],
        )?;
    }

    Ok(())
}

fn seed_skills(conn: &Connection) -> Result<()> {
    let skills: &[(i32, &str, &str, i32)] = &[
        (1, "Attack Up", "Increases attack power", 7),
        (2, "Defense Up", "Increases defense", 7),
        (3, "Health +1", "Increases max health by 10", 1),
        (4, "Health +2", "Increases max health by 20", 1),
        (5, "Elemental Attack", "Increases elemental damage", 5),
        (6, "Sharpness +1", "Weapon sharpness extended", 1),
        (7, "Sharpness +2", "Weapon sharpness greatly extended", 1),
        (8, "Recoil Down", "Reduces bowgun recoil", 3),
        (9, "Reload Speed", "Increases bowgun reload speed", 3),
        (10, "Evasion +1", "Increases evasion distance", 1),
        (11, "Evasion +2", "Greatly increases evasion distance", 1),
        (12, "Stamina Recov", "Recovers stamina faster", 2),
        (13, "Wind Pressure", "Reduces wind pressure from monsters", 1),
        (14, "Earplugs", "Nullifies small monster roars", 3),
        (15, "Poison Res", "Reduces poison damage", 1),
        (16, "Fire Res +1", "Increases fire resistance", 1),
        (17, "Ice Res +1", "Increases ice resistance", 1),
        (18, "Thunder Res +1", "Increases thunder resistance", 1),
        (19, "Dragon Res +1", "Increases dragon resistance", 1),
        (20, "Razor Sharp", "Reduces sharpness loss", 3),
    ];

    for (id, name, desc, max_lvl) in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language)
             VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![id, MH2G, name, desc, max_lvl],
        )?;
    }

    Ok(())
}
