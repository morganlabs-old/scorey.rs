use crate::db::schema;
use crate::db::{
    structs::{
        Event, EventEntry, NewEvent, NewEventEntry, NewParticipant, NewTeam, Participant, Team,
    },
    Database,
};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn new_team(&self, name: &str, individual: bool) -> Result<Team> {
        use schema::team;

        let mut connection = self.connect()?;
        let new_team = NewTeam { name, individual };

        let db_team = diesel::insert_into(team::table)
            .values(&new_team)
            .returning(Team::as_returning())
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseNewEntryFailure(e.to_string()))?;

        Ok(db_team)
    }

    pub fn new_participant(
        &self,
        first_name: &str,
        last_name: &str,
        team_id: i32,
    ) -> Result<Participant> {
        use schema::participant;

        let mut connection = self.connect()?;
        let new_participant = NewParticipant {
            first_name,
            last_name,
            team_id,
        };

        let db_participant = diesel::insert_into(participant::table)
            .values(&new_participant)
            .returning(Participant::as_returning())
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseNewEntryFailure(e.to_string()))?;

        Ok(db_participant)
    }

    pub fn new_event(&self, name: &str, event_type: &str) -> Result<Event> {
        use schema::event;

        let event_type = match event_type {
            "ACADEMIC" | "SPORT" => event_type,
            _ => return Err(Error::IncorrectEventType(event_type.into())),
        };

        let mut connection = self.connect()?;
        let new_event = NewEvent { name, event_type };

        let mut db_event = diesel::insert_into(event::table)
            .values(&new_event)
            .returning(Event::as_returning())
            .get_result::<Event>(&mut connection)
            .map_err(|e| Error::DatabaseNewEntryFailure(e.to_string()))?;

        if db_event.event_type == "ACADEMIC" {
            db_event.event_type = "Academics".into()
        } else if db_event.event_type == "SPORT" {
            db_event.event_type = "Sports".into()
        };

        Ok(db_event)
    }

    pub fn new_event_entry(&self, team_id: i32, event_id: i32) -> Result<EventEntry> {
        use schema::event_entry;

        let mut connection = self.connect()?;
        let new_event_entry = NewEventEntry { team_id, event_id };

        let db_event_entry = diesel::insert_into(event_entry::table)
            .values(&new_event_entry)
            .returning(EventEntry::as_returning())
            .get_result::<EventEntry>(&mut connection)
            .map_err(|e| Error::DatabaseNewEntryFailure(e.to_string()))?;

        Ok(db_event_entry)
    }
}
