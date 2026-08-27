use rusqlite::{Connection, OptionalExtension, Result, params};
use serde::{Deserialize, Serialize};

/// Global search result across all game entities, normalized accent-insensitive.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub kind: String,   // monster | item | skill | weapon | armor | armor_set | quest | decoration
    pub id: i32,
    pub name: String,
    pub subtitle: String,
    pub route: String,  // relative to game, e.g. /monsters/12
}

fn strip_accents(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => out.push('a'),
            'ç' => out.push('c'),
            'è' | 'é' | 'ê' | 'ë' => out.push('e'),
            'ì' | 'í' | 'î' | 'ï' => out.push('i'),
            'ñ' => out.push('n'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => out.push('o'),
            'ù' | 'ú' | 'û' | 'ü' => out.push('u'),
            'ý' | 'ÿ' => out.push('y'),
            other => out.push(other),
        }
    }
    out
}

fn norm_key(s: &str) -> String {
    strip_accents(s)
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn score_match(name_key: &str, tokens: &[&str]) -> i32 {
    let name = name_key;
    let full = tokens.join(" ");
    // exact full match = 100, prefix = 60, any token prefix = 40, contains = 20
    if name == full {
        return 100;
    }
    if tokens.len() > 1 && (name == full) {
        return 100;
    }
    // prefix of name (all tokens leading)
    if name.starts_with(&full) {
        return 60;
    }
    let mut best = 0;
    for t in tokens {
        if name.starts_with(t) {
            best = best.max(40);
        } else if name.contains(t) {
            best = best.max(20);
        }
    }
    // multi-token: reward if all tokens contained (AND)
    if tokens.iter().all(|t| name.contains(t)) {
        best = best.max(30);
    }
    best
}

fn matches_tokens(name_key: &str, tokens: &[&str]) -> bool {
    // All tokens must be present (AND) to qualify as a suggestion
    tokens.iter().all(|t| name_key.contains(t))
}

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
    pub drops: Vec<MonsterDrop>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterDrop {
    pub id: i32,
    pub monster_id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub method: String,
    pub part: Option<String>,
    pub rank: Option<String>,
    pub quantity: i32,
    pub probability: f64,
    pub condition: Option<String>,
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
    pub sharpness: Option<String>,
    pub slots: Option<String>,
    pub status_type: Option<String>,
    pub status_value: Option<i32>,
    pub defense_bonus: Option<i32>,
    pub crafting_cost: Option<i32>,
    pub upgrade_path: Option<String>,
    pub sort_order: Option<i32>,
    pub is_forgeable: bool,
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
    pub sharpness: Option<String>,
    pub slots: Option<String>,
    pub skills: Option<String>,
    pub status_type: Option<String>,
    pub status_value: Option<i32>,
    pub defense_bonus: Option<i32>,
    pub crafting_cost: Option<i32>,
    pub upgrade_path: Option<String>,
    pub sort_order: Option<i32>,
    pub description: Option<String>,
    pub materials: Vec<MaterialRef>,
    pub forge_materials: Vec<MaterialRef>,
    pub upgrade_materials: Vec<MaterialRef>,
    pub is_forgeable: bool,
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
    pub slots: Option<String>,
    pub skills: Option<String>,
    pub armor_type: Option<String>,
    pub set_id: Option<i32>,
    pub gender: Option<String>,
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
    pub slots: Option<String>,
    pub skills: Option<String>,
    pub set_id: Option<i32>,
    pub armor_type: Option<String>,
    pub gender: Option<String>,
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
    pub name_original: Option<String>,
    pub r#type: Option<String>,
    pub rank: Option<String>,
    pub hub: Option<String>,
    pub stars: Option<i32>,
    pub objective: Option<String>,
    pub location: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub is_urgent: bool,
    pub client: Option<String>,
    pub requirements: Option<String>,
    pub reward_money: Option<i32>,
    pub contract_fee: Option<i32>,
    pub main_monsters: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestReward {
    pub id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub quantity: i32,
    pub probability: Option<f64>,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub name_original: Option<String>,
    pub r#type: Option<String>,
    pub rank: Option<String>,
    pub hub: Option<String>,
    pub stars: Option<i32>,
    pub objective: Option<String>,
    pub location: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub is_urgent: bool,
    pub description: Option<String>,
    pub client: Option<String>,
    pub requirements: Option<String>,
    pub reward_money: Option<i32>,
    pub contract_fee: Option<i32>,
    pub main_monsters: Option<String>,
    pub rewards: Vec<QuestReward>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub rarity: Option<i32>,
    pub sell_price: Option<i32>,
    pub buy_price: Option<i32>,
    pub description: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub rarity: Option<i32>,
    pub sell_price: Option<i32>,
    pub buy_price: Option<i32>,
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
    pub rank: Option<String>,
    pub part: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombineRecipe {
    pub component_item_id: i32,
    pub component_name: String,
    pub quantity: i32,
    pub result_quantity: i32,
    pub combine_type: String,
    pub chance: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombineView {
    pub result_item_id: i32,
    pub result_name: String,
    pub category: Option<String>,
    pub rarity: Option<i32>,
    pub combine_type: String,
    pub chance: Option<i32>,
    pub components: Vec<CombineRecipe>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillLevel {
    pub id: i32,
    pub points: i32,
    pub ability_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecoMaterial {
    pub item_id: Option<i32>,
    pub item_name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillDecoration {
    pub id: i32,
    pub name: String,
    pub slot_size: Option<i32>,
    pub skill_points: i32,
    pub secondary_skill_name: Option<String>,
    pub secondary_points: Option<i32>,
    pub price: Option<i32>,
    pub rarity: Option<i32>,
    pub materials: Vec<DecoMaterial>,
    pub unlock: String,
    pub acquisition: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillArmorRef {
    pub id: i32,
    pub name: String,
    pub slot_type: String,
    pub rank: String,
    pub rarity: Option<i32>,
    pub defense_base: Option<i32>,
    pub defense_max: Option<i32>,
    pub slots: Option<String>,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillWeaponRef {
    pub id: i32,
    pub name: String,
    pub weapon_type: String,
    pub rarity: Option<i32>,
    pub attack: Option<i32>,
    pub slots: Option<String>,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub max_level: Option<i32>,
    pub language: String,
    pub levels: Vec<SkillLevel>,
    pub decorations: Vec<SkillDecoration>,
    pub armors: Vec<SkillArmorRef>,
    pub weapons: Vec<SkillWeaponRef>,
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
    let drops = get_monster_drops(conn, id)?;
    let armor = get_monster_related_armor(conn, id)?;
    let weapons = get_monster_related_weapons(conn, id)?;

    Ok(Some(MonsterDetail {
        id,
        game_id,
        name,
        species,
        size,
        description,
        weaknesses,
        drops,
        armor,
        weapons,
        language,
    }))
}

pub fn get_monster_dedicated_sets(conn: &Connection, monster_id: i32, rank: Option<&str>) -> Result<Vec<ArmorSetDetail>> {
    // Dedicated sets: score >=0.40 monster materials vs total, rank-filtered, sub-species safe via item_id exact match (Lao Shan Auroros 0.54, Borealis 0.45)
    let mut stmt = conn.prepare(
        "SELECT s.id, s.game_id, s.name, s.language FROM armor_sets s
         WHERE s.id IN (
           SELECT DISTINCT a.set_id FROM armor a
           JOIN monster_equipment me ON me.equipment_id = a.id
           WHERE me.monster_id = ?1 AND me.equipment_kind='armor'
         )",
    )?;
    let set_rows: Vec<(i32,i32,String,String)> = stmt.query_map(params![monster_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))?.filter_map(|r| r.ok()).collect();
    drop(stmt);
    let mut out = Vec::new();
    for (sid, gid, sname, lang) in set_rows {
        // Rank filter: check set has at least one piece of requested rank
        if let Some(r) = rank {
            let cnt: i64 = conn.query_row("SELECT COUNT(*) FROM armor WHERE set_id=?1 AND rank=?2", params![sid, r], |row| row.get(0)).unwrap_or(0);
            if cnt==0 { continue; }
        }
        let mut mat_stmt = conn.prepare(
            "SELECT am.quantity, CASE WHEN md.item_id IS NOT NULL THEN 1 ELSE 0 END as is_monster
             FROM armor a
             JOIN armor_materials am ON am.armor_id = a.id
             LEFT JOIN (SELECT DISTINCT item_id FROM monster_drops WHERE monster_id=?1) md ON md.item_id = am.item_id
             WHERE a.set_id=?2",
        )?;
        let mut monster_qty: i64 = 0;
        let mut total_qty: i64 = 0;
        let rows = mat_stmt.query_map(params![monster_id, sid], |row| Ok((row.get::<_,i64>(0)?, row.get::<_,i64>(1)?)))?;
        for r in rows {
            if let Ok((qty, is_mon)) = r {
                total_qty += qty;
                if is_mon!=0 { monster_qty += qty; }
            }
        }
        drop(mat_stmt);
        if total_qty==0 { continue; }
        let score = monster_qty as f64 / total_qty as f64;
        if score < 0.40 { continue; }
        // Fetch pieces for this set, filtered by rank if needed
        let pieces_sql = if rank.is_some() {
            "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, armor_type, set_id, gender, language FROM armor WHERE set_id=?1 AND rank=?2 ORDER BY CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END, id"
        } else {
            "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, armor_type, set_id, gender, language FROM armor WHERE set_id=?1 ORDER BY CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END, id"
        };
        let mut p_stmt = conn.prepare(pieces_sql)?;
        let pieces: Vec<Armor> = if let Some(r) = rank {
            p_stmt.query_map(params![sid, r], |row| Ok(Armor{ id: row.get(0)?, game_id: row.get(1)?, name: row.get(2)?, slot_type: row.get(3)?, rank: row.get(4)?, rarity: row.get(5)?, defense_base: row.get(6)?, defense_max: row.get(7)?, resistance_fire: row.get(8)?, resistance_water: row.get(9)?, resistance_thunder: row.get(10)?, resistance_ice: row.get(11)?, resistance_dragon: row.get(12)?, slots: row.get(13)?, skills: row.get(14)?, armor_type: row.get(15)?, set_id: row.get(16)?, gender: row.get(17)?, language: row.get(18)? }))?.filter_map(|r| r.ok()).collect()
        } else {
            p_stmt.query_map(params![sid], |row| Ok(Armor{ id: row.get(0)?, game_id: row.get(1)?, name: row.get(2)?, slot_type: row.get(3)?, rank: row.get(4)?, rarity: row.get(5)?, defense_base: row.get(6)?, defense_max: row.get(7)?, resistance_fire: row.get(8)?, resistance_water: row.get(9)?, resistance_thunder: row.get(10)?, resistance_ice: row.get(11)?, resistance_dragon: row.get(12)?, slots: row.get(13)?, skills: row.get(14)?, armor_type: row.get(15)?, set_id: row.get(16)?, gender: row.get(17)?, language: row.get(18)? }))?.filter_map(|r| r.ok()).collect()
        };
        if pieces.is_empty() { continue; }
        out.push(ArmorSetDetail{ id: sid, game_id: gid, name: sname, pieces, language: lang });
    }
    out.sort_by_key(|s| s.id);
    Ok(out)
}

fn get_monster_related_armor(conn: &Connection, monster_id: i32) -> Result<Vec<Armor>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.game_id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, a.armor_type, a.set_id, a.gender, a.language
         FROM armor a
         JOIN monster_equipment me ON a.id = me.equipment_id
         WHERE me.monster_id = ?1 AND me.equipment_kind = 'armor'
         ORDER BY a.rarity, a.name",
    )?;

    let armor = stmt
        .query_map(params![monster_id], |row| {
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
                slots: row.get(13)?,
                skills: row.get(14)?,
                armor_type: row.get(15)?,
                set_id: row.get(16)?,
                gender: row.get(17)?,
                language: row.get(18)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(armor)
}

fn get_monster_related_weapons(conn: &Connection, monster_id: i32) -> Result<Vec<Weapon>> {
    let mut stmt = conn.prepare(
        "SELECT w.id, w.game_id, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value,
                w.sharpness, w.slots, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path,
                EXISTS(SELECT 1 FROM weapon_craft wc WHERE wc.weapon_id = w.id AND wc.craft_kind = 'forge'), w.sort_order, w.language
         FROM weapons w
         JOIN monster_equipment me ON w.id = me.equipment_id
         WHERE me.monster_id = ?1 AND me.equipment_kind = 'weapon'
         ORDER BY
            CASE w.weapon_type
                WHEN 'Great Sword' THEN 0
                WHEN 'Long Sword' THEN 1
                WHEN 'Sword & Shield' THEN 2
                WHEN 'Sword and Shield' THEN 2
                WHEN 'Dual Blades' THEN 3
                WHEN 'Hammer' THEN 4
                WHEN 'Hunting Horn' THEN 5
                WHEN 'Lance' THEN 6
                WHEN 'Gunlance' THEN 7
                WHEN 'Switch Axe' THEN 8
                WHEN 'Light Bowgun' THEN 9
                WHEN 'Heavy Bowgun' THEN 10
                WHEN 'Bow' THEN 11
                ELSE 12
            END, COALESCE(w.sort_order, w.id)",
    )?;

    let weapons = stmt
        .query_map(params![monster_id], |row| {
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
                sharpness: row.get(9)?,
                slots: row.get(10)?,
                status_type: row.get(11)?,
                status_value: row.get(12)?,
                defense_bonus: row.get(13)?,
                crafting_cost: row.get(14)?,
                upgrade_path: row.get(15)?,
                is_forgeable: row.get::<_, i64>(16)? != 0,
                sort_order: row.get(17)?,
                language: row.get(18)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(weapons)
}

fn get_monster_drops(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterDrop>> {
    let mut stmt = conn.prepare(
        "SELECT md.id, md.monster_id, md.item_id, i.name, md.method, md.part, md.rank,
                md.quantity, md.probability, md.condition
         FROM monster_drops md
         JOIN items i ON i.id = md.item_id
         WHERE md.monster_id = ?1
         ORDER BY md.rank, md.method, md.part, md.probability DESC",
    )?;

    let drops = stmt
        .query_map(params![monster_id], |row| {
            Ok(MonsterDrop {
                id: row.get(0)?,
                monster_id: row.get(1)?,
                item_id: row.get(2)?,
                item_name: row.get(3)?,
                method: row.get(4)?,
                part: row.get(5)?,
                rank: row.get(6)?,
                quantity: row.get(7)?,
                probability: row.get(8)?,
                condition: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(drops)
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
    // Smith order: game weapon-trees order (Great Sword → Bow) verified via ISO tree via upgrade_path
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                sharpness, slots, status_type, status_value, defense_bonus, crafting_cost, upgrade_path,
                EXISTS(SELECT 1 FROM weapon_craft wc WHERE wc.weapon_id = weapons.id AND wc.craft_kind = 'forge'), sort_order, language
         FROM weapons WHERE game_id = ?1 ORDER BY
            CASE weapon_type
                WHEN 'Great Sword' THEN 0
                WHEN 'Long Sword' THEN 1
                WHEN 'Sword & Shield' THEN 2
                WHEN 'Sword and Shield' THEN 2
                WHEN 'Dual Blades' THEN 3
                WHEN 'Hammer' THEN 4
                WHEN 'Hunting Horn' THEN 5
                WHEN 'Lance' THEN 6
                WHEN 'Gunlance' THEN 7
                WHEN 'Switch Axe' THEN 8
                WHEN 'Light Bowgun' THEN 9
                WHEN 'Heavy Bowgun' THEN 10
                WHEN 'Bow' THEN 11
                ELSE 12
            END, COALESCE(sort_order, id)",
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
                sharpness: row.get(9)?,
                slots: row.get(10)?,
                status_type: row.get(11)?,
                status_value: row.get(12)?,
                defense_bonus: row.get(13)?,
                crafting_cost: row.get(14)?,
                upgrade_path: row.get(15)?,
                is_forgeable: row.get::<_, i64>(16)? != 0,
                sort_order: row.get(17)?,
                language: row.get(18)?,
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
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<i32>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                    sharpness, slots, skills, status_type, status_value, defense_bonus,
                    crafting_cost, upgrade_path, sort_order, description, language
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
                    row.get(12)?,
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                    row.get(16)?,
                    row.get(17)?,
                    row.get(18)?,
                    row.get(19)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, sharpness, slots, skills, status_type, status_value, defense_bonus, crafting_cost, upgrade_path, sort_order, description, language)) = row else {
        return Ok(None);
    };

    let materials = get_weapon_materials(conn, id)?;
    let forge_materials = get_weapon_craft_materials(conn, id, "forge")?;
    let upgrade_materials = get_weapon_craft_materials(conn, id, "upgrade")?;
    let is_forgeable = !forge_materials.is_empty();

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
        sharpness,
        slots,
        skills,
        status_type,
        status_value,
        defense_bonus,
        crafting_cost,
        upgrade_path,
        sort_order,
        description,
        materials,
        forge_materials,
        upgrade_materials,
        is_forgeable,
        language,
    }))
}

fn get_weapon_craft_materials(conn: &Connection, weapon_id: i32, kind: &str) -> Result<Vec<MaterialRef>> {
    let mut stmt = conn.prepare(
        "SELECT wc.item_id, i.name, wc.quantity
         FROM weapon_craft wc
         JOIN items i ON i.id = wc.item_id
         WHERE wc.weapon_id = ?1 AND wc.craft_kind = ?2
         ORDER BY wc.id",
    )?;

    let materials = stmt
        .query_map(params![weapon_id, kind], |row| {
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
    // Smith order: faithful to armorer list (rank -> slot -> creation order = id) verified via ISO armor string table order at 37652906
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                slots, skills, armor_type, set_id, gender, language
         FROM armor WHERE game_id = ?1 ORDER BY
            CASE rank WHEN 'Low' THEN 0 WHEN 'High' THEN 1 WHEN 'G' THEN 2 ELSE 3 END,
            CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END,
            id",
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
                slots: row.get(13)?,
                skills: row.get(14)?,
                armor_type: row.get(15)?,
                set_id: row.get(16)?,
                gender: row.get(17)?,
                language: row.get(18)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(armor)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorSet {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub piece_count: i32,
    pub rank: Option<String>,
    pub rarity: Option<i32>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorSetDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub pieces: Vec<Armor>,
    pub language: String,
}

pub fn get_armor_sets_by_game(conn: &Connection, game_id: i32) -> Result<Vec<ArmorSet>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.game_id, s.name, COUNT(a.id) as piece_count,
                (SELECT rank FROM armor WHERE set_id = s.id LIMIT 1) as rank,
                (SELECT MAX(rarity) FROM armor WHERE set_id = s.id) as rarity,
                s.language
         FROM armor_sets s
         LEFT JOIN armor a ON a.set_id = s.id
         WHERE s.game_id = ?1
         GROUP BY s.id, s.game_id, s.name, s.language
         ORDER BY s.id",
    )?;
    let sets = stmt
        .query_map(params![game_id], |row| {
            Ok(ArmorSet {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                piece_count: row.get(3)?,
                rank: row.get(4)?,
                rarity: row.get(5)?,
                language: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(sets)
}

pub fn get_armor_set_detail(conn: &Connection, id: i32) -> Result<Option<ArmorSetDetail>> {
    let row: Option<(i32, i32, String, String)> = conn
        .query_row(
            "SELECT id, game_id, name, language FROM armor_sets WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .optional()?;
    let Some((id, game_id, name, language)) = row else {
        return Ok(None);
    };
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                slots, skills, armor_type, set_id, gender, language
         FROM armor WHERE set_id = ?1 ORDER BY
            CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END,
            id",
    )?;
    let pieces = stmt
        .query_map(params![id], |row| {
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
                slots: row.get(13)?,
                skills: row.get(14)?,
                armor_type: row.get(15)?,
                set_id: row.get(16)?,
                gender: row.get(17)?,
                language: row.get(18)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(Some(ArmorSetDetail { id, game_id, name, pieces, language }))
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
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                    resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                    slots, skills, armor_type, gender, set_id, crafting_cost, description, language
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
                    row.get(16)?,
                    row.get(17)?,
                    row.get(18)?,
                    row.get(19)?,
                    row.get(20)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, fire, water, thunder, ice, dragon, slots, skills, armor_type, gender, set_id, crafting_cost, description, language)) = row else {
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
        slots,
        skills,
        set_id,
        armor_type,
        gender,
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
        "SELECT id, game_id, name, name_original, type, rank, hub, stars, objective, location, time_limit, faints_allowed, is_key_quest, is_urgent, client, requirements, reward_money, contract_fee, main_monsters, language
         FROM quests WHERE game_id = ?1 ORDER BY
            CASE hub WHEN 'elder' THEN 0 WHEN 'nekoto' THEN 1 WHEN 'village' THEN 2 WHEN 'village_low' THEN 2 WHEN 'village_high' THEN 3 WHEN 'guild_low' THEN 4 WHEN 'guild_high' THEN 5 WHEN 'guild_g' THEN 6 WHEN 'event' THEN 7 WHEN 'challenge' THEN 8 WHEN 'training' THEN 9 WHEN 'treasure' THEN 10 WHEN 'hot_spring' THEN 11 WHEN 'drink' THEN 12 WHEN 'nyanta' THEN 13 ELSE 14 END,
            stars, id",
    )?;

    let quests = stmt
        .query_map(params![game_id], |row| {
            Ok(Quest {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                name_original: row.get(3)?,
                r#type: row.get(4)?,
                rank: row.get(5)?,
                hub: row.get(6)?,
                stars: row.get(7)?,
                objective: row.get(8)?,
                location: row.get(9)?,
                time_limit: row.get(10)?,
                faints_allowed: row.get(11)?,
                is_key_quest: row.get(12)?,
                is_urgent: row.get(13)?,
                client: row.get(14)?,
                requirements: row.get(15)?,
                reward_money: row.get(16)?,
                contract_fee: row.get(17)?,
                main_monsters: row.get(18)?,
                language: row.get(19)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(quests)
}

fn get_quest_rewards(conn: &Connection, quest_id: i32) -> Result<Vec<QuestReward>> {
    let mut stmt = conn.prepare(
        "SELECT qr.id, qr.item_id, i.name, qr.quantity, qr.probability, qr.condition
         FROM quest_rewards qr JOIN items i ON i.id = qr.item_id WHERE qr.quest_id = ?1 ORDER BY qr.probability DESC",
    )?;
    let rewards = stmt
        .query_map(params![quest_id], |row| {
            Ok(QuestReward {
                id: row.get(0)?,
                item_id: row.get(1)?,
                item_name: row.get(2)?,
                quantity: row.get(3)?,
                probability: row.get(4)?,
                condition: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rewards)
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
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        bool,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, name_original, type, rank, hub, stars, objective, location, time_limit, faints_allowed, is_key_quest, is_urgent, description, client, requirements, reward_money, contract_fee, main_monsters, language
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
                    row.get(12)?,
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                    row.get(16)?,
                    row.get(17)?,
                    row.get(18)?,
                    row.get(19)?,
                    row.get(20)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, name_original, r#type, rank, hub, stars, objective, location, time_limit, faints_allowed, is_key_quest, is_urgent, description, client, requirements, reward_money, contract_fee, main_monsters, language)) = row else {
        return Ok(None);
    };

    let rewards = get_quest_rewards(conn, id)?;

    Ok(Some(QuestDetail {
        id,
        game_id,
        name,
        name_original,
        r#type,
        rank,
        hub,
        stars,
        objective,
        location,
        time_limit,
        faints_allowed,
        is_key_quest,
        is_urgent,
        description,
        client,
        requirements,
        reward_money,
        contract_fee,
        main_monsters,
        rewards,
        language,
    }))
}

pub fn get_items_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Item>> {
    // Chest order: faithful to PSP item box (hex ID order) verified via ISO DATA.BIN file 15 string table
    // Alternative sorts handled client-side; keep DB default as game chest.
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, language
         FROM items WHERE game_id = ?1 ORDER BY id",
    )?;

    let items = stmt
        .query_map(params![game_id], |row| {
            Ok(Item {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                category: row.get(3)?,
                subcategory: row.get(4)?,
                rarity: row.get(5)?,
                sell_price: row.get(6)?,
                buy_price: row.get(7)?,
                description: row.get(8)?,
                language: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

pub fn get_item_detail(conn: &Connection, id: i32) -> Result<Option<ItemDetail>> {
    let row: Option<(i32, i32, String, Option<String>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<String>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, language
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
                    row.get(8)?,
                    row.get(9)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, category, subcategory, rarity, sell_price, buy_price, description, language)) = row else {
        return Ok(None);
    };

    let sources = get_item_sources(conn, id)?;
    let recipes = get_item_combine_recipes(conn, id)?;

    Ok(Some(ItemDetail {
        id,
        game_id,
        name,
        category,
        subcategory,
        rarity,
        sell_price,
        buy_price,
        description,
        sources,
        recipes,
        language,
    }))
}

fn get_item_sources(conn: &Connection, item_id: i32) -> Result<Vec<ItemSource>> {
    // Unified sources: monster_drops (carve/capture/break/drop) + quest_rewards + gathering (item_sources)
    // item_sources rows of type carve/capture/drop/break/quest_reward are filtered out to avoid
    // duplication with the two authoritative tables above.
    let mut stmt = conn.prepare(
        "SELECT id, source_type, source_id, source_name, quantity_min, quantity_max, probability, location, rank, part, condition FROM (
            SELECT md.id as id, md.method as source_type, md.monster_id as source_id, m.name as source_name,
                   md.quantity as quantity_min, md.quantity as quantity_max, md.probability as probability,
                   NULL as location, md.rank as rank, md.part as part, md.condition as condition
            FROM monster_drops md
            JOIN monsters m ON m.id = md.monster_id
            WHERE md.item_id = ?1
            UNION ALL
            SELECT qr.id + 1000000 as id, 'quest_reward' as source_type, qr.quest_id as source_id, q.name as source_name,
                   qr.quantity as quantity_min, qr.quantity as quantity_max, qr.probability as probability,
                   q.location as location, q.rank as rank, NULL as part, qr.condition as condition
            FROM quest_rewards qr
            JOIN quests q ON q.id = qr.quest_id
            WHERE qr.item_id = ?1
            UNION ALL
            SELECT s.id + 2000000 as id, s.source_type as source_type, s.source_id as source_id,
                   COALESCE(m2.name, q2.name) as source_name,
                   s.quantity_min as quantity_min, s.quantity_max as quantity_max, s.probability as probability,
                   s.location as location, NULL as rank, NULL as part, s.conditions as condition
            FROM item_sources s
            LEFT JOIN monsters m2 ON s.source_type IN ('carve', 'capture', 'drop', 'break') AND m2.id = s.source_id
            LEFT JOIN quests q2 ON s.source_type = 'quest_reward' AND q2.id = s.source_id
            WHERE s.item_id = ?1 AND s.source_type NOT IN ('carve', 'capture', 'drop', 'break', 'quest_reward')
        ) ORDER BY
            CASE rank WHEN 'Low' THEN 0 WHEN 'High' THEN 1 WHEN 'G' THEN 2 ELSE 3 END,
            probability DESC"
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
                rank: row.get(8)?,
                part: row.get(9)?,
                condition: row.get(10)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(sources)
}

fn get_item_combine_recipes(conn: &Connection, item_id: i32) -> Result<Vec<CombineRecipe>> {
    let mut stmt = conn.prepare(
        "SELECT ic.component_item_id, i.name, ic.quantity, ic.result_quantity, COALESCE(ic.combine_type,'normal'), ic.chance
         FROM item_combine ic
         JOIN items i ON i.id = ic.component_item_id
         WHERE ic.result_item_id = ?1
         ORDER BY ic.id",
    )?;

    let recipes = stmt
        .query_map(params![item_id], |row| {
            Ok(CombineRecipe {
                component_item_id: row.get(0)?,
                component_name: row.get(1)?,
                quantity: row.get(2)?,
                result_quantity: row.get(3)?,
                combine_type: row.get(4)?,
                chance: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(recipes)
}

pub fn get_combinations_by_game(conn: &Connection, game_id: i32) -> Result<Vec<CombineView>> {
    // Game order: by item_combine.id (insertion order = book order from ISO, verified via upstream)
    let mut stmt = conn.prepare(
        "SELECT ic.result_item_id, ri.name, ri.category, ri.rarity, COALESCE(ic.combine_type,'normal'), ic.chance, ic.component_item_id, ci.name, ic.quantity, ic.result_quantity, ic.id
         FROM item_combine ic
         JOIN items ri ON ri.id = ic.result_item_id
         JOIN items ci ON ci.id = ic.component_item_id
         WHERE ri.game_id = ?1
         ORDER BY ic.id",
    )?;
    let mut map: std::collections::BTreeMap<i32, CombineView> = std::collections::BTreeMap::new();
    let mut order: Vec<i32> = Vec::new();
    let rows = stmt.query_map(params![game_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, Option<i32>>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, Option<i32>>(5)?,
            row.get::<_, i32>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, i32>(8)?,
            row.get::<_, i32>(9)?,
        ))
    })?;
    for r in rows {
        let (rid, rname, cat, rar, ctype, chance, cid, cname, qty, rqty) = r?;
        let entry = map.entry(rid).or_insert_with(|| {
            order.push(rid);
            CombineView {
                result_item_id: rid,
                result_name: rname.clone(),
                category: cat.clone(),
                rarity: rar,
                combine_type: ctype.clone(),
                chance,
                components: Vec::new(),
            }
        });
        // keep first type/chance (all components of same result share same)
        entry.components.push(CombineRecipe {
            component_item_id: cid,
            component_name: cname,
            quantity: qty,
            result_quantity: rqty,
            combine_type: ctype,
            chance,
        });
    }
    // Return in game order (by first appearance)
    let mut out: Vec<CombineView> = Vec::new();
    for rid in order {
        if let Some(v) = map.remove(&rid) {
            out.push(v);
        }
    }
    Ok(out)
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

pub fn get_skill_detail(conn: &Connection, id: i32) -> Result<Option<SkillDetail>> {
    let skill: Option<(i32, i32, String, Option<String>, Option<i32>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, description, max_level, language FROM skills WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )
        .optional()?;

    let Some((id, game_id, name, description, max_level, language)) = skill else {
        return Ok(None);
    };

    let levels = get_skill_levels(conn, id)?;
    let decorations = get_skill_decorations(conn, id)?;
    let armors = get_skill_armors(conn, id)?;
    let weapons = get_skill_weapons(conn, id)?;

    Ok(Some(SkillDetail {
        id,
        game_id,
        name,
        description,
        max_level,
        language,
        levels,
        decorations,
        armors,
        weapons,
    }))
}

fn get_skill_levels(conn: &Connection, skill_id: i32) -> Result<Vec<SkillLevel>> {
    let mut stmt = conn.prepare(
        "SELECT id, points, ability_name, description FROM skill_levels WHERE skill_id = ?1 ORDER BY points DESC",
    )?;
    let rows = stmt
        .query_map(params![skill_id], |row| {
            Ok(SkillLevel {
                id: row.get(0)?,
                points: row.get(1)?,
                ability_name: row.get(2)?,
                description: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

fn decoration_unlock_and_acquisition(materials: &[DecoMaterial], price: Option<i32>) -> (String, String) {
    let has_lapis = materials.iter().any(|m| m.item_name == "LapisLazuliJewel");
    let has_battlefield = materials.iter().any(|m| m.item_name == "BattlefieldJewel");
    let has_akito = materials.iter().any(|m| m.item_name == "Akito Jewel");
    let has_suiko = materials.iter().any(|m| m.item_name == "Suiko Jewel");
    let unlock = if has_lapis {
        "G Rank - Craft at Village/Hall Smith (G Rank jewel)"
    } else if has_battlefield {
        "High Rank (G* / HR 5+) - Craft at Smith, requires Battlefield Jewel"
    } else if has_akito {
        "High Rank (HR 4+) - Craft at Smith, requires Akito Jewel"
    } else if has_suiko {
        "Low Rank (HR 1+) - Craft at Village Smith from start"
    } else {
        "Craft at Equipment Smith"
    }
    .to_string();
    let acquisition = if let Some(p) = price {
        format!("Crafted at Smith for {}z + materials below", p)
    } else {
        "Crafted at Smith with materials below".to_string()
    };
    (unlock, acquisition)
}

fn get_decoration_materials(conn: &Connection, decoration_id: i32) -> Result<Vec<DecoMaterial>> {
    let mut stmt = conn.prepare(
        "SELECT item_id, item_name, quantity FROM decoration_materials WHERE decoration_id = ?1 ORDER BY item_name",
    )?;
    let rows = stmt
        .query_map(params![decoration_id], |row| {
            Ok(DecoMaterial {
                item_id: row.get(0)?,
                item_name: row.get(1)?,
                quantity: row.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

fn get_skill_decorations(conn: &Connection, skill_id: i32) -> Result<Vec<SkillDecoration>> {
    let mut stmt = conn.prepare(
        "SELECT d.id, d.name, d.slot_size, d.skill_id, d.skill_points, d.secondary_skill_id, d.secondary_points, d.price, d.rarity,
                s1.name, s2.name
         FROM decorations d
         LEFT JOIN skills s1 ON s1.id = d.skill_id
         LEFT JOIN skills s2 ON s2.id = d.secondary_skill_id
         WHERE d.skill_id = ?1 OR d.secondary_skill_id = ?1
         ORDER BY d.slot_size, d.name",
    )?;
    let base_rows: Vec<(i32, String, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>)> = stmt
        .query_map(params![skill_id], |row| {
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
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let mut out = Vec::new();
    for (id, name, slot_size, prim_id, prim_pts, _sec_id, sec_pts, price, rarity, prim_name, sec_name) in base_rows {
        let is_primary = prim_id == Some(skill_id);
        let pts = if is_primary { prim_pts } else { sec_pts }.unwrap_or(0);
        let (other_name, other_pts) = if is_primary {
            (sec_name, sec_pts)
        } else {
            (prim_name, prim_pts)
        };
        let materials = get_decoration_materials(conn, id)?;
        let (unlock, acquisition) = decoration_unlock_and_acquisition(&materials, price);
        out.push(SkillDecoration {
            id,
            name,
            slot_size,
            skill_points: pts,
            secondary_skill_name: other_name,
            secondary_points: other_pts,
            price,
            rarity,
            materials,
            unlock,
            acquisition,
        });
    }
    Ok(out)
}

fn get_skill_armors(conn: &Connection, skill_id: i32) -> Result<Vec<SkillArmorRef>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max, a.slots, asp.points
         FROM armor_skill_points asp
         JOIN armor a ON a.id = asp.armor_id
         WHERE asp.skill_id = ?1
         ORDER BY a.rank, a.slot_type, a.name
         LIMIT 200",
    )?;
    let rows = stmt
        .query_map(params![skill_id], |row| {
            Ok(SkillArmorRef {
                id: row.get(0)?,
                name: row.get(1)?,
                slot_type: row.get(2)?,
                rank: row.get(3)?,
                rarity: row.get(4)?,
                defense_base: row.get(5)?,
                defense_max: row.get(6)?,
                slots: row.get(7)?,
                points: row.get(8)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

fn get_skill_weapons(conn: &Connection, skill_id: i32) -> Result<Vec<SkillWeaponRef>> {
    let mut stmt = conn.prepare(
        "SELECT w.id, w.name, w.weapon_type, w.rarity, w.attack, w.slots, wsp.points
         FROM weapon_skill_points wsp
         JOIN weapons w ON w.id = wsp.weapon_id
         WHERE wsp.skill_id = ?1
         ORDER BY w.weapon_type, w.name
         LIMIT 200",
    )?;
    let rows = stmt
        .query_map(params![skill_id], |row| {
            Ok(SkillWeaponRef {
                id: row.get(0)?,
                name: row.get(1)?,
                weapon_type: row.get(2)?,
                rarity: row.get(3)?,
                attack: row.get(4)?,
                slots: row.get(5)?,
                points: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Decoration {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub skill_id: Option<i32>,
    pub skill_name: Option<String>,
    pub skill_points: Option<i32>,
    pub secondary_skill_id: Option<i32>,
    pub secondary_skill_name: Option<String>,
    pub secondary_points: Option<i32>,
    pub slot_size: Option<i32>,
    pub rarity: Option<i32>,
    pub price: Option<i32>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecorationDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub skill_id: Option<i32>,
    pub skill_name: Option<String>,
    pub skill_points: Option<i32>,
    pub secondary_skill_id: Option<i32>,
    pub secondary_skill_name: Option<String>,
    pub secondary_points: Option<i32>,
    pub slot_size: Option<i32>,
    pub rarity: Option<i32>,
    pub price: Option<i32>,
    pub language: String,
    pub materials: Vec<DecoMaterial>,
    pub unlock: String,
    pub acquisition: String,
}

pub fn get_decorations_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Decoration>> {
    let mut stmt = conn.prepare(
        "SELECT d.id, d.game_id, d.name, d.skill_id, s1.name, d.skill_points, d.secondary_skill_id, s2.name, d.secondary_points, d.slot_size, d.rarity, d.price, d.language
         FROM decorations d
         LEFT JOIN skills s1 ON s1.id = d.skill_id
         LEFT JOIN skills s2 ON s2.id = d.secondary_skill_id
         WHERE d.game_id = ?1
         ORDER BY d.slot_size, d.name",
    )?;
    let rows = stmt
        .query_map(params![game_id], |row| {
            Ok(Decoration {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                skill_id: row.get(3)?,
                skill_name: row.get(4)?,
                skill_points: row.get(5)?,
                secondary_skill_id: row.get(6)?,
                secondary_skill_name: row.get(7)?,
                secondary_points: row.get(8)?,
                slot_size: row.get(9)?,
                rarity: row.get(10)?,
                price: row.get(11)?,
                language: row.get(12)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

pub fn get_decoration_detail(conn: &Connection, id: i32) -> Result<Option<DecorationDetail>> {
    let row: Option<(i32, i32, String, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, String)> = conn
        .query_row(
            "SELECT d.id, d.game_id, d.name, d.skill_id, s1.name, d.skill_points, d.secondary_skill_id, s2.name, d.secondary_points, d.slot_size, d.rarity, d.price, d.language
             FROM decorations d
             LEFT JOIN skills s1 ON s1.id = d.skill_id
             LEFT JOIN skills s2 ON s2.id = d.secondary_skill_id
             WHERE d.id = ?1",
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
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, skill_id, skill_name, skill_points, secondary_skill_id, secondary_skill_name, secondary_points, slot_size, rarity, price, language)) = row else {
        return Ok(None);
    };

    let materials = get_decoration_materials(conn, id)?;
    let (unlock, acquisition) = decoration_unlock_and_acquisition(&materials, price);

    Ok(Some(DecorationDetail {
        id,
        game_id,
        name,
        skill_id,
        skill_name,
        skill_points,
        secondary_skill_id,
        secondary_skill_name,
        secondary_points,
        slot_size,
        rarity,
        price,
        language,
        materials,
        unlock,
        acquisition,
    }))
}

/// Global accent-insensitive search across all MH2G entities, grouped by kind.
pub fn get_global_search(conn: &Connection, game_id: i32, query: &str) -> Result<Vec<SearchResult>> {
    let q = norm_key(query);
    let tokens: Vec<&str> = q.split(' ').filter(|t| !t.is_empty()).collect();
    if tokens.is_empty() {
        return Ok(Vec::new());
    }
    let per_kind = 6;
    let mut out: Vec<SearchResult> = Vec::new();

    // Monsters
    {
        let mut stmt = conn.prepare(
            "SELECT id, name, species FROM monsters WHERE game_id = ?1 ORDER BY id",
        )?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<String>>(2)?)))?;
        let mut local: Vec<(i32, String, String, i32)> = Vec::new();
        for r in rows {
            let (id, name, species) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), species.unwrap_or_default(), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, species, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "monster".into(), id, name, subtitle: species, route: format!("/monsters/{}", id) });
        }
    }

    // Items
    {
        let mut stmt = conn.prepare(
            "SELECT id, name, category FROM items WHERE game_id = ?1 AND id != 1 ORDER BY id",
        )?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<String>>(2)?)))?;
        let mut local: Vec<(i32, String, String, i32)> = Vec::new();
        for r in rows {
            let (id, name, cat) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), cat.unwrap_or_default(), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, cat, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "item".into(), id, name, subtitle: cat, route: format!("/items/{}", id) });
        }
    }

    // Skills
    {
        let mut stmt = conn.prepare("SELECT id, name FROM skills WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?)))?;
        let mut local: Vec<(i32, String, i32)> = Vec::new();
        for r in rows {
            let (id, name) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "skill".into(), id, name, subtitle: "Skill".into(), route: format!("/skills/{}", id) });
        }
    }

    // Weapons
    {
        let mut stmt = conn.prepare("SELECT id, name, weapon_type FROM weapons WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)))?;
        let mut local: Vec<(i32, String, String, i32)> = Vec::new();
        for r in rows {
            let (id, name, wtype) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), wtype, id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, wtype, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "weapon".into(), id, name, subtitle: wtype, route: format!("/weapons/{}", id) });
        }
    }

    // Armor (pieces)
    {
        let mut stmt = conn.prepare("SELECT id, name, slot_type, rank FROM armor WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, String>(3)?)))?;
        let mut local: Vec<(i32, String, String, i32)> = Vec::new();
        for r in rows {
            let (id, name, slot, rank) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), format!("{} · {}", slot, rank), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, sub, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "armor".into(), id, name, subtitle: sub, route: format!("/armor/{}", id) });
        }
    }

    // Armor sets
    {
        let mut stmt = conn.prepare("SELECT id, name FROM armor_sets WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?)))?;
        let mut local: Vec<(i32, String, i32)> = Vec::new();
        for r in rows {
            let (id, name) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "armor_set".into(), id, name, subtitle: "Armor Set".into(), route: format!("/armor/sets/{}", id) });
        }
    }

    // Quests
    {
        let mut stmt = conn.prepare("SELECT id, name, rank FROM quests WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<String>>(2)?)))?;
        let mut local: Vec<(i32, String, String, i32)> = Vec::new();
        for r in rows {
            let (id, name, rank) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), rank.unwrap_or_default(), id));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, rank, id) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "quest".into(), id, name, subtitle: rank, route: format!("/quests/{}", id) });
        }
    }

    // Decorations
    {
        let mut stmt = conn.prepare("SELECT id, name, slot_size FROM decorations WHERE game_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![game_id], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<i32>>(2)?)))?;
        let mut local: Vec<(i32, String, i32, std::option::Option<i32>)> = Vec::new();
        for r in rows {
            let (id, name, slot) = r?;
            let key = norm_key(&name);
            if matches_tokens(&key, &tokens) {
                local.push((score_match(&key, &tokens), name.clone(), id, slot));
            }
        }
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, id, slot) in local.into_iter().take(per_kind) {
            out.push(SearchResult { kind: "decoration".into(), id, name, subtitle: slot.map(|s| format!("{} slot", s)).unwrap_or_default(), route: format!("/decorations/{}", id) });
        }
    }

    Ok(out)
}
