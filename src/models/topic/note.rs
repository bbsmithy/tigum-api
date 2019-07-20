use serde::ser::{Serialize, SerializeStruct, Serializer};

// Note struct defines title, description, and note_id of note.
// note_id can be used to get the full NoteDetail which holds teh contents of the Note


pub enum NoteItem {
    NoteReference {
        note_item_type: String,
        link: String,
        ref_id: u64
    },
    NoteText {
        body: String
    }
}




pub struct Note {
    pub title: String,
    pub description: String,
    pub note_id: u64,
    pub notes: Vec<NoteItem>,
}

// A NoteDetail holds the same as Note plus Vec<UserNote>
// UserNote is an enum with types UserNote::Text and UserNote::Reference

impl Note {
    pub fn new(title: String, description: String, note_id: u64) -> Note {
        Note {
            title: title,
            description: description,
            note_id: note_id,
            notes: Vec::new(),
        }
    }

    /// Add a NoteBody
    pub fn add_note_text<'a>(&'a mut self, note_item: NoteItem) -> &'a mut Note {
        self.notes.push(note_item);
        self
    }

    /// Add a NoteBody
    pub fn add_note_reference<'a>(&'a mut self, note_item: NoteItem) -> &'a mut Note {
        self.notes.push(note_item);
        self
    }

    // /// Add multiple NoteBodys
    // pub fn add_note_items<'a>(&'a mut self, note_items: &[NoteItem]) -> &'a mut Note {
    //     for note_item in note_items.iter() {
    //         self.notes.push(*note_item)
    //     }
    //     self
    // }
}

impl Serialize for Note {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Note", 3)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.description)?;
        state.serialize_field("note_id", &self.note_id)?;
        state.end()
    }
}
