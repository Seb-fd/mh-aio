mod commands;
pub mod db;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();

            let db_path = app_dir.join("mh-aio.db");
            let db = db::Database::new(db_path.to_str().unwrap())
                .expect("failed to initialize database");

            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_games,
            commands::get_monsters,
            commands::get_weapons,
            commands::get_armor,
            commands::get_quests,
            commands::get_items,
            commands::get_skills,
            commands::get_monster_detail,
            commands::get_weapon_detail,
            commands::get_armor_detail,
            commands::get_quest_detail,
            commands::get_item_detail,
            commands::get_skill_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
