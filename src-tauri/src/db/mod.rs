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
    /// Initialise the database file.
    ///
    /// # Arguments
    ///
    /// * `app_data_dir` - The application data directory.
    /// * `run_migrations` - Whether to run migrations.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// * If the database file cannot be created.
    /// * If the database connection fails.
    /// * If the migrations fail.
    pub fn init(&mut self, app_data_dir: PathBuf, run_migrations: bool) -> Result<()> {
        self.path.push(app_data_dir);
        self.path.push("scorey.db");

        self.ensure_db_exists()?;
        if run_migrations {
            self.run_migrations()?;
        };

        Ok(())
    }

    /// Connect to the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing the database connection.
    ///
    /// # Errors
    ///
    /// * If the connection fails.
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

    /// Run any pending database migrations.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// * If the migrations fail.
    fn run_migrations(&self) -> Result<()> {
        println!("Running any possible database migrations...");
        let mut connection = self.connect()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|_| Error::DatabaseMigrationFailure)?;

        Ok(())
    }

    // Ensure Stuff Happens

    /// Ensure the database directory exists.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// * If the directory cannot be created.
    fn ensure_db_dir_exists(&self) -> Result<()> {
        let dir = &self.path.parent().unwrap();
        if !dir.exists() {
            fs::create_dir(dir).map_err(|_| {
                let dir = &dir.to_str().unwrap_or("<Failed to unwrap path>");
                Error::DatabaseDirectoryCreation(dir.to_string())
            })?;
        }

        Ok(())
    }

    /// Ensure the database file exists.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// * If the database directory cannot be created.
    /// * If the database cannot be created.
    fn ensure_db_exists(&self) -> Result<()> {
        self.ensure_db_dir_exists()?;
        let path = &self.path;

        if !path.exists() {
            fs::File::create(path).map_err(|_| Error::DatabaseCreation(path.clone()))?;
        }

        Ok(())
    }
}
