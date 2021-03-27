use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new Video
#[derive(Serialize, Deserialize, Debug)]
pub struct NewVideo {
    pub topic_id: i32,
    pub title: String,
    pub iframe: String,
    pub origin: String,
    pub thumbnail_img: String,
}

// Used when reading or updating a Video
#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub id: i32,
    pub topic_id: i32,
    pub user_id: i32,
    pub title: String,
    pub iframe: String,
    pub origin: String,
    pub date_created: NaiveDateTime,
    pub thumbnail_img: String,
    pub date_updated: NaiveDateTime
}
