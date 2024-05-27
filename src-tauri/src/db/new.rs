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

/// A simple helper function to quickly and easily map the errors output by Diesel into
/// Scorey-specific errors.
///
/// # Arguments
///
/// * `e` - The error to map.
///
/// # Returns
///
/// The mapped error.
fn map_err(e: diesel::result::Error) -> Error {
    match e {
        DBError(kind, _) => match kind {
            DBErrorKind::UniqueViolation => Error::ValidationUniqueConstraintFailed(e.to_string()),
            DBErrorKind::NotNullViolation => Error::ValidationFieldIsRequired(e.to_string()),
            DBErrorKind::CheckViolation => Error::ValidationCheckViolation(e.to_string()),
            _ => Error::DatabaseNewEntryFailure(e.to_string()),
        },
        _ => Error::DatabaseNewEntryFailure(e.to_string()),
    }
}

impl Database {
    /// Create a new team in the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the team.
    /// * `individual` - Whether the team is an individual team.
    ///
    /// # Returns
    ///
    /// The newly created team.
    ///
    /// # Errors
    ///
    /// * If the name is empty.
    /// * If the name contains characters other than letters and spaces.
    /// * If the creation fails.
    /// * If the connection to the database fails.
    pub fn new_team(&self, name: &str, individual: bool) -> Result<Team> {
        use schema::team;

        if name.is_empty() {
            return Err(Error::ValidationFieldIsRequired(":team.name".into()));
        } else if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
            return Err(Error::ValidationMustOnlyContainLettersAndSpaces(
                ":team.name".into(),
            ));
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

    /// Create a new participant in the database.
    ///
    /// # Arguments
    ///
    /// * `first_name` - The first name of the participant.
    /// * `last_name` - The last name of the participant.
    /// * `team_id` - The ID of the team the participant is on.
    ///
    /// # Returns
    ///
    /// The newly created participant.
    ///
    /// # Errors
    ///
    /// * If the first name is empty.
    /// * If the first name contains characters other than letters.
    /// * If the last name is empty.
    /// * If the last name contains characters other than letters.
    /// * If the team ID is less than or equal to 0.
    /// * If the creation fails.
    /// * If the connection to the database fails.
    pub fn new_participant(
        &self,
        first_name: &str,
        last_name: &str,
        team_id: i32,
    ) -> Result<Participant> {
        use schema::participant;

        if first_name.is_empty() {
            return Err(Error::ValidationFieldIsRequired(":.first name".into()));
        } else if !first_name.chars().all(|c| c.is_alphabetic()) {
            return Err(Error::ValidationMustOnlyContainLetters(
                ":.first name".into(),
            ));
        }

        if last_name.is_empty() {
            return Err(Error::ValidationFieldIsRequired(":.last name".into()));
        } else if !last_name.chars().all(|c| c.is_alphabetic()) {
            return Err(Error::ValidationMustOnlyContainLetters(
                ":.last name".into(),
            ));
        }

        if team_id <= 0 {
            return Err(Error::ValidationInvalidTeam);
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

    /// Create a new event in the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the event.
    /// * `event_type` - The type of the event.
    ///
    /// # Returns
    ///
    /// The newly created event.
    ///
    /// # Errors
    ///
    /// * If the name is empty.
    /// * If the name contains characters other than letters and spaces.
    /// * If the event type is not "ACADEMIC" or "SPORT".
    /// * If the creation fails.
    /// * If the connection to the database fails.
    pub fn new_event(&self, name: &str, event_type: &str) -> Result<Event> {
        use schema::event;

        if name.is_empty() {
            return Err(Error::ValidationFieldIsRequired(":event.name".into()));
        } else if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
            return Err(Error::ValidationMustOnlyContainLettersAndSpaces(
                ":event.name".into(),
            ));
        }

        let event_type = match event_type {
            "ACADEMIC" | "SPORT" => event_type,
            _ => return Err(Error::ValidationIncorrectEventType(event_type.into())),
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

    /// Create a new event entry in the database.
    ///
    /// # Arguments
    ///
    /// * `team_id` - The ID of the team that is entering the event.
    /// * `event_id` - The ID of the event that the team is entering.
    ///
    /// # Returns
    ///
    /// The newly created event entry.
    ///
    /// # Errors
    ///
    /// * If the team ID is less than or equal to 0.
    /// * If the event ID is less than or equal to 0.
    /// * If the creation fails.
    /// * If the connection to the database fails.
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
