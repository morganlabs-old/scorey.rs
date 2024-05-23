#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod prelude;

use db::structs::{Event, EventEntry, Participant, Team};
use db::Database;
use error::Error;
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
        .invoke_handler(tauri::generate_handler![
            new_team,
            new_participant,
            new_event,
            new_event_entry
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}

#[tauri::command(rename_all = "snake_case")]
fn new_team(
    app_handle: tauri::AppHandle,
    name: &str,
    individual: bool,
) -> core::result::Result<Team, Error> {
    let database = connect_to_db(app_handle);
    let new_team = database.new_team(name, individual)?;

    Ok(new_team)
}

#[tauri::command(rename_all = "snake_case")]
fn new_participant(
    app_handle: tauri::AppHandle,
    first_name: &str,
    last_name: &str,
    team_id: i32,
) -> core::result::Result<Participant, String> {
    let database = connect_to_db(app_handle);
    let new_participant = database
        .new_participant(first_name, last_name, team_id)
        .map_err(|e| e.to_string())?;

    Ok(new_participant)
}

#[tauri::command(rename_all = "snake_case")]
fn new_event(
    app_handle: tauri::AppHandle,
    name: &str,
    event_type: &str,
) -> core::result::Result<Event, String> {
    let database = connect_to_db(app_handle);
    let new_event = database
        .new_event(name, event_type)
        .map_err(|e| e.to_string())?;

    Ok(new_event)
}

#[tauri::command(rename_all = "snake_case")]
fn new_event_entry(
    app_handle: tauri::AppHandle,
    team_id: i32,
    event_id: i32,
) -> core::result::Result<EventEntry, String> {
    let database = connect_to_db(app_handle);
    let new_event_entry = database
        .new_event_entry(team_id, event_id)
        .map_err(|e| e.to_string())?;

    Ok(new_event_entry)
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
