use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct Topic {
    pub title: String,
    pub date_created: String,
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Topic", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("date_created", &self.date_created)?;
        state.end()
    }
}