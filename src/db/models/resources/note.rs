use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable};

// ORDER OF STRUCT FIELDS MUCH MATCH ORDER OF FIELDS IN TABLE

// Used when creating a new Note
#[derive(Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub topic_id: i32,
}

// Used when reading or updating a Note
#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub date_created: NaiveDateTime,
    pub topic_id: i32,
    pub user_id: i32,
    pub date_updated: NaiveDateTime,
    pub published: bool
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
        date_updated: NaiveDateTime,
        published: bool
    ) -> Note {
        Note {
            id: id,
            title: title,
            date_created: date_created,
            topic_id: topic_id,
            user_id: user_id,
            date_updated: date_updated,
            published: published
        }
    }
}
