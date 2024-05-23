use crate::db::schema::*;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use serde::Serialize;

#[derive(Serialize, Debug, Queryable)]
pub struct ParticipantAndTeam {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub team_id: i32,
    pub team_name: Option<String>,
    pub team_individual: Option<bool>,
    pub team_points: Option<i32>,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = participant)]
#[diesel(check_for_backend(Sqlite))]
pub struct Participant {
    pub id: i32,
    pub team_id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = participant)]
pub struct NewParticipant<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub team_id: i32,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = team)]
#[diesel(check_for_backend(Sqlite))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub individual: bool,
    pub points: i32,
}

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = team)]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub individual: bool,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = event)]
#[diesel(check_for_backend(Sqlite))]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub event_type: String,
}

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = event)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub event_type: &'a str,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = event_entry)]
#[diesel(check_for_backend(Sqlite))]
pub struct EventEntry {
    pub team_id: i32,
    pub event_id: i32,
}

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = event_entry)]
pub struct NewEventEntry {
    pub team_id: i32,
    pub event_id: i32,
}
