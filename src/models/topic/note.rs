use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct NoteItem {
    pub id: u64,
    pub content: String,
    pub item_structure: String,
}

impl NoteItem {
    pub fn new(item_structure: String, id: u64, content: String) -> NoteItem {
        NoteItem {
            id: id,
            content: content,
            item_structure: item_structure,
        }
    }
}

impl Serialize for NoteItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Note", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("item_structure", &self.item_structure)?;
        state.end()
    }
}

pub struct Note {
    pub title: String,
    pub description: String,
    pub note_id: u64,
    pub note_content: Vec<NoteItem>,
}

impl Note {
    pub fn new(title: String, description: String, note_id: u64) -> Note {
        Note {
            title: title,
            description: description,
            note_id: note_id,
            note_content: Vec::new(),
        }
    }
    pub fn add_note_item<'a>(&'a mut self, note_item: NoteItem) -> &'a mut Note {
        self.note_content.push(note_item);
        self
    }
}

impl Serialize for Note {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Note", 4)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.description)?;
        state.serialize_field("note_id", &self.note_id)?;
        state.serialize_field("note_content", &self.note_content)?;
        state.end()
    }
}
