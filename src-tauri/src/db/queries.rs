use rusqlite::{Connection, OptionalExtension, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub release_year: Option<i32>,
    pub platform: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub species: Option<String>,
    pub size: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub species: Option<String>,
    pub size: Option<String>,
    pub description: Option<String>,
    pub weaknesses: Vec<MonsterWeakness>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterWeakness {
    pub id: i32,
    pub part_name: String,
    pub sever: Option<i32>,
    pub blunt: Option<i32>,
    pub projectile: Option<i32>,
    pub fire: Option<i32>,
    pub water: Option<i32>,
    pub thunder: Option<i32>,
    pub ice: Option<i32>,
    pub dragon: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub weapon_type: String,
    pub rarity: Option<i32>,
    pub attack: Option<i32>,
    pub affinity: Option<i32>,
    pub element_type: Option<String>,
    pub element_value: Option<i32>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub weapon_type: String,
    pub rarity: Option<i32>,
    pub attack: Option<i32>,
    pub affinity: Option<i32>,
    pub element_type: Option<String>,
    pub element_value: Option<i32>,
    pub crafting_cost: Option<i32>,
    pub description: Option<String>,
    pub materials: Vec<MaterialRef>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialRef {
    pub item_id: i32,
    pub item_name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slot_type: String,
    pub rank: String,
    pub rarity: Option<i32>,
    pub defense_base: Option<i32>,
    pub defense_max: Option<i32>,
    pub resistance_fire: Option<i32>,
    pub resistance_water: Option<i32>,
    pub resistance_thunder: Option<i32>,
    pub resistance_ice: Option<i32>,
    pub resistance_dragon: Option<i32>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slot_type: String,
    pub rank: String,
    pub rarity: Option<i32>,
    pub defense_base: Option<i32>,
    pub defense_max: Option<i32>,
    pub resistance_fire: Option<i32>,
    pub resistance_water: Option<i32>,
    pub resistance_thunder: Option<i32>,
    pub resistance_ice: Option<i32>,
    pub resistance_dragon: Option<i32>,
    pub crafting_cost: Option<i32>,
    pub description: Option<String>,
    pub materials: Vec<MaterialRef>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quest {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub rank: Option<String>,
    pub objective: Option<String>,
    pub location: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub rank: Option<String>,
    pub objective: Option<String>,
    pub location: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub description: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub category: Option<String>,
    pub rarity: Option<i32>,
    pub sell_price: Option<i32>,
    pub description: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub category: Option<String>,
    pub rarity: Option<i32>,
    pub sell_price: Option<i32>,
    pub description: Option<String>,
    pub sources: Vec<ItemSource>,
    pub recipes: Vec<CombineRecipe>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemSource {
    pub id: i32,
    pub source_type: String,
    pub source_id: Option<i32>,
    pub source_name: Option<String>,
    pub quantity_min: Option<i32>,
    pub quantity_max: Option<i32>,
    pub probability: Option<f64>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombineRecipe {
    pub component_item_id: i32,
    pub component_name: String,
    pub quantity: i32,
    pub result_quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub max_level: Option<i32>,
    pub language: String,
}

pub fn get_games(conn: &Connection) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare("SELECT id, name, abbreviation, release_year, platform FROM games ORDER BY id")?;

    let games = stmt
        .query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                abbreviation: row.get(2)?,
                release_year: row.get(3)?,
                platform: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(games)
}

pub fn get_monsters_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Monster>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, species, size, language FROM monsters WHERE game_id = ?1 ORDER BY id",
    )?;

    let monsters = stmt
        .query_map(params![game_id], |row| {
            Ok(Monster {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                species: row.get(3)?,
                size: row.get(4)?,
                language: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(monsters)
}

pub fn get_monster_detail(conn: &Connection, id: i32) -> Result<Option<MonsterDetail>> {
    let monster: Option<(i32, i32, String, Option<String>, Option<String>, Option<String>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, species, size, description, language FROM monsters WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?)),
        )
        .optional()?;

    let Some((id, game_id, name, species, size, description, language)) = monster else {
        return Ok(None);
    };

    let weaknesses = get_monster_weaknesses(conn, id)?;

    Ok(Some(MonsterDetail {
        id,
        game_id,
        name,
        species,
        size,
        description,
        weaknesses,
        language,
    }))
}

fn get_monster_weaknesses(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterWeakness>> {
    let mut stmt = conn.prepare(
        "SELECT id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon
         FROM monster_weaknesses WHERE monster_id = ?1 ORDER BY id",
    )?;

    let weaknesses = stmt
        .query_map(params![monster_id], |row| {
            Ok(MonsterWeakness {
                id: row.get(0)?,
                part_name: row.get(1)?,
                sever: row.get(2)?,
                blunt: row.get(3)?,
                projectile: row.get(4)?,
                fire: row.get(5)?,
                water: row.get(6)?,
                thunder: row.get(7)?,
                ice: row.get(8)?,
                dragon: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(weaknesses)
}

pub fn get_weapons_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Weapon>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, language
         FROM weapons WHERE game_id = ?1 ORDER BY weapon_type, name",
    )?;

    let weapons = stmt
        .query_map(params![game_id], |row| {
            Ok(Weapon {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                weapon_type: row.get(3)?,
                rarity: row.get(4)?,
                attack: row.get(5)?,
                affinity: row.get(6)?,
                element_type: row.get(7)?,
                element_value: row.get(8)?,
                language: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(weapons)
}

pub fn get_weapon_detail(conn: &Connection, id: i32) -> Result<Option<WeaponDetail>> {
    let row: Option<(
        i32,
        i32,
        String,
        String,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                    crafting_cost, description, language
             FROM weapons WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                    row.get(10)?,
                    row.get(11)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, crafting_cost, description, language)) = row else {
        return Ok(None);
    };

    let materials = get_weapon_materials(conn, id)?;

    Ok(Some(WeaponDetail {
        id,
        game_id,
        name,
        weapon_type,
        rarity,
        attack,
        affinity,
        element_type,
        element_value,
        crafting_cost,
        description,
        materials,
        language,
    }))
}

fn get_weapon_materials(conn: &Connection, weapon_id: i32) -> Result<Vec<MaterialRef>> {
    let mut stmt = conn.prepare(
        "SELECT wm.item_id, i.name, wm.quantity
         FROM weapon_materials wm
         JOIN items i ON i.id = wm.item_id
         WHERE wm.weapon_id = ?1
         ORDER BY wm.id",
    )?;

    let materials = stmt
        .query_map(params![weapon_id], |row| {
            Ok(MaterialRef {
                item_id: row.get(0)?,
                item_name: row.get(1)?,
                quantity: row.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(materials)
}

pub fn get_armor_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Armor>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, language
         FROM armor WHERE game_id = ?1 ORDER BY rank, slot_type, name",
    )?;

    let armor = stmt
        .query_map(params![game_id], |row| {
            Ok(Armor {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                slot_type: row.get(3)?,
                rank: row.get(4)?,
                rarity: row.get(5)?,
                defense_base: row.get(6)?,
                defense_max: row.get(7)?,
                resistance_fire: row.get(8)?,
                resistance_water: row.get(9)?,
                resistance_thunder: row.get(10)?,
                resistance_ice: row.get(11)?,
                resistance_dragon: row.get(12)?,
                language: row.get(13)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(armor)
}

pub fn get_armor_detail(conn: &Connection, id: i32) -> Result<Option<ArmorDetail>> {
    let row: Option<(
        i32,
        i32,
        String,
        String,
        String,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                    resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                    crafting_cost, description, language
             FROM armor WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                    row.get(10)?,
                    row.get(11)?,
                    row.get(12)?,
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, fire, water, thunder, ice, dragon, crafting_cost, description, language)) = row else {
        return Ok(None);
    };

    let materials = get_armor_materials(conn, id)?;

    Ok(Some(ArmorDetail {
        id,
        game_id,
        name,
        slot_type,
        rank,
        rarity,
        defense_base,
        defense_max,
        resistance_fire: fire,
        resistance_water: water,
        resistance_thunder: thunder,
        resistance_ice: ice,
        resistance_dragon: dragon,
        crafting_cost,
        description,
        materials,
        language,
    }))
}

fn get_armor_materials(conn: &Connection, armor_id: i32) -> Result<Vec<MaterialRef>> {
    let mut stmt = conn.prepare(
        "SELECT am.item_id, i.name, am.quantity
         FROM armor_materials am
         JOIN items i ON i.id = am.item_id
         WHERE am.armor_id = ?1
         ORDER BY am.id",
    )?;

    let materials = stmt
        .query_map(params![armor_id], |row| {
            Ok(MaterialRef {
                item_id: row.get(0)?,
                item_name: row.get(1)?,
                quantity: row.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(materials)
}

pub fn get_quests_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Quest>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, type, rank, objective, location, time_limit, faints_allowed, is_key_quest, language
         FROM quests WHERE game_id = ?1 ORDER BY rank, id",
    )?;

    let quests = stmt
        .query_map(params![game_id], |row| {
            Ok(Quest {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                r#type: row.get(3)?,
                rank: row.get(4)?,
                objective: row.get(5)?,
                location: row.get(6)?,
                time_limit: row.get(7)?,
                faints_allowed: row.get(8)?,
                is_key_quest: row.get(9)?,
                language: row.get(10)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(quests)
}

pub fn get_quest_detail(conn: &Connection, id: i32) -> Result<Option<QuestDetail>> {
    let row: Option<(
        i32,
        i32,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        bool,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, type, rank, objective, location, time_limit, faints_allowed, is_key_quest, description, language
             FROM quests WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                    row.get(10)?,
                    row.get(11)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, r#type, rank, objective, location, time_limit, faints_allowed, is_key_quest, description, language)) = row else {
        return Ok(None);
    };

    Ok(Some(QuestDetail {
        id,
        game_id,
        name,
        r#type,
        rank,
        objective,
        location,
        time_limit,
        faints_allowed,
        is_key_quest,
        description,
        language,
    }))
}

pub fn get_items_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, category, rarity, sell_price, description, language
         FROM items WHERE game_id = ?1 ORDER BY category, name",
    )?;

    let items = stmt
        .query_map(params![game_id], |row| {
            Ok(Item {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                category: row.get(3)?,
                rarity: row.get(4)?,
                sell_price: row.get(5)?,
                description: row.get(6)?,
                language: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

pub fn get_item_detail(conn: &Connection, id: i32) -> Result<Option<ItemDetail>> {
    let row: Option<(i32, i32, String, Option<String>, Option<i32>, Option<i32>, Option<String>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, category, rarity, sell_price, description, language
             FROM items WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, category, rarity, sell_price, description, language)) = row else {
        return Ok(None);
    };

    let sources = get_item_sources(conn, id)?;
    let recipes = get_item_combine_recipes(conn, id)?;

    Ok(Some(ItemDetail {
        id,
        game_id,
        name,
        category,
        rarity,
        sell_price,
        description,
        sources,
        recipes,
        language,
    }))
}

fn get_item_sources(conn: &Connection, item_id: i32) -> Result<Vec<ItemSource>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.source_type, s.source_id, s.quantity_min, s.quantity_max, s.probability, s.location,
                COALESCE(m.name, q.name, 'Unknown')
         FROM item_sources s
         LEFT JOIN monsters m ON s.source_type = 'carve' AND m.id = s.source_id
         LEFT JOIN quests q ON s.source_type = 'quest_reward' AND q.id = s.source_id
         WHERE s.item_id = ?1
         ORDER BY s.probability DESC",
    )?;

    let sources = stmt
        .query_map(params![item_id], |row| {
            Ok(ItemSource {
                id: row.get(0)?,
                source_type: row.get(1)?,
                source_id: row.get(2)?,
                source_name: row.get(3)?,
                quantity_min: row.get(4)?,
                quantity_max: row.get(5)?,
                probability: row.get(6)?,
                location: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(sources)
}

fn get_item_combine_recipes(conn: &Connection, item_id: i32) -> Result<Vec<CombineRecipe>> {
    let mut stmt = conn.prepare(
        "SELECT ic.component_item_id, i.name, ic.quantity, ic.result_quantity
         FROM item_combine ic
         JOIN items i ON i.id = ic.component_item_id
         WHERE ic.result_item_id = ?1",
    )?;

    let recipes = stmt
        .query_map(params![item_id], |row| {
            Ok(CombineRecipe {
                component_item_id: row.get(0)?,
                component_name: row.get(1)?,
                quantity: row.get(2)?,
                result_quantity: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(recipes)
}

pub fn get_skills_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Skill>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, description, max_level, language
         FROM skills WHERE game_id = ?1 ORDER BY name",
    )?;

    let skills = stmt
        .query_map(params![game_id], |row| {
            Ok(Skill {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                max_level: row.get(4)?,
                language: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(skills)
}

pub fn get_skill_detail(conn: &Connection, id: i32) -> Result<Option<Skill>> {
    let skill = conn
        .query_row(
            "SELECT id, game_id, name, description, max_level, language FROM skills WHERE id = ?1",
            params![id],
            |row| {
                Ok(Skill {
                    id: row.get(0)?,
                    game_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    max_level: row.get(4)?,
                    language: row.get(5)?,
                })
            },
        )
        .ok();

    Ok(skill)
}
