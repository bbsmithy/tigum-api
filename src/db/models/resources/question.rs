use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new ArticleSnippet
#[derive(Serialize, Deserialize, Debug)]
pub struct NewCode {
    pub content: String,
    pub language: String,
    pub origin: String,
    pub topic_id: i32,
}

// Used when reading or updating a ArtcileSnippet
#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    pub id: i32,
    pub content: String,
    pub date_created: NaiveDateTime,
    pub language: String,
    pub origin: String,
    pub topic_id: i32,
    pub user_id: i32,
}
