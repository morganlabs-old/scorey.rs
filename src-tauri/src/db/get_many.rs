use crate::db::schema;
use crate::db::{
    structs::{Event, EventEntry, Participant, ParticipantAndTeam, Team},
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

    pub fn get_teams_enrolled_in_event(&self, id: i32) -> Result<Vec<i32>> {
        let event_entries = self.get_event_entries()?;
        let enrolled = event_entries
            .into_iter()
            .filter(|e| e.event_id == id)
            .map(|e| e.team_id)
            .collect::<Vec<_>>();

        Ok(enrolled)
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

    pub fn get_team_events(&self, id: i32) -> Result<Vec<i32>> {
        use schema::event_entry::dsl;

        let mut connection = self.connect()?;
        let team_events = dsl::event_entry
            .filter(dsl::team_id.eq(id))
            .select(dsl::event_id)
            .load::<i32>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(team_events)
    }

    pub fn get_event_entries(&self) -> Result<Vec<EventEntry>> {
        use schema::event_entry::dsl::*;

        let mut connection = self.connect()?;
        let event_entries = event_entry
            .load::<EventEntry>(&mut connection)
            .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

        Ok(event_entries)
    }
}
