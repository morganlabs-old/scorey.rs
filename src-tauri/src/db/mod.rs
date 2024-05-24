mod schema;
pub mod structs;

mod delete;
mod get;
mod get_many;
mod new;
mod update;

use crate::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs;
use std::path::PathBuf;

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
    pub fn init(&mut self, app_data_dir: PathBuf, run_migrations: bool) -> Result<()> {
        self.path.push(app_data_dir);
        self.path.push("scorey.db");

        self.ensure_db_exists()?;
        if run_migrations {
            self.run_migrations()?;
        };

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

        SqliteConnection::establish(path).map_err(|_| Error::DatabaseConnectionFailure)
    }

    fn run_migrations(&self) -> Result<()> {
        println!("Running any possible database migrations...");
        let mut connection = self.connect()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|_| Error::DatabaseMigrationFailure)?;

        Ok(())
    }

    // Ensure Stuff Happens

    fn ensure_db_dir_exists(&self) -> Result<()> {
        let dir = &self.path.parent().unwrap();
        if !dir.exists() {
            fs::create_dir(dir).map_err(|_| {
                let dir = &dir.to_str().unwrap_or("<Failed to unwrap path>");
                Error::DatabaseCreation(f!("Failed to create database directory at {dir}"))
            })?;
        }

        Ok(())
    }

    fn ensure_db_exists(&self) -> Result<()> {
        self.ensure_db_dir_exists()?;
        let path = &self.path;

        if !path.exists() {
            fs::File::create(path).map_err(|_| {
                let path = &path.to_str().unwrap_or("<Failed to unwrap path>");
                Error::DatabaseCreation(f!("Failed to create database file at {path}"))
            })?;
        }

        Ok(())
    }
}
