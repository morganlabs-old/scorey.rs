mod schema;
pub mod structs;

use crate::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs::File;
use std::path::PathBuf;
use structs::{
    Event, EventEntry, NewEvent, NewEventEntry, NewParticipant, NewTeam, Participant, Team,
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }
}

impl Database {
    pub fn init(&mut self, app_data_dir: PathBuf) -> Result<()> {
        self.path.push(app_data_dir);
        self.path.push("scorey.db");

        self.ensure_db_exists()?;
        self.run_migrations()?;

        Ok(())
    }

    fn connect(&self) -> Result<SqliteConnection> {
        let path = &self.path.to_str();
        let path = match path {
            Some(p) => p,
            None => {
                return Err(Error::Generic(
                    "Failed to convert PathBuf to String.".into(),
                ))
            }
        };

        SqliteConnection::establish(path).map_err(Error::DatabaseConnectionFailure)
    }

    fn run_migrations(&self) -> Result<()> {
        println!("Running any possible database migrations...");
        let mut connection = self.connect()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(Error::DatabaseMigrationFailure)?;

        Ok(())
    }

    // New Items

    pub fn new_team(&self, name: &str, individual: bool) -> Result<Team> {
        use schema::team;

        let mut connection = self.connect()?;
        let new_team = NewTeam { name, individual };

        let db_team = diesel::insert_into(team::table)
            .values(&new_team)
            .returning(Team::as_returning())
            .get_result::<Team>(&mut connection)
            .map_err(|_| Error::DatabaseNewEntryFailure("Team".into()))?;

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
            .get_result::<Participant>(&mut connection)
            .map_err(|_| Error::DatabaseNewEntryFailure("Participant".into()))?;

        Ok(db_participant)
    }

    pub fn new_event(&self, name: &str, event_type: &str) -> Result<Event> {
        use schema::event;

        let event_type = match event_type {
            "ACADEMICS" | "SPORTS" => event_type,
            _ => return Err(Error::IncorrectEventType(event_type.into())),
        };

        let mut connection = self.connect()?;
        let new_event = NewEvent { name, event_type };

        let db_event = diesel::insert_into(event::table)
            .values(&new_event)
            .returning(Event::as_returning())
            .get_result::<Event>(&mut connection)
            .map_err(|_| Error::DatabaseNewEntryFailure("Event".into()))?;

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
            .map_err(|_| Error::DatabaseNewEntryFailure("EventEntry".into()))?;

        Ok(db_event_entry)
    }

    // Ensure Stuff Happens

    fn ensure_db_dir_exists(&self) -> Result<()> {
        let dir = &self.path.parent().unwrap();
        if !dir.exists() {
            File::create(dir).map_err(|_| {
                let dir = &dir.to_str().unwrap_or("<Failed to unwrap path>");
                Error::DatabaseCreationError(f!("Failed to create database directory at {dir}"))
            })?;
        }

        Ok(())
    }

    fn ensure_db_exists(&self) -> Result<()> {
        self.ensure_db_dir_exists()?;
        let path = &self.path;

        if !path.exists() {
            File::create(path).map_err(|_| {
                let path = &path.to_str().unwrap_or("<Failed to unwrap path>");
                Error::DatabaseCreationError(f!("Failed to create database file at {path}"))
            })?;
        }

        Ok(())
    }
}
