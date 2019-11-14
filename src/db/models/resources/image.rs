use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new ArticleSnippet
#[derive(Serialize, Deserialize, Debug)]
pub struct NewImage {
    pub src: String,
    pub origin: String,
    pub topic_id: i32,
    pub user_id: i32,
}

// Used when reading or updating a ArtcileSnippet
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub src: String,
    pub origin: String,
    pub date_created: NaiveDateTime,
    pub topic_id: i32,
    pub user_id: i32,
}