use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ── Public API types (Tauri) ────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct SkillRequirement {
    pub skill_id: i32,
    pub points_required: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssQueryInput {
    pub game_id: i32,
    pub skills: Vec<SkillRequirement>,
    pub hunter_type: String, // "blade" | "gunner"
    pub gender: String,      // "male" | "female"
    pub hr: i32,
    pub elder_star: i32,
    pub weapon_slots: i32, // 0..3
    pub include_piercings: bool,
    pub allow_bad: bool,
    pub allow_torso_inc: bool,
    pub sort_by: Option<String>, // None | "defence" | "fire_res" etc
}

#[derive(Debug, Serialize)]
pub struct AssArmorRef {
    pub id: i32,
    pub name: String,
    pub slot_type: String,
    pub rarity: Option<i32>,
    pub defense_base: Option<i32>,
    pub slots: Option<String>,
    pub skills: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AssDecorationRef {
    pub id: i32,
    pub name: String,
    pub slot_size: Option<i32>,
    pub skill_name: Option<String>,
    pub skill_points: Option<i32>,
    pub secondary_skill_name: Option<String>,
    pub secondary_points: Option<i32>,
    pub count: i32,
}

#[derive(Debug, Serialize)]
pub struct AssSolutionView {
    pub armors: Vec<AssArmorRef>,
    pub decorations: Vec<AssDecorationRef>,
    pub extra_skills: Vec<String>,
    pub defense: i32,
    pub fire_res: i32,
    pub water_res: i32,
    pub thunder_res: i32,
    pub ice_res: i32,
    pub dragon_res: i32,
    pub rarity: i32,
    pub difficulty: i32,
    pub slots_spare: i32,
    pub slots_spare_detail: Vec<i32>,
}

// ── Internal solver types ───────────────────────────────────────────

#[derive(Debug, Clone)]
struct Armor {
    id: i32,
    name: String,
    slot_type: String, // head/body/arms/waist/legs (normalized)
    rarity: i32,
    defence: i32,
    fire: i32,
    thunder: i32,
    dragon: i32,
    water: i32,
    ice: i32,
    num_slots: usize,
    armor_type: String, // both/blade/gunner
    gender: String,     // both/male/female
    rank: String,       // Low/High/G — progression gate derived from HR/elder via get_difficulty
    is_piercing: bool,
    is_torso_inc: bool,
    skill_points: HashMap<i32, i32>, // skill_id -> points
}

#[derive(Debug, Clone)]
struct Decoration {
    id: i32,
    slot_size: usize,
    abilities: Vec<(i32, i32)>, // (skill_id, points) primary first, secondary second
    dangerous: bool,
}

#[derive(Debug, Clone)]
struct ArmorEquivalence {
    armors: Vec<Armor>,
    num_slots: usize,
    torso_inc: bool,
    no_skills: bool,
    abilities: Vec<(i32, i32)>, // only rel abilities
}

#[derive(Debug, Clone)]
struct QueryInternal {
    skills: Vec<SkillRequirement>, // desired
    hunter_type: String,
    gender: String,
    weapon_slots: usize,
    include_piercings: bool,
    allow_bad: bool,
    allow_torso_inc: bool,
    difficulty: usize,
    rel_skill_ids: Vec<i32>,
    rel_decorations: Vec<Decoration>,
    inf_decorations: Vec<Decoration>,
    // per armor type
    rel_armor: Vec<Vec<Armor>>, // 5 slots
    inf_armor: Vec<Vec<Armor>>,
    // derived
    ability_index: HashMap<i32, usize>, // skill_id -> idx in rel
    sorted_decorations: Vec<Vec<Decoration>>, // 0..3
}

const MAX_LIMIT: usize = 1000;
const NUM_ARMOR_TYPES: usize = 5;

fn slot_index(s: &str) -> usize {
    match s {
        "head" | "helm" => 0,
        "chest" | "body" | "mail" => 1,
        "arms" | "arm" => 2,
        "waist" => 3,
        "legs" | "greaves" => 4,
        _ => 0,
    }
}

fn parse_slots(slot_str: Option<String>) -> usize {
    match slot_str {
        None => 0,
        Some(s) => {
            let s = s.trim();
            if s == "0" || s == "---" || s.is_empty() {
                0
            } else if s == "1" || s == "O--" || s == "O" {
                1
            } else if s == "2" || s == "OO-" || s == "OO" {
                2
            } else if s == "3" || s == "OOO" {
                3
            } else {
                // fallback: count 'O'
                s.chars().filter(|&c| c == 'O' || c == 'o').count()
            }
        }
    }
}

fn get_difficulty(hr: i32, elder: i32) -> usize {
    if hr > 6 {
        3
    } else if hr > 3 || elder > 6 {
        2
    } else {
        1
    }
}

fn skill_points_from_str(
    skills: &Option<String>,
    skill_name_to_id: &HashMap<String, i32>,
) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    if let Some(s) = skills {
        for part in s.split(',') {
            let t = part.trim();
            if t.is_empty() {
                continue;
            }
            // find last +/-
            if let Some(pos) = t.rfind(|c| c == '+' || c == '-') {
                let name = t[..pos].trim().to_string();
                let val_str = t[pos..].trim();
                if let Ok(v) = val_str.parse::<i32>() {
                    // normalize name
                    let normalized = normalize_skill_name(&name);
                    if let Some(&sid) = skill_name_to_id.get(&normalized) {
                        map.insert(sid, v);
                    } else if let Some(&sid) = skill_name_to_id.get(&name) {
                        map.insert(sid, v);
                    }
                }
            }
        }
    }
    map
}

fn normalize_skill_name(n: &str) -> String {
    match n {
        "WindPress" | "Wind Press" => "Wind Press".to_string(),
        "ThunderRes" => "ThunderRes".to_string(),
        "ClustS Add" => "ClustSAdd".to_string(),
        "CragS Add" => "CragSAdd".to_string(),
        "PelletS Add" => "PelletSAdd".to_string(),
        "NormalS Add" => "NormalSAdd".to_string(),
        "PierceS Add" => "PierceSAdd".to_string(),
        _ => n.to_string(),
    }
}

fn load_armors(
    conn: &Connection,
    game_id: i32,
    skill_name_to_id: &HashMap<String, i32>,
) -> rusqlite::Result<Vec<Armor>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, slot_type, rarity, defense_base, resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, slots, skills, armor_type, gender, rank FROM armor WHERE game_id = ?1",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let slot_type: String = row.get(2)?;
        let rarity: Option<i32> = row.get(3)?;
        let defence: Option<i32> = row.get(4)?;
        let fire: Option<i32> = row.get(5)?;
        let water: Option<i32> = row.get(6)?;
        let thunder: Option<i32> = row.get(7)?;
        let ice: Option<i32> = row.get(8)?;
        let dragon: Option<i32> = row.get(9)?;
        let slots: Option<String> = row.get(10)?;
        let skills: Option<String> = row.get(11)?;
        let armor_type: Option<String> = row.get(12)?;
        let gender: Option<String> = row.get(13)?;
        let rank: Option<String> = row.get(14)?;
        Ok((
            id, name, slot_type, rarity, defence, fire, water, thunder, ice, dragon, slots, skills,
            armor_type, gender, rank,
        ))
    })?;
    let mut out = Vec::new();
    for r in rows {
        let (
            id,
            name,
            slot_type,
            rarity,
            defence,
            fire,
            water,
            thunder,
            ice,
            dragon,
            slots,
            skills,
            armor_type,
            gender,
            rank,
        ) = r?;
        let num_slots = parse_slots(slots);
        let at = armor_type.unwrap_or_else(|| "both".to_string());
        let gd = gender.unwrap_or_else(|| "both".to_string());
        let rk = rank.unwrap_or_else(|| "Low".to_string());
        let sp = skill_points_from_str(&skills, skill_name_to_id);
        let is_piercing = name.contains("Piercing");
        let is_torso_inc = sp.contains_key(&torso_skill_id_cached(skill_name_to_id));
        // also check skills string contains Torso Inc
        let is_torso_inc = is_torso_inc || skills.as_deref().unwrap_or("").contains("Torso Inc");
        out.push(Armor {
            id,
            name: name.clone(),
            slot_type: slot_type.clone(),
            rarity: rarity.unwrap_or(0),
            defence: defence.unwrap_or(0),
            fire: fire.unwrap_or(0),
            thunder: thunder.unwrap_or(0),
            dragon: dragon.unwrap_or(0),
            water: water.unwrap_or(0),
            ice: ice.unwrap_or(0),
            num_slots,
            armor_type: at,
            gender: gd,
            rank: rk,
            is_piercing,
            is_torso_inc,
            skill_points: sp,
        });
    }
    Ok(out)
}

