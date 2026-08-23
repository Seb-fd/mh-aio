use crate::db::Database;
use crate::db::queries;
use tauri::State;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Welcome, {}! This is MH-AIO 🎮", name)
}

#[tauri::command]
pub fn get_games(db: State<'_, Database>) -> Result<Vec<queries::Game>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let games = queries::get_games(&conn).map_err(|e| e.to_string())?;
    Ok(games)
}

#[tauri::command]
pub fn get_monsters(db: State<'_, Database>, game_id: i32) -> Result<Vec<queries::Monster>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let monsters = queries::get_monsters_by_game(&conn, game_id)
        .map_err(|e| e.to_string())?;
    Ok(monsters)
}
