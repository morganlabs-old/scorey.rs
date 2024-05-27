use crate::prelude::*;
use crate::utils::connect_to_db;
use tauri::AppHandle;

macro_rules! create_delete_command {
    ($fn_name:ident) => {
        #[tauri::command(rename_all = "snake_case")]
        pub fn $fn_name(app: AppHandle, id: i32) -> Result<()> {
            let database = connect_to_db(app);
            database.$fn_name(id)?;
            Ok(())
        }
    };
}

create_delete_command!(delete_team);
create_delete_command!(delete_event);

#[tauri::command(rename_all = "snake_case")]
pub fn delete_participant(app: AppHandle, id: i32) -> Result<()> {
    let database = connect_to_db(app);
    database.delete_participant(id, true)?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn unenroll_team_in_events(app: AppHandle, team_id: i32, event_id: i32) -> Result<()> {
    let database = connect_to_db(app);
    database.delete_event_entry(team_id, event_id)?;

    Ok(())
}
