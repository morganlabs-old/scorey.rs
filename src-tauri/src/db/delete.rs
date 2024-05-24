use crate::db::{schema, structs::EventEntry, Database};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn delete_event_entry(&self, team_id: i32, event_id: i32) -> Result<()> {
        use schema::event_entry::dsl;

        let mut connection = self.connect()?;
        diesel::delete(
            dsl::event_entry
                .filter(dsl::team_id.eq(team_id))
                .filter(dsl::event_id.eq(event_id)),
        )
        .execute(&mut connection)
        .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(())
    }

    pub fn delete_participant(&self, participant_id: i32) -> Result<()> {
        use schema::participant::dsl::*;

        let mut connection = self.connect()?;
        diesel::delete(participant.filter(id.eq(participant_id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(())
    }
}
