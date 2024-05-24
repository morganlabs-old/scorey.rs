use crate::db::{schema, structs::Team, Database};
use crate::error::Error;
use crate::prelude::*;
use diesel::prelude::*;

impl Database {
    pub fn update_team(
        &self,
        id: i32,
        name: String,
        individual: bool,
        points: i32,
    ) -> Result<Team> {
        use schema::team::dsl;

        let mut connection = self.connect()?;
        let updated_team = diesel::update(dsl::team.filter(dsl::id.eq(id)))
            .set((
                dsl::name.eq(name),
                dsl::individual.eq(individual),
                dsl::points.eq(points),
            ))
            .get_result(&mut connection)
            .map_err(|e| Error::DatabaseUpdateEntryFailure(e.to_string()))?;

        Ok(updated_team)
    }
}
