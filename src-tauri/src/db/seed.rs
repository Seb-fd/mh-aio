use rusqlite::{Connection, OptionalExtension, Result};
use serde::Deserialize;

const MHW: i32 = 1;
const MH2G: i32 = 5;
const MHP3RD: i32 = 4;

pub fn seed(conn: &Connection) -> Result<()> {
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
    seed_mhw_weapons(conn)?;
    seed_mhw_weapon_materials(conn)?;
    seed_mhw_weapon_craft(conn)?;
    seed_mhw_skills(conn)?;
    seed_mhw_skill_levels(conn)?;
    seed_mhw_armor_sets(conn)?;
    seed_mhw_armor(conn)?;
    seed_mhw_armor_materials(conn)?;
    seed_mhw_armor_skill_points(conn)?;
    seed_mhw_mantles(conn)?;
    seed_palico_gadgets(conn)?;
    seed_palico_gadget_levels(conn)?;
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
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, icon_url, icon_name, icon_color, description, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, NULL, 'en')",
            rusqlite::params![it.id, MH2G, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.icon_url, it.icon_name, it.icon_color],
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
        "hunting" | "hunt" => "hunt",
        "gathering" | "gather" | "collection" => "gather",
        "slaying" | "slay" => "slay",
        "capturing" | "capture" => "capture",
        "training" | "arena" => "training",
        "event" => "event",
        "challenge" => "challenge",
        _ => "hunt",
    }
}

fn quest_type_color(qtype: &str) -> &'static str {
    match qtype.to_ascii_lowercase().as_str() {
        "hunting" | "hunt" => "Red",
        "gathering" | "gather" => "Green",
        "slaying" | "slay" => "Orange",
        "capturing" | "capture" => "Blue",
        "training" | "arena" => "Yellow",
        _ => "Gray",
    }
}

fn quest_hub_slug(hub: &str) -> String {
    // normalize hub to slug: elder, nekoto, guild_low -> guild-low etc.
    let h = hub.to_ascii_lowercase();
    match h.as_str() {
        "village" | "village_low" | "village_high" => "village".to_string(),
        "guild_low" | "guild_high" | "guild_g" => h.replace('_', "-"),
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
