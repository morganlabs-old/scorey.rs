#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::Database;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path_resolver().app_data_dir();
            let app_data_dir = match app_data_dir {
                Some(d) => d,
                None => panic!("Failed to get application data directory!"),
            };

            Database::default().init(app_data_dir);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
