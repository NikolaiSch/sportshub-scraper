use diesel::prelude::*;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Queryable, Deserialize, Clone)]
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

#[derive(Debug, Insertable, Deserialize, Clone)]
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

impl Serialize for Stream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let split_streams: Vec<&str> = self.stream_link.split(",").collect();
        let mut stream = serializer.serialize_struct("Stream", 8)?;
        stream.serialize_field("id", &self.id)?;
        stream.serialize_field("home", &self.home)?;
        stream.serialize_field("away", &self.away)?;
        stream.serialize_field("start_time", &self.start_time)?;
        stream.serialize_field("league", &self.league)?;
        stream.serialize_field("country", &self.country)?;
        stream.serialize_field("url", &self.url)?;
        stream.serialize_field("stream_link", &split_streams)?;
        stream.end()
    }
}
