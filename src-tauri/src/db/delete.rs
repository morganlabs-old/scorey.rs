use crate::db::{schema, Database};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    /// Delete a participant from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the participant to delete
    /// * `delete_team_if_individual` - Whether or not to delete the team associated with the participant if the team is marked as "individual."
    ///
    /// # Errors
    ///
    /// * If the participant does not exist
    /// * If the delete fails
    /// * If the connection to the database fails
    pub fn delete_participant(&self, id: i32, delete_team_if_individual: bool) -> Result<()> {
        use schema::participant::dsl;

        let mut connection = self.connect()?;
        let db_participant = self.get_participant(id)?;
        let db_team = self.get_team(db_participant.team_id);

        if delete_team_if_individual {
            if let Ok(team) = db_team {
                if team.individual {
                    self.delete_team(team.id)?;
                }
            }
        }

        diesel::delete(dsl::participant.filter(dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

    /// Delete a team from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the team to delete
    ///
    /// # Errors
    ///
    /// * If the team does not exist
    /// * If the team is not an individual team and has members
    /// * If the delete fails
    /// * If the connection to the database fails
    pub fn delete_team(&self, id: i32) -> Result<()> {
        use schema::team::dsl;

        let db_team = self.get_team(id)?;
        let team_members = self.get_team_members(id)?;
        let events = self.get_team_events(id)?;

        if !db_team.individual && !team_members.is_empty() {
            return Err(Error::ValidationCannotDeleteNonIndividualTeamWithMembers);
        } else if db_team.individual && team_members.len() == 1 {
            let member = team_members.first().unwrap();
            self.delete_participant(member.id, false)?;
        }

        for event in events {
            self.delete_event_entry(id, event)?;
        }

        let mut connection = self.connect()?;
        diesel::delete(dsl::team.filter(dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

    /// Delete an event from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the event to delete
    ///
    /// # Errors
    ///
    /// * If the event does not exist
    /// * If the event has teams enrolled
    /// * If the delete fails
    pub fn delete_event(&self, id: i32) -> Result<()> {
        use schema::event::dsl;

        let teams_enrolled = self.get_teams_enrolled_in_event(id)?;

        if !teams_enrolled.is_empty() {
            return Err(Error::ValidationCannotDeleteEventsWithTeamsEnrolled);
        }

        let mut connection = self.connect()?;
        diesel::delete(dsl::event.filter(dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

    /// Delete an event entry from the database.
    ///
    /// # Arguments
    ///
    /// * `team_id` - The ID of the team to delete the event entry for
    /// * `event_id` - The ID of the event to delete the entry for
    ///
    /// # Errors
    ///
    /// * If the event entry does not exist
    /// * If the delete fails
    /// * If the connection to the database fails
    pub fn delete_event_entry(&self, team_id: i32, event_id: i32) -> Result<()> {
        use schema::event_entry::dsl;

        let mut connection = self.connect()?;
        diesel::delete(
            dsl::event_entry
                .filter(dsl::team_id.eq(team_id))
                .filter(dsl::event_id.eq(event_id)),
        )
        .execute(&mut connection)
        .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }
}
