use serde::ser::{Serialize, SerializeStruct, Serializer};

pub mod note;

use note::Note;

pub struct Topic {
    pub topic_id: u32,
    pub title: String,
    pub date_created: String,
    pub my_notes: Vec<Note>,
}



//Serialize implementations
impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Topic", 3)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("date_created", &self.date_created)?;
        state.serialize_field("topic_id", &self.topic_id)?;
        state.serialize_field("my_notes", &self.my_notes)?;
        state.end()
    }
}
