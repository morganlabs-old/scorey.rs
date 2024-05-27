use crate::db::{
    schema,
    structs::{Event, Participant, Team},
    Database,
};
use crate::prelude::*;
use diesel::prelude::*;

macro_rules! create_get_fn {
    ($(#[$outer:meta])* $fn_name:ident, $schema:ident, $return_type:ty) => {
        $(#[$outer])*
        ///
        /// # Arguments
        ///
        /// * `id` - The ID to use to get the entry
        ///
        /// # Return
        ///
        /// The entry with the corresponding ID
        ///
        /// # Errors
        ///
        /// * If the entry does not exist
        /// * If the query fails
        /// * If the connection to the database fails
        pub fn $fn_name(&self, id: i32) -> Result<$return_type> {
            use schema::$schema::dsl;

            let mut connection = self.connect()?;
            let db_obj = dsl::$schema
                .filter(dsl::id.eq(id))
                .first::<$return_type>(&mut connection)
                .map_err(|e| Error::DatabaseQueryFailure(e.to_string()))?;

            Ok(db_obj)
        }
    };
}

impl Database {
    create_get_fn![
        /// Gets a team from the database
        get_team,
        team,
        Team
    ];
    create_get_fn![
        /// Gets an event from the database
        get_event,
        event,
        Event
    ];
    create_get_fn![
        /// Gets a participant from the database
        get_participant,
        participant,
        Participant
    ];
}
