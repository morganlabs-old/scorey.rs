use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "info")]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    // Database Errors
    #[error("Failed to create SQLite Database directory at path {0}.")]
    DatabaseDirectoryCreation(String),

    #[error("Failed to create SQLite Database at path {0}.")]
    DatabaseCreation(String),

    #[error("Failed to connect to the SQLite Database.")]
    DatabaseConnectionFailure,

    #[error("Failed to push migrations to the SQLite Database.")]
    DatabaseMigrationFailure,

    #[error("Failed to query database.")]
    DatabaseQueryFailure(String),

    #[error("Failed to create a new entry in the SQLite Database: {0}")]
    DatabaseNewEntryFailure(String),

    #[error("Failed to delete an entry in the SQLite Database: {0}")]
    DatabaseDeleteEntryFailure(String),

    #[error("Failed to update value(s) in database.\n{0}")]
    DatabaseUpdateEntryFailure(String),

    // Validation Errors
    #[error("Event type {0} is incorrect. Allowed values are ACADEMIC and SPORT.")]
    ValidationIncorrectEventType(String),

    #[error("A field must be unique.\n{0}")]
    ValidationUniqueConstraintFailed(String),

    #[error("The field {0} has violated a check.")]
    ValidationCheckViolation(String),

    #[error("A required field is empty.\n")]
    ValidationFieldIsRequired(String),

    #[error("The field {0} must only contain letters and spaces.")]
    ValidationMustOnlyContainLettersAndSpaces(String),

    #[error("The field {0} must only contain letters.")]
    ValidationMustOnlyContainLetters(String),

    #[error("Cannot delete a non-individual team that contains members.")]
    ValidationCannotDeleteNonIndividualTeamWithMembers,

    #[error("Cannot delete an event that has teams enrolled.")]
    ValidationCannotDeleteEventsWithTeamsEnrolled,

    #[error("Please select a valid team.")]
    ValidationInvalidTeam,
}
