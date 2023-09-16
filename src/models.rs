use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Stream {
    pub id: i32,
    pub home: String,
    pub away: String,
    pub start_time: String,
    pub league: String,
    pub country: String,
}

#[derive(Debug, Insertable)]
#[table_name = "stream"]
pub struct UserNew<'a> {
    pub home: &'a str,
    pub away: &'a str,
    pub start_time: &'a str,
    pub league: &'a str,
    pub country: &'a str,
}
