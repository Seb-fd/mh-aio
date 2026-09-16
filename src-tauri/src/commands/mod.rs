use crate::ass;
use crate::builds;
use crate::db::queries;
use crate::db::Database;
use crate::favorites;
use crate::import;
use tauri::{Manager, State};

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

#[tauri::command]
pub fn get_melder_recipes(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::MelderRecipe>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_melder_recipes_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mhw_mantles(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::MhwMantle>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_mhw_mantles_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mhw_mantle_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::MhwMantle>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_mhw_mantle_detail(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_palico_gadgets(
    db: State<'_, Database>,
    game_id: i32,
) -> Result<Vec<queries::PalicoGadget>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_palico_gadgets_by_game(&conn, game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_palico_gadget_detail(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<queries::PalicoGadgetDetail>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    queries::get_palico_gadget_detail(&conn, id).map_err(|e| e.to_string())
}

fn app_data_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path().app_data_dir().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_builds(
    app: tauri::AppHandle,
    game_id: Option<i32>,
) -> Result<Vec<builds::SavedBuild>, String> {
    builds::list_builds(&app_data_dir(&app)?, game_id)
}

#[tauri::command]
pub fn save_build(
    app: tauri::AppHandle,
    name: String,
    game_id: i32,
    query: ass::AssQueryInput,
    solution: ass::AssSolutionView,
) -> Result<builds::SavedBuild, String> {
    builds::save_build(&app_data_dir(&app)?, name, game_id, query, solution)
}

#[tauri::command]
pub fn get_build(app: tauri::AppHandle, id: String) -> Result<Option<builds::SavedBuild>, String> {
    builds::get_build(&app_data_dir(&app)?, &id)
}

#[tauri::command]
pub fn delete_build(app: tauri::AppHandle, id: String) -> Result<bool, String> {
    builds::delete_build(&app_data_dir(&app)?, &id)
}

#[tauri::command]
pub fn import_build(app: tauri::AppHandle, json: String) -> Result<builds::SavedBuild, String> {
    builds::import_build(&app_data_dir(&app)?, &json)
}

#[tauri::command]
pub fn list_favorites(
    app: tauri::AppHandle,
    game_id: Option<i32>,
) -> Result<Vec<favorites::Favorite>, String> {
    favorites::list_favorites(&app_data_dir(&app)?, game_id)
}

#[tauri::command]
pub fn toggle_favorite(
    app: tauri::AppHandle,
    game_id: i32,
    kind: String,
    id: i32,
    name: String,
) -> Result<bool, String> {
    favorites::toggle_favorite(&app_data_dir(&app)?, game_id, kind, id, name)
}

#[tauri::command]
pub fn remove_favorite(
    app: tauri::AppHandle,
    game_id: i32,
    kind: String,
    id: i32,
) -> Result<bool, String> {
    favorites::remove_favorite(&app_data_dir(&app)?, game_id, &kind, id)
}

#[tauri::command]
pub fn preview_import(
    db: State<'_, Database>,
    game_id: i32,
    kind: String,
    rows: Vec<serde_json::Value>,
) -> Result<import::ImportPreview, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    import::preview_import(&conn, game_id, &kind, &rows)
}

#[tauri::command]
pub fn apply_import(
    db: State<'_, Database>,
    game_id: i32,
    kind: String,
    rows: Vec<serde_json::Value>,
) -> Result<import::ImportResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    import::apply_import(&conn, game_id, &kind, &rows)
}
