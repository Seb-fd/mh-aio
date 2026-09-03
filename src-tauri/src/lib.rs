mod ass;
mod commands;
pub mod db;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            if !app_dir.exists() {
                std::fs::create_dir_all(&app_dir)?;
            }

            let db_path = app_dir.join("mh-aio.db");
            let db = db::Database::new(&db_path)?;

            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_monsters,
            commands::get_weapons,
            commands::get_armor,
            commands::get_quests,
            commands::get_items,
            commands::get_skills,
            commands::get_monster_detail,
            commands::get_monster_dedicated_sets,
            commands::get_weapon_detail,
            commands::get_armor_detail,
            commands::get_armor_sets,
            commands::get_armor_set_detail,
            commands::global_search,
            commands::get_quest_detail,
            commands::get_item_detail,
            commands::get_skill_detail,
            commands::get_decorations,
            commands::get_decoration_detail,
            commands::search_armor_sets,
            commands::get_combinations,
            commands::get_melder_recipes,
            commands::get_mhw_mantles,
            commands::get_mhw_mantle_detail,
            commands::get_palico_gadgets,
            commands::get_palico_gadget_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
