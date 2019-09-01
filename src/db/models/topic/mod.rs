use serde::{Serialize, Deserialize};

pub mod note;

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub topic_id: i32,
    pub title: String,
    pub topic_content: Vec<i32>
}

impl Topic {
    pub fn new(title: String, topic_id: i32, topic_content: Vec<i32>) -> Topic {
        Topic {
            topic_id: topic_id,
            title: title,
            topic_content: topic_content
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
