use serde::{Serialize, Deserialize};

pub mod note;

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub topic_id: i32,
    pub title: String,
    pub date_created: String,
}

impl Topic {
    pub fn new(title: String, date_created: String, topic_id: i32) -> Topic {
        Topic {
            topic_id: topic_id,
            title: title,
            date_created: date_created
        }
    }
}

#[derive(Deserialize)]
pub struct TopicId {
    pub topic_id: u64
}

#[derive(Deserialize)]
pub struct TopicIds {
    pub ids: Vec<i32>
}
