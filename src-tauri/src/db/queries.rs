use rusqlite::{params, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};

/// Global search result across all game entities, normalized accent-insensitive.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub kind: String, // monster | item | skill | weapon | armor | armor_set | quest | decoration
    pub id: i32,
    pub name: String,
    pub subtitle: String,
    pub route: String, // relative to game, e.g. /monsters/12
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

pub(crate) fn norm_key(s: &str) -> String {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub species: Option<String>,
    pub size: Option<String>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
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
    pub ailments: Vec<MonsterAilment>,
    pub tools: Vec<MonsterTool>,
    pub part_break: Vec<MonsterPartBreak>,
    pub drops: Vec<MonsterDrop>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub icon_url_lg: Option<String>,
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
    pub probability: Option<f64>,
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
    pub stagger_hp: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterAilment {
    pub id: i32,
    pub monster_id: i32,
    pub ailment: String,
    pub initial: Option<i32>,
    pub increase: Option<i32>,
    pub max: Option<i32>,
    pub decay_step: Option<i32>,
    pub decay_interval: Option<i32>,
    pub duration_sec: Option<i32>,
    pub damage: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterTool {
    pub id: i32,
    pub monster_id: i32,
    pub tool: String,
    pub normal: Option<i32>,
    pub notfound: Option<i32>,
    pub enraged: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterPartBreak {
    pub id: i32,
    pub monster_id: i32,
    pub part: String,
    pub part_hp: Option<i32>,
    pub break_text: Option<String>,
    pub extract: Option<String>,
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
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
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
    pub rampage: Vec<RampageRef>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialRef {
    pub item_id: i32,
    pub item_name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RampageRef {
    pub skill_id: i32,
    pub skill_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
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
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub language: String,
    pub set_name: Option<String>,
    pub set_bonus_skill: Option<String>,
    pub set_bonus_required: Option<i32>,
    pub set_bonus_skill2: Option<String>,
    pub set_bonus_required2: Option<i32>,
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
    pub category: Option<String>,
    pub stars: Option<i32>,
    pub objective: Option<String>,
    pub objective_original: Option<String>,
    pub location: Option<String>,
    pub location_original: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub is_urgent: bool,
    pub client: Option<String>,
    pub requirements: Option<String>,
    pub reward_money: Option<i32>,
    pub contract_fee: Option<i32>,
    pub main_monsters: Option<String>,
    pub description: Option<String>,
    pub description_original: Option<String>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub hub_icon_name: Option<String>,
    pub hub_icon_color: Option<String>,
    pub hub_icon_url: Option<String>,
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
    pub category: Option<String>,
    pub stars: Option<i32>,
    pub objective: Option<String>,
    pub objective_original: Option<String>,
    pub location: Option<String>,
    pub location_original: Option<String>,
    pub time_limit: Option<i32>,
    pub faints_allowed: Option<i32>,
    pub is_key_quest: bool,
    pub is_urgent: bool,
    pub description: Option<String>,
    pub description_original: Option<String>,
    pub client: Option<String>,
    pub requirements: Option<String>,
    pub reward_money: Option<i32>,
    pub contract_fee: Option<i32>,
    pub main_monsters: Option<String>,
    pub rewards: Vec<QuestReward>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub hub_icon_name: Option<String>,
    pub hub_icon_color: Option<String>,
    pub hub_icon_url: Option<String>,
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
    pub carry_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub description: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MelderRecipe {
    pub id: i32,
    pub result_item_id: i32,
    pub result_name: String,
    pub research_cost: i32,
    pub melding_cost: i32,
    pub unlock_condition: Option<String>,
    pub melder_type: String,
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
    pub carry_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub description: Option<String>,
    pub sources: Vec<ItemSource>,
    pub recipes: Vec<CombineRecipe>,
    pub melder: Option<MelderRecipe>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetBonusRef {
    pub set_id: i32,
    pub set_name: String,
    pub rank: Option<String>,
    pub bonus_skill: String,
    pub required_pieces: i32,
}

fn get_skill_set_bonuses(conn: &Connection, skill_id: i32) -> Result<Vec<SetBonusRef>> {
    // Sets whose skill1 or skill2 bonus grants this skill (name-resolved
    // per-game so cross-game name collisions cannot mislink).
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name,
                (SELECT rank FROM armor WHERE set_id = s.id LIMIT 1) as rank,
                s.bonus_skill, s.bonus_required
         FROM armor_sets s
         JOIN skills sk ON sk.id = ?1 AND sk.game_id = s.game_id
            AND sk.name = s.bonus_skill
         ORDER BY s.id
         LIMIT 200",
    )?;
    let mut out: Vec<SetBonusRef> = stmt
        .query_map(params![skill_id], |row| {
            Ok(SetBonusRef {
                set_id: row.get(0)?,
                set_name: row.get(1)?,
                rank: row.get(2)?,
                bonus_skill: row.get(3)?,
                required_pieces: row.get(4)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    let mut stmt2 = conn.prepare(
        "SELECT s.id, s.name,
                (SELECT rank FROM armor WHERE set_id = s.id LIMIT 1) as rank,
                s.bonus_skill2, s.bonus_required2
         FROM armor_sets s
         JOIN skills sk ON sk.id = ?1 AND sk.game_id = s.game_id
            AND sk.name = s.bonus_skill2
         ORDER BY s.id
         LIMIT 200",
    )?;
    let more: Vec<SetBonusRef> = stmt2
        .query_map(params![skill_id], |row| {
            Ok(SetBonusRef {
                set_id: row.get(0)?,
                set_name: row.get(1)?,
                rank: row.get(2)?,
                bonus_skill: row.get(3)?,
                required_pieces: row.get(4)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    out.extend(more);
    out.sort_by_key(|r| r.set_id);
    Ok(out)
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
    pub set_bonuses: Vec<SetBonusRef>,
}

pub fn get_monsters_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Monster>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, species, size, icon_name, icon_color, icon_url, language FROM monsters WHERE game_id = ?1 ORDER BY CASE size WHEN 'Small' THEN 0 ELSE 1 END, COALESCE(sort_order, id), id",
    )?;

    let monsters = stmt
        .query_map(params![game_id], |row| {
            Ok(Monster {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                species: row.get(3)?,
                size: row.get(4)?,
                icon_name: row.get(5)?,
                icon_color: row.get(6)?,
                icon_url: row.get(7)?,
                language: row.get(8)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(monsters)
}

pub fn get_monster_detail(conn: &Connection, id: i32) -> Result<Option<MonsterDetail>> {
    let monster: Option<(i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, species, size, description, icon_name, icon_color, icon_url, icon_url_lg, language FROM monsters WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get(10)?)),
        )
        .optional()?;

    let Some((
        id,
        game_id,
        name,
        species,
        size,
        description,
        icon_name,
        icon_color,
        icon_url,
        icon_url_lg,
        language,
    )) = monster
    else {
        return Ok(None);
    };

    let weaknesses = get_monster_weaknesses(conn, id)?;
    let ailments = get_monster_ailments(conn, id)?;
    let tools = get_monster_tools(conn, id)?;
    let part_break = get_monster_part_break(conn, id)?;
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
        ailments,
        tools,
        part_break,
        drops,
        armor,
        weapons,
        icon_name,
        icon_color,
        icon_url,
        icon_url_lg,
        language,
    }))
}

pub fn get_monster_dedicated_sets(
    conn: &Connection,
    monster_id: i32,
    rank: Option<&str>,
) -> Result<Vec<ArmorSetDetail>> {
    // Dedicated sets: score >=0.40 monster materials vs total, rank-filtered, sub-species safe via item_id exact match (Lao Shan Auroros 0.54, Borealis 0.45)
    let mut stmt = conn.prepare(
        "SELECT s.id, s.game_id, s.name, s.language FROM armor_sets s
         WHERE s.id IN (
           SELECT DISTINCT a.set_id FROM armor a
           JOIN monster_equipment me ON me.equipment_id = a.id
           WHERE me.monster_id = ?1 AND me.equipment_kind='armor'
         )",
    )?;
    let set_rows: Vec<(i32, i32, String, String)> = stmt
        .query_map(params![monster_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    drop(stmt);
    // Batch material score for all candidate sets in a single aggregation (eliminates N+1 mat_stmt prepares).
    let mut score_map: std::collections::HashMap<i32, (i64, i64)> =
        std::collections::HashMap::new();
    if !set_rows.is_empty() {
        let set_ids: Vec<i32> = set_rows.iter().map(|(sid, _, _, _)| *sid).collect();
        // Build IN list safely — ids are integers from trusted DB (not user input), no injection.
        let in_list = set_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let agg_sql = format!(
            "SELECT a.set_id, SUM(am.quantity) as total_qty, SUM(CASE WHEN md.item_id IS NOT NULL THEN am.quantity ELSE 0 END) as mon_qty
             FROM armor a
             JOIN armor_materials am ON am.armor_id = a.id
             LEFT JOIN (SELECT DISTINCT item_id FROM monster_drops WHERE monster_id=?) md ON md.item_id = am.item_id
             WHERE a.set_id IN ({}) GROUP BY a.set_id",
            in_list
        );
        let mut agg_stmt = conn.prepare(&agg_sql)?;
        let agg_rows = agg_stmt.query_map(params![monster_id], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })?;
        for r in agg_rows {
            if let Ok((sid, total, mon)) = r {
                score_map.insert(sid, (mon, total));
            }
        }
    }
    // Batch fetch pieces for all candidate sets (eliminates N per-set queries + rank COUNT).
    let mut pieces_by_set: std::collections::HashMap<i32, Vec<Armor>> =
        std::collections::HashMap::new();
    if !set_rows.is_empty() {
        let set_ids_str = set_rows
            .iter()
            .map(|(sid, _, _, _)| sid.to_string())
            .collect::<Vec<_>>()
            .join(",");
        if let Some(r) = rank {
            let sql = format!("SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, armor_type, set_id, gender, icon_name, icon_color, icon_url, language FROM armor WHERE set_id IN ({}) AND rank = ?1 ORDER BY set_id, CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END, id", set_ids_str);
            let mut p_stmt = conn.prepare(&sql)?;
            for row in p_stmt.query_map(params![r], |row| {
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
                    icon_name: row.get(18)?,
                    icon_color: row.get(19)?,
                    icon_url: row.get(20)?,
                    language: row.get(21)?,
                })
            })? {
                match row {
                    Ok(armor) => {
                        if let Some(sid) = armor.set_id {
                            pieces_by_set.entry(sid).or_default().push(armor);
                        }
                    }
                    Err(e) => eprintln!("[queries] armor row decode skipped: {}", e),
                }
            }
        } else {
            let sql = format!("SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, armor_type, set_id, gender, icon_name, icon_color, icon_url, language FROM armor WHERE set_id IN ({}) ORDER BY set_id, CASE slot_type WHEN 'head' THEN 0 WHEN 'chest' THEN 1 WHEN 'arms' THEN 2 WHEN 'waist' THEN 3 WHEN 'legs' THEN 4 ELSE 5 END, id", set_ids_str);
            let mut p_stmt = conn.prepare(&sql)?;
            for row in p_stmt.query_map([], |row| {
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
                    icon_name: row.get(18)?,
                    icon_color: row.get(19)?,
                    icon_url: row.get(20)?,
                    language: row.get(21)?,
                })
            })? {
                match row {
                    Ok(armor) => {
                        if let Some(sid) = armor.set_id {
                            pieces_by_set.entry(sid).or_default().push(armor);
                        }
                    }
                    Err(e) => eprintln!("[queries] armor row decode skipped: {}", e),
                }
            }
        }
    }
    let mut out = Vec::new();
    for (sid, gid, sname, lang) in set_rows {
        let (monster_qty, total_qty) = score_map.get(&sid).copied().unwrap_or((0, 0));
        if total_qty == 0 {
            continue;
        }
        let score = monster_qty as f64 / total_qty as f64;
        if score < 0.40 {
            continue;
        }
        let pieces = pieces_by_set.get(&sid).cloned().unwrap_or_default();
        if pieces.is_empty() {
            continue;
        }
        out.push(ArmorSetDetail {
            id: sid,
            game_id: gid,
            name: sname,
            pieces,
            language: lang,
            // Dedicated monster sets are dynamic groupings; canonical bonus
            // display lives on the sets browser (get_armor_set_detail).
            bonus_skill: None,
            bonus_required: None,
            bonus_skill_id: None,
            bonus_skill2: None,
            bonus_required2: None,
            bonus_skill2_id: None,
        });
    }
    out.sort_by_key(|s| s.id);
    Ok(out)
}

fn get_monster_related_armor(conn: &Connection, monster_id: i32) -> Result<Vec<Armor>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.game_id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                a.slots, a.skills, a.armor_type, a.set_id, a.gender, a.icon_name, a.icon_color, a.icon_url, a.language
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
                icon_name: row.get(18)?,
                icon_color: row.get(19)?,
                icon_url: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(armor)
}

fn get_monster_related_weapons(conn: &Connection, monster_id: i32) -> Result<Vec<Weapon>> {
    let mut stmt = conn.prepare(
        "SELECT w.id, w.game_id, w.name, w.weapon_type, w.rarity, w.attack, w.affinity, w.element_type, w.element_value,
                w.sharpness, w.slots, w.status_type, w.status_value, w.defense_bonus, w.crafting_cost, w.upgrade_path,
                EXISTS(SELECT 1 FROM weapon_craft wc WHERE wc.weapon_id = w.id AND wc.craft_kind = 'forge'), w.sort_order, w.icon_name, w.icon_color, w.icon_url, w.language
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
                WHEN 'Charge Blade' THEN 9
                WHEN 'Insect Glaive' THEN 10
                WHEN 'Light Bowgun' THEN 11
                WHEN 'Heavy Bowgun' THEN 12
                WHEN 'Bow' THEN 13
                ELSE 14
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
                icon_name: row.get(18)?,
                icon_color: row.get(19)?,
                icon_url: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(drops)
}

fn get_monster_weaknesses(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterWeakness>> {
    let mut stmt = conn.prepare(
        "SELECT id, part_name, sever, blunt, projectile, fire, water, thunder, ice, dragon, stagger_hp
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
                stagger_hp: row.get(10)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(weaknesses)
}

fn get_monster_ailments(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterAilment>> {
    let mut stmt = conn.prepare(
        "SELECT id, monster_id, ailment, initial, increase, max, decay_step, decay_interval, duration_sec, damage, notes
         FROM monster_ailments WHERE monster_id = ?1 ORDER BY id",
    )?;

    let ailments = stmt
        .query_map(params![monster_id], |row| {
            Ok(MonsterAilment {
                id: row.get(0)?,
                monster_id: row.get(1)?,
                ailment: row.get(2)?,
                initial: row.get(3)?,
                increase: row.get(4)?,
                max: row.get(5)?,
                decay_step: row.get(6)?,
                decay_interval: row.get(7)?,
                duration_sec: row.get(8)?,
                damage: row.get(9)?,
                notes: row.get(10)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(ailments)
}

fn get_monster_tools(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterTool>> {
    let mut stmt = conn.prepare(
        "SELECT id, monster_id, tool, normal, notfound, enraged, notes
         FROM monster_tools WHERE monster_id = ?1 ORDER BY id",
    )?;

    let tools = stmt
        .query_map(params![monster_id], |row| {
            Ok(MonsterTool {
                id: row.get(0)?,
                monster_id: row.get(1)?,
                tool: row.get(2)?,
                normal: row.get(3)?,
                notfound: row.get(4)?,
                enraged: row.get(5)?,
                notes: row.get(6)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(tools)
}

fn get_monster_part_break(conn: &Connection, monster_id: i32) -> Result<Vec<MonsterPartBreak>> {
    let mut stmt = conn.prepare(
        "SELECT id, monster_id, part, part_hp, break_text, extract
         FROM monster_part_break WHERE monster_id = ?1 ORDER BY id",
    )?;

    let out = stmt
        .query_map(params![monster_id], |row| {
            Ok(MonsterPartBreak {
                id: row.get(0)?,
                monster_id: row.get(1)?,
                part: row.get(2)?,
                part_hp: row.get(3)?,
                break_text: row.get(4)?,
                extract: row.get(5)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(out)
}

pub fn get_weapons_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Weapon>> {
    // Smith order: game weapon-trees order (Great Sword → Bow) verified via ISO tree via upgrade_path
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                sharpness, slots, status_type, status_value, defense_bonus, crafting_cost, upgrade_path,
                EXISTS(SELECT 1 FROM weapon_craft wc WHERE wc.weapon_id = weapons.id AND wc.craft_kind = 'forge'), sort_order, icon_name, icon_color, icon_url, language
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
                WHEN 'Charge Blade' THEN 9
                WHEN 'Insect Glaive' THEN 10
                WHEN 'Light Bowgun' THEN 11
                WHEN 'Heavy Bowgun' THEN 12
                WHEN 'Bow' THEN 13
                ELSE 14
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
                icon_name: row.get(18)?,
                icon_color: row.get(19)?,
                icon_url: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        Option<String>,
        Option<String>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value,
                    sharpness, slots, skills, status_type, status_value, defense_bonus,
                    crafting_cost, upgrade_path, sort_order, description, icon_name, icon_color, icon_url, language
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
                    row.get(20)?,
                    row.get(21)?,
                    row.get(22)?,
                ))
            },
        )
        .optional()?;

    let Some((
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
        icon_name,
        icon_color,
        icon_url,
        language,
    )) = row
    else {
        return Ok(None);
    };

    let materials = get_weapon_materials(conn, id)?;
    let forge_materials = get_weapon_craft_materials(conn, id, "forge")?;
    let upgrade_materials = get_weapon_craft_materials(conn, id, "upgrade")?;
    let is_forgeable = !forge_materials.is_empty();
    let rampage = get_weapon_rampage(conn, id)?;

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
        rampage,
        icon_name,
        icon_color,
        icon_url,
        language,
    }))
}

fn get_weapon_rampage(conn: &Connection, weapon_id: i32) -> Result<Vec<RampageRef>> {
    let mut stmt = conn.prepare(
        "SELECT rs.id, rs.name, rs.description
         FROM weapon_rampage wr JOIN rampage_skills rs ON rs.id = wr.skill_id
         WHERE wr.weapon_id = ?1 ORDER BY rs.id",
    )?;
    let out = stmt
        .query_map(params![weapon_id], |row| {
            Ok(RampageRef {
                skill_id: row.get(0)?,
                skill_name: row.get(1)?,
                description: row.get(2)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(out)
}

fn get_weapon_craft_materials(
    conn: &Connection,
    weapon_id: i32,
    kind: &str,
) -> Result<Vec<MaterialRef>> {
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(materials)
}

pub fn get_armor_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Armor>> {
    // Smith order: faithful to armorer list (rank -> slot -> creation order = id) verified via ISO armor string table order at 37652906
    let mut stmt = conn.prepare(
         "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                 resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                 slots, skills, armor_type, set_id, gender, icon_name, icon_color, icon_url, language
          FROM armor WHERE game_id = ?1 ORDER BY
             CASE rank WHEN 'Low' THEN 0 WHEN 'High' THEN 1 WHEN 'G' THEN 2 WHEN 'Master' THEN 3 ELSE 4 END,
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
                icon_name: row.get(18)?,
                icon_color: row.get(19)?,
                icon_url: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
    pub bonus_skill: Option<String>,
    pub bonus_required: Option<i32>,
    pub bonus_skill2: Option<String>,
    pub bonus_required2: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorSetDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub pieces: Vec<Armor>,
    pub language: String,
    pub bonus_skill: Option<String>,
    pub bonus_required: Option<i32>,
    pub bonus_skill_id: Option<i32>,
    pub bonus_skill2: Option<String>,
    pub bonus_required2: Option<i32>,
    pub bonus_skill2_id: Option<i32>,
}

pub fn get_armor_sets_by_game(conn: &Connection, game_id: i32) -> Result<Vec<ArmorSet>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.game_id, s.name, COUNT(a.id) as piece_count,
                (SELECT rank FROM armor WHERE set_id = s.id LIMIT 1) as rank,
                (SELECT MAX(rarity) FROM armor WHERE set_id = s.id) as rarity,
                s.language, s.bonus_skill, s.bonus_required, s.bonus_skill2, s.bonus_required2
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
                bonus_skill: row.get(7)?,
                bonus_required: row.get(8)?,
                bonus_skill2: row.get(9)?,
                bonus_required2: row.get(10)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(sets)
}

pub fn get_armor_set_detail(conn: &Connection, id: i32) -> Result<Option<ArmorSetDetail>> {
    let row: Option<(i32, i32, String, String, Option<String>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>)> = conn
        .query_row(
            "SELECT s.id, s.game_id, s.name, s.language, s.bonus_skill, s.bonus_required, sk1.id, s.bonus_skill2, s.bonus_required2, sk2.id
             FROM armor_sets s
             LEFT JOIN skills sk1 ON sk1.name = s.bonus_skill AND sk1.game_id = s.game_id
             LEFT JOIN skills sk2 ON sk2.name = s.bonus_skill2 AND sk2.game_id = s.game_id
             WHERE s.id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?)),
        )
        .optional()?;
    let Some((
        id,
        game_id,
        name,
        language,
        bonus_skill,
        bonus_required,
        bonus_skill_id,
        bonus_skill2,
        bonus_required2,
        bonus_skill2_id,
    )) = row
    else {
        return Ok(None);
    };
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon,
                slots, skills, armor_type, set_id, gender, icon_name, icon_color, icon_url, language
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
                icon_name: row.get(18)?,
                icon_color: row.get(19)?,
                icon_url: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(Some(ArmorSetDetail {
        id,
        game_id,
        name,
        pieces,
        language,
        bonus_skill,
        bonus_required,
        bonus_skill_id,
        bonus_skill2,
        bonus_required2,
        bonus_skill2_id,
    }))
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
        Option<String>,
        Option<String>,
        Option<String>,
        String,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<String>,
        Option<i32>,
    )> = conn
        .query_row(
            "SELECT a.id, a.game_id, a.name, a.slot_type, a.rank, a.rarity, a.defense_base, a.defense_max,
                    a.resistance_fire, a.resistance_water, a.resistance_thunder, a.resistance_ice, a.resistance_dragon,
                    a.slots, a.skills, a.armor_type, a.gender, a.set_id, a.crafting_cost, a.description, a.icon_name, a.icon_color, a.icon_url, a.language,
                    s.name, s.bonus_skill, s.bonus_required, s.bonus_skill2, s.bonus_required2
             FROM armor a LEFT JOIN armor_sets s ON s.id = a.set_id WHERE a.id = ?1",
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
                    row.get(21)?,
                    row.get(22)?,
                    row.get(23)?,
                    row.get(24)?,
                    row.get(25)?,
                    row.get(26)?,
                    row.get(27)?,
                    row.get(28)?,
                ))
            },
        )
        .optional()?;

    let Some((
        id,
        game_id,
        name,
        slot_type,
        rank,
        rarity,
        defense_base,
        defense_max,
        fire,
        water,
        thunder,
        ice,
        dragon,
        slots,
        skills,
        armor_type,
        gender,
        set_id,
        crafting_cost,
        description,
        icon_name,
        icon_color,
        icon_url,
        language,
        set_name,
        set_bonus_skill,
        set_bonus_required,
        set_bonus_skill2,
        set_bonus_required2,
    )) = row
    else {
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
        icon_name,
        icon_color,
        icon_url,
        language,
        set_name,
        set_bonus_skill,
        set_bonus_required,
        set_bonus_skill2,
        set_bonus_required2,
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(materials)
}

pub fn get_quests_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Quest>> {
    let mut stmt = conn.prepare(
         "SELECT id, game_id, name, name_original, type, rank, hub, category, stars, objective, objective_original, location, location_original, time_limit, faints_allowed, is_key_quest, is_urgent, client, requirements, reward_money, contract_fee, main_monsters, description, description_original, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language
         FROM quests WHERE game_id = ?1 ORDER BY
            CASE hub WHEN 'low_high' THEN 0 WHEN 'master' THEN 1 WHEN 'siege' THEN 2 WHEN 'elder' THEN 3 WHEN 'nekoto' THEN 4 WHEN 'village' THEN 5 WHEN 'village_low' THEN 5 WHEN 'village_high' THEN 6 WHEN 'guild_low' THEN 7 WHEN 'guild_high' THEN 8 WHEN 'guild_g' THEN 9 WHEN 'event' THEN 10 WHEN 'challenge' THEN 11 WHEN 'training' THEN 12 WHEN 'treasure' THEN 13 WHEN 'hot_spring' THEN 14 WHEN 'drink' THEN 15 WHEN 'nyanta' THEN 16 ELSE 17 END,
            CASE category WHEN 'assigned' THEN 0 WHEN 'optional' THEN 1 WHEN 'event' THEN 2 WHEN 'arena' THEN 3 WHEN 'challenge' THEN 4 WHEN 'special' THEN 5 WHEN 'siege' THEN 6 ELSE 7 END,
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
                category: row.get(7)?,
                stars: row.get(8)?,
                objective: row.get(9)?,
                objective_original: row.get(10)?,
                location: row.get(11)?,
                location_original: row.get(12)?,
                time_limit: row.get(13)?,
                faints_allowed: row.get(14)?,
                is_key_quest: row.get(15)?,
                is_urgent: row.get(16)?,
                client: row.get(17)?,
                requirements: row.get(18)?,
                reward_money: row.get(19)?,
                contract_fee: row.get(20)?,
                main_monsters: row.get(21)?,
                description: row.get(22)?,
                description_original: row.get(23)?,
                icon_name: row.get(24)?,
                icon_color: row.get(25)?,
                icon_url: row.get(26)?,
                hub_icon_name: row.get(27)?,
                hub_icon_color: row.get(28)?,
                hub_icon_url: row.get(29)?,
                language: row.get(30)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        Option<String>,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        bool,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        String,
    )> = conn
        .query_row(
            "SELECT id, game_id, name, name_original, type, rank, hub, category, stars, objective, objective_original, location, location_original, time_limit, faints_allowed, is_key_quest, is_urgent, description, description_original, client, requirements, reward_money, contract_fee, main_monsters, icon_name, icon_color, icon_url, hub_icon_name, hub_icon_color, hub_icon_url, language
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
                    row.get(21)?,
                    row.get(22)?,
                    row.get(23)?,
                    row.get(24)?,
                    row.get(25)?,
                    row.get(26)?,
                    row.get(27)?,
                    row.get(28)?,
                    row.get(29)?,
                    row.get(30)?,
                ))
            },
        )
        .optional()?;

    let Some((
        id,
        game_id,
        name,
        name_original,
        r#type,
        rank,
        hub,
        category,
        stars,
        objective,
        objective_original,
        location,
        location_original,
        time_limit,
        faints_allowed,
        is_key_quest,
        is_urgent,
        description,
        description_original,
        client,
        requirements,
        reward_money,
        contract_fee,
        main_monsters,
        icon_name,
        icon_color,
        icon_url,
        hub_icon_name,
        hub_icon_color,
        hub_icon_url,
        language,
    )) = row
    else {
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
        category,
        stars,
        objective,
        objective_original,
        location,
        location_original,
        time_limit,
        faints_allowed,
        is_key_quest,
        is_urgent,
        description,
        description_original,
        client,
        requirements,
        reward_money,
        contract_fee,
        main_monsters,
        rewards,
        icon_name,
        icon_color,
        icon_url,
        hub_icon_name,
        hub_icon_color,
        hub_icon_url,
        language,
    }))
}

pub fn get_items_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Item>> {
    // Chest order: faithful to in-game item box (sort_order = MHWorldData id for MHW, DATA.BIN for PSP)
    // Alternative sorts handled client-side; keep DB default as game chest.
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, sort_order, icon_name, icon_color, icon_url, description, language
         FROM items WHERE game_id = ?1 ORDER BY COALESCE(sort_order, id), id",
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
                carry_limit: row.get(8)?,
                sort_order: row.get(9)?,
                icon_name: row.get(10)?,
                icon_color: row.get(11)?,
                icon_url: row.get(12)?,
                description: row.get(13)?,
                language: row.get(14)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(items)
}

pub fn get_item_detail(conn: &Connection, id: i32) -> Result<Option<ItemDetail>> {
    let row: Option<(
        i32,
        i32,
        String,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        String,
    )> = conn.query_row(
        "SELECT id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, sort_order, icon_name, icon_color, icon_url, description, language
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
                row.get(10)?,
                row.get(11)?,
                row.get(12)?,
                row.get(13)?,
                row.get(14)?,
            ))
        },
    ).optional()?;

    let Some((
        id,
        game_id,
        name,
        category,
        subcategory,
        rarity,
        sell_price,
        buy_price,
        carry_limit,
        sort_order,
        icon_name,
        icon_color,
        icon_url,
        description,
        language,
    )) = row
    else {
        return Ok(None);
    };

    let sources = get_item_sources(conn, id)?;
    let recipes = get_item_combine_recipes(conn, id)?;
    let melder = get_melder_recipe(conn, id)?;

    Ok(Some(ItemDetail {
        id,
        game_id,
        name,
        category,
        subcategory,
        rarity,
        sell_price,
        buy_price,
        carry_limit,
        sort_order,
        icon_name,
        icon_color,
        icon_url,
        description,
        sources,
        recipes,
        melder,
        language,
    }))
}

fn get_melder_recipe(conn: &Connection, item_id: i32) -> Result<Option<MelderRecipe>> {
    let row: Option<(i32, i32, String, i32, i32, Option<String>, String)> = conn
        .query_row(
            "SELECT mr.id, mr.result_item_id, i.name, mr.research_cost, mr.melding_cost, mr.unlock_condition, mr.melder_type
             FROM melder_recipes mr JOIN items i ON i.id = mr.result_item_id
             WHERE mr.result_item_id = ?1 LIMIT 1",
            params![item_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .optional()?;
    Ok(row.map(
        |(
            id,
            result_item_id,
            result_name,
            research_cost,
            melding_cost,
            unlock_condition,
            melder_type,
        )| MelderRecipe {
            id,
            result_item_id,
            result_name,
            research_cost,
            melding_cost,
            unlock_condition,
            melder_type,
        },
    ))
}

pub fn get_melder_recipes_by_game(conn: &Connection, game_id: i32) -> Result<Vec<MelderRecipe>> {
    let mut stmt = conn.prepare(
        "SELECT mr.id, mr.result_item_id, i.name, mr.research_cost, mr.melding_cost, mr.unlock_condition, mr.melder_type
         FROM melder_recipes mr JOIN items i ON i.id = mr.result_item_id
         WHERE mr.game_id = ?1 ORDER BY mr.id",
    )?;
    let recs = stmt
        .query_map(params![game_id], |row| {
            Ok(MelderRecipe {
                id: row.get(0)?,
                result_item_id: row.get(1)?,
                result_name: row.get(2)?,
                research_cost: row.get(3)?,
                melding_cost: row.get(4)?,
                unlock_condition: row.get(5)?,
                melder_type: row.get(6)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(recs)
}

fn get_item_sources(conn: &Connection, item_id: i32) -> Result<Vec<ItemSource>> {
    // Unified sources: monster_drops (carve/capture/break/drop) + quest_rewards + gathering (item_sources)
    // item_sources rows of type carve/capture/drop/break/quest_reward WITH a source_id are filtered
    // out to avoid duplication with the two authoritative tables above (the seed mirrors
    // monster_drops into item_sources). Rows with source_id IS NULL are small-monster
    // carve/drop/capture text rows (location like 'Conga - carving') with no counterpart
    // in monster_drops, so they stay visible.
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
            WHERE s.item_id = ?1 AND (s.source_type NOT IN ('carve', 'capture', 'drop', 'break', 'quest_reward') OR s.source_id IS NULL)
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharmSkillView {
    pub skill_id: i32,
    pub skill_name: String,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComradeWeapon {
    pub id: i32,
    pub name_jp: String,
    pub price: Option<i32>,
    pub rarity: Option<i32>,
    pub attack: Option<i32>,
    pub element: Option<String>,
    pub sharpness: Option<String>,
    pub affinity: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComradeArmor {
    pub id: i32,
    pub name_jp: String,
    pub slot: String,
    pub rarity: Option<i32>,
    pub price: Option<i32>,
    pub defense: Option<i32>,
    pub fire: Option<i32>,
    pub water: Option<i32>,
    pub thunder: Option<i32>,
    pub ice: Option<i32>,
    pub dragon: Option<i32>,
    pub materials_jp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComradeSkill {
    pub id: i32,
    pub name_jp: String,
    pub memory: Option<String>,
    pub effect_jp: Option<String>,
    pub cost: Option<i32>,
    pub learn_level: Option<String>,
}

pub fn get_comrade_weapons(conn: &Connection, game_id: i32) -> Result<Vec<ComradeWeapon>> {
    let mut stmt = conn.prepare(
        "SELECT id, name_jp, price, rarity, attack, element, sharpness, affinity, style
         FROM comrade_weapons WHERE game_id = ?1 ORDER BY rarity, id",
    )?;

    let weapons = stmt
        .query_map(params![game_id], |row| {
            Ok(ComradeWeapon {
                id: row.get(0)?,
                name_jp: row.get(1)?,
                price: row.get(2)?,
                rarity: row.get(3)?,
                attack: row.get(4)?,
                element: row.get(5)?,
                sharpness: row.get(6)?,
                affinity: row.get(7)?,
                style: row.get(8)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(weapons)
}

pub fn get_comrade_armor(conn: &Connection, game_id: i32) -> Result<Vec<ComradeArmor>> {
    let mut stmt = conn.prepare(
        "SELECT id, name_jp, slot, rarity, price, defense, fire, water, thunder, ice, dragon, materials_jp
         FROM comrade_armor WHERE game_id = ?1 ORDER BY slot, rarity, id",
    )?;

    let armor = stmt
        .query_map(params![game_id], |row| {
            Ok(ComradeArmor {
                id: row.get(0)?,
                name_jp: row.get(1)?,
                slot: row.get(2)?,
                rarity: row.get(3)?,
                price: row.get(4)?,
                defense: row.get(5)?,
                fire: row.get(6)?,
                water: row.get(7)?,
                thunder: row.get(8)?,
                ice: row.get(9)?,
                dragon: row.get(10)?,
                materials_jp: row.get(11)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(armor)
}

pub fn get_comrade_skills(conn: &Connection, game_id: i32) -> Result<Vec<ComradeSkill>> {
    let mut stmt = conn.prepare(
        "SELECT id, name_jp, memory, effect_jp, cost, learn_level
         FROM comrade_skills WHERE game_id = ?1 ORDER BY id",
    )?;

    let skills = stmt
        .query_map(params![game_id], |row| {
            Ok(ComradeSkill {
                id: row.get(0)?,
                name_jp: row.get(1)?,
                memory: row.get(2)?,
                effect_jp: row.get(3)?,
                cost: row.get(4)?,
                learn_level: row.get(5)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(skills)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuddyWeapon {
    pub id: i32,
    pub kind: String,
    pub name: String,
    pub attack_type: Option<String>,
    pub attack_melee: Option<i32>,
    pub attack_ranged: Option<i32>,
    pub element_type: Option<String>,
    pub element_melee: Option<i32>,
    pub element_ranged: Option<i32>,
    pub status_type: Option<String>,
    pub status_melee: Option<i32>,
    pub status_ranged: Option<i32>,
    pub affinity: Option<i32>,
    pub defense_bonus: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuddyArmor {
    pub id: i32,
    pub kind: String,
    pub name: String,
    pub defense: Option<i32>,
    pub fire: Option<i32>,
    pub water: Option<i32>,
    pub thunder: Option<i32>,
    pub ice: Option<i32>,
    pub dragon: Option<i32>,
}

pub fn get_buddy_weapons(
    conn: &Connection,
    game_id: i32,
    kind: Option<String>,
) -> Result<Vec<BuddyWeapon>> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, name, attack_type, attack_melee, attack_ranged,
                element_type, element_melee, element_ranged,
                status_type, status_melee, status_ranged, affinity, defense_bonus
         FROM buddy_weapons WHERE game_id = ?1 AND (?2 IS NULL OR kind = ?2)
         ORDER BY kind, id",
    )?;

    let weapons = stmt
        .query_map(params![game_id, kind], |row| {
            Ok(BuddyWeapon {
                id: row.get(0)?,
                kind: row.get(1)?,
                name: row.get(2)?,
                attack_type: row.get(3)?,
                attack_melee: row.get(4)?,
                attack_ranged: row.get(5)?,
                element_type: row.get(6)?,
                element_melee: row.get(7)?,
                element_ranged: row.get(8)?,
                status_type: row.get(9)?,
                status_melee: row.get(10)?,
                status_ranged: row.get(11)?,
                affinity: row.get(12)?,
                defense_bonus: row.get(13)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(weapons)
}

pub fn get_buddy_armor(
    conn: &Connection,
    game_id: i32,
    kind: Option<String>,
) -> Result<Vec<BuddyArmor>> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, name, defense, fire, water, thunder, ice, dragon
         FROM buddy_armor WHERE game_id = ?1 AND (?2 IS NULL OR kind = ?2)
         ORDER BY kind, id",
    )?;

    let armor = stmt
        .query_map(params![game_id, kind], |row| {
            Ok(BuddyArmor {
                id: row.get(0)?,
                kind: row.get(1)?,
                name: row.get(2)?,
                defense: row.get(3)?,
                fire: row.get(4)?,
                water: row.get(5)?,
                thunder: row.get(6)?,
                ice: row.get(7)?,
                dragon: row.get(8)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(armor)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharmView {
    pub id: i32,
    pub charm_type: String,
    pub charm_table: Option<i32>,
    pub slots: i32,
    pub skills: Vec<CharmSkillView>,
}

pub fn get_charms_by_game(
    conn: &Connection,
    game_id: i32,
    skill_id: Option<i32>,
    min_points: Option<i32>,
) -> Result<Vec<CharmView>> {
    let min_pts = min_points.unwrap_or(i32::MIN);
    let mut stmt = conn.prepare(
        "SELECT c.id, c.charm_type, c.charm_table, c.slots, cs.skill_id, s.name, cs.points
         FROM charms c
         JOIN charm_skills cs ON cs.charm_id = c.id
         JOIN skills s ON s.id = cs.skill_id
         WHERE c.game_id = ?1
           AND (?2 IS NULL OR EXISTS (SELECT 1 FROM charm_skills cs2 WHERE cs2.charm_id = c.id AND cs2.skill_id = ?2 AND cs2.points >= ?3))
         ORDER BY c.slots DESC, cs.points DESC, c.id
         LIMIT 500",
    )?;
    let mut map: std::collections::BTreeMap<i32, CharmView> = std::collections::BTreeMap::new();
    let mut order: Vec<i32> = Vec::new();
    let rows = stmt.query_map(params![game_id, skill_id, min_pts], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<i32>>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, i32>(6)?,
        ))
    })?;
    for r in rows {
        let (cid, ctype, ctable, slots, sid, sname, pts) = r?;
        let entry = map.entry(cid).or_insert_with(|| {
            order.push(cid);
            CharmView {
                id: cid,
                charm_type: ctype.clone(),
                charm_table: ctable,
                slots,
                skills: Vec::new(),
            }
        });
        entry.skills.push(CharmSkillView {
            skill_id: sid,
            skill_name: sname,
            points: pts,
        });
    }
    let mut out: Vec<CharmView> = Vec::new();
    for cid in order {
        if let Some(v) = map.remove(&cid) {
            out.push(v);
        }
    }
    Ok(out)
}

// 010-mhw-100: hunter charms rank tree (MHW). Skills denormalized upstream;
// materials joined with item names for the rank-tree UI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HunterCharmMaterialView {
    pub craft_kind: String,
    pub item_id: i32,
    pub item_name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HunterCharmView {
    pub id: i32,
    pub name: String,
    pub charm_level: i32,
    pub rarity: i32,
    pub previous_id: Option<i32>,
    pub previous_name: Option<String>,
    pub skill1_name: Option<String>,
    pub skill1_level: Option<i32>,
    pub skill2_name: Option<String>,
    pub skill2_level: Option<i32>,
    pub materials: Vec<HunterCharmMaterialView>,
}

pub fn get_hunter_charms_by_game(conn: &Connection, game_id: i32) -> Result<Vec<HunterCharmView>> {
    let mut stmt = conn.prepare(
        "SELECT hc.id, hc.name, hc.charm_level, hc.rarity, hc.previous_id, p.name,
                hc.skill1_name, hc.skill1_level, hc.skill2_name, hc.skill2_level
         FROM hunter_charms hc
         LEFT JOIN hunter_charms p ON p.id = hc.previous_id AND p.game_id = hc.game_id
         WHERE hc.game_id = ?1
         ORDER BY hc.name",
    )?;
    let charms: Vec<HunterCharmView> = stmt
        .query_map(params![game_id], |row| {
            Ok(HunterCharmView {
                id: row.get(0)?,
                name: row.get(1)?,
                charm_level: row.get(2)?,
                rarity: row.get(3)?,
                previous_id: row.get(4)?,
                previous_name: row.get(5)?,
                skill1_name: row.get(6)?,
                skill1_level: row.get(7)?,
                skill2_name: row.get(8)?,
                skill2_level: row.get(9)?,
                materials: Vec::new(),
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    let mut out = Vec::with_capacity(charms.len());
    for mut c in charms {
        let mut mstmt = conn.prepare(
            "SELECT m.craft_kind, m.item_id, i.name, m.quantity
             FROM hunter_charm_materials m
             JOIN items i ON i.id = m.item_id
             WHERE m.charm_id = ?1
             ORDER BY m.craft_kind, m.item_id",
        )?;
        c.materials = mstmt
            .query_map(params![c.id], |row| {
                Ok(HunterCharmMaterialView {
                    craft_kind: row.get(0)?,
                    item_id: row.get(1)?,
                    item_name: row.get(2)?,
                    quantity: row.get(3)?,
                })
            })?
            .filter_map(|r| {
                r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                    .ok()
            })
            .collect();
        out.push(c);
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    Ok(skills)
}

pub fn get_skill_detail(conn: &Connection, id: i32) -> Result<Option<SkillDetail>> {
    let skill: Option<(i32, i32, String, Option<String>, Option<i32>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, description, max_level, language FROM skills WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .optional()?;

    let Some((id, game_id, name, description, max_level, language)) = skill else {
        return Ok(None);
    };

    let levels = get_skill_levels(conn, id)?;
    let decorations = get_skill_decorations(conn, id)?;
    let armors = get_skill_armors(conn, id)?;
    let weapons = get_skill_weapons(conn, id)?;
    let set_bonuses = get_skill_set_bonuses(conn, id)?;

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
        set_bonuses,
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(rows)
}

fn decoration_unlock_and_acquisition(
    materials: &[DecoMaterial],
    price: Option<i32>,
) -> (String, String) {
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
    let base_rows: Vec<(
        i32,
        String,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<String>,
    )> = stmt
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();

    let mut out = Vec::new();
    for (
        id,
        name,
        slot_size,
        prim_id,
        prim_pts,
        _sec_id,
        sec_pts,
        price,
        rarity,
        prim_name,
        sec_name,
    ) in base_rows
    {
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
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
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
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
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub language: String,
    pub materials: Vec<DecoMaterial>,
    pub unlock: String,
    pub acquisition: String,
}

pub fn get_decorations_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Decoration>> {
    let mut stmt = conn.prepare(
        "SELECT d.id, d.game_id, d.name, d.skill_id, s1.name, d.skill_points, d.secondary_skill_id, s2.name, d.secondary_points, d.slot_size, d.rarity, d.price, d.icon_name, d.icon_color, d.icon_url, d.language
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
                icon_name: row.get(12)?,
                icon_color: row.get(13)?,
                icon_url: row.get(14)?,
                language: row.get(15)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(rows)
}

pub fn get_decoration_detail(conn: &Connection, id: i32) -> Result<Option<DecorationDetail>> {
    let row: Option<(i32, i32, String, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, String, Option<String>)> = conn
        .query_row(
            "SELECT d.id, d.game_id, d.name, d.skill_id, s1.name, d.skill_points, d.secondary_skill_id, s2.name, d.secondary_points, d.slot_size, d.rarity, d.price, d.icon_name, d.icon_color, d.icon_url, d.language, d.acquisition
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
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                    row.get(16)?,
                ))
            },
        )
        .optional()?;

    let Some((
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
        icon_name,
        icon_color,
        icon_url,
        language,
        acquisition_db,
    )) = row
    else {
        return Ok(None);
    };

    let materials = get_decoration_materials(conn, id)?;
    let (unlock, derived_acquisition) = decoration_unlock_and_acquisition(&materials, price);
    // 010-mhw-100 Phase B: stored acquisition (melder costs + feystone odds)
    // wins over the smith-derived fallback.
    let acquisition: String = acquisition_db.unwrap_or(derived_acquisition);

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
        icon_name,
        icon_color,
        icon_url,
        language,
        materials,
        unlock,
        acquisition,
    }))
}

// ── MHW Mantles / Boosters + Palico Gadgets ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MhwMantle {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub tool_type: String,
    pub rarity: Option<i32>,
    pub description: Option<String>,
    pub effect: String,
    pub duration_sec: Option<i32>,
    pub cooldown_sec: Option<i32>,
    pub cooldown_upgraded_sec: Option<i32>,
    pub slots: Option<String>,
    pub acquisition: Option<String>,
    pub upgrade_quest: Option<String>,
    pub upgrade_effect: Option<String>,
    pub sort_order: Option<i32>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub icon_name_plus: Option<String>,
    pub icon_color_plus: Option<String>,
    pub icon_url_plus: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PalicoGadget {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub gadget_type: String,
    pub tribe: Option<String>,
    pub description: Option<String>,
    pub effect: Option<String>,
    pub acquisition: Option<String>,
    pub sort_order: Option<i32>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PalicoGadgetLevel {
    pub id: i32,
    pub proficiency: i32,
    pub ability_name: String,
    pub description: Option<String>,
    pub unlock_condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PalicoGadgetDetail {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub gadget_type: String,
    pub tribe: Option<String>,
    pub description: Option<String>,
    pub effect: Option<String>,
    pub acquisition: Option<String>,
    pub sort_order: Option<i32>,
    pub icon_name: Option<String>,
    pub icon_color: Option<String>,
    pub icon_url: Option<String>,
    pub language: String,
    pub levels: Vec<PalicoGadgetLevel>,
}

pub fn get_mhw_mantles_by_game(conn: &Connection, game_id: i32) -> Result<Vec<MhwMantle>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, tool_type, rarity, description, effect, duration_sec, cooldown_sec, cooldown_upgraded_sec, slots, acquisition, upgrade_quest, upgrade_effect, sort_order, icon_name, icon_color, icon_url, icon_name_plus, icon_color_plus, icon_url_plus, language FROM mhw_mantles WHERE game_id = ?1 ORDER BY COALESCE(sort_order, id)",
    )?;
    let rows = stmt
        .query_map(params![game_id], |row| {
            Ok(MhwMantle {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                tool_type: row.get(3)?,
                rarity: row.get(4)?,
                description: row.get(5)?,
                effect: row.get(6)?,
                duration_sec: row.get(7)?,
                cooldown_sec: row.get(8)?,
                cooldown_upgraded_sec: row.get(9)?,
                slots: row.get(10)?,
                acquisition: row.get(11)?,
                upgrade_quest: row.get(12)?,
                upgrade_effect: row.get(13)?,
                sort_order: row.get(14)?,
                icon_name: row.get(15)?,
                icon_color: row.get(16)?,
                icon_url: row.get(17)?,
                icon_name_plus: row.get(18)?,
                icon_color_plus: row.get(19)?,
                icon_url_plus: row.get(20)?,
                language: row.get(21)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(rows)
}

pub fn get_mhw_mantle_detail(conn: &Connection, id: i32) -> Result<Option<MhwMantle>> {
    let row = conn
        .query_row(
            "SELECT id, game_id, name, tool_type, rarity, description, effect, duration_sec, cooldown_sec, cooldown_upgraded_sec, slots, acquisition, upgrade_quest, upgrade_effect, sort_order, icon_name, icon_color, icon_url, icon_name_plus, icon_color_plus, icon_url_plus, language FROM mhw_mantles WHERE id = ?1",
            params![id],
            |row| {
                Ok(MhwMantle {
                    id: row.get(0)?,
                    game_id: row.get(1)?,
                    name: row.get(2)?,
                    tool_type: row.get(3)?,
                    rarity: row.get(4)?,
                    description: row.get(5)?,
                    effect: row.get(6)?,
                    duration_sec: row.get(7)?,
                    cooldown_sec: row.get(8)?,
                    cooldown_upgraded_sec: row.get(9)?,
                    slots: row.get(10)?,
                    acquisition: row.get(11)?,
                    upgrade_quest: row.get(12)?,
                    upgrade_effect: row.get(13)?,
                    sort_order: row.get(14)?,
                    icon_name: row.get(15)?,
                    icon_color: row.get(16)?,
                    icon_url: row.get(17)?,
                    icon_name_plus: row.get(18)?,
                    icon_color_plus: row.get(19)?,
                    icon_url_plus: row.get(20)?,
                    language: row.get(21)?,
                })
            },
        )
        .optional()?;
    Ok(row)
}

pub fn get_palico_gadgets_by_game(conn: &Connection, game_id: i32) -> Result<Vec<PalicoGadget>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, gadget_type, tribe, description, effect, acquisition, sort_order, icon_name, icon_color, icon_url, language FROM palico_gadgets WHERE game_id = ?1 ORDER BY COALESCE(sort_order, id)",
    )?;
    let rows = stmt
        .query_map(params![game_id], |row| {
            Ok(PalicoGadget {
                id: row.get(0)?,
                game_id: row.get(1)?,
                name: row.get(2)?,
                gadget_type: row.get(3)?,
                tribe: row.get(4)?,
                description: row.get(5)?,
                effect: row.get(6)?,
                acquisition: row.get(7)?,
                sort_order: row.get(8)?,
                icon_name: row.get(9)?,
                icon_color: row.get(10)?,
                icon_url: row.get(11)?,
                language: row.get(12)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(rows)
}

pub fn get_palico_gadget_detail(conn: &Connection, id: i32) -> Result<Option<PalicoGadgetDetail>> {
    let base: Option<(i32, i32, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, String)> = conn
        .query_row(
            "SELECT id, game_id, name, gadget_type, tribe, description, effect, acquisition, sort_order, icon_name, icon_color, icon_url, language FROM palico_gadgets WHERE id = ?1",
            params![id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?, r.get(10)?, r.get(11)?, r.get(12)?)),
        )
        .optional()?;
    let Some((
        id,
        game_id,
        name,
        gadget_type,
        tribe,
        description,
        effect,
        acquisition,
        sort_order,
        icon_name,
        icon_color,
        icon_url,
        language,
    )) = base
    else {
        return Ok(None);
    };
    let mut stmt = conn.prepare(
        "SELECT id, proficiency, ability_name, description, unlock_condition FROM palico_gadget_levels WHERE gadget_id = ?1 ORDER BY proficiency",
    )?;
    let levels = stmt
        .query_map(params![id], |row| {
            Ok(PalicoGadgetLevel {
                id: row.get(0)?,
                proficiency: row.get(1)?,
                ability_name: row.get(2)?,
                description: row.get(3)?,
                unlock_condition: row.get(4)?,
            })
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("[queries] row decode skipped: {}", e))
                .ok()
        })
        .collect();
    Ok(Some(PalicoGadgetDetail {
        id,
        game_id,
        name,
        gadget_type,
        tribe,
        description,
        effect,
        acquisition,
        sort_order,
        icon_name,
        icon_color,
        icon_url,
        language,
        levels,
    }))
}

/// Global accent-insensitive search across all MH2G entities, grouped by kind.
// Escape LIKE metacharacters so a user query can't act as a wildcard.
fn escape_like(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// Build a filtered SELECT that pushes the substring match into SQLite via the
/// registered `norm_key` scalar function, so only matching rows cross the
/// Rust boundary instead of every row in the table.
fn search_filter_sql(
    table: &str,
    subtitle_cols: &str,
    extra_where: &str,
    tokens: &[&str],
) -> (String, Vec<String>) {
    let likes: Vec<String> = tokens
        .iter()
        .map(|t| format!("%{}%", escape_like(t)))
        .collect();
    let likes_sql = tokens
        .iter()
        .map(|_| "norm_key(name) LIKE ? ESCAPE '\\'")
        .collect::<Vec<_>>()
        .join(" AND ");
    let sql = format!(
        "SELECT id, name, {} FROM {} WHERE game_id = ?1 {} AND {} ORDER BY id",
        subtitle_cols, table, extra_where, likes_sql
    );
    (sql, likes)
}

fn pluck_search(
    conn: &Connection,
    sql: &str,
    game_id: i32,
    likes: &[String],
) -> Result<Vec<(i32, String, String)>> {
    let mut stmt = conn.prepare(sql)?;
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(game_id)];
    for l in likes {
        params.push(Box::new(l.clone()));
    }
    let rows = stmt.query_map(
        rusqlite::params_from_iter(params.iter().map(|b| b.as_ref())),
        |r| {
            Ok((
                r.get::<_, i32>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        },
    )?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

// A per-table descriptor so the 8 search blocks share one loop.
struct SearchTable<'a> {
    table: &'a str,
    kind: &'a str,
    subtitle_cols: &'a str,
    extra_where: &'a str,
    route_prefix: &'a str,
    fallback_subtitle: &'a str,
}

pub fn get_global_search(
    conn: &Connection,
    game_id: i32,
    query: &str,
) -> Result<Vec<SearchResult>> {
    let q = norm_key(query);
    let tokens: Vec<&str> = q.split(' ').filter(|t| !t.is_empty()).collect();
    if tokens.is_empty() {
        return Ok(Vec::new());
    }
    let per_kind = 6;
    let mut out: Vec<SearchResult> = Vec::new();

    const TABLES: &[SearchTable] = &[
        SearchTable {
            table: "monsters",
            kind: "monster",
            subtitle_cols: "COALESCE(species,'')",
            extra_where: "",
            route_prefix: "/monsters/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "items",
            kind: "item",
            subtitle_cols: "COALESCE(category,'')",
            extra_where: "AND id != 1",
            route_prefix: "/items/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "skills",
            kind: "skill",
            subtitle_cols: "'Skill'",
            extra_where: "",
            route_prefix: "/skills/",
            fallback_subtitle: "Skill",
        },
        SearchTable {
            table: "weapons",
            kind: "weapon",
            subtitle_cols: "weapon_type",
            extra_where: "",
            route_prefix: "/weapons/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "armor",
            kind: "armor",
            subtitle_cols: "slot_type || ' · ' || rank",
            extra_where: "",
            route_prefix: "/armor/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "armor_sets",
            kind: "armor_set",
            subtitle_cols: "'Armor Set'",
            extra_where: "",
            route_prefix: "/armor/sets/",
            fallback_subtitle: "Armor Set",
        },
        SearchTable {
            table: "quests",
            kind: "quest",
            subtitle_cols: "COALESCE(rank,'')",
            extra_where: "",
            route_prefix: "/quests/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "decorations",
            kind: "decoration",
            subtitle_cols:
                "CASE WHEN slot_size IS NULL THEN '' ELSE CAST(slot_size AS TEXT) || ' slot' END",
            extra_where: "",
            route_prefix: "/decorations/",
            fallback_subtitle: "",
        },
        SearchTable {
            table: "mhw_mantles",
            kind: "mantle",
            subtitle_cols: "tool_type",
            extra_where: "",
            route_prefix: "/tools/mantles/",
            fallback_subtitle: "Mantle",
        },
        SearchTable {
            table: "palico_gadgets",
            kind: "palico_gadget",
            subtitle_cols: "COALESCE(tribe,'')",
            extra_where: "",
            route_prefix: "/tools/palico/",
            fallback_subtitle: "Palico",
        },
    ];

    for t in TABLES {
        let (sql, likes) = search_filter_sql(t.table, t.subtitle_cols, t.extra_where, &tokens);
        let rows = pluck_search(conn, &sql, game_id, &likes)?;
        let mut local: Vec<(i32, String, String, i32)> = rows
            .into_iter()
            .map(|(id, name, sub)| {
                let key = norm_key(&name);
                (score_match(&key, &tokens), name, sub, id)
            })
            .collect();
        local.sort_by(|a, b| b.0.cmp(&a.0));
        for (_sc, name, sub, id) in local.into_iter().take(per_kind) {
            let subtitle = if sub.is_empty() {
                t.fallback_subtitle.to_string()
            } else {
                sub
            };
            out.push(SearchResult {
                kind: t.kind.into(),
                id,
                name,
                subtitle,
                route: format!("{}{}", t.route_prefix, id),
            });
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn() -> rusqlite::Connection {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::register_functions(&c).unwrap();
        crate::db::schema::create_tables(&c).unwrap();
        crate::db::seed::seed(&c).unwrap();
        c
    }

    #[test]
    fn global_search_returns_matches_across_kinds() {
        let c = conn();
        // "attack" is a skill; "slash" or a monster name should also appear.
        let results = get_global_search(&c, 5, "attack").unwrap();
        assert!(
            !results.is_empty(),
            "global search for 'attack' returned nothing"
        );
        assert!(
            results
                .iter()
                .any(|r| r.kind == "skill" || r.kind == "item" || r.kind == "weapon"),
            "expected at least a skill/item/weapon result, got {:?}",
            results.iter().map(|r| r.kind.as_str()).collect::<Vec<_>>()
        );
        for r in &results {
            assert!(r.route.starts_with('/'), "bad route: {}", r.route);
            assert!(!r.name.is_empty(), "empty name in result");
        }
    }

    #[test]
    fn global_search_handles_multi_token_and_empty() {
        let c = conn();
        assert!(
            get_global_search(&c, 5, "   ").unwrap().is_empty(),
            "whitespace query should be empty"
        );

        let single = get_global_search(&c, 5, "attack").unwrap();
        let multi = get_global_search(&c, 5, "dragon attack").unwrap();
        // multi-token is a stricter (AND) filter, so it can never return more than single.
        assert!(multi.len() <= single.len(), "AND filter must be a subset");

        // '%' must be treated literally (escaped), not as a wildcard.
        let wild = get_global_search(&c, 5, "%%%").unwrap();
        assert!(wild.is_empty(), "percent query should not match everything");
    }

    #[test]
    fn migrating_dirty_db_dedups_before_creating_unique_indexes() {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        // Simulate a pre-fix DB: no unique index and duplicate rows.
        c.execute_batch("CREATE TABLE monster_equipment (id INTEGER PRIMARY KEY, game_id INTEGER, monster_id INTEGER, equipment_kind TEXT NOT NULL, equipment_id INTEGER NOT NULL);").unwrap();
        c.execute_batch(
            "INSERT INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id) VALUES \
             (5,1,'x',10),(5,1,'x',10),(5,1,'x',11),(5,2,'y',20),(5,2,'y',20);",
        )
        .unwrap();

        crate::db::schema::create_tables(&c).unwrap();

        let count: i64 = c
            .query_row("SELECT COUNT(*) FROM monster_equipment", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 3, "duplicates should collapse to 3 unique rows");

        // The index now exists; inserting a duplicate must be ignored, not error.
        crate::db::seed::seed(&c).unwrap();
    }

    #[test]
    fn database_new_migrates_dirty_existing_db() {
        // Reproduces the exact failing startup path from a pre-fix database file.
        let path = std::env::temp_dir().join(format!("mh_migrate_test_{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);

        {
            let c = rusqlite::Connection::open(&path).unwrap();
            c.execute_batch("CREATE TABLE monster_equipment (id INTEGER PRIMARY KEY, game_id INTEGER, monster_id INTEGER, equipment_kind TEXT NOT NULL, equipment_id INTEGER NOT NULL);").unwrap();
            c.execute_batch("INSERT INTO monster_equipment (game_id, monster_id, equipment_kind, equipment_id) VALUES (5,1,'x',10),(5,1,'x',10),(5,2,'y',20),(5,2,'y',20);").unwrap();
        }

        let result = crate::db::Database::new(path.to_str().unwrap());
        assert!(
            result.is_ok(),
            "Database::new must migrate a dirty DB instead of panicking: {:?}",
            result.err()
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn seed_is_idempotent_and_non_destructive() {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::register_functions(&c).unwrap();
        crate::db::schema::create_tables(&c).unwrap();
        crate::db::seed::seed(&c).unwrap();

        let tables = [
            "monsters",
            "items",
            "weapons",
            "armor",
            "armor_sets",
            "skills",
            "decorations",
            "quests",
            "item_combine",
            "monster_drops",
            "item_sources",
            "decoration_materials",
        ];
        let count = |c: &rusqlite::Connection, t: &str| -> i64 {
            c.query_row(&format!("SELECT COUNT(*) FROM {t}"), [], |r| r.get(0))
                .unwrap()
        };
        let first: Vec<i64> = tables.iter().map(|t| count(&c, t)).collect();

        // Second boot: seed runs again over the SAME database.
        crate::db::seed::seed(&c).unwrap();

        for (i, t) in tables.iter().enumerate() {
            let after = count(&c, t);
            assert_eq!(
                first[i], after,
                "table '{}' changed size on re-seed: {} -> {} (idempotency busted)",
                t, first[i], after
            );
        }
    }

    #[test]
    fn seed_records_data_version_and_skips_when_current() {
        // Boot gate: the first seed stamps schema_version; repeat boots
        // must skip the seed body (warm boots stay instant) while leaving
        // all rows untouched.
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::register_functions(&c).unwrap();
        crate::db::schema::create_tables(&c).unwrap();
        assert_eq!(crate::db::schema::get_schema_version(&c).unwrap(), 0);
        crate::db::seed::seed(&c).unwrap();
        assert_eq!(
            crate::db::schema::get_schema_version(&c).unwrap(),
            crate::db::seed::DATA_VERSION
        );
        let weapons: i64 = c
            .query_row("SELECT COUNT(*) FROM weapons", [], |r| r.get(0))
            .unwrap();
        assert!(weapons > 10000, "seed must populate all games");
        crate::db::seed::seed(&c).unwrap();
        let again: i64 = c
            .query_row("SELECT COUNT(*) FROM weapons", [], |r| r.get(0))
            .unwrap();
        assert_eq!(weapons, again, "re-seed of a current DB must be a no-op");
    }

    #[test]
    fn mhw_quests_survive_cross_game_id_ranges() {
        // Regression: MHWorldData canonical ids (101..67841) overlap MH2G quest ids
        // (1..610) on the single-column PK; the seed offsets MHW quest ids (+100000)
        // so INSERT OR IGNORE drops nothing (258 low_high, not 191).
        let c = conn();
        let json: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../../data/mhw_quests.json")).unwrap();

        let db_count: i64 = c
            .query_row("SELECT COUNT(*) FROM quests WHERE game_id = 1", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(
            db_count,
            json.len() as i64,
            "MHW quest count must match JSON (rows were PK-dropped)"
        );

        for (hub, category) in [("low_high", None), ("low_high", Some("assigned"))] {
            let (sql, params): (String, Vec<&str>) = match category {
                None => (
                    "SELECT COUNT(*) FROM quests WHERE game_id = 1 AND hub = 'low_high'"
                        .to_string(),
                    vec![],
                ),
                Some(cat) => (
                    "SELECT COUNT(*) FROM quests WHERE game_id = 1 AND hub = 'low_high' AND category = ?1"
                        .to_string(),
                    vec![cat],
                ),
            };
            let db_n: i64 = c
                .query_row(&sql, rusqlite::params_from_iter(params), |r| r.get(0))
                .unwrap();
            let expected = json
                .iter()
                .filter(|q| {
                    q["hub"] == hub && category.map(|cat| q["category"] == cat).unwrap_or(true)
                })
                .count();
            assert_eq!(
                db_n, expected as i64,
                "MHW {hub}/{category:?} lost rows to PK collision"
            );
        }

        // A quest whose canonical id collides with MH2G must exist under the offset.
        let colliding = json
            .iter()
            .find(|q| q["id"].as_i64().unwrap() < 611)
            .expect("JSON should contain a colliding id");
        let offset_id = colliding["id"].as_i64().unwrap() + 100_000;
        let name: String = c
            .query_row(
                "SELECT name FROM quests WHERE game_id = 1 AND id = ?1",
                [offset_id],
                |r| r.get(0),
            )
            .expect("colliding quest missing under offset id");
        assert_eq!(name, colliding["name"].as_str().unwrap());

        // No id may exist in two games at once.
        let overlap: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM quests q1 JOIN quests q2 ON q1.id = q2.id AND q1.game_id < q2.game_id",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(overlap, 0, "cross-game quest id overlap");

        // Every MHW reward must point at an MHW quest (no orphans on other games).
        let orphans: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM quest_rewards qr LEFT JOIN quests q ON q.id = qr.quest_id AND q.game_id = 1 WHERE qr.id BETWEEN 800001 AND 899999 AND q.id IS NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(orphans, 0, "MHW rewards pointing outside MHW quests");
    }

    #[test]
    fn pre_offset_db_migrates_mhw_quest_ids() {
        // Simulates a DB seeded before the offset: MH2G owns id 150 and an MHW
        // reward for the dropped quest 150 hangs off the MH2G row. Full seed must
        // heal it: MH2G row untouched, MHW quest at 100150, reward repointed.
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::register_functions(&c).unwrap();
        crate::db::schema::create_tables(&c).unwrap();
        c.execute_batch(
            "INSERT INTO games (id, name, abbreviation) VALUES (1, 'MHW', 'MHW'), (5, 'MH2G', 'MH2G');",
        )
        .unwrap();
        // Use a REAL colliding id from the MHW JSON (canonical ids are sparse).
        let mhw_json: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../../data/mhw_quests.json")).unwrap();
        let cid = mhw_json
            .iter()
            .find(|q| q["id"].as_i64().unwrap() < 611)
            .expect("JSON should contain a colliding id")["id"]
            .as_i64()
            .unwrap();
        c.execute(
            "INSERT INTO quests (id, game_id, name, hub, language) VALUES (?1, 5, 'MH2G Q', 'elder', 'en')",
            [cid],
        )
        .unwrap();
        c.execute_batch("INSERT INTO items (id, game_id, name) VALUES (20001, 1, 'stub');")
            .unwrap();
        c.execute(
            "INSERT INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition) VALUES (800001, ?1, 20001, 1, 1.0, 'A')",
            [cid],
        )
        .unwrap();

        crate::db::seed::seed(&c).unwrap();

        let mh2g_name: String = c
            .query_row(
                "SELECT name FROM quests WHERE game_id = 5 AND id = ?1",
                [cid],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(mh2g_name, "MH2G Q", "MH2G row must survive migration");
        let mhw_count: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM quests WHERE game_id = 1 AND id = ?1",
                [cid + 100_000],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            mhw_count, 1,
            "colliding MHW quest must be re-inserted under offset"
        );
        let reward_target: i64 = c
            .query_row(
                "SELECT quest_id FROM quest_rewards WHERE id = 800001",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            reward_target,
            cid + 100_000,
            "MHW reward must be repointed to the offset quest"
        );

        // Second boot: migration guards must make this a no-op.
        crate::db::seed::seed(&c).unwrap();
        let reward_target_again: i64 = c
            .query_row(
                "SELECT quest_id FROM quest_rewards WHERE id = 800001",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            reward_target_again,
            cid + 100_000,
            "migration must be idempotent"
        );
    }

    #[test]
    fn seed_inside_transaction_with_fk_on_like_production() {
        // Mirrors db/mod.rs: PRAGMA foreign_keys = ON, BEGIN IMMEDIATE, seed, COMMIT.
        // The pre-offset migration must survive this path (deferred FK checks);
        // without deferral the re-id UPDATEs fail exactly like the startup crash.
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::register_functions(&c).unwrap();
        crate::db::schema::create_tables(&c).unwrap();
        c.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
        c.execute_batch(
            "INSERT INTO games (id, name, abbreviation) VALUES (1, 'MHW', 'MHW'), (5, 'MH2G', 'MH2G');",
        )
        .unwrap();
        let mhw_json: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../../data/mhw_quests.json")).unwrap();
        let cid = mhw_json
            .iter()
            .find(|q| q["id"].as_i64().unwrap() < 611)
            .expect("JSON should contain a colliding id")["id"]
            .as_i64()
            .unwrap();
        c.execute(
            "INSERT INTO quests (id, game_id, name, hub, language) VALUES (?1, 5, 'MH2G Q', 'elder', 'en')",
            [cid],
        )
        .unwrap();
        c.execute_batch("INSERT INTO items (id, game_id, name) VALUES (20001, 1, 'stub');")
            .unwrap();
        c.execute(
            "INSERT INTO quest_rewards (id, quest_id, item_id, quantity, probability, condition) VALUES (800001, ?1, 20001, 1, 1.0, 'A')",
            [cid],
        )
        .unwrap();

        c.execute_batch("BEGIN IMMEDIATE;").unwrap();
        crate::db::seed::seed(&c).unwrap();
        c.execute_batch("COMMIT;").unwrap();

        let mhw_count: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM quests WHERE game_id = 1 AND id = ?1",
                [cid + 100_000],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            mhw_count, 1,
            "MHW quest must exist under offset after COMMIT"
        );
        let reward_target: i64 = c
            .query_row(
                "SELECT quest_id FROM quest_rewards WHERE id = 800001",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            reward_target,
            cid + 100_000,
            "reward must be repointed after COMMIT"
        );
        let total: i64 = c
            .query_row("SELECT COUNT(*) FROM quests WHERE game_id = 1", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(total, mhw_json.len() as i64, "full MHW set present");
    }

    #[test]
    fn small_monster_sources_are_visible_without_duplicates() {
        // SDD 001 T3-T4: small-monster carve/drop/capture rows live in item_sources
        // with source_id NULL (location like 'Conga - carving'); the method-type
        // exclusion in get_item_sources must not hide them, while mirrored
        // monster_drops rows (source_id NOT NULL) must stay excluded.
        let c = conn();
        // MHP3rd Potion (10009): shop row + small-monster drop rows, zero monster_drops.
        let potion = get_item_sources(&c, 10009).unwrap();
        assert!(
            potion.iter().any(|s| s.source_type == "shop"),
            "potion shop row missing"
        );
        let small: Vec<_> = potion
            .iter()
            .filter(|s| {
                ["carve", "capture", "drop", "break"].contains(&s.source_type.as_str())
                    && s.source_id.is_none()
            })
            .collect();
        assert!(
            !small.is_empty(),
            "small-monster rows (source_id NULL) must be visible"
        );
        assert!(
            small.iter().all(|s| s.location.is_some()),
            "small-monster rows carry location text"
        );
        // Potion has no monster_drops, so any source_id-bearing method row is a leak.
        assert!(
            potion
                .iter()
                .filter(|s| {
                    ["carve", "capture", "drop", "break", "quest_reward"]
                        .contains(&s.source_type.as_str())
                        && s.source_id.is_some()
                })
                .count()
                == 0,
            "mirrored junction rows must stay excluded"
        );

        // Item with real large-monster drops (10385 = break @10021): break rows
        // must come only from the monster_drops branch (no junction dupes).
        let db_breaks: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM monster_drops WHERE item_id = 10385 AND method = 'break'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(db_breaks > 0, "fixture needs a large-monster break row");
        let shown = get_item_sources(&c, 10385)
            .unwrap()
            .iter()
            .filter(|s| s.source_type == "break")
            .count() as i64;
        assert_eq!(
            shown, db_breaks,
            "break rows must come only from monster_drops (no junction dupes)"
        );
    }

    #[test]
    fn mhw_002b_coverage_decorations_weaknesses_skills_equipment() {
        // 002b T9: MHW gaps closed — decorations + per-part weaknesses +
        // weapon skill points + derived monster equipment.
        let c = conn();
        let decos: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM decorations WHERE game_id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(decos >= 300, "expected 300+ MHW decorations, got {}", decos);
        // No decoration may carry a NULL skill FK.
        let null_skill: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM decorations WHERE game_id = 1 AND skill_id IS NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(null_skill, 0, "decorations must always resolve a skill");
        let wmons: i64 = c
            .query_row(
                "SELECT COUNT(DISTINCT mw.monster_id) FROM monster_weaknesses mw
                 JOIN monsters m ON m.id = mw.monster_id AND m.game_id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            wmons >= 80,
            "expected 80+ MHW monsters with weaknesses, got {}",
            wmons
        );
        let wsp: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM weapon_skill_points wsp
                 JOIN weapons w ON w.id = wsp.weapon_id AND w.game_id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(wsp > 0, "expected MHW weapon_skill_points, got 0");
        let eq_armor: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM monster_equipment WHERE game_id = 1 AND equipment_kind = 'armor'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let eq_weapon: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM monster_equipment WHERE game_id = 1 AND equipment_kind = 'weapon'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            eq_armor > 1000,
            "expected 1000+ MHW armor links, got {}",
            eq_armor
        );
        assert!(
            eq_weapon > 1000,
            "expected 1000+ MHW weapon links, got {}",
            eq_weapon
        );
        // List + detail smoke through the public queries.
        let list = get_decorations_by_game(&c, 1).unwrap();
        assert!(
            !list.is_empty(),
            "get_decorations_by_game(1) must not be empty"
        );
        let gather: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM item_sources s
                 JOIN items i ON i.id = s.item_id AND i.game_id = 1
                 WHERE s.source_type = 'gather'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            gather >= 800,
            "expected 800+ MHW gather rows, got {}",
            gather
        );
        let mid: i32 = c
            .query_row(
                "SELECT monster_id FROM monster_weaknesses GROUP BY monster_id
                 ORDER BY COUNT(*) DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("monster detail");
        assert!(
            !detail.weaknesses.is_empty(),
            "MHW monster detail must show weaknesses"
        );
        assert!(
            !detail.armor.is_empty() || !detail.weapons.is_empty(),
            "MHW monster detail must show derived equipment"
        );
    }

    #[test]
    fn mhw_010_phase_a_ailments_tools_charms_roster_bonuses() {
        // 010 Phase A: MHWorldData zero-scrape wins — hunter charms rank tree +
        // quest roster + set bonus completion + phantom delete + stat-generated
        // descriptions. (Ailment/tool exact counts live in the Phase B test: the
        // Phase B JSON supersedes Phase A with Kiranico numerics.)
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        // Hunter charms rank tree: 317 charms, 1247 materials, 207 linked previous.
        assert_eq!(
            count("SELECT COUNT(*) FROM hunter_charms WHERE game_id = 1"),
            317
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM hunter_charm_materials WHERE charm_id IN (SELECT id FROM hunter_charms WHERE game_id = 1)"),
            1247
        );
        assert_eq!(
            count(
                "SELECT COUNT(*) FROM hunter_charms WHERE game_id = 1 AND previous_id IS NOT NULL"
            ),
            207
        );
        // No charm material may dangle (seed never FK NULL).
        assert_eq!(
            count("SELECT COUNT(*) FROM hunter_charm_materials WHERE item_id NOT IN (SELECT id FROM items)"),
            0
        );
        // Query smoke through the public API.
        let charms = get_hunter_charms_by_game(&c, 1).unwrap();
        assert_eq!(charms.len(), 317);
        assert!(charms
            .iter()
            .any(|ch| !ch.materials.is_empty() && ch.previous_name.is_some()));
        // Quest roster: 517/521 with monsters (4 gathering quests correctly empty).
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 1 AND main_monsters IS NOT NULL AND main_monsters != '[]'"),
            517
        );
        // Set bonuses: skill1 preserved everywhere upstream has one; 58 skill2 fills.
        assert_eq!(
            count("SELECT COUNT(*) FROM armor_sets WHERE game_id = 1 AND bonus_skill2 IS NOT NULL"),
            58
        );
        // Phantom delete: Uragaan + Fulgurbug capture row must not exist.
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 1078 AND item_id = 20927 AND method = 'capture'"),
            0
        );
        // Stat-generated descriptions land on every weapon + armor piece.
        assert_eq!(
            count("SELECT COUNT(*) FROM weapons WHERE game_id = 1 AND description IS NOT NULL AND description != ''"),
            3544
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM armor WHERE game_id = 1 AND description IS NOT NULL AND description != ''"),
            1595
        );
        // Monster detail smoke: ailments + tools render for a covered monster.
        let mid: i32 = c
            .query_row(
                "SELECT monster_id FROM monster_ailments WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1) LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("monster detail");
        assert!(
            !detail.ailments.is_empty(),
            "MHW monster detail must show ailments"
        );
        assert!(
            !detail.tools.is_empty(),
            "MHW monster detail must show traps"
        );
    }

    #[test]
    fn mhw_010_phase_b_kiranico_numbers_melder_deco_shop() {
        // 010 Phase B: Kiranico buildup/decay numbers + trap durations/chains +
        // part-break stagger proxy + melder sources + provisions + deco acquisition.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let mhw = "monster_id IN (SELECT id FROM monsters WHERE game_id = 1)";
        // Ailments: 721 rows (66 Phase A + Kiranico fills/new incl. 3 late larges
        // + smalls); 501 with buildup numerics; no Blastblight leftovers.
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_ailments WHERE {mhw}"
            )),
            721
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_ailments WHERE {mhw} AND initial IS NOT NULL"
            )),
            501
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_ailments WHERE {mhw} AND ailment = 'Blastblight'"
            )),
            0
        );
        // Traps: 465 rows across 5 tools (Flash Bomb + Dung added); 386 with
        // seconds; 186 diminishing chains in notes.
        assert_eq!(
            count(&format!("SELECT COUNT(*) FROM monster_tools WHERE {mhw}")),
            465
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(DISTINCT tool) FROM monster_tools WHERE {mhw}"
            )),
            5
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_tools WHERE {mhw} AND normal IS NOT NULL"
            )),
            386
        );
        // Stagger proxy: 317 hitzone rows carry Part-Break values.
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_weaknesses WHERE {mhw} AND stagger_hp IS NOT NULL"
            )),
            317
        );
        // Melder sources: 212 Kiranico meld rows (RP/MP in conditions).
        assert_eq!(
            count("SELECT COUNT(*) FROM item_sources WHERE source_type = 'melder' AND location = 'Elder Melder' AND conditions LIKE '%Research Points%'"),
            212
        );
        // Shop: 56 provisions rows; buy prices present for all of them.
        assert_eq!(
            count("SELECT COUNT(*) FROM item_sources WHERE source_type = 'shop' AND item_id IN (SELECT id FROM items WHERE game_id = 1)"),
            56
        );
        assert!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 1 AND buy_price IS NOT NULL AND buy_price > 0") >= 55
        );
        // Deco acquisition: 398/404 (5 event jewels have no feystone/meld row).
        assert_eq!(
            count("SELECT COUNT(*) FROM decorations WHERE game_id = 1 AND acquisition IS NOT NULL"),
            398
        );
        // Stored acquisition wins over the smith-derived fallback in detail.
        let jewel: String = c
            .query_row(
                "SELECT acquisition FROM decorations WHERE game_id = 1 AND name = 'Defense Jewel 1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            jewel.contains("Elder Melder"),
            "meldable jewel must show melder cost: {jewel}"
        );
        // Detail smoke: tools carry durations + chains for a Kiranico monster.
        let mid: i32 = c
            .query_row(
                &format!("SELECT monster_id FROM monster_tools WHERE {mhw} AND notes IS NOT NULL LIMIT 1"),
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("monster detail");
        assert!(detail
            .tools
            .iter()
            .any(|t| t.normal.is_some() && t.notes.is_some()));
        assert!(detail.ailments.iter().any(|a| a.initial.is_some()));
    }

    #[test]
    fn mhw_011_set_bonus_display() {
        // 010 follow-up: set bonuses surfaced on set detail + list + piece
        // detail + skill reverse refs (MHW only; other games stay NULL).
        let c = conn();
        // Anja (20019): single bonus Adrenaline 3, skill id resolves.
        let anja = get_armor_set_detail(&c, 20019).unwrap().expect("anja set");
        assert_eq!(anja.bonus_skill.as_deref(), Some("Adrenaline"));
        assert_eq!(anja.bonus_required, Some(3));
        assert_eq!(anja.bonus_skill_id, Some(20111));
        assert_eq!(anja.bonus_skill2, None);
        // Anja Alpha (20074): two bonuses with resolved skill ids.
        let alpha = get_armor_set_detail(&c, 20074)
            .unwrap()
            .expect("anja alpha");
        assert_eq!(alpha.bonus_skill.as_deref(), Some("Adrenaline"));
        assert_eq!(alpha.bonus_skill_id, Some(20111));
        assert_eq!(alpha.bonus_skill2.as_deref(), Some("Stamina Cap Up"));
        assert_eq!(alpha.bonus_required2, Some(4));
        assert!(alpha.bonus_skill2_id.is_some());
        // Leather (20001): no bonus — section stays hidden.
        let leather = get_armor_set_detail(&c, 20001).unwrap().expect("leather");
        assert_eq!(leather.bonus_skill, None);
        assert_eq!(leather.bonus_skill2, None);
        // Other games: NULL bonuses (mh2g set sampled live, no hardcoded id).
        let old_set: i32 = c
            .query_row(
                "SELECT id FROM armor_sets WHERE game_id = 5 LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let old = get_armor_set_detail(&c, old_set)
            .unwrap()
            .expect("mh2g set");
        assert_eq!(old.bonus_skill, None);
        assert_eq!(old.bonus_skill2, None);
        // Reverse lookup: Adrenaline granted by Anja (3 pcs).
        let refs = get_skill_set_bonuses(&c, 20111).unwrap();
        let anja_ref = refs.iter().find(|r| r.set_id == 20019).expect("anja ref");
        assert_eq!(anja_ref.required_pieces, 3);
        // Piece detail carries the parent bonus (Anja Helm 40084).
        let helm = get_armor_detail(&c, 40084).unwrap().expect("anja helm");
        assert_eq!(helm.set_bonus_skill.as_deref(), Some("Adrenaline"));
        assert_eq!(helm.set_bonus_required, Some(3));
        // Skill detail carries set-bonus refs.
        let skill = get_skill_detail(&c, 20111).unwrap().expect("adrenaline");
        assert!(skill.set_bonuses.iter().any(|r| r.set_id == 20019));
        // List query exposes bonuses for cards.
        let sets = get_armor_sets_by_game(&c, 1).unwrap();
        let anja_list = sets.iter().find(|s| s.id == 20019).expect("anja list");
        assert_eq!(anja_list.bonus_skill.as_deref(), Some("Adrenaline"));
    }

    #[test]
    fn mhw_010_phase_c_fandom_gather_descriptions_combine() {
        // 010 Phase C: Fandom How-To-Get gather rows + 4 monster descriptions +
        // combine arbiter (wiki 98 fully covered, 4 legitimate local extras).
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let g1 = "item_id IN (SELECT id FROM items WHERE game_id = 1)";
        // Hoarfrost Reach + Guiding Lands gather/mining/fishing: 75 rows.
        assert_eq!(
            count(&format!("SELECT COUNT(*) FROM item_sources WHERE {g1} AND location IN ('Hoarfrost Reach', 'Guiding Lands', 'Hoarfrost Reach / Guiding Lands')")),
            75
        );
        // Region conditions present (Forest/Wildspire/Coral/Rotten/Volcanic/Tundra).
        assert!(
            count(&format!(
                "SELECT COUNT(*) FROM item_sources WHERE {g1} AND conditions LIKE '%Region%'"
            )) >= 20
        );
        // Monster descriptions: 94/94 (Fatalis/Gajalaka/Anteka/Boaboa from Fandom).
        assert_eq!(
            count("SELECT COUNT(*) FROM monsters WHERE game_id = 1 AND description IS NOT NULL AND description != ''"),
            94
        );
        // Combine arbiter: wiki 98 fully covered by local rows. DB holds 186 of
        // 188 JSON rows: the 2 truly-identical halves (Ancient Potion/Kelbi Horn,
        // Mega Barrel Bomb/Large Barrel Bomb - same values, zero info loss)
        // collapse under uq_item_combine2; the Gunpowder 20/40 yield variants
        // land as distinct rows.
        assert_eq!(
            count("SELECT COUNT(*) FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE game_id = 1)"),
            186
        );
        // Drop-loss fix: every JSON row lands (NULL rate = unknown, no silent
        // skips); 35 xeno rows pruned, 32 Kiranico-filled.
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1)"),
            5826
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1) AND probability IS NULL"),
            116
        );
    }

    #[test]
    fn mhwilds_004_coverage_lists_details_and_junctions() {
        // 004: Wilds core (MHDB API) — every list non-empty, junctions wired.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert!(count("SELECT COUNT(*) FROM monsters WHERE game_id = 3") >= 30);
        assert!(count("SELECT COUNT(*) FROM items WHERE game_id = 3") >= 700);
        assert!(count("SELECT COUNT(*) FROM weapons WHERE game_id = 3") >= 1000);
        assert!(count("SELECT COUNT(*) FROM armor WHERE game_id = 3") >= 700);
        assert!(count("SELECT COUNT(*) FROM armor_sets WHERE game_id = 3") >= 150);
        assert!(count("SELECT COUNT(*) FROM skills WHERE game_id = 3") >= 150);
        assert!(count("SELECT COUNT(*) FROM decorations WHERE game_id = 3") >= 300);
        assert!(count("SELECT COUNT(*) FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 3)") > 1000);
        assert!(count("SELECT COUNT(*) FROM weapon_materials WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 3)") > 2000);
        assert!(count("SELECT COUNT(*) FROM armor_materials WHERE armor_id IN (SELECT id FROM armor WHERE game_id = 3)") > 1500);
        assert!(count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 3)") >= 200);
        assert!(count("SELECT COUNT(*) FROM monster_equipment WHERE game_id = 3") > 1000);
        assert!(count("SELECT COUNT(*) FROM armor_skill_points WHERE armor_id IN (SELECT id FROM armor WHERE game_id = 3)") > 500);
        assert!(count("SELECT COUNT(*) FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE game_id = 3)") > 50);
        // Every monster/weapon/armor/skill/deco/item row must be reachable by id (no PK collision with other games).
        for table in [
            "monsters",
            "items",
            "weapons",
            "armor",
            "skills",
            "decorations",
        ] {
            let orphans: i64 = c
                .query_row(
                    &format!("SELECT COUNT(*) FROM {table} WHERE game_id = 3 AND id IS NULL"),
                    [],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(orphans, 0, "{table} has NULL ids");
        }
        // Detail smoke: monster with most weakness rows shows weaknesses + drops + equipment.
        let mid: i32 = c
            .query_row(
                "SELECT mw.monster_id FROM monster_weaknesses mw
                 JOIN monsters m ON m.id = mw.monster_id AND m.game_id = 3
                 GROUP BY mw.monster_id ORDER BY COUNT(*) DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("wilds monster detail");
        assert_eq!(detail.game_id, 3);
        assert!(!detail.weaknesses.is_empty());
        assert!(!detail.drops.is_empty());
        assert!(!detail.armor.is_empty() || !detail.weapons.is_empty());
        // Weapon detail shows materials; item detail shows sources.
        let wid: i32 = c
            .query_row(
                "SELECT weapon_id FROM weapon_materials GROUP BY weapon_id ORDER BY COUNT(*) DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let wdetail = get_weapon_detail(&c, wid)
            .unwrap()
            .expect("wilds weapon detail");
        assert!(
            !wdetail.materials.is_empty()
                || !wdetail.forge_materials.is_empty()
                || !wdetail.upgrade_materials.is_empty()
        );
        assert!(!get_decorations_by_game(&c, 3).unwrap().is_empty());
        assert!(!get_combinations_by_game(&c, 3).unwrap().is_empty());
    }

    #[test]
    fn mhwilds_weapon_smith_order() {
        // Wilds sort_order is the Kiranico Smith DFS sequence
        // (scripts/reorder_mhwilds_weapons.py), not rarity+alphabetical.
        let c = conn();
        // Smith head: the Expedition/Hope line opens Great Sword.
        let head: Vec<String> = c
            .prepare(
                "SELECT name FROM weapons WHERE game_id = 3 AND weapon_type = 'Great Sword'
                 ORDER BY COALESCE(sort_order, id) LIMIT 8",
            )
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(
            head,
            vec![
                "Hope Blade I",
                "Hope Blade II",
                "Hope Blade III",
                "Hope Blade IV",
                "Hope Blade V",
                "Valkyrie Blade I",
                "Valkyrie Blade II",
                "Sieglinde",
            ]
        );
        // Tree-link integrity: every upgrade parent resolves to a same-type
        // weapon (the frontend builds trees by parent name), and every
        // declared branch reciprocates (child.previous == parent).
        // NOTE: no parent-before-child sort assert — Wilds lists forge lines
        // (e.g. Immane, Nihil) as their own Smith rows ahead of the Bone
        // line they upgrade from; cross-line branches legitimately sort
        // before their MHDB parent.
        let rows: Vec<(String, String, Option<String>)> = c
            .prepare("SELECT weapon_type, name, upgrade_path FROM weapons WHERE game_id = 3")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let known: std::collections::HashSet<(String, String)> = rows
            .iter()
            .map(|(t, n, _)| (t.clone(), n.clone()))
            .collect();
        let prev_of: std::collections::HashMap<(String, String), Option<String>> = rows
            .iter()
            .map(|(t, n, upath)| {
                let prev = upath.as_deref().and_then(|up| {
                    match serde_json::from_str::<serde_json::Value>(up) {
                        Ok(v) => v
                            .get("previous")
                            .and_then(|p| p.as_str())
                            .map(str::to_string),
                        Err(_) => Some(up.to_string()),
                    }
                });
                ((t.clone(), n.clone()), prev)
            })
            .collect();
        let mut orphans = Vec::new();
        let mut nonrecip = Vec::new();
        for (t, n, upath) in &rows {
            let Some(up) = upath.as_deref() else { continue };
            let parsed: Option<(Option<String>, Vec<String>)> =
                match serde_json::from_str::<serde_json::Value>(up) {
                    Ok(v) => Some((
                        v.get("previous")
                            .and_then(|p| p.as_str())
                            .map(str::to_string),
                        v.get("branches")
                            .and_then(|b| b.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|x| x.as_str().map(str::to_string))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    )),
                    Err(_) => Some((Some(up.to_string()), Vec::new())),
                };
            let (prev, branches) = parsed.unwrap();
            if let Some(p) = prev {
                if p != *n && !known.contains(&(t.clone(), p.clone())) {
                    orphans.push(format!("{t}: {p} -> {n}"));
                }
            }
            for b in branches {
                match prev_of.get(&(t.clone(), b.clone())) {
                    Some(Some(p)) if p == n => {}
                    _ => nonrecip.push(format!("{t}: {n} -> {b}")),
                }
            }
        }
        assert!(
            orphans.is_empty(),
            "Wilds upgrade parents must resolve: {orphans:?}"
        );
        assert!(
            nonrecip.is_empty(),
            "Wilds branches must reciprocate: {nonrecip:?}"
        );
    }

    #[test]
    fn mhr_003a_coverage_bulk_base_rise() {
        // 003 Phase A: bulk base-Rise (Badge87 + CrimsonNynja) — every list
        // non-empty, materials resolve, quests present, no PK collisions.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert!(count("SELECT COUNT(*) FROM monsters WHERE game_id = 2") >= 100);
        assert!(count("SELECT COUNT(*) FROM items WHERE game_id = 2") >= 900);
        assert!(count("SELECT COUNT(*) FROM weapons WHERE game_id = 2") >= 1800);
        assert!(count("SELECT COUNT(*) FROM armor WHERE game_id = 2") >= 600);
        assert!(count("SELECT COUNT(*) FROM armor_sets WHERE game_id = 2") >= 80);
        assert!(count("SELECT COUNT(*) FROM skills WHERE game_id = 2") >= 100);
        assert!(count("SELECT COUNT(*) FROM decorations WHERE game_id = 2") >= 90);
        assert!(count("SELECT COUNT(*) FROM quests WHERE game_id = 2") >= 300);
        assert!(count("SELECT COUNT(*) FROM weapon_materials WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 2)") > 5000);
        assert!(count("SELECT COUNT(*) FROM armor_materials WHERE armor_id IN (SELECT id FROM armor WHERE game_id = 2)") > 2000);
        assert!(count("SELECT COUNT(*) FROM armor_skill_points WHERE armor_id IN (SELECT id FROM armor WHERE game_id = 2)") > 500);
        // Every content row carries a distinct in-game id (offsets hold).
        let dupes: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM (SELECT id FROM monsters WHERE game_id = 2 GROUP BY id HAVING COUNT(*) > 1)",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(dupes, 0, "MHR monster ids must be unique");
        // Detail smoke: weapon with materials, armor with materials, quest, decoration.
        let wid: i32 = c
            .query_row(
                "SELECT weapon_id FROM weapon_materials GROUP BY weapon_id ORDER BY COUNT(*) DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let g2: i32 = c
            .query_row("SELECT game_id FROM weapons WHERE id = ?1", [wid], |r| {
                r.get(0)
            })
            .unwrap();
        if g2 == 2 {
            let wdetail = get_weapon_detail(&c, wid)
                .unwrap()
                .expect("mhr weapon detail");
            assert!(
                !wdetail.materials.is_empty()
                    || !wdetail.forge_materials.is_empty()
                    || !wdetail.upgrade_materials.is_empty()
            );
        }
        let mid: i32 = c
            .query_row(
                "SELECT id FROM monsters WHERE game_id = 2 LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let mdetail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("mhr monster detail");
        assert_eq!(mdetail.game_id, 2);
        assert!(!get_decorations_by_game(&c, 2).unwrap().is_empty());
    }

    #[test]
    fn mhr_003b_kiranico_monster_data() {
        // 003 Phase B (part 1): Kiranico hitzones + drops merged — weaknesses,
        // drops, sources and derived equipment are now non-empty for game 2.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert!(count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 2)") >= 500);
        assert!(count("SELECT COUNT(*) FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 2)") > 5000);
        assert!(count("SELECT COUNT(*) FROM item_sources s JOIN items i ON i.id = s.item_id AND i.game_id = 2") > 5000);
        assert!(count("SELECT COUNT(*) FROM monster_equipment WHERE game_id = 2") > 500);
        assert!(count("SELECT COUNT(*) FROM items WHERE game_id = 2") >= 1400);
        assert!(count("SELECT COUNT(*) FROM quests WHERE game_id = 2") >= 900);
        assert!(count("SELECT COUNT(*) FROM quest_rewards WHERE quest_id IN (SELECT id FROM quests WHERE game_id = 2)") > 5000);
        // Phase B (weapons/armor): full Sunbreak v16 trees with stats + materials.
        assert!(count("SELECT COUNT(*) FROM weapons WHERE game_id = 2") >= 3900);
        assert!(count("SELECT COUNT(*) FROM weapon_materials WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 2)") > 10000);
        assert!(count("SELECT COUNT(*) FROM armor WHERE game_id = 2") >= 1500);
        assert!(count("SELECT COUNT(*) FROM armor_sets WHERE game_id = 2") >= 300);
        assert!(count("SELECT COUNT(*) FROM skills WHERE game_id = 2") >= 140);
        assert!(count("SELECT COUNT(*) FROM decorations WHERE game_id = 2") >= 200);
        // Every drop references a real item (no orphans from the merge).
        let orphans: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM monster_drops md WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 2)
                 AND NOT EXISTS (SELECT 1 FROM items i WHERE i.id = md.item_id AND i.game_id = 2)",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(orphans, 0, "MHR drops must not reference missing items");
        // Detail smoke: monster shows weaknesses + drops + equipment.
        let mid: i32 = c
            .query_row(
                "SELECT mw.monster_id FROM monster_weaknesses mw
                 JOIN monsters m ON m.id = mw.monster_id AND m.game_id = 2
                 GROUP BY mw.monster_id ORDER BY COUNT(*) DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_monster_detail(&c, mid)
            .unwrap()
            .expect("mhr monster detail");
        assert!(!detail.weaknesses.is_empty());
        assert!(!detail.drops.is_empty());
        assert!(!detail.armor.is_empty() || !detail.weapons.is_empty());
    }

    #[test]
    fn mhr_011a_monster_status_partbreak_tools() {
        // 011 Phase A: Kiranico status buildup (112x10, zero nulls) +
        // part-break (72 monsters) + Fextra trap lists (54 monsters).
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let g2 = "monster_id IN (SELECT id FROM monsters WHERE game_id = 2)";
        assert_eq!(
            count(&format!("SELECT COUNT(*) FROM monster_ailments WHERE {g2}")),
            1120
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(DISTINCT monster_id) FROM monster_ailments WHERE {g2}"
            )),
            112
        );
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM monster_part_break WHERE {g2}"
            )),
            // 541 raw rows collapse to 468: Kiranico repeats identical
            // (part, hp) rows for multi-break parts (e.g. Aknosom Wing x2);
            // uq_monster_part_break keeps one (same part_hp either way).
            468
        );
        assert_eq!(
            count(&format!("SELECT COUNT(*) FROM monster_tools WHERE {g2}")),
            270
        );
        let mid = |name: &str| -> i32 {
            c.query_row(
                "SELECT id FROM monsters WHERE game_id = 2 AND name = ?1",
                [name],
                |r| r.get(0),
            )
            .unwrap()
        };
        // Spot: Magnamalo Poison has full numerics.
        let p: (Option<i32>, Option<i32>, Option<i32>, Option<i32>) = c
            .query_row(
                "SELECT initial, increase, max, duration_sec FROM monster_ailments
                 WHERE monster_id = ?1 AND ailment = 'Poison'",
                [mid("Magnamalo")],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .unwrap();
        assert!(p.0.is_some() && p.1.is_some() && p.2.is_some() && p.3.is_some());
        // Spot: Aknosom Shock Trap effective (NULL), Sonic Bomb immune (0).
        let shock: Option<i32> = c
            .query_row(
                "SELECT normal FROM monster_tools WHERE monster_id = ?1 AND tool = 'Shock Trap'",
                [mid("Aknosom")],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(shock, None);
        let sonic: Option<i32> = c
            .query_row(
                "SELECT normal FROM monster_tools WHERE monster_id = ?1 AND tool = 'Sonic Bomb'",
                [mid("Aknosom")],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(sonic, Some(0));
        // Detail exposes the new part-break rows.
        let detail = get_monster_detail(&c, mid("Aknosom"))
            .unwrap()
            .expect("aknosom detail");
        assert_eq!(detail.ailments.len(), 10);
        assert!(!detail.part_break.is_empty());
        assert!(detail.part_break.iter().all(|b| b.part_hp.is_some()));
    }

    #[test]
    fn mhr_011e_combine() {
        // 011 Phase E1: 178 (result, component) combine rows from the
        // Kiranico item-mix view, chance 100 (Rise crafting always succeeds).
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let g2_result = "result_item_id IN (SELECT id FROM items WHERE game_id = 2)";
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM item_combine WHERE {g2_result} AND combine_type = 'normal'"
            )),
            178
        );
        let chance_100: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM item_combine
                 WHERE result_item_id IN (SELECT id FROM items WHERE game_id = 2)
                   AND combine_type = 'normal' AND chance = 100",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(chance_100, 178);
        // Spot: Potion = Herb (well-known recipe).
        let potion: i32 = c
            .query_row(
                "SELECT id FROM items WHERE game_id = 2 AND name = 'Potion'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let herb: i32 = c
            .query_row(
                "SELECT id FROM items WHERE game_id = 2 AND name = 'Herb'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let link: Option<i32> = c
            .query_row(
                "SELECT quantity FROM item_combine
                 WHERE result_item_id = ?1 AND component_item_id = ?2 AND combine_type = 'normal'",
                [potion, herb],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(link, Some(1));
    }

    #[test]
    fn mhr_011d_talismans() {
        // 011 Phase D: 11849 generator talisman rows (4 base methods
        // anima/rebirth/reincarnation/wisp-of-mystery; Sunbreak eve/haze
        // not in upstream CSVs — gap), each with 1-2 charm_skills joined
        // to game-2 skills.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let g2 = "game_id = 2";
        assert_eq!(
            count(&format!(
                "SELECT COUNT(*) FROM charms WHERE {g2} AND charm_type = 'talisman'"
            )),
            11849
        );
        // charm_skills: each talisman has 1 or 2 (the s2 may resolve to
        // the same skill as s1; we INSERT OR IGNORE so dedupe is automatic).
        assert!(
            count("SELECT COUNT(*) FROM charm_skills
                   WHERE charm_id IN (SELECT id FROM charms WHERE game_id = 2 AND charm_type = 'talisman')")
                > 11849
        );
        // Skill join: Agitator 2 / Weakness Exploit 2 is a popular meta
        // combo — verify it exists as a charm.
        let cid: i32 = c
            .query_row(
                "SELECT c.id FROM charms c
                 JOIN charm_skills cs1 ON cs1.charm_id = c.id
                 JOIN skills s1 ON s1.id = cs1.skill_id AND s1.name = 'Agitator' AND cs1.points >= 2
                 JOIN charm_skills cs2 ON cs2.charm_id = c.id AND cs2.skill_id != cs1.skill_id
                 JOIN skills s2 ON s2.id = cs2.skill_id AND s2.name = 'Weakness Exploit' AND cs2.points >= 2
                 WHERE c.game_id = 2 AND c.charm_type = 'talisman'
                 LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        // Direct SELECT (LIMIT 500 in get_charms_by_game would cut this off).
        let mut stmt = c
            .prepare(
                "SELECT s.name FROM charm_skills cs
                 JOIN skills s ON s.id = cs.skill_id
                 WHERE cs.charm_id = ?1 ORDER BY s.name",
            )
            .unwrap();
        let names: Vec<String> = stmt
            .query_map([cid], |r| r.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(names.iter().any(|n| n == "Agitator"), "skills: {names:?}");
        assert!(
            names.iter().any(|n| n == "Weakness Exploit"),
            "skills: {names:?}"
        );
    }

    #[test]
    fn mhr_011c_rampage() {
        // 011 Phase C: 247 unique rampage skills (32 Kiranico dupes —
        // same name on two weapon-type pages — collapsed server-side) +
        // 6822 (skill, weapon) links (33 dupes from the same collapse) +
        // 26 rampage decos as items. Spot: a popular Bow has multiple
        // rampage skills.
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert_eq!(
            count("SELECT COUNT(*) FROM rampage_skills WHERE game_id = 2"),
            247
        );
        assert_eq!(count("SELECT COUNT(*) FROM weapon_rampage"), 6822);
        assert_eq!(
            count("SELECT COUNT(*) FROM rampage_decorations WHERE game_id = 2"),
            26
        );
        // Items: decos stored under Decoration category.
        assert_eq!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 2 AND category = 'Decoration'"),
            26
        );
        // Spot: Rampage Bow S (a buildable Rampage Tree Bow) carries 29
        // possible rampage skills; verify a few and the FK chain.
        let bow: i32 = c
            .query_row(
                "SELECT id FROM weapons WHERE game_id = 2 AND name = 'Rampage Bow S'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let detail = get_weapon_detail(&c, bow).unwrap().expect("rampage bow s");
        assert!(
            detail.rampage.len() >= 20,
            "Rampage Bow S should have many rampage skills, got {}",
            detail.rampage.len()
        );
        assert!(detail.rampage.iter().all(|r| r.skill_id > 0));
    }

    #[test]
    fn mhr_011b_buddy_gear() {
        // 011 Phase B: buddy weapons (322 = 161 palico + 161 palamute) +
        // buddy armor (362 = 181 + 181, upstream x2 dupes collapsed).
        let c = conn();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert_eq!(
            count("SELECT COUNT(*) FROM buddy_weapons WHERE game_id = 2"),
            322
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM buddy_armor WHERE game_id = 2"),
            362
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM buddy_weapons WHERE game_id = 2 AND kind = 'palico'"),
            161
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM buddy_armor WHERE game_id = 2 AND kind = 'palamute'"),
            181
        );
        // Element/status decoding survived the seed (anchored ElementType codes).
        assert!(
            count(
                "SELECT COUNT(*) FROM buddy_weapons WHERE game_id = 2 AND element_type IS NOT NULL"
            ) > 100
        );
        assert!(
            count(
                "SELECT COUNT(*) FROM buddy_weapons WHERE game_id = 2 AND status_type IS NOT NULL"
            ) > 50
        );
        // Spot: Rathalos Sword X (palico) is Fire 25/25.
        let r: (Option<String>, Option<i32>, Option<i32>) = c
            .query_row(
                "SELECT element_type, element_melee, element_ranged FROM buddy_weapons
                 WHERE game_id = 2 AND kind = 'palico' AND name = 'Rathalos Sword X'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(r, (Some("Fire".to_string()), Some(25), Some(25)));
        // Kind filter works.
        let palamute = get_buddy_weapons(&c, 2, Some("palamute".to_string())).unwrap();
        assert_eq!(palamute.len(), 161);
        assert!(palamute.iter().all(|w| w.kind == "palamute"));
        let armor = get_buddy_armor(&c, 2, Some("palico".to_string())).unwrap();
        assert_eq!(armor.len(), 181);
    }

    #[test]
    fn mhr_weapon_trees_game8_edges() {
        // Weapon-trees fix: Rise upgrade_path is Game8 tree edges (the old
        // Kiranico-pagination chain is gone), Bow/LBG labels are corrected,
        // sort_order is Smith DFS, slots are backfilled.
        let c = conn();
        let rows: Vec<(String, String, Option<String>)> = c
            .prepare("SELECT weapon_type, name, upgrade_path FROM weapons WHERE game_id = 2")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(rows.len() >= 3900, "Rise catalog intact");
        let known: std::collections::HashSet<(String, String)> = rows
            .iter()
            .map(|(t, n, _)| (t.clone(), n.clone()))
            .collect();
        let prev_of: std::collections::HashMap<(String, String), Option<String>> = rows
            .iter()
            .map(|(t, n, upath)| {
                let prev = upath.as_deref().and_then(|up| {
                    match serde_json::from_str::<serde_json::Value>(up) {
                        Ok(v) => v
                            .get("previous")
                            .and_then(|p| p.as_str())
                            .map(str::to_string),
                        Err(_) => Some(up.to_string()),
                    }
                });
                ((t.clone(), n.clone()), prev)
            })
            .collect();
        // No cross-type parents, no orphans, branches reciprocate.
        let mut orphans = Vec::new();
        let mut nonrecip = Vec::new();
        let mut roots_per_type: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for (t, n, upath) in &rows {
            let Some(up) = upath.as_deref() else {
                *roots_per_type.entry(t.clone()).or_default() += 1;
                continue;
            };
            let (prev, branches): (Option<String>, Vec<String>) =
                match serde_json::from_str::<serde_json::Value>(up) {
                    Ok(v) => (
                        v.get("previous")
                            .and_then(|p| p.as_str())
                            .map(str::to_string),
                        v.get("branches")
                            .and_then(|b| b.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|x| x.as_str().map(str::to_string))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    ),
                    Err(_) => (Some(up.to_string()), Vec::new()),
                };
            match prev {
                None => *roots_per_type.entry(t.clone()).or_default() += 1,
                Some(p) if p == *n => *roots_per_type.entry(t.clone()).or_default() += 1,
                Some(p) => {
                    if !known.contains(&(t.clone(), p.clone())) {
                        orphans.push(format!("{t}: {p} -> {n}"));
                    }
                }
            }
            for b in branches {
                match prev_of.get(&(t.clone(), b.clone())) {
                    Some(Some(p)) if p == n => {}
                    _ => nonrecip.push(format!("{t}: {n} -> {b}")),
                }
            }
        }
        assert!(
            orphans.is_empty(),
            "Rise upgrade parents must resolve: {orphans:?}"
        );
        assert!(
            nonrecip.is_empty(),
            "Rise branches must reciprocate: {nonrecip:?}"
        );
        // Branching, not one chain: every type has many roots.
        assert_eq!(roots_per_type.len(), 14, "all 14 types present");
        for (t, roots) in &roots_per_type {
            assert!(*roots >= 15, "{t} must branch ({roots} roots)");
        }
        // Depth is tree-like (the pagination chain ran ~300 deep per type),
        // and every walk-up must terminate at a root (no cycles — e.g. the
        // Game8 Bow duplicate that once linked Sinister Soulpiercer <-> +).
        let mut max_depth = 0;
        let mut unterminated = Vec::new();
        for (t, n, _) in &rows {
            let mut depth = 0;
            let mut cur = (t.clone(), n.clone());
            let mut seen = std::collections::HashSet::new();
            loop {
                let next = prev_of.get(&cur).cloned().flatten();
                let Some(p) = next else { break };
                if p == cur.1 || !seen.insert(cur.clone()) || depth > 100 {
                    break;
                }
                cur = (cur.0.clone(), p);
                depth += 1;
            }
            max_depth = max_depth.max(depth);
            // Terminated at a root (no previous, or a self-parent which the
            // UI treats as root) — anything else is a cycle (orphans were
            // already asserted empty above).
            match prev_of.get(&cur).cloned().flatten() {
                None => {}
                Some(p) if p == cur.1 => {}
                Some(_) => unterminated.push(format!("{}: {}", cur.0, cur.1)),
            }
        }
        assert!(
            max_depth < 40,
            "Rise tree depth must be tree-like, got {max_depth}"
        );
        assert!(
            unterminated.is_empty(),
            "Rise walk-ups must reach roots: {unterminated:?}"
        );
        // Spot-checks: linear Kamura chain, Ninja branch, cross-branch Goss.
        let prev = |t: &str, n: &str| -> Option<String> {
            prev_of
                .get(&(t.to_string(), n.to_string()))
                .cloned()
                .flatten()
        };
        assert_eq!(
            prev("Great Sword", "Kamura Cleaver II"),
            Some("Kamura Cleaver I".to_string())
        );
        assert_eq!(
            prev("Great Sword", "Kamura Warrior Cleaver"),
            Some("Kamura Ninja Cleaver".to_string())
        );
        assert_eq!(
            prev("Great Sword", "Duke's Claymore"),
            Some("Kamura Ninja Cleaver".to_string())
        );
        assert_eq!(
            prev("Great Sword", "Gossblade I"),
            Some("Kamura Cleaver III".to_string())
        );
        assert_eq!(
            prev("Great Sword", "Abominable Frostblade"),
            Some("Abominable Snowblade+".to_string())
        );
        // Bow/LBG relabel: Kiranico views are 11=Bow, 13=LBG.
        let wtype: String = c
            .query_row(
                "SELECT weapon_type FROM weapons WHERE game_id = 2 AND name = 'Defender Bow I'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(wtype, "Bow");
        let wtype: String = c
            .query_row(
                "SELECT weapon_type FROM weapons WHERE game_id = 2 AND name = 'Defender Light Bowgun I'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(wtype, "Light Bowgun");
        // Smith head + slots backfill.
        let head: String = c
            .query_row(
                "SELECT name FROM weapons WHERE game_id = 2 AND weapon_type = 'Great Sword'
                 ORDER BY COALESCE(sort_order, id) LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(head, "Defender Great Sword I");
        let slotted: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM weapons WHERE game_id = 2 AND slots IS NOT NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(slotted > 2000, "Rise slots backfilled, got {slotted}");
    }

    #[test]
    fn icons_have_no_nulls() {
        // Icon reuse (mhfu sets) + Kiranico/game8 monster portraits: every
        // icon-bearing row must resolve to a URL — the UI fallback (Package
        // glyph) should only ever trigger on a missing *file*, never NULL.
        let c = conn();
        let nulls = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        for game_id in [1, 2, 3, 4, 5] {
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM items WHERE game_id = {game_id} AND icon_url IS NULL"
                )),
                0,
                "game {game_id} items must all have icon_url"
            );
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM monsters WHERE game_id = {game_id} AND icon_url IS NULL"
                )),
                0,
                "game {game_id} monsters must all have icon_url"
            );
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM weapons WHERE game_id = {game_id} AND icon_url IS NULL"
                )),
                0,
                "game {game_id} weapons must all have icon_url"
            );
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM armor WHERE game_id = {game_id} AND icon_url IS NULL"
                )),
                0,
                "game {game_id} armor must all have icon_url"
            );
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM decorations WHERE game_id = {game_id} AND icon_url IS NULL"
                )),
                0,
                "game {game_id} decorations must all have icon_url"
            );
        }
        // Quests: every game with quest rows has both type + hub icons.
        // (Wilds ships no quests upstream — nothing to assert for game 3.)
        for game_id in [1, 2, 4, 5] {
            assert_eq!(
                nulls(&format!(
                    "SELECT COUNT(*) FROM quests WHERE game_id = {game_id} AND (icon_url IS NULL OR hub_icon_url IS NULL)"
                )),
                0,
                "game {game_id} quests must all have icon_url + hub_icon_url"
            );
        }
        // Per-game monster dirs (no more mhw fallback for Rise/Wilds).
        // Lists render the 96px `-sm` variant; detail pages use the master.
        let (url, url_lg): (String, Option<String>) = c
            .query_row(
                "SELECT icon_url, icon_url_lg FROM monsters WHERE game_id = 2 AND name = 'Magnamalo'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(url, "/icons/mhr/monsters/magnamalo-sm.png");
        assert_eq!(url_lg.as_deref(), Some("/icons/mhr/monsters/magnamalo.png"));
        let (url, url_lg): (String, Option<String>) = c
            .query_row(
                "SELECT icon_url, icon_url_lg FROM monsters WHERE game_id = 3 AND name = 'Arkveld'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(url, "/icons/mhwilds/monsters/arkveld-sm.png");
        assert_eq!(
            url_lg.as_deref(),
            Some("/icons/mhwilds/monsters/arkveld.png")
        );
        // Older games have no large masters: detail falls back to icon_url.
        let lg_nulls: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM monsters WHERE game_id IN (1, 4, 5) AND icon_url_lg IS NOT NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(lg_nulls, 0, "only Rise/Wilds carry icon_url_lg");
    }

    #[test]
    fn data_patches_backfill_without_full_reseed() {
        // Simulates a pre-patch install: full seed ran, then icons regressed
        // to NULL / legacy dirs with no patch rows recorded. Patches must
        // refill everything, record themselves, and be a no-op on re-run —
        // all without touching row counts (no full re-seed).
        let c = conn();
        c.execute_batch(
            "DELETE FROM data_patches;
             UPDATE items SET icon_url = NULL, icon_name = NULL WHERE game_id IN (2, 3, 4);
             UPDATE quests SET icon_url = NULL, hub_icon_url = NULL WHERE game_id = 2;
             UPDATE monsters SET icon_url = REPLACE(icon_url, '/icons/mhr/monsters/', '/icons/mhw/monsters/') WHERE game_id = 2;
             UPDATE monsters SET icon_url = REPLACE(icon_url, '/icons/mhwilds/monsters/', '/icons/mhw/monsters/') WHERE game_id = 3;
             UPDATE monster_ailments SET initial = NULL, notes = NULL WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1);
             UPDATE monster_tools SET normal = NULL, notes = NULL WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1);
             DELETE FROM hunter_charm_materials;
             DELETE FROM hunter_charms;
             UPDATE decorations SET acquisition = NULL WHERE game_id = 1;
             UPDATE weapons SET description = NULL WHERE game_id = 1;
             UPDATE armor SET description = NULL WHERE game_id = 1;
             UPDATE armor_sets SET bonus_skill2 = NULL, bonus_required2 = NULL WHERE game_id = 1;
             UPDATE monster_weaknesses SET stagger_hp = NULL WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1);
             UPDATE quests SET main_monsters = '[]' WHERE game_id = 1 AND id IN (100101, 100201);
             UPDATE items SET buy_price = NULL WHERE game_id = 1;
             INSERT INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language)
             VALUES (1078, 20927, 'capture', NULL, 'Master', 1, 0.5, NULL, 'en');
             DELETE FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE name LIKE 'Gunpowder Level%' AND game_id = 1);",
        )
        .unwrap();
        let count = |sql: &str| -> i64 { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        let items_before = count("SELECT COUNT(*) FROM items");
        assert!(
            count("SELECT COUNT(*) FROM items WHERE game_id IN (2,3,4) AND icon_url IS NULL")
                > 3000
        );

        crate::db::seed::apply_data_patches(&c).unwrap();

        assert_eq!(
            count("SELECT COUNT(*) FROM items"),
            items_before,
            "patches must not add rows"
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM items WHERE icon_url IS NULL"),
            0
        );
        assert_eq!(count("SELECT COUNT(*) FROM quests WHERE game_id = 2 AND (icon_url IS NULL OR hub_icon_url IS NULL)"), 0);
        assert_eq!(count("SELECT COUNT(*) FROM monsters WHERE game_id = 2 AND icon_url LIKE '/icons/mhw/monsters/%'"), 0);
        assert_eq!(count("SELECT COUNT(*) FROM monsters WHERE game_id = 3 AND icon_url LIKE '/icons/mhw/monsters/%'"), 0);
        // Thumbs patch: lists on `-sm` variants, masters in icon_url_lg.
        assert_eq!(count("SELECT COUNT(*) FROM monsters WHERE game_id IN (2, 3) AND icon_url NOT LIKE '%-sm.png'"), 0);
        assert_eq!(
            count("SELECT COUNT(*) FROM monsters WHERE game_id IN (2, 3) AND icon_url_lg IS NULL"),
            0
        );
        assert_eq!(count("SELECT COUNT(*) FROM data_patches"), 23);
        // MHFU equipment backfill: variants have gear links now.
        assert!(
            count("SELECT COUNT(*) FROM monster_equipment WHERE game_id = 5 AND monster_id = 26")
                > 30
        );
        assert!(
            count("SELECT COUNT(*) FROM monster_equipment WHERE game_id = 5 AND monster_id = 32")
                > 100
        );
        // MHFU items backfill: 1175+ rows + shop prices + carry limits.
        assert_eq!(count("SELECT COUNT(*) FROM items WHERE game_id = 5"), 1244);
        assert_eq!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 5 AND name = 'Book of Combos 2' AND rarity = 4 AND sell_price = 200"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 5 AND name = 'Potion' AND buy_price = 66"),
            1
        );
        // MHFU quests fix: Nekoht-9 restored, urgents flagged, JUMP G3.
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 5 AND hub = 'nekoto'"),
            62
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 5 AND hub = 'guild_g'"),
            89
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 5 AND is_urgent = 1"),
            20
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 5 AND id = 565 AND stars = 3"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM quest_rewards WHERE quest_id IN (SELECT id FROM quests WHERE game_id = 5) AND condition = 'Special Reward'"),
            4
        );
        // MHFU weapon craft split: all 1500 weapons have forge/upgrade rows.
        assert_eq!(
            count("SELECT COUNT(DISTINCT weapon_id) FROM weapon_craft WHERE weapon_id IN (SELECT id FROM weapons WHERE game_id = 5)"),
            1500
        );
        // MHP3rd drops sync: all 60 monsters covered, xeno rows pruned.
        assert_eq!(
            count("SELECT COUNT(DISTINCT monster_id) FROM monster_drops WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 4)"),
            60
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 10032 AND item_id IN (SELECT id FROM items WHERE name LIKE '%Tigrex%' OR name LIKE '%Rathalos%')"),
            0
        );
        // MHP3rd weaknesses rewrite: real hitzone parts, no placeholders.
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 4)"),
            213
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id = 10045 AND part_name = 'Head' AND sever = 80 AND dragon = 35"),
            1
        );
        // MHP3rd roundout: real descriptions, Akantor gear, trap tools.
        assert_eq!(
            count("SELECT COUNT(*) FROM monsters WHERE game_id = 4 AND description LIKE '%in Monster Hunter Portable 3rd (ULJM-05800)%'"),
            1
        );
        assert!(
            count(
                "SELECT COUNT(*) FROM monster_equipment WHERE game_id = 4 AND monster_id = 10023"
            ) > 5
        );
        assert!(count("SELECT COUNT(*) FROM monster_tools WHERE monster_id = 10057") >= 5);
        // MHP3rd items fix: 10526 placeholder kept (6 Ludroth U armor rows
        // FK-depend on it), Yukumo strings fixed.
        assert_eq!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 4 AND id = 10526"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM item_sources WHERE item_id IN (SELECT id FROM items WHERE game_id = 4) AND (location LIKE '%Pokke%' OR conditions LIKE '%Pokke%')"),
            0
        );
        // MHP3rd key flags: wiki-confirmed Hunt 1 Barroth.
        assert_eq!(
            count(
                "SELECT COUNT(*) FROM quests WHERE game_id = 4 AND id = 10108 AND is_key_quest = 1"
            ),
            1
        );
        // MHP3rd gear fix: elements filled, mats complete, junk gone.
        assert!(
            count("SELECT COUNT(*) FROM weapons WHERE game_id = 4 AND element_type IS NOT NULL")
                > 300
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM weapons WHERE game_id = 4 AND id NOT IN (SELECT weapon_id FROM weapon_materials)"),
            0
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM armor WHERE game_id = 4 AND name LIKE '%engname%'"),
            0
        );
        // MHP3rd charms + comrades seeded.
        assert_eq!(
            count("SELECT COUNT(*) FROM charms WHERE game_id = 4"),
            50635
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM comrade_weapons WHERE game_id = 4"),
            92
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM comrade_armor WHERE game_id = 4"),
            154
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM comrade_skills WHERE game_id = 4"),
            52
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id = 26"),
            7
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id = 74"),
            7
        );
        // MHFU drop backfill: Scarred Garuga + Rusted Kushala no longer empty.
        assert!(count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 26") > 30);
        assert!(count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 74") > 30);
        // MHFU extraction-arbitrated corrections (mhfu-db wins over guides).
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 25 AND item_id = 428 AND method = 'carve' AND part = 'Body' AND rank = 'G' AND ABS(probability - 0.53) < 1e-9"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 34 AND item_id = 562 AND method = 'break' AND part = 'Head' AND rank = 'Low' AND ABS(probability - 0.65) < 1e-9"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 20 AND item_id = 379 AND method = 'carve' AND part = 'Body' AND rank = 'High' AND ABS(probability - 0.05) < 1e-9"),
            1
        );
        // Shadow-rank row removed (Gendrome High held Low-rank Scale values).
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 20 AND item_id = 375 AND method = 'carve' AND part = 'Body' AND rank = 'High'"),
            0
        );

        // MHW 010 backfill heals the regressed rows above (existing installs
        // converge without a DATA_VERSION bump).
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_ailments WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1) AND initial IS NOT NULL"),
            501
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_tools WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1) AND normal IS NOT NULL"),
            386
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM hunter_charms WHERE game_id = 1"),
            317
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM hunter_charm_materials WHERE charm_id IN (SELECT id FROM hunter_charms WHERE game_id = 1)"),
            1247
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM decorations WHERE game_id = 1 AND acquisition IS NOT NULL"),
            398
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM weapons WHERE game_id = 1 AND description IS NOT NULL AND description != ''"),
            3544
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM armor WHERE game_id = 1 AND description IS NOT NULL AND description != ''"),
            1595
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM armor_sets WHERE game_id = 1 AND bonus_skill2 IS NOT NULL"),
            58
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_weaknesses WHERE monster_id IN (SELECT id FROM monsters WHERE game_id = 1) AND stagger_hp IS NOT NULL"),
            317
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM quests WHERE game_id = 1 AND id IN (100101, 100201) AND main_monsters != '[]'"),
            2
        );
        assert!(
            count("SELECT COUNT(*) FROM items WHERE game_id = 1 AND buy_price IS NOT NULL AND buy_price > 0") >= 55
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 1078 AND item_id = 20927 AND method = 'capture'"),
            0
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM item_combine WHERE result_item_id IN (SELECT id FROM items WHERE name LIKE 'Gunpowder Level%' AND game_id = 1)"),
            10
        );
        // Second run: pure no-op (23 PK lookups, warm-boot fast path).
        crate::db::seed::apply_data_patches(&c).unwrap();
        assert_eq!(count("SELECT COUNT(*) FROM data_patches"), 23);
        assert_eq!(count("SELECT COUNT(*) FROM items"), items_before);

        // Regression simulation: revert corrected rows to pre-fix values and
        // re-run patches — the corrections patch must heal them.
        c.execute_batch(
            "UPDATE monster_drops SET probability = 0.64 WHERE monster_id = 25 AND item_id = 428 AND method = 'carve' AND part = 'Body' AND rank = 'G';
             INSERT INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language)
             VALUES (20, 375, 'carve', 'Body', 'High', 1, 0.15, NULL, 'en');
             DELETE FROM data_patches WHERE name = 'mh2g_drop_corrections';",
        )
        .unwrap();
        crate::db::seed::apply_data_patches(&c).unwrap();
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 25 AND item_id = 428 AND method = 'carve' AND part = 'Body' AND rank = 'G' AND ABS(probability - 0.53) < 1e-9"),
            1
        );
        assert_eq!(
            count("SELECT COUNT(*) FROM monster_drops WHERE monster_id = 20 AND item_id = 375 AND method = 'carve' AND part = 'Body' AND rank = 'High'"),
            0
        );
        assert_eq!(count("SELECT COUNT(*) FROM data_patches"), 23);
        assert_eq!(count("SELECT COUNT(*) FROM items"), items_before);
    }
}
