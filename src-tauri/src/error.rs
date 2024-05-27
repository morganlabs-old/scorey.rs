use std::path::PathBuf;

use serde::Serialize;
use thiserror::Error;

/// An enum containing all errors that Scorey should ever emit.
#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "info")]
pub enum Error {
    /// A generic "catch-all" error type.
    #[error("An error occurred. {0}")]
    Generic(String),

    // Database Errors
    /// Occurs when a suitable for the SQLite database cannot be created.
    ///
    /// This is usually caused by insufficient permissions.
    #[error("Failed to create SQLite Database directory at path {0}.")]
    DatabaseDirectoryCreation(String),

    /// Occurs when the SQLite `scorey.db` file cannot be created.
    ///
    /// This is usually caused by insufficient permissions.
    #[error("Failed to create SQLite Database at path {0}.")]
    DatabaseCreation(PathBuf),

    /// Occurs when there is a failure when attempting to connect to the SQLite database.
    #[error("Failed to connect to the SQLite Database.")]
    DatabaseConnectionFailure,

    /// Occurs when there is an issue when pushing any pending migrations to the SQLite databse.
    #[error("Failed to push migrations to the SQLite Database.")]
    DatabaseMigrationFailure,

    /// Occurs whenever there is an issue querying the database.
    #[error("Failed to query database.")]
    DatabaseQueryFailure(String),

    /// Occurs whenever there is an issue creating a new entry in the database.
    #[error("Failed to create a new entry in the SQLite Database: {0}")]
    DatabaseNewEntryFailure(String),

    /// Occurs when there is an issue deleting an entry in the database.
    #[error("Failed to delete an entry in the SQLite Database: {0}")]
    DatabaseDeleteEntryFailure(String),

    /// Occurs when there is an issue updating an entry in the database.
    #[error("Failed to update value(s) in database.\n{0}")]
    DatabaseUpdateEntryFailure(String),

    // Validation Errors
    /// Occurs swhen the event type selected for a new event is not "ACADEMIC" or "SPORT"
    #[error("Event type {0} is incorrect. Allowed values are ACADEMIC and SPORT.")]
    ValidationIncorrectEventType(String),

    /// Occurs when a field that must be unique is not.
    #[error("A field must be unique.\n{0}")]
    ValidationUniqueConstraintFailed(String),

    /// Occurs when a field has violated a CHECK in the database.
    #[error("The field {0} has violated a CHECK.")]
    ValidationCheckViolation(String),

    /// Occurs when a required field is empty.
    #[error("A required field is empty.\n")]
    ValidationFieldIsRequired(String),

    /// Occurs when a field must only contain letters and spaces, but it contains other characters.
    #[error("The field {0} must only contain letters and spaces.")]
    ValidationMustOnlyContainLettersAndSpaces(String),

    /// Occurs when a field must only contain letters, but it contains other characters.
    #[error("The field {0} must only contain letters.")]
    ValidationMustOnlyContainLetters(String),

    /// Occurs when a team that is NOT marked as `individual` is attempted to be deleted, but it has members attached to it.
    ///
    /// Any teams marked as `individual` will automatically delete the associated individual participant.
    #[error("Cannot delete a non-individual team that contains members.")]
    ValidationCannotDeleteNonIndividualTeamWithMembers,

    /// Occurs when someone attempts to delete an event that has teams enrolled.
    #[error("Cannot delete an event that has teams enrolled.")]
    ValidationCannotDeleteEventsWithTeamsEnrolled,

    /// Occurs when a user attempts to create a participant with either a team ID of 0, or a team ID that does not exist.
    ///
    /// If using the Scorey GUI, this error should never occur.
    #[error("Please select a valid team.")]
    ValidationInvalidTeam,
}
