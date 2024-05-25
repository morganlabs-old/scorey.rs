use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Failed to create SQLite Database.")]
    DatabaseCreation(String),

    #[error("Failed to connect to the SQLite Database.")]
    DatabaseConnectionFailure,

    #[error("Failed to push migrations to the SQLite Database.")]
    DatabaseMigrationFailure,

    #[error("Failed to create a new entry in the SQLite Database: {0}")]
    DatabaseNewEntryFailure(String),

    #[error("Event type {0} is incorrect. Allowed values are ACADEMIC and SPORT.")]
    IncorrectEventType(String),

    #[error("Failed to query database.")]
    DatabaseQueryFailure(String),

    #[error("Failed to update value(s) in database.\n{0}")]
    DatabaseUpdateEntryFailure(String),

    #[error("Cannot delete a non-individual team that contains members.")]
    CannotDeleteNonIndividualTeamWithMembers,

    #[error("A field must be unique.\n{0}")]
    UniqueConstraintFailed(String),

    #[error("A required field is empty.\n")]
    FieldIsRequired(String),

    #[error("The field {0} has violated a check.")]
    CheckViolation(String),

    #[error("The field {0} must only contain letters and spaces.")]
    MustOnlyContainLettersAndSpaces(String),

    #[error("The field {0} must only contain letters.")]
    MustOnlyContainLetters(String),

    #[error("Please select a valid team.")]
    InvalidTeam,
}
