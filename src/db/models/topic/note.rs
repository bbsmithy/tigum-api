use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct NewResource {
    pub content_type: String,
    pub content: String,
    pub generated_by: String
}


#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub resource_id: i32,
    pub date_created: String,
    pub content_type: String,
    pub content: String,
    pub generated_by: String
}

impl Resource {
    pub fn new(resource_id: i32, date_created: String, content_type: String, content: String, generated_by: String) -> Resource {
        Resource {
            resource_id: resource_id,
            date_created: date_created,
            content_type: content_type,
            content: content,
            generated_by: generated_by
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub note_id: i32,
    pub note_content: Vec<i32>,
}

impl Note {
    pub fn new(title: String, note_id: i32, note_content: Vec<i32>) -> Note {
        Note {
            title: title,
            note_id: note_id,
            note_content: note_content,
        }
    }
    pub fn add_note_item<'a>(&'a mut self, note_item: i32) -> &'a mut Note {
        self.note_content.push(note_item);
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct NoteId {
    pub id: i32
}


#[derive(Serialize, Deserialize)]
pub struct NoteIds {
    pub ids: Vec<i32>
}
