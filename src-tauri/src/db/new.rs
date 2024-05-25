use crate::db::schema;
use crate::db::{
    structs::{
        Event, EventEntry, NewEvent, NewEventEntry, NewParticipant, NewTeam, Participant, Team,
    },
    Database,
};
use crate::prelude::*;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind as DBErrorKind;
use diesel::result::Error::DatabaseError as DBError;

fn map_err(e: diesel::result::Error) -> Error {
    match e {
        DBError(kind, _) => match kind {
            DBErrorKind::UniqueViolation => Error::UniqueConstraintFailed(e.to_string()),
            DBErrorKind::NotNullViolation => Error::FieldIsRequired(e.to_string()),
            DBErrorKind::CheckViolation => Error::CheckViolation(e.to_string()),
            _ => Error::DatabaseNewEntryFailure(e.to_string()),
        },
        _ => Error::DatabaseNewEntryFailure(e.to_string()),
    }
}

impl Database {
    pub fn new_team(&self, name: &str, individual: bool) -> Result<Team> {
        use schema::team;

        if name.is_empty() {
            return Err(Error::FieldIsRequired(":team.name".into()));
        } else if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
            return Err(Error::MustOnlyContainLettersAndSpaces(":team.name".into()));
        }

        let mut connection = self.connect()?;
        let new_team = NewTeam { name, individual };

        let db_team = diesel::insert_into(team::table)
            .values(&new_team)
            .returning(Team::as_returning())
            .get_result(&mut connection)
            .map_err(map_err)?;

        Ok(db_team)
    }

    pub fn new_participant(
        &self,
        first_name: &str,
        last_name: &str,
        team_id: i32,
    ) -> Result<Participant> {
        use schema::participant;

        if first_name.is_empty() {
            return Err(Error::FieldIsRequired(":.first name".into()));
        } else if !first_name.chars().all(|c| c.is_alphabetic()) {
            return Err(Error::MustOnlyContainLetters(":.first name".into()));
        }

        if last_name.is_empty() {
            return Err(Error::FieldIsRequired(":.last name".into()));
        } else if !last_name.chars().all(|c| c.is_alphabetic()) {
            return Err(Error::MustOnlyContainLetters(":.last name".into()));
        }

        if team_id <= 0 {
            return Err(Error::InvalidTeam);
        }

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
            .map_err(map_err)?;

        Ok(db_participant)
    }

    pub fn new_event(&self, name: &str, event_type: &str) -> Result<Event> {
        use schema::event;

        if name.is_empty() {
            return Err(Error::FieldIsRequired(":event.name".into()));
        } else if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
            return Err(Error::MustOnlyContainLettersAndSpaces(":event.name".into()));
        }

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
            .map_err(map_err)?;

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
            .map_err(map_err)?;

        Ok(db_event_entry)
    }
}
