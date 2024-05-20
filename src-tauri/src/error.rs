use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Failed to create SQLite Database.\n{0}")]
    DatabaseCreationError(String),

    #[error("Failed to connect to the SQLite Database.\n{0}")]
    DatabaseConnectionFailure(diesel::ConnectionError),

    #[error("Failed to push migrations to the SQLite Database.\n{0}")]
    DatabaseMigrationFailure(
        std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
    ),

    #[error("Failed to add a new entry into table: {0}")]
    DatabaseNewEntryFailure(String),

    #[error("Event type {0} is incorrect. Allowed values are ACADEMICS and SPORTS.")]
    IncorrectEventType(String),
}
