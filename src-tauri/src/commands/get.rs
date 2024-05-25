use crate::db::structs::{Event, Participant, Team};
use crate::utils::connect_to_db;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn get_team(app: AppHandle, team_id: i32) -> Result<Team, String> {
    let database = connect_to_db(app);
    let team = database.get_team(team_id).map_err(|e| e.to_string())?;
    Ok(team)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_participant(app: AppHandle, participant_id: i32) -> Result<Participant, String> {
    let database = connect_to_db(app);
    let participant = database
        .get_participant(participant_id)
        .map_err(|e| e.to_string())?;
    Ok(participant)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_event(app: AppHandle, event_id: i32) -> Result<Event, String> {
    let database = connect_to_db(app);
    let event = database.get_event(event_id).map_err(|e| e.to_string())?;
    Ok(event)
}
