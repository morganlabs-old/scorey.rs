use crate::db::{
    schema,
    structs::{Event, Participant, Team},
    Database,
};
use crate::error::Error;
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn update_team(
        &self,
        id: i32,
        name: String,
        individual: bool,
        points: i32,
    ) -> Result<Team> {
        use schema::team::dsl;

        let mut connection = self.connect()?;
        let updated_team = diesel::update(dsl::team.filter(dsl::id.eq(id)))
            .set((
                dsl::name.eq(name),
                dsl::individual.eq(individual),
                dsl::points.eq(points),
            ))
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseUpdateEntryFailure(e.to_string()))?;

        Ok(updated_team)
    }

    pub fn update_event(&self, id: i32, name: String, event_type: String) -> Result<Event> {
        use schema::event::dsl;

        let mut connection = self.connect()?;
        let updated_event = diesel::update(dsl::event.filter(dsl::id.eq(id)))
            .set((dsl::name.eq(name), dsl::event_type.eq(event_type)))
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseUpdateEntryFailure(e.to_string()))?;

        Ok(updated_event)
    }

    pub fn update_participant(
        &self,
        id: i32,
        team_id: i32,
        first_name: String,
        last_name: String,
    ) -> Result<Participant> {
        use schema::participant::dsl;

        let mut connection = self.connect()?;
        let updated_participant = diesel::update(dsl::participant.filter(dsl::id.eq(id)))
            .set((
                dsl::team_id.eq(team_id),
                dsl::first_name.eq(first_name),
                dsl::last_name.eq(last_name),
            ))
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseUpdateEntryFailure(e.to_string()))?;

        Ok(updated_participant)
    }
}
