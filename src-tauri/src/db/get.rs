use crate::db::{
    schema,
    structs::{Event, Participant, Team},
    Database,
};
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

    pub fn get_team_members(&self, team_id: i32) -> Result<Vec<Participant>> {
        use schema::participant::dsl;

        let mut connection = self.connect()?;
        let team_members = dsl::participant
            .filter(dsl::team_id.eq(team_id))
            .load::<Participant>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(team_members)
    }

    pub fn get_event(&self, event_id: i32) -> Result<Event> {
        use schema::event::dsl::*;

        let mut connection = self.connect()?;
        let db_event = event
            .filter(id.eq(event_id))
            .first::<Event>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(db_event)
    }

    pub fn get_participant(&self, participant_id: i32) -> Result<Participant> {
        use schema::participant::dsl::*;

        let mut connection = self.connect()?;
        let db_participant = participant
            .filter(id.eq(participant_id))
            .first::<Participant>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(db_participant)
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
