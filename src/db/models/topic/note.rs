use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used when creating a new Resource
#[derive(Serialize, Deserialize)]
pub struct NewResource {
    pub content_type: String,
    pub content: String,
    pub title: String,
    pub generated_by: String,
}

// Used when reading or updating a Resource
#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub resource_id: i32,
    pub date_created: NaiveDateTime,
    pub content_type: String,
    pub content: String,
    pub generated_by: String,
    pub title: String,
}

impl Resource {
    pub fn new(
        resource_id: i32,
        date_created: NaiveDateTime,
        content_type: String,
        content: String,
        generated_by: String,
        title: String,
    ) -> Resource {
        Resource {
            resource_id: resource_id,
            date_created: date_created,
            content_type: content_type,
            content: content,
            generated_by: generated_by,
            title: title,
        }
    }
}

// Used when creating a new Note
#[derive(Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub note_content: Vec<i32>,
}

// Used when reading or updating a Note
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub note_id: i32,
    pub note_content: Vec<i32>,
    pub date_created: NaiveDateTime
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
    pub fn new(title: String, note_id: i32, note_content: Vec<i32>, date_created: NaiveDateTime) -> Note {
        Note {
            title: title,
            note_id: note_id,
            note_content: note_content,
            date_created: date_created
        }
    }
    pub fn add_note_item<'a>(&'a mut self, note_item: i32) -> &'a mut Note {
        self.note_content.push(note_item);
        self
    }
}
