use crate::db::structs::{Event, ParticipantAndTeam, Team};
use crate::utils::connect_to_db;
use tauri::AppHandle;

macro_rules! create_get_many_command {
    ($fn_name:ident, $return_type:ty) => {
        #[tauri::command(rename_all = "snake_case")]
        pub fn $fn_name(app: AppHandle) -> Result<Vec<$return_type>, String> {
            let database = connect_to_db(app);
            let item = database.$fn_name().map_err(|e| e.to_string())?;
            Ok(item)
        }
    };
}

create_get_many_command!(get_teams, Team);
create_get_many_command!(get_participants, ParticipantAndTeam);
create_get_many_command!(get_events, Event);

#[tauri::command(rename_all = "snake_case")]
pub fn get_team_events(app: AppHandle, id: i32) -> Result<Vec<i32>, String> {
    let database = connect_to_db(app);
    let team_events = database.get_team_events(id).map_err(|e| e.to_string())?;

    Ok(team_events)
}
