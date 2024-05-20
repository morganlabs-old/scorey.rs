#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod prelude;

use db::Database;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    tauri::Builder::default()
        .setup(|app_handle| {
            let app_data_dir = app_handle.path_resolver().app_data_dir();
            let app_data_dir = match app_data_dir {
                Some(d) => d,
                None => panic!("Failed to get application data directory!"),
            };

            Database::default().init(app_data_dir)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![new_team, new_participant])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}

#[tauri::command]
fn new_team(
    app_handle: tauri::AppHandle,
    name: &str,
    individual: bool,
) -> core::result::Result<(), String> {
    let database = connect_to_db(app_handle);
    database
        .new_team(name, individual)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn new_participant(
    app_handle: tauri::AppHandle,
    first_name: &str,
    last_name: &str,
    team_id: i32,
) -> core::result::Result<(), String> {
    let database = connect_to_db(app_handle);
    database
        .new_participant(first_name, last_name, team_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn connect_to_db(app_handle: tauri::AppHandle) -> Database {
    let app_data_dir = get_app_data_dir(app_handle);

    let mut database = Database::default();
    if let Err(e) = database.init(app_data_dir) {
        println!("{e:?}");
        exit(1);
    }

    database
}

fn get_app_data_dir(app_handle: tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle.path_resolver().app_data_dir();
    match app_data_dir {
        Some(d) => d,
        None => panic!("Failed to get application data directory!"),
    }
}
