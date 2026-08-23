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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
