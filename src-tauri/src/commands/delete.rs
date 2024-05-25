use crate::utils::connect_to_db;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn delete_participant(app: AppHandle, participant_id: i32) -> Result<(), String> {
    let database = connect_to_db(app);
    database
        .delete_participant(participant_id, true)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_team(app: AppHandle, team_id: i32) -> Result<(), String> {
    let database = connect_to_db(app);
    database.delete_team(team_id).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_event(app: AppHandle, event_id: i32) -> Result<(), String> {
    let database = connect_to_db(app);
    database.delete_event(event_id).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn unenroll_team_in_events(
    app: AppHandle,
    team_id: i32,
    event_id: i32,
) -> core::result::Result<(), String> {
    let database = connect_to_db(app);
    database
        .delete_event_entry(team_id, event_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}
