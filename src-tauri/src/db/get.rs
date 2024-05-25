use crate::db::{
    schema,
    structs::{Event, Participant, Team},
    Database,
};
use crate::prelude::*;
use diesel::prelude::*;

macro_rules! create_get_fn {
    ($fn_name:ident, $schema:ident, $return_type:ty) => {
        pub fn $fn_name(&self, item_id: i32) -> Result<$return_type> {
            use schema::$schema::dsl::*;

            let mut connection = self.connect()?;
            let db_obj = $schema
                .filter(id.eq(item_id))
                .first::<$return_type>(&mut connection)
                .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

            Ok(db_obj)
        }
    };
}

impl Database {
    create_get_fn!(get_team, team, Team);
    create_get_fn!(get_event, event, Event);
    create_get_fn!(get_participant, participant, Participant);

    pub fn get_team_members(&self, team_id: i32) -> Result<Vec<Participant>> {
        use schema::participant::dsl;

        let mut connection = self.connect()?;
        let team_members = dsl::participant
            .filter(dsl::team_id.eq(team_id))
            .load::<Participant>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(team_members)
    }

    pub fn get_team_events(&self, team_id: i32) -> Result<Vec<i32>> {
        use schema::event_entry::dsl;

        let mut connection = self.connect()?;
        let team_events = dsl::event_entry
            .filter(dsl::team_id.eq(team_id))
            .select(dsl::event_id)
            .load::<i32>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(team_events)
    }
}
