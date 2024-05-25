use crate::db::structs::{Event, Participant, Team};
use crate::utils::connect_to_db;
use tauri::AppHandle;

macro_rules! create_get_command {
    ($fn_name:ident, $return_type:ty) => {
        #[tauri::command(rename_all = "snake_case")]
        pub fn $fn_name(app: AppHandle, item_id: i32) -> Result<$return_type, String> {
            let database = connect_to_db(app);
            let item = database.$fn_name(item_id).map_err(|e| e.to_string())?;
            Ok(item)
        }
    };
}

create_get_command!(get_team, Team);
create_get_command!(get_participant, Participant);
create_get_command!(get_event, Event);
