use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new Document
#[derive(Serialize, Deserialize, Debug)]
pub struct NewDocument {
    pub title: String,
    pub topic_id: i32,
    pub user_id: i32,
    pub extension: String,
    pub origin: String,
    pub source: String,
}

// Used when reading or updating a Document
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub topic_id: i32,
    pub user_id: i32,
    pub extension: String,
    pub origin: String,
    pub source: String,
    pub date_created: NaiveDateTime,
}
