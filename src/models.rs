use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Stream {
    pub id: Option<i32>,
    pub away: String,
    pub home: String,
    pub start_time: String,
    pub league: String,
    pub country: String,
    pub url: String,
    pub stream_link: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::stream)]
pub struct StreamNew<'a> {
    pub home: &'a str,
    pub away: &'a str,
    pub start_time: &'a str,
    pub league: &'a str,
    pub country: &'a str,
    pub url: &'a str,
    pub stream_link: &'a str,
}
