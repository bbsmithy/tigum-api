use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new Note
#[derive(Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub topic_id: i32,
    pub user_id: i32,
}

// Used when reading or updating a Note
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub date_created: NaiveDateTime,
    pub topic_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NoteId {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NoteIds {
    pub ids: Vec<i32>,
}

impl Note {
    pub fn new(
        id: i32,
        title: String,
        date_created: NaiveDateTime,
        topic_id: i32,
        user_id: i32,
    ) -> Note {
        Note {
            id: id,
            title: title,
            date_created: date_created,
            topic_id: topic_id,
            user_id: user_id,
        }
    }
}
