use crate::db::{schema, Database};
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn delete_participant(
        &self,
        participant_id: i32,
        delete_team_if_individual: bool,
    ) -> Result<()> {
        use schema::participant::dsl::*;

        let mut connection = self.connect()?;
        let db_participant = self.get_participant(participant_id)?;
        let db_team = self.get_team(db_participant.team_id);

        if let Ok(team) = db_team {
            if team.individual && delete_team_if_individual {
                self.delete_team(team.id)?;
            }
        }

        diesel::delete(participant.filter(id.eq(participant_id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

    pub fn delete_team(&self, team_id: i32) -> Result<()> {
        use schema::team::dsl::*;

        let db_team = self.get_team(team_id)?;
        let team_members = self.get_team_members(team_id)?;
        let events = self.get_team_events(team_id)?;

        if !db_team.individual && !team_members.is_empty() {
            return Err(Error::ValidationCannotDeleteNonIndividualTeamWithMembers);
        } else if db_team.individual && team_members.len() == 1 {
            let member = team_members.first().unwrap();
            self.delete_participant(member.id, false)?;
        }

        for event in events {
            self.delete_event_entry(team_id, event)?;
        }

        let mut connection = self.connect()?;
        diesel::delete(team.filter(id.eq(team_id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

    pub fn delete_event(&self, event_id: i32) -> Result<()> {
        use schema::event::dsl::*;

        let teams_enrolled = self.get_teams_enrolled_in_event(event_id)?;

        if !teams_enrolled.is_empty() {
            return Err(Error::ValidationCannotDeleteEventsWithTeamsEnrolled);
        }

        let mut connection = self.connect()?;
        diesel::delete(event.filter(id.eq(event_id)))
            .execute(&mut connection)
            .map_err(|e| Error::DatabaseDeleteEntryFailure(e.to_string()))?;

        Ok(())
    }

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
