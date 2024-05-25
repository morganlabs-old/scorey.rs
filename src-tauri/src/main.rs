#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod error;
mod prelude;
mod utils;

use commands::*;
use db::Database;
use tauri::Manager;
#[allow(unused_imports)]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path_resolver().app_data_dir();
            let app_data_dir = match app_data_dir {
                Some(d) => d,
                None => panic!("Failed to get application data directory!"),
            };

            Database::default().init(app_data_dir, true)?;

            #[allow(unused_variables)]
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            new_team,
            new_participant,
            new_event,
            enroll_team_in_events,
            get_team,
            get_participant,
            get_event,
            get_teams,
            get_participants,
            get_events,
            get_team_events,
            update_team,
            update_participant,
            update_event,
            delete_team,
            delete_participant,
            delete_event,
            unenroll_team_in_events,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
