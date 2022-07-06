use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable};


// Used when creating a new Link
#[derive(Serialize, Deserialize, Debug)]
pub struct NewLink {
    pub title: String,
    pub topic_id: i32,
    pub source: String,
    pub favicon_source: String
}

// Used when reading or updating a Link
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub topic_id: i32,
    pub date_created: NaiveDateTime,
    pub source: String,
    date_updated: Option<NaiveDateTime>,
    published: bool,
    pub favicon_source: String,
}
