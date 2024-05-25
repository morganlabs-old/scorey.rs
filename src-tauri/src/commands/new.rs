use crate::db::structs::{Event, EventEntry, Participant, Team};
use crate::error::Error;
use crate::utils::connect_to_db;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn new_team(app: AppHandle, name: &str, individual: bool) -> Result<Team, Error> {
    let database = connect_to_db(app);
    let new_team = database.new_team(name, individual)?;

    Ok(new_team)
}

#[tauri::command(rename_all = "snake_case")]
pub fn new_participant(
    app: AppHandle,
    first_name: &str,
    last_name: &str,
    team_id: i32,
) -> Result<Participant, Error> {
    let database = connect_to_db(app);
    let new_participant = database.new_participant(first_name, last_name, team_id)?;

    Ok(new_participant)
}

#[tauri::command(rename_all = "snake_case")]
pub fn new_event(app: AppHandle, name: &str, event_type: &str) -> Result<Event, Error> {
    let database = connect_to_db(app);
    let new_event = database.new_event(name, event_type)?;

    Ok(new_event)
}

#[tauri::command(rename_all = "snake_case")]
pub fn enroll_team_in_events(
    app: AppHandle,
    team_id: i32,
    event_id: i32,
) -> Result<EventEntry, Error> {
    let database = connect_to_db(app);
    let new_event_entry = database.new_event_entry(team_id, event_id)?;

    Ok(new_event_entry)
}
