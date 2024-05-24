use crate::db::schema;
use crate::db::{structs::Team, Database};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn get_team(&self, team_id: i32) -> Result<Team> {
        use schema::team::dsl::*;

        let mut connection = self.connect()?;
        let db_team = team
            .filter(id.eq(team_id))
            .first::<Team>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(db_team)
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
