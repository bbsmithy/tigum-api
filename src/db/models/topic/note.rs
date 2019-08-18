use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Resource {
    pub resource_id: u64,
    pub content_type: String,
    pub content: String,
}

impl Resource {
    pub fn new(resource_id: u64, content_type: String, content: String) -> Resource {
        Resource {
            resource_id: resource_id,
            content_type: content_type,
            content: content,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub note_id: u64,
    pub note_content: Vec<i64>,
}

impl Note {
    pub fn new(title: String, note_id: u64, note_content: Vec<i64>) -> Note {
        Note {
            title: title,
            note_id: note_id,
            note_content: note_content,
        }
    }
    pub fn add_note_item<'a>(&'a mut self, note_item: i64) -> &'a mut Note {
        self.note_content.push(note_item);
        self
    }
}
