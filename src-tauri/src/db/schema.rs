// @generated automatically by Diesel CLI.

diesel::table! {
    event (id) {
        id -> Integer,
        name -> Text,
        event_type -> Text,
    }
}

diesel::table! {
    event_entry (team_id, event_id) {
        team_id -> Integer,
        event_id -> Integer,
    }
}

diesel::table! {
    participant (id) {
        id -> Integer,
        team_id -> Integer,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    team (id) {
        id -> Integer,
        name -> Text,
        individual -> Bool,
        points -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    event,
    event_entry,
    participant,
    team,
);
