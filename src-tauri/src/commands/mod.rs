use crate::ass;
use crate::db::queries;
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn get_monsters(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::Monster>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_monsters_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_weapons(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Weapon>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_weapons_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_armor(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Armor>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_armor_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_quests(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Quest>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_quests_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_items(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Item>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_items_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skills(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Skill>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_skills_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_monster_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::MonsterDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_monster_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_monster_dedicated_sets(
    db: State<'_, Database>,
    monster_id: i32,
    rank: Option<String>,
) -> Result<Vec<queries::ArmorSetDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_monster_dedicated_sets(&conn, monster_id, rank.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_weapon_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::WeaponDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_weapon_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_armor_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::ArmorDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_armor_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_quest_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::QuestDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_quest_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_item_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::ItemDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_item_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skill_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::SkillDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_skill_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_decorations(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::Decoration>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_decorations_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_decoration_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::DecorationDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_decoration_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_armor_sets(
    db: State<'_, Database>,
    query: ass::AssQueryInput,
) -> Result<Vec<ass::AssSolutionView>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    ass::search(&conn, query)
}

#[tauri::command]
pub fn get_armor_sets(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::ArmorSet>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_armor_sets_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn global_search(
    db: State<'_, Database>,
    game_id: i32,
    query: String,
) -> Result<Vec<queries::SearchResult>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_global_search(&conn, game_id, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_armor_set_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::ArmorSetDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_armor_set_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_combinations(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::CombineView>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_combinations_by_game(&conn, game_id).map_err(|e| e.to_string())
}
