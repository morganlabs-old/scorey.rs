#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod prelude;

use db::structs::{Event, EventEntry, Participant, ParticipantAndTeam, Team};
use db::Database;
use error::Error;
use std::path::PathBuf;
use std::process::exit;
use tauri::Manager;
#[allow(unused_imports)]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    tauri::Builder::default()
        .setup(|app_handle| {
            let app_data_dir = app_handle.path_resolver().app_data_dir();
            let app_data_dir = match app_data_dir {
                Some(d) => d,
                None => panic!("Failed to get application data directory!"),
            };

            let window = app_handle.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Database::default().init(app_data_dir)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            new_team,
            new_participant,
            new_event,
            enroll_team_in_events,
            get_teams,
            get_team,
            get_team_events,
            get_participants,
            get_events,
            update_team,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}

#[tauri::command(rename_all = "snake_case")]
fn get_team_events(app_handle: tauri::AppHandle, team_id: i32) -> Result<Vec<i32>, String> {
    let database = connect_to_db(app_handle);
    let team_events = database
        .get_team_events(team_id)
        .map_err(|e| e.to_string())?;

    Ok(team_events)
}

#[tauri::command(rename_all = "snake_case")]
fn update_team(
    app_handle: tauri::AppHandle,
    id: i32,
    name: String,
    individual: bool,
    points: i32,
) -> Result<Team, String> {
    let database = connect_to_db(app_handle);
    let updated_team = database
        .update_team(id, name, individual, points)
        .map_err(|e| e.to_string())?;

    Ok(updated_team)
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
fn enroll_team_in_events(
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

#[tauri::command(rename_all = "snake_case")]
fn get_participants(app_handle: tauri::AppHandle) -> Result<Vec<ParticipantAndTeam>, String> {
    let database = connect_to_db(app_handle);
    let participants = database.get_participants().map_err(|e| e.to_string())?;
    Ok(participants)
}

#[tauri::command(rename_all = "snake_case")]
fn get_teams(app_handle: tauri::AppHandle) -> Result<Vec<Team>, String> {
    let database = connect_to_db(app_handle);
    let teams = database.get_teams().map_err(|e| e.to_string())?;
    Ok(teams)
}

#[tauri::command(rename_all = "snake_case")]
fn get_team(app_handle: tauri::AppHandle, team_id: i32) -> Result<Team, String> {
    let database = connect_to_db(app_handle);
    let team = database.get_team(team_id).map_err(|e| e.to_string())?;
    Ok(team)
}

#[tauri::command(rename_all = "snake_case")]
fn get_events(app_handle: tauri::AppHandle) -> Result<Vec<Event>, String> {
    let database = connect_to_db(app_handle);
    let events = database.get_events().map_err(|e| e.to_string())?;
    Ok(events)
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
