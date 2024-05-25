use crate::db::structs::{Event, ParticipantAndTeam, Team};
use crate::utils::connect_to_db;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn get_team_events(app: AppHandle, team_id: i32) -> Result<Vec<i32>, String> {
    let database = connect_to_db(app);
    let team_events = database
        .get_team_events(team_id)
        .map_err(|e| e.to_string())?;

    Ok(team_events)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_teams(app: AppHandle) -> Result<Vec<Team>, String> {
    let database = connect_to_db(app);
    let teams = database.get_teams().map_err(|e| e.to_string())?;
    Ok(teams)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_participants(app: AppHandle) -> Result<Vec<ParticipantAndTeam>, String> {
    let database = connect_to_db(app);
    let participants = database.get_participants().map_err(|e| e.to_string())?;
    Ok(participants)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_events(app: AppHandle) -> Result<Vec<Event>, String> {
    let database = connect_to_db(app);
    let events = database.get_events().map_err(|e| e.to_string())?;
    Ok(events)
}