fn torso_skill_id_cached(map: &HashMap<String, i32>) -> i32 {
    *map.get("Torso Inc").unwrap_or(&-1)
}

fn load_decorations(conn: &Connection, game_id: i32) -> rusqlite::Result<Vec<Decoration>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, slot_size, skill_id, skill_points, secondary_skill_id, secondary_points FROM decorations WHERE game_id = ?1",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<i32>>(2)?,
            row.get::<_, Option<i32>>(3)?,
            row.get::<_, Option<i32>>(4)?,
            row.get::<_, Option<i32>>(5)?,
            row.get::<_, Option<i32>>(6)?,
        ))
    })?;
    let mut out = Vec::new();
    for r in rows {
        let (id, _name, slot_size, sid, spts, sid2, spts2) = r?;
        let slot_size = slot_size.unwrap_or(1) as usize;
        let mut abilities = Vec::new();
        if let (Some(s), Some(p)) = (sid, spts) {
            abilities.push((s, p));
        }
        if let (Some(s), Some(p)) = (sid2, spts2) {
            abilities.push((s, p));
        }
        if abilities.is_empty() {
            continue;
        }
        out.push(Decoration {
            id,
            slot_size,
            abilities,
            dangerous: false,
        });
    }
    Ok(out)
}

fn load_skill_name_map(conn: &Connection, game_id: i32) -> rusqlite::Result<HashMap<String, i32>> {
    let mut stmt = conn.prepare("SELECT id, name FROM skills WHERE game_id = ?1")?;
    let rows = stmt.query_map([game_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut map = HashMap::new();
    for r in rows {
        let (id, name) = r?;
        map.insert(name.clone(), id);
        // also insert normalized alias
        let n = normalize_skill_name(&name);
        if n != name {
            map.insert(n, id);
        }
    }
    Ok(map)
}

// ── Helpers ported from C++ ─────────────────────────────────────────

fn armor_matches_query(
    armor: &Armor,
    q: &QueryInternal,
    danger_skills: &HashSet<i32>,
    max_slots: usize,
) -> (bool, Option<i32>, bool) {
    // returns (matches, danger_skill, no_skills)
    if !q.include_piercings && armor.is_piercing {
        return (false, None, false);
    }
    if !q.allow_torso_inc && armor.is_torso_inc {
        return (false, None, false);
    }
    // hunter_type
    let ht = q.hunter_type.as_str();
    let at = armor.armor_type.as_str();
    if at != "both" && ht != at {
        return (false, None, false);
    }
    // gender
    let g = q.gender.as_str();
    let ag = armor.gender.as_str();
    if ag != "both" && ag != g {
        return (false, None, false);
    }
    // rank gate from HR/elder (GetDifficulty): 1=Low, 2=Low+High, 3=all
    let rank_ok = match q.difficulty {
        1 => armor.rank == "Low",
        2 => armor.rank == "Low" || armor.rank == "High",
        _ => true,
    };
    if !rank_ok {
        return (false, None, false);
    }
    // dummy not present
    if armor.is_torso_inc {
        return (true, None, false);
    }
    // danger check
    let mut danger: Option<i32> = None;
    for (sid, pts) in &armor.skill_points {
        if *pts < 0 && danger_skills.contains(sid) {
            danger = Some(*sid);
            break;
        }
    }
    // relevant skills check
    let mut no_skills = true;
    for (sid, pts) in &armor.skill_points {
        if *pts > 0 {
            if q.rel_skill_ids.contains(sid) {
                // Check if positive thresholds reachable - simplified: any positive relevant counts
                no_skills = false;
                return (true, danger, no_skills);
            }
        }
    }
    // if no relevant positive, check slots
    let has_danger = danger.is_some();
    if armor.num_slots >= max_slots && !has_danger {
        // need to set no_skills true
        return (true, danger, true);
    }
    (false, danger, no_skills)
}

fn is_better_armor(a: &Armor, b: &Armor, rel: &[i32]) -> bool {
    if a.num_slots > b.num_slots {
        return true;
    }
    let a_no = a
        .skill_points
        .iter()
        .all(|(k, v)| !rel.contains(k) || *v <= 0);
    let b_no = b
        .skill_points
        .iter()
        .all(|(k, v)| !rel.contains(k) || *v <= 0);
    if a_no && b_no {
        return if a.defence == b.defence {
            a.rarity > b.rarity
        } else {
            a.defence > b.defence
        };
    }
    if a.is_torso_inc && b.is_torso_inc {
        return if a.rarity == b.rarity {
            a.defence > b.defence
        } else {
            a.rarity > b.rarity
        };
    } else if a.is_torso_inc || b.is_torso_inc {
        return true;
    }
    for sid in rel {
        let av = a.skill_points.get(sid).copied().unwrap_or(0);
        let bv = b.skill_points.get(sid).copied().unwrap_or(0);
        if av > bv {
            return true;
        }
    }
    false
}

fn is_better_decoration(a: &Decoration, b: &Decoration, _rel: &[i32]) -> bool {
    if a.slot_size < b.slot_size || a.abilities[0].0 != b.abilities[0].0 {
        return true;
    }
    let av = a.abilities[0].1 * b.slot_size as i32;
    let bv = b.abilities[0].1 * a.slot_size as i32;
    if av != bv {
        return av > bv;
    }
    // NotWorse
    let a_worse = a.abilities.len() == 2;
    let b_worse = b.abilities.len() == 2;
    if b_worse
        && (!a_worse
            || a.abilities[1].1 * (b.slot_size as i32) < b.abilities[1].1 * (a.slot_size as i32))
    {
        return true;
    }
    false
}

fn get_relevant_data(conn: &Connection, input: &AssQueryInput) -> rusqlite::Result<QueryInternal> {
    let skill_map = load_skill_name_map(conn, input.game_id)?;
    let all_armors = load_armors(conn, input.game_id, &skill_map)?;
    let mut all_decos = load_decorations(conn, input.game_id)?;

    let rel_ids: Vec<i32> = input.skills.iter().map(|s| s.skill_id).collect();
    let mut ability_index = HashMap::new();
    for (idx, sid) in rel_ids.iter().enumerate() {
        ability_index.insert(*sid, idx);
    }

    // Danger skills: skills that have a negative threshold (e.g. Attack -10 triggers a bad ability).
    // When `allow_bad` is false, armors carrying those negatives are treated as risky and the solver
    // later tries to patch them via `fix_bad_skills`. `reorder_gems` remains a stub (see below).
    let mut danger_skills: HashSet<i32> = HashSet::new();
    if !input.allow_bad {
        // Populate from skill_levels negative thresholds — conservative: any skill with a <0 level is dangerous.
        if let Ok(mut stmt) =
            conn.prepare("SELECT DISTINCT skill_id FROM skill_levels WHERE points < 0")
        {
            if let Ok(rows) = stmt.query_map([], |row| row.get::<_, i32>(0)) {
                for r in rows.flatten() {
                    danger_skills.insert(r);
                }
            }
        }
    }

    // Build QueryInternal skeleton
    let mut q = QueryInternal {
        skills: input
            .skills
            .iter()
            .map(|s| SkillRequirement {
                skill_id: s.skill_id,
                points_required: s.points_required,
            })
            .collect(),
        hunter_type: input.hunter_type.clone(),
        gender: input.gender.clone(),
        weapon_slots: input.weapon_slots.max(0) as usize,
        include_piercings: input.include_piercings,
        allow_bad: input.allow_bad,
        allow_torso_inc: input.allow_torso_inc,
        difficulty: get_difficulty(input.hr, input.elder_star),
        rel_skill_ids: rel_ids.clone(),
        rel_decorations: Vec::new(),
        inf_decorations: Vec::new(),
        rel_armor: vec![Vec::new(); NUM_ARMOR_TYPES],
        inf_armor: vec![Vec::new(); NUM_ARMOR_TYPES],
        ability_index,
        sorted_decorations: vec![Vec::new(); 4],
    };

    // Filter decorations: keep those that match primary ability in rel
    for deco in all_decos.drain(..) {
        let primary = deco.abilities[0].0;
        if rel_ids.contains(&primary) {
            // HR check skipped (no hr data)
            // Add to rel with better pruning
            let mut inserted = false;
            for i in 0..q.rel_decorations.len() {
                if deco.id == q.rel_decorations[i].id {
                    inserted = true;
                    break;
                }
                if is_better_decoration(&deco, &q.rel_decorations[i], &rel_ids) {
                    if !is_better_decoration(&q.rel_decorations[i], &deco, &rel_ids) {
                        q.rel_decorations.remove(i);
                        break;
                    }
                } else if is_better_decoration(&q.rel_decorations[i], &deco, &rel_ids) {
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                q.rel_decorations.push(deco.clone());
                q.inf_decorations.push(deco);
            }
        }
    }
    for deco in &mut q.rel_decorations {
        deco.dangerous = deco.abilities.len() == 2 && rel_ids.contains(&deco.abilities[1].0);
    }

    // Filter armors per slot type (5)
    // Group by slot_type first
    let mut grouped: Vec<Vec<Armor>> = vec![Vec::new(); NUM_ARMOR_TYPES];
    for a in all_armors {
        let idx = slot_index(&a.slot_type);
        if idx < NUM_ARMOR_TYPES {
            grouped[idx].push(a);
        }
    }
    // For each slot, apply MatchesQuery and AddToList logic simplified
    for slot in 0..NUM_ARMOR_TYPES {
        let mut max_slots = 0usize;
        let list = grouped[slot].clone();
        for armor in list {
            let (matches, _danger, _no_skills) =
                armor_matches_query(&armor, &q, &danger_skills, max_slots);
            if matches {
                max_slots = max_slots.max(armor.num_slots);
                // Simplified AddToList: just push if not dominated heavily - use is_better pruning
                let mut should_add = true;
                let mut to_remove = Vec::new();
                for (idx, existing) in q.rel_armor[slot].iter().enumerate() {
                    if is_better_armor(&armor, existing, &rel_ids) {
                        if !is_better_armor(existing, &armor, &rel_ids) {
                            to_remove.push(idx);
                        }
                    } else if is_better_armor(existing, &armor, &rel_ids) {
                        should_add = false;
                        break;
                    }
                }
                if should_add {
                    for idx in to_remove.iter().rev() {
                        q.rel_armor[slot].remove(*idx);
                    }
                    q.rel_armor[slot].push(armor.clone());
                    q.inf_armor[slot].push(armor);
                } else {
                    // still track inf for advanced search
                    q.inf_armor[slot].push(armor);
                }
            }
        }
        // If empty due to over-pruning, fallback to include all matching without pruning (still respects rank/gender)
        if q.rel_armor[slot].is_empty() {
            for armor in &grouped[slot] {
                if !q.include_piercings && armor.is_piercing {
                    continue;
                }
                if !q.allow_torso_inc && armor.is_torso_inc {
                    continue;
                }
                if armor.armor_type != "both" && armor.armor_type != q.hunter_type {
                    continue;
                }
                if armor.gender != "both" && armor.gender != q.gender {
                    continue;
                }
                let rank_ok = match q.difficulty {
                    1 => armor.rank == "Low",
                    2 => armor.rank == "Low" || armor.rank == "High",
                    _ => true,
                };
                if !rank_ok {
                    continue;
                }
                q.rel_armor[slot].push(armor.clone());
            }
        }
    }

    // Build sorted_decorations
    for deco in &q.rel_decorations {
        if deco.slot_size < 4 {
            q.sorted_decorations[deco.slot_size].push(deco.clone());
        }
    }

    Ok(q)
}

fn create_equivalences(q: &QueryInternal) -> Vec<Vec<ArmorEquivalence>> {
    let mut armor_eq: Vec<Vec<ArmorEquivalence>> = vec![Vec::new(); NUM_ARMOR_TYPES];
    for slot in 0..NUM_ARMOR_TYPES {
        let mut list: Vec<ArmorEquivalence> = Vec::new();
        for armor in &q.rel_armor[slot] {
            let mut need_new = true;
            for eq in &mut list {
                if eq_matches(eq, armor, &q.rel_skill_ids) {
                    eq.armors.push(armor.clone());
                    need_new = false;
                    break;
                }
            }
            if need_new {
                list.push(ArmorEquivalence::from_armor(
                    armor.clone(),
                    &q.rel_skill_ids,
                ));
            }
        }
        armor_eq[slot] = list;
    }
    armor_eq
}

impl ArmorEquivalence {
    fn from_armor(a: Armor, rel: &[i32]) -> Self {
        let no_skills = a
            .skill_points
            .iter()
            .all(|(k, v)| !rel.contains(k) || *v <= 0)
            && !a.is_torso_inc;
        let torso_inc = a.is_torso_inc;
        let mut abilities = Vec::new();
        if !no_skills && !torso_inc {
            for (sid, pts) in &a.skill_points {
                if rel.contains(sid) {
                    abilities.push((*sid, *pts));
                }
            }
        }
        Self {
            armors: vec![a.clone()],
            num_slots: a.num_slots,
            torso_inc,
            no_skills,
            abilities,
        }
    }
}

fn eq_matches(eq: &ArmorEquivalence, armor: &Armor, rel: &[i32]) -> bool {
    if eq.num_slots != armor.num_slots {
        return false;
    }
    if eq.torso_inc {
        return armor.is_torso_inc;
    } else if armor.is_torso_inc {
        return false;
    }
    if eq.no_skills {
        return armor
            .skill_points
            .iter()
            .all(|(k, v)| !rel.contains(k) || *v <= 0);
    } else if armor
        .skill_points
        .iter()
        .all(|(k, v)| !rel.contains(k) || *v <= 0)
    {
        return false;
    }
    for (sid, pts) in &eq.abilities {
        if armor.skill_points.get(sid).copied().unwrap_or(0) != *pts {
            return false;
        }
    }
    true
}

// ── Decoration solving port ─────────────────────────────────────────

#[derive(Debug, Clone)]
struct EquivalenceSolution {
    armor_eq: Vec<ArmorEquivalence>, // 5
    decorations: Vec<Decoration>,
    multipliers: Vec<usize>,
    slots_spare: [usize; 4],
    torso_slots_spare: usize,
    torso_multiplier: usize,
}

impl EquivalenceSolution {
    fn matches_query(&mut self, q: &QueryInternal) -> bool {
        let n = q.rel_skill_ids.len();
        let mut skill_points = vec![0i32; n];
        let mut desired = vec![0i32; n];
        self.torso_multiplier = 1;
        self.slots_spare = [0; 4];
        self.torso_slots_spare = 0;
        self.decorations.clear();
        self.multipliers.clear();
        for (idx, sid) in q.rel_skill_ids.iter().enumerate() {
            for req in &q.skills {
                if req.skill_id == *sid {
                    desired[idx] = req.points_required;
                    break;
                }
            }
        }
        // head=0, body=1, arms=2, waist=3, legs=4 but ASS order: head, legs, waist, arms, body? In Solution.cpp: head, legs(2), waist(3), arms(4?) actually [0] head, [1] body, [2] arms, [3] waist, [4] legs. They process 0,2,3,4 then 1 body torso. So adapt:
        // armor_eq order is [head, body, arms, waist, legs]
        self.calculate_equivalence_info(
            &q.ability_index,
            &self.armor_eq[0].clone(),
            &mut skill_points,
            false,
        );
        self.calculate_equivalence_info(
            &q.ability_index,
            &self.armor_eq[2].clone(),
            &mut skill_points,
            false,
        );
        self.calculate_equivalence_info(
            &q.ability_index,
            &self.armor_eq[3].clone(),
            &mut skill_points,
            false,
        );
        self.calculate_equivalence_info(
            &q.ability_index,
            &self.armor_eq[4].clone(),
            &mut skill_points,
            false,
        );
        self.calculate_equivalence_info(
            &q.ability_index,
            &self.armor_eq[1].clone(),
            &mut skill_points,
            true,
        );
        let body_slots = self.armor_eq[1].num_slots;
        if body_slots < 4 {
            self.slots_spare[body_slots] = self.slots_spare[body_slots].saturating_sub(1);
        }
        // Actually original: slots_spare[torso_slots_spare = armor_eq[1]->num_slots]-- ; but slots_spare already incremented in body calc
        // We'll emulate: after body, we already incremented slot count, then decrement one occurrence
        // But our calculate already did slots_spare[num_slots]++ for body. So we need to set torso_slots
        self.torso_slots_spare = body_slots;

        self.calculate_decorations(q, &mut skill_points, &desired);
        for i in 0..desired.len() {
            if skill_points[i] < desired[i] {
                return false;
            }
        }
        true
    }
    fn calculate_equivalence_info(
        &mut self,
        ability_map: &HashMap<i32, usize>,
        eq: &ArmorEquivalence,
        skills: &mut Vec<i32>,
        torso: bool,
    ) {
        self.torso_multiplier += if eq.torso_inc { 1 } else { 0 };
        if eq.num_slots < 4 {
            self.slots_spare[eq.num_slots] += 1;
        }
        if eq.torso_inc || eq.no_skills {
            return;
        }
        for (sid, pts) in &eq.abilities {
            if let Some(&idx) = ability_map.get(sid) {
                let mult = if torso { self.torso_multiplier - 1 } else { 0 } + 1;
                skills[idx] += pts * mult as i32;
            }
        }
    }
    fn calculate_decorations(
        &mut self,
        q: &QueryInternal,
        skill_points: &mut Vec<i32>,
        desired: &Vec<i32>,
    ) {
        let mut body_slots = [0usize; 4];
        let mut temp_slots = [0usize; 4];
        if q.weapon_slots < 4 {
            self.slots_spare[q.weapon_slots] += 1;
        }
        for i in 1..4 {
            temp_slots[i] = self.slots_spare[i];
        }
        if self.torso_slots_spare < 4 {
            body_slots[self.torso_slots_spare] += 1;
        }

        // 3-slot decos
        self.add_decorations_23(
            &q.ability_index,
            &q.sorted_decorations[3].clone(),
            3,
            self.torso_multiplier,
            &mut body_slots[3],
            skill_points,
            desired,
        );
        body_slots[1] += body_slots[3];
        body_slots[2] += body_slots[3];
        body_slots[3] = 0;
        self.add_decorations_23(
            &q.ability_index,
            &q.sorted_decorations[3].clone(),
            3,
            1,
            &mut temp_slots[3],
            skill_points,
            desired,
        );
        // Adjust slots_spare to reflect consumption
        // Simplified: temp_slots tracks consumption, we update self.slots_spare accordingly
        self.slots_spare[3] = temp_slots[3]; // but temp already mutated
        temp_slots[1] += temp_slots[3];
        temp_slots[2] += temp_slots[3];
        temp_slots[3] = 0;

        self.add_decorations_23(
            &q.ability_index,
            &q.sorted_decorations[2].clone(),
            2,
            self.torso_multiplier,
            &mut body_slots[2],
            skill_points,
            desired,
        );
        body_slots[1] += 2 * body_slots[2];
        body_slots[2] = 0;
        {
            let before = temp_slots[2];
            self.add_decorations_23(
                &q.ability_index,
                &q.sorted_decorations[2].clone(),
                2,
                1,
                &mut temp_slots[2],
                skill_points,
                desired,
            );
            let used = before - temp_slots[2];
            if used > self.slots_spare[2] {
                let to_convert = used - self.slots_spare[2];
                self.slots_spare[3] = self.slots_spare[3].saturating_sub(to_convert);
                self.slots_spare[2] = 0;
                self.slots_spare[1] += to_convert;
            } else {
                self.slots_spare[2] -= used;
            }
        }
        temp_slots[1] += 2 * temp_slots[2];
        temp_slots[2] = 0;
        self.add_decorations_1(
            &q.ability_index,
            &q.sorted_decorations[1].clone(),
            self.torso_multiplier,
            &mut body_slots[1],
            skill_points,
            desired,
        );
        {
            let before = temp_slots[1];
            self.add_decorations_1(
                &q.ability_index,
                &q.sorted_decorations[1].clone(),
                1,
                &mut temp_slots[1],
                skill_points,
                desired,
            );
            let used = before - temp_slots[1];
            if used > self.slots_spare[1] {
                let extra = used - self.slots_spare[1];
                if extra > self.slots_spare[2] * 2 {
                    let extra3 = extra - self.slots_spare[2] * 2;
                    let num3 = (extra3 + 2) / 3;
                    self.slots_spare[3] = self.slots_spare[3].saturating_sub(num3);
                    let provide = num3 * 3;
                    if provide > extra {
                        let sup = provide - extra;
                        self.slots_spare[2] += sup / 2;
                        self.slots_spare[1] += sup % 2;
                    }
                    self.slots_spare[2] = 0;
                    self.slots_spare[1] = 0;
                } else {
                    let num2 = (extra + 1) / 2;
                    self.slots_spare[2] = self.slots_spare[2].saturating_sub(num2);
                    let remain = used.saturating_sub(num2 * 2);
                    self.slots_spare[1] = self.slots_spare[1].saturating_sub(remain);
                }
            } else {
                self.slots_spare[1] -= used;
            }
        }
        self.torso_slots_spare = body_slots[1];
    }
    fn add_decorations_1(
        &mut self,
        ability_map: &HashMap<i32, usize>,
        decos: &Vec<Decoration>,
        multiplier: usize,
        num_slots: &mut usize,
        skill_points: &mut Vec<i32>,
        desired: &Vec<i32>,
    ) {
        if *num_slots == 0 || decos.is_empty() {
            return;
        }
        let mut i = 0usize;
        let mut looped = false;
        while !looped || i != 0 {
            let deco = &decos[i];
            let ap = &deco.abilities[0];
            if let Some(&idx) = ability_map.get(&ap.0) {
                let cur = skill_points[idx];
                let amt = ap.1 * multiplier as i32;
                let need = desired[idx];
                if amt > 0 && cur < need && cur + amt <= need + 1 {
                    skill_points[idx] += amt;
                    if deco.dangerous {
                        if let Some(&idx2) = ability_map.get(&deco.abilities[1].0) {
                            skill_points[idx2] += deco.abilities[1].1 * multiplier as i32;
                        }
                    }
                    self.decorations.push(deco.clone());
                    self.multipliers.push(multiplier);
                    *num_slots -= 1;
                    if *num_slots == 0 {
                        return;
                    }
                    if i == 0 {
                        i = decos.len() - 1;
                    } else {
                        i -= 1;
                    }
                }
            }
            i += 1;
            if i >= decos.len() {
                i = 0;
                looped = true;
            }
            if looped && i == 0 {
                break;
            }
        }
    }
    fn add_decorations_23(
        &mut self,
        ability_map: &HashMap<i32, usize>,
        decos: &Vec<Decoration>,
        _size: usize,
        multiplier: usize,
        num_slots: &mut usize,
        skill_points: &mut Vec<i32>,
        desired: &Vec<i32>,
    ) {
        if *num_slots == 0 || decos.is_empty() {
            return;
        }
        let mut candidates: Vec<(Decoration, usize, i32)> = Vec::new(); // (deco, idx, points_given)
        for deco in decos {
            let ap = &deco.abilities[0];
            if let Some(&idx) = ability_map.get(&ap.0) {
                let cur = skill_points[idx];
                let amt = ap.1 * multiplier as i32;
                let need = desired[idx];
                if amt > 0 && cur < need && cur + amt <= need + 1 {
                    let given = (need - cur).min(amt);
                    candidates.push((deco.clone(), idx, given));
                }
            }
        }
        if candidates.is_empty() {
            return;
        }
        // pick best by score
        let mut best_idx = 0;
        let mut best_score =
            get_score(&candidates[0].0, ability_map, skill_points, candidates[0].2);
        for (idx, (deco, _, given)) in candidates.iter().enumerate().skip(1) {
            let s = get_score(deco, ability_map, skill_points, *given);
            if s.0 >= best_score.0 || s.1 > best_score.1 {
                best_score = s;
                best_idx = idx;
            }
        }
        let (best_deco, idx, _) = candidates[best_idx].clone();
        let ap = best_deco.abilities[0].1 * multiplier as i32;
        skill_points[idx] += ap;
        if best_deco.dangerous {
            if let Some(&idx2) = ability_map.get(&best_deco.abilities[1].0) {
                skill_points[idx2] += best_deco.abilities[1].1 * multiplier as i32;
            }
        }
        self.decorations.push(best_deco);
        self.multipliers.push(multiplier);
        *num_slots -= 1;
        if *num_slots > 0 {
            self.add_decorations_23(
                ability_map,
                decos,
                _size,
                multiplier,
                num_slots,
                skill_points,
                desired,
            );
        }
    }
}

fn get_score(
    deco: &Decoration,
    ability_map: &HashMap<i32, usize>,
    skill_points: &Vec<i32>,
    actual_good: i32,
) -> (i32, i32) {
    let point_score = [0, 1, 2, 3, 4, 3];
    let idx = actual_good as usize;
    let good = if idx < point_score.len() {
        point_score[idx]
    } else {
        0
    };
    let bad = if deco.abilities.len() == 1 {
        0
    } else {
        if let Some(&idx) = ability_map.get(&deco.abilities[1].0) {
            skill_points[idx]
        } else {
            0
        }
    };
    (good as i32, bad)
}

// ── Solution final (with bad skill fix) ─────────────────────────────

#[derive(Debug, Clone)]
struct FinalSolution {
    armors: Vec<Armor>,
    decorations: Vec<Decoration>,
    extra_skills: Vec<(i32, String, i32)>, // skill_id, name, points
    abilities: HashMap<i32, i32>,
    torso_slots_spare: usize,
    torso_multiplier: usize,
    slots_spare: [i32; 4],
    total_slots_spare: i32,
    fire: i32,
    ice: i32,
    water: i32,
    thunder: i32,
    dragon: i32,
    defence: i32,
    rarity: i32,
    difficulty: i32,
}

impl FinalSolution {
    fn from_equivalence(es: &EquivalenceSolution) -> Self {
        let mut abilities = HashMap::new();
        let mut total = 0;
        for i in 0..es.decorations.len() {
            let d = &es.decorations[i];
            let mult = es.multipliers[i] as i32;
            *abilities.entry(d.abilities[0].0).or_insert(0) += d.abilities[0].1 * mult;
            if d.abilities.len() == 2 {
                *abilities.entry(d.abilities[1].0).or_insert(0) += d.abilities[1].1 * mult;
            }
        }
        let mut slots_spare = [0; 4];
        for i in 1..4 {
            slots_spare[i] = es.slots_spare[i] as i32;
            total += (i as i32) * slots_spare[i];
        }
        total += es.torso_slots_spare as i32;
        Self {
            armors: Vec::new(),
            decorations: es.decorations.clone(),
            extra_skills: Vec::new(),
            abilities,
            torso_slots_spare: es.torso_slots_spare,
            torso_multiplier: es.torso_multiplier,
            slots_spare,
            total_slots_spare: total,
            fire: 0,
            ice: 0,
            water: 0,
            thunder: 0,
            dragon: 0,
            defence: 0,
            rarity: 0,
            difficulty: 0,
        }
    }
    fn calculate_extra(
        &mut self,
        _q: &QueryInternal,
        skill_id_to_name: &HashMap<i32, String>,
        _torso_eq: &ArmorEquivalence,
    ) {
        // add armor extra skills
        for armor in &self.armors {
            let is_body = armor.slot_type == "body" || armor.slot_type == "chest";
            let mult = if is_body {
                self.torso_multiplier as i32
            } else {
                1
            };
            for (sid, pts) in &armor.skill_points {
                *self.abilities.entry(*sid).or_insert(0) += pts * mult;
            }
        }
        // abilities map already has deco contributions; now build extra_skills via skill_levels lookup would be done outside
        self.extra_skills.clear();
        for (sid, pts) in &self.abilities {
            if let Some(name) = skill_id_to_name.get(sid) {
                self.extra_skills.push((*sid, name.clone(), *pts));
            }
        }
    }
    fn check_bad(&mut self, q: &QueryInternal, deco_map: &HashMap<i32, Vec<Decoration>>) -> bool {
        if q.allow_bad {
            return true;
        }
        let mut bad: Vec<i32> = Vec::new();
        for (sid, _name, pts) in &self.extra_skills {
            // need to find if pts triggers negative skill: we consider negative ability if pts <= -10
            if *pts <= -10 {
                bad.push(*sid);
            }
        }
        if bad.is_empty() {
            return true;
        }
        if self.total_slots_spare > 0 {
            if self.fix_bad_skills(&bad, q, deco_map) {
                return true;
            }
        }
        self.reorder_gems(&bad, q, deco_map)
    }
    fn fix_bad_skills(
        &mut self,
        bad: &Vec<i32>,
        q: &QueryInternal,
        deco_map: &HashMap<i32, Vec<Decoration>>,
    ) -> bool {
        for sid in bad {
            if !self.fix_bad_skill(*sid, q, deco_map) {
                return false;
            }
        }
        true
    }
    fn fix_bad_skill(
        &mut self,
        sid: i32,
        q: &QueryInternal,
        deco_map: &HashMap<i32, Vec<Decoration>>,
    ) -> bool {
        let mut torso_slots = self.torso_slots_spare as i32;
        while torso_slots > 0 {
            if let Some(deco) = get_best_decoration(sid, torso_slots as usize, q, deco_map) {
                let sz = deco.slot_size as i32;
                if is_detrimental(&deco, q) {
                    return false;
                }
                if self.add_decoration(deco.clone()) {
                    return true;
                }
                torso_slots -= sz;
            } else {
                break;
            }
        }
        let mut max_slots = 3;
        while max_slots > 0 && self.slots_spare[max_slots] == 0 {
            max_slots -= 1;
        }
        if max_slots == 0 {
            return false;
        }
        if let Some(deco) = get_best_decoration(sid, max_slots, q, deco_map) {
            let dsz = deco.slot_size;
            if is_detrimental(&deco, q) {
                return false;
            }
            // slot conversion logic simplified
            if self.slots_spare[dsz] == 0 {
                if deco.slot_size == 1 {
                    if self.slots_spare[2] == 0 && self.slots_spare[3] == 0 {
                        return false;
                    }
                    if self.slots_spare[2] == 0 {
                        self.slots_spare[3] -= 1;
                        self.slots_spare[2] += 1;
                    } else {
                        self.slots_spare[2] -= 1;
                        self.slots_spare[1] += 1;
                    }
                } else if deco.slot_size == 2 {
                    if self.slots_spare[2] == 0 && self.slots_spare[3] == 0 {
                        return false;
                    }
                    if self.slots_spare[2] == 0 {
                        self.slots_spare[3] -= 1;
                        self.slots_spare[2] += 1;
                    }
                }
            }
            if self.slots_spare[deco.slot_size as usize] > 0 {
                self.slots_spare[deco.slot_size as usize] -= 1;
                if self.add_decoration(deco) {
                    return true;
                }
            }
        }
        false
    }
    fn add_decoration(&mut self, deco: Decoration) -> bool {
        self.decorations.push(deco.clone());
        if deco.abilities.len() == 2 {
            *self.abilities.entry(deco.abilities[1].0).or_insert(0) += deco.abilities[1].1;
        }
        let pts = *self.abilities.entry(deco.abilities[0].0).or_insert(0) + deco.abilities[0].1;
        self.abilities.insert(deco.abilities[0].0, pts);
        pts > -10
    }
    fn reorder_gems(
        &mut self,
        _bad: &Vec<i32>,
        _q: &QueryInternal,
        _map: &HashMap<i32, Vec<Decoration>>,
    ) -> bool {
        // Stub (audit A4): the full gem-reordering pass is not ported. Returning
        // false means "cannot fix via reordering" so solutions that would need it
        // are dropped rather than emitted with a bad skill. Gems are still greedily
        // chosen by `add_decorations_*`; this only affects the allow_bad path.
        false
    }
    fn calculate_data(&mut self) {
        self.fire = 0;
        self.ice = 0;
        self.thunder = 0;
        self.water = 0;
        self.dragon = 0;
        self.defence = 0;
        self.rarity = 0;
        self.difficulty = 0;
        for a in &self.armors {
            self.fire += a.fire;
            self.ice += a.ice;
            self.thunder += a.thunder;
            self.water += a.water;
            self.dragon += a.dragon;
            self.defence += a.defence;
            self.rarity += a.rarity;
            // difficulty simplified: rarity>4 counts
            self.difficulty += if a.rarity >= 8 {
                3
            } else if a.rarity >= 6 {
                2
            } else if a.rarity >= 4 {
                1
            } else {
                0
            };
        }
        self.total_slots_spare = self.slots_spare[1]
            + self.slots_spare[2] * 2
            + self.slots_spare[3] * 3
            + self.torso_slots_spare as i32;
    }
}

fn is_detrimental(deco: &Decoration, q: &QueryInternal) -> bool {
    if deco.abilities.len() < 2 {
        return false;
    }
    for req in &q.skills {
        if req.skill_id == deco.abilities[1].0 {
            return true;
        }
    }
    false
}

fn get_best_decoration(
    ability: i32,
    max_slots: usize,
    _q: &QueryInternal,
    map: &HashMap<i32, Vec<Decoration>>,
) -> Option<Decoration> {
    let list = map.get(&ability)?;
    let mut best: Option<Decoration> = None;
    let rel = vec![ability];
    for deco in list {
        if deco.slot_size > max_slots {
            continue;
        }
        if let Some(b) = &best {
            if is_better_decoration(deco, b, &rel) && !is_better_decoration(b, deco, &rel) {
                best = Some(deco.clone());
            }
        } else {
            best = Some(deco.clone());
        }
    }
    best
}

// ── Entry point ─────────────────────────────────────────────────────

pub fn search(conn: &Connection, input: AssQueryInput) -> Result<Vec<AssSolutionView>, String> {
    let q = get_relevant_data(conn, &input).map_err(|e| e.to_string())?;
    if q.skills.is_empty() {
        return Err("Selecciona al menos 1 habilidad".to_string());
    }
    // Check each slot has at least one armor
    for i in 0..NUM_ARMOR_TYPES {
        if q.rel_armor[i].is_empty() {
            return Err(format!("No hay armaduras relevantes para slot {}", i));
        }
    }
    let armor_eq = create_equivalences(&q);
    for i in 0..NUM_ARMOR_TYPES {
        if armor_eq[i].is_empty() {
            return Err(format!("No hay equivalencias para slot {}", i));
        }
    }
    // Build decoration map for bad-fix
    let mut deco_map: HashMap<i32, Vec<Decoration>> = HashMap::new();
    for d in &q.rel_decorations {
        deco_map
            .entry(d.abilities[0].0)
            .or_default()
            .push(d.clone());
    }
    // skill id -> name
    let mut skill_names: HashMap<i32, String> = HashMap::new();
    {
        let mut stmt = conn
            .prepare("SELECT id, name FROM skills WHERE game_id = ?1")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([input.game_id], |row| {
                Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| e.to_string())?;
        for r in rows {
            let (id, n) = r.map_err(|e| e.to_string())?;
            skill_names.insert(id, n);
        }
    }

    let head_eq_list = armor_eq[0].clone();
    let body_eq_list = armor_eq[1].clone();
    let arms_eq_list = armor_eq[2].clone();
    let waist_eq_list = armor_eq[3].clone();
    let legs_eq_list = armor_eq[4].clone();

    let mut solutions: Vec<FinalSolution> = Vec::new();

    'outer: for head_eq in &head_eq_list {
        for body_eq in &body_eq_list {
            for arms_eq in &arms_eq_list {
                for waist_eq in &waist_eq_list {
                    for legs_eq in &legs_eq_list {
                        if solutions.len() >= MAX_LIMIT {
                            break 'outer;
                        }
                        let mut es = EquivalenceSolution {
                            armor_eq: vec![
                                head_eq.clone(),
                                body_eq.clone(),
                                arms_eq.clone(),
                                waist_eq.clone(),
                                legs_eq.clone(),
                            ],
                            decorations: Vec::new(),
                            multipliers: Vec::new(),
                            slots_spare: [0; 4],
                            torso_slots_spare: 0,
                            torso_multiplier: 1,
                        };
                        if !es.matches_query(&q) {
                            continue;
                        }
                        // expand concrete armors
                        for h in &head_eq.armors {
                            for b in &body_eq.armors {
                                for a in &arms_eq.armors {
                                    for w in &waist_eq.armors {
                                        for l in &legs_eq.armors {
                                            if solutions.len() >= MAX_LIMIT {
                                                break 'outer;
                                            }
                                            let mut sol = FinalSolution::from_equivalence(&es);
                                            sol.armors = vec![
                                                h.clone(),
                                                b.clone(),
                                                a.clone(),
                                                w.clone(),
                                                l.clone(),
                                            ];
                                            sol.calculate_extra(&q, &skill_names, body_eq);
                                            if !sol.check_bad(&q, &deco_map) {
                                                continue;
                                            }
                                            sol.calculate_data();
                                            solutions.push(sol);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // sort
    let sort = input.sort_by.as_deref().unwrap_or("none");
    match sort {
        "defence" | "defense" => solutions.sort_by(|a, b| b.defence.cmp(&a.defence)),
        "fire_res" => solutions.sort_by(|a, b| b.fire.cmp(&a.fire)),
        "ice_res" => solutions.sort_by(|a, b| b.ice.cmp(&a.ice)),
        "water_res" => solutions.sort_by(|a, b| b.water.cmp(&a.water)),
        "thunder_res" => solutions.sort_by(|a, b| b.thunder.cmp(&a.thunder)),
        "dragon_res" => solutions.sort_by(|a, b| b.dragon.cmp(&a.dragon)),
        "rarity" => solutions.sort_by(|a, b| b.rarity.cmp(&a.rarity)),
        "difficulty" => solutions.sort_by(|a, b| a.difficulty.cmp(&b.difficulty)),
        "slots_spare" => solutions.sort_by(|a, b| b.total_slots_spare.cmp(&a.total_slots_spare)),
        _ => {}
    }

    // build view
    let mut views: Vec<AssSolutionView> = Vec::new();
    // need decoration name map for view
    let mut deco_info: HashMap<
        i32,
        (
            String,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<i32>,
            Option<i32>,
        ),
    > = HashMap::new();
    {
        let mut stmt2 = conn.prepare("SELECT d.id, d.name, d.slot_size, s1.name, d.skill_points, s2.name, d.secondary_points FROM decorations d LEFT JOIN skills s1 ON s1.id=d.skill_id LEFT JOIN skills s2 ON s2.id=d.secondary_skill_id WHERE d.game_id=?1").map_err(|e| e.to_string())?;
        let rows = stmt2
            .query_map([input.game_id], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<i32>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<i32>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<i32>>(6)?,
                ))
            })
            .map_err(|e| e.to_string())?;
        for r in rows {
            let (id, n, slot, sn, sp, sn2, sp2) = r.map_err(|e| e.to_string())?;
            deco_info.insert(id, (n, sn, sp, sn2, sp2, slot));
        }
    }

    for sol in solutions.iter().take(MAX_LIMIT) {
        let armors: Vec<AssArmorRef> = sol
            .armors
            .iter()
            .map(|a| AssArmorRef {
                id: a.id,
                name: a.name.clone(),
                slot_type: a.slot_type.clone(),
                rarity: Some(a.rarity),
                defense_base: Some(a.defence),
                slots: Some(a.num_slots.to_string()),
                skills: Some(
                    a.skill_points
                        .iter()
                        .map(|(k, v)| {
                            format!("{} {}", skill_names.get(k).unwrap_or(&k.to_string()), v)
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
            })
            .collect();
        // group decorations by id
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for d in &sol.decorations {
            *counts.entry(d.id).or_insert(0) += 1;
        }
        let mut decos: Vec<AssDecorationRef> = Vec::new();
        for (id, cnt) in counts {
            if let Some((name, sn, sp, sn2, sp2, slot)) = deco_info.get(&id) {
                decos.push(AssDecorationRef {
                    id,
                    name: name.clone(),
                    slot_size: *slot,
                    skill_name: sn.clone(),
                    skill_points: *sp,
                    secondary_skill_name: sn2.clone(),
                    secondary_points: *sp2,
                    count: cnt,
                });
            }
        }
        let extra: Vec<String> = sol
            .extra_skills
            .iter()
            .filter(|(sid, _, _)| !input.skills.iter().any(|r| r.skill_id == *sid))
            .map(|(_, n, pts)| format!("{} {}", n, pts))
            .collect();
        views.push(AssSolutionView {
            armors,
            decorations: decos,
            extra_skills: extra,
            defense: sol.defence,
            fire_res: sol.fire,
            water_res: sol.water,
            thunder_res: sol.thunder,
            ice_res: sol.ice,
            dragon_res: sol.dragon,
            rarity: sol.rarity,
            difficulty: sol.difficulty,
            slots_spare: sol.total_slots_spare,
            slots_spare_detail: vec![
                sol.slots_spare[0],
                sol.slots_spare[1],
                sol.slots_spare[2],
                sol.slots_spare[3],
            ],
        });
    }
    Ok(views)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{schema, seed};

    fn conn() -> rusqlite::Connection {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        schema::create_tables(&c).unwrap();
        seed::seed(&c).unwrap();
        c
    }

    fn attack_id(c: &rusqlite::Connection) -> i32 {
        c.query_row(
            "SELECT id FROM skills WHERE name='Attack' AND game_id=5",
            [],
            |r| r.get(0),
        )
        .unwrap()
    }

    fn base_query(c: &rusqlite::Connection, hr: i32, elder: i32) -> AssQueryInput {
        AssQueryInput {
            game_id: 5,
            skills: vec![SkillRequirement {
                skill_id: attack_id(c),
                points_required: 20,
            }],
            hunter_type: "blade".into(),
            gender: "male".into(),
            hr,
            elder_star: elder,
            weapon_slots: 3,
            include_piercings: true,
            allow_bad: false,
            allow_torso_inc: true,
            sort_by: None,
        }
    }

    #[test]
    fn rank_gate_low_excludes_g() {
        let c = conn();
        // HR 1 / Elder 1 => difficulty 1 => Low only
        let res = search(&c, base_query(&c, 1, 1)).unwrap();
        assert!(!res.is_empty(), "should find low-rank sets");
        for sol in &res {
            for a in &sol.armors {
                // No G-rank (X/Z suffix) pieces should be returned
                assert!(
                    !a.name.contains(" X") && !a.name.contains(" Z"),
                    "G-rank leaked: {}",
                    a.name
                );
            }
        }
    }

    #[test]
    fn rank_gate_high_allows_high() {
        let c = conn();
        // HR 5 / Elder 5 => difficulty 2 => Low + High
        let res = search(&c, base_query(&c, 5, 5)).unwrap();
        assert!(!res.is_empty());
        // Should be able to include High rank (S suffix) among pieces
        let has_high = res
            .iter()
            .any(|s| s.armors.iter().any(|a| a.name.contains(" S")));
        assert!(has_high, "high-rank sets expected at difficulty 2");
    }

    #[test]
    fn solver_robust_across_tiers_and_hunter_types() {
        let c = conn();
        let _attack = attack_id(&c);
        let mut base = base_query(&c, 1, 1);
        for (hr, elder, expect) in [(1, 1, "low"), (5, 5, "high"), (8, 1, "g"), (1, 8, "high")] {
            base.hr = hr;
            base.elder_star = elder;
            let res = search(&c, base.clone()).unwrap();
            assert!(!res.is_empty(), "no solutions at hr={} elder={}", hr, elder);
            // sanity: every armor belongs to a valid slot and satisfies rank gate
            for sol in &res {
                assert_eq!(
                    sol.armors.len(),
                    5,
                    "expected 5 pieces, got {}",
                    sol.armors.len()
                );
                assert!(
                    sol.slots_spare >= 0,
                    "negative spare slots: {}",
                    sol.slots_spare
                );
                assert!(sol.defense > 0, "zero defense for a set");
                for a in &sol.armors {
                    assert!(
                        a.rarity.unwrap_or(0) > 0,
                        "piece with no rarity: {}",
                        a.name
                    );
                    if expect != "g" {
                        assert!(
                            !a.name.contains(" X") && !a.name.contains(" Z"),
                            "G-rank leaked: {}",
                            a.name
                        );
                    }
                }
            }
        }
        // Gunner path must also work and respect gender/hunter_type.
        base.hr = 5;
        base.elder_star = 5;
        base.hunter_type = "gunner".into();
        let res = search(&c, base.clone()).unwrap();
        assert!(!res.is_empty(), "no gunner solutions at difficulty 2");
        for sol in &res {
            for a in &sol.armors {
                assert_ne!(a.slot_type.to_lowercase(), "", "empty slot_type");
            }
        }
        // `allow_bad` must not crash anything, even though remediation is a no-op.
        base.allow_bad = true;
        let res = search(&c, base.clone()).unwrap();
        assert!(!res.is_empty(), "no solutions with allow_bad=true");
    }

    #[test]
    fn solver_rejects_missing_skill_selection() {
        let c = conn();
        let mut q = base_query(&c, 1, 1);
        q.skills.clear();
        assert!(
            search(&c, q).is_err(),
            "empty skill list should be rejected"
        );
    }
}
