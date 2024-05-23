use crate::db::schema;
use crate::db::{
    structs::{Event, ParticipantAndTeam, Team},
    Database,
};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn get_teams(&self) -> Result<Vec<Team>> {
        use schema::team::dsl::*;

        let mut connection = self.connect()?;
        let teams = team
            .load::<Team>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(teams)
    }

    pub fn get_participants(&self) -> Result<Vec<ParticipantAndTeam>> {
        use schema::participant::dsl::*;
        use schema::team::dsl as team_dsl;

        let mut connection = self.connect()?;
        let participants = participant
            .left_join(team_dsl::team.on(team_dsl::id.eq(team_id)))
            .select((
                id,
                first_name,
                last_name,
                team_id,
                team_dsl::name.nullable(),
                team_dsl::individual.nullable(),
                team_dsl::points.nullable(),
            ))
            .load::<ParticipantAndTeam>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(participants)
    }

    pub fn get_events(&self) -> Result<Vec<Event>> {
        use schema::event::dsl::*;

        let mut connection = self.connect()?;
        let events = event
            .load::<Event>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        let events = events
            .into_iter()
            .map(|mut e| {
                if e.event_type == "ACADEMIC" {
                    e.event_type = "Academics".into()
                } else if e.event_type == "SPORT" {
                    e.event_type = "Sports".into()
                };

                e
            })
            .collect();

        Ok(events)
    }
}
