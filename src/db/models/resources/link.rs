use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new Link
#[derive(Serialize, Deserialize, Debug)]
pub struct NewLink {
    pub title: String,
    pub topic_id: i32,
    pub user_id: i32,
    pub source: String,
}

// Used when reading or updating a Link
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub topic_id: i32,
    pub user_id: i32,
    pub source: String,
    pub date_created: NaiveDateTime,
}
