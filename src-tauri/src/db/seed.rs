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
    // MHW + Iceborne (game_id 1) — items 100% (MHWorldData + Kiranico), combine + melder
    seed_mhw_items(conn)?;
    seed_mhw_item_combine(conn)?;
    seed_mhw_melder_recipes(conn)?;
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
        "Light Bowgun" => "light-bowgun",
        "Heavy Bowgun" => "heavy-bowgun",
        "Bow" => "bow",
        _ => "great-sword",
    }
}

fn weapon_icon_color(rarity: i32) -> &'static str {
    match rarity {
        1..=2 => "Gray",
        3..=4 => "White",
        5..=6 => "Green",
        _ => "Gold",
    }
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
        let icon_url = format!("/icons/mhfu/weapons/{}.png", slug);
        let icon_color = weapon_icon_color(w.rarity);
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
    // Backfill for existing DBs where icon was NULL or type changed
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let icon_url = format!("/icons/mhfu/weapons/{}.png", slug);
        let icon_color = weapon_icon_color(w.rarity);
        conn.execute(
            "UPDATE weapons SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 5 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![w.weapon_type, icon_color, icon_url, w.id],
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

    let mut set_map: Vec<(String, i32)> = Vec::new();
    let mut set_id: i32 = 0;
    for a in &armors {
        let set_name = derive_set_name(a);
        if !set_map.iter().any(|(s, _)| s == &set_name) {
            set_id += 1;
            set_map.push((set_name.clone(), set_id));
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
        let icon_color = armor_icon_color(&a.rank);
        let icon_url = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        conn.execute(
            "INSERT OR IGNORE INTO armor
                (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                 resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                 slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, 'en')",
            rusqlite::params![
                a.id, MH2G, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, set_id_of_armor(a), a.armor_type, gender, a.crafting_cost, a.description,
                a.slot_type, icon_color, icon_url
            ],
        )?;
    }
    // Backfill existing DBs
    for a in &armors {
        let icon_color = armor_icon_color(&a.rank);
        let icon_url = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        conn.execute(
            "UPDATE armor SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 5 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![a.slot_type, icon_color, icon_url, a.id],
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
        let icon_url = format!("/icons/mhfu/weapons/{}.png", slug);
        let icon_color = weapon_icon_color(w.rarity);
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, description, sort_order, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, 'en')",
            rusqlite::params![w.id, MHP3RD, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value, w.sharpness, w.slots, w.skills, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path, w.description, w.sort_order, w.weapon_type, icon_color, icon_url],
        )?;
    }
    for w in &weapons {
        let slug = weapon_icon_slug(&w.weapon_type);
        let icon_url = format!("/icons/mhfu/weapons/{}.png", slug);
        let icon_color = weapon_icon_color(w.rarity);
        conn.execute(
            "UPDATE weapons SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 4 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![w.weapon_type, icon_color, icon_url, w.id],
        )?;
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
        let icon_color = armor_icon_color(&a.rank);
        let icon_url = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, set_id, armor_type, gender, crafting_cost, description, icon_name, icon_color, icon_url, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, 'en')",
            rusqlite::params![a.id, MHP3RD, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon, a.slots, a.skills, set_id_of_armor(a), a.armor_type, gender, a.crafting_cost, a.description, a.slot_type, icon_color, icon_url],
        )?;
    }
    for a in &armors {
        let icon_color = armor_icon_color(&a.rank);
        let icon_url = format!("/icons/mhfu/armor/{}.png", a.slot_type);
        conn.execute(
            "UPDATE armor SET icon_name = COALESCE(icon_name, ?1), icon_color = COALESCE(icon_color, ?2), icon_url = COALESCE(icon_url, ?3) WHERE id = ?4 AND game_id = 4 AND (icon_url IS NULL OR icon_name IS NULL)",
            rusqlite::params![a.slot_type, icon_color, icon_url, a.id],
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
}

fn seed_mhw_items(conn: &Connection) -> Result<()> {
    let json_data = include_str!("../../data/mhw_items.json");
    let items: Vec<MhwItemJson> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    for it in &items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, icon_name, icon_color, icon_url, description, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![it.id, MHW, it.name, it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_name, it.icon_color, it.icon_url, it.description],
        )?;
    }
    // Backfill for existing DBs where category/subcategory/icon changed
    for it in &items {
        conn.execute(
            "UPDATE items SET category = ?1, subcategory = ?2, rarity = COALESCE(?3, rarity), sell_price = COALESCE(?4, sell_price), buy_price = COALESCE(?5, buy_price), carry_limit = COALESCE(?6, carry_limit), icon_name = COALESCE(?7, icon_name), icon_color = COALESCE(?8, icon_color), icon_url = COALESCE(?9, icon_url), description = COALESCE(NULLIF(description,''), ?10) WHERE id = ?11 AND game_id = 1",
            rusqlite::params![it.category, it.subcategory, it.rarity, it.sell_price, it.buy_price, it.carry_limit, it.icon_name, it.icon_color, it.icon_url, it.description, it.id],
        )?;
    }
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
