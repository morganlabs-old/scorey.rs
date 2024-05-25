use crate::db::structs::{Event, Participant, Team};
use crate::utils::connect_to_db;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn update_team(
    app: AppHandle,
    id: i32,
    name: String,
    individual: bool,
    points: i32,
) -> Result<Team, String> {
    let database = connect_to_db(app);
    let updated_team = database
        .update_team(id, name, individual, points)
        .map_err(|e| e.to_string())?;

    Ok(updated_team)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_participant(
    app: AppHandle,
    id: i32,
    team_id: i32,
    first_name: String,
    last_name: String,
) -> Result<Participant, String> {
    let database = connect_to_db(app);
    let updated_participant = database
        .update_participant(id, team_id, first_name, last_name)
        .map_err(|e| e.to_string())?;

    Ok(updated_participant)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_event(
    app: AppHandle,
    id: i32,
    name: String,
    event_type: String,
) -> Result<Event, String> {
    let database = connect_to_db(app);
    let updated_event = database
        .update_event(id, name, event_type)
        .map_err(|e| e.to_string())?;

    Ok(updated_event)
}
