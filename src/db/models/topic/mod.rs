pub mod note;

use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct Topic {
    pub topic_id: u64,
    pub title: String,
    pub date_created: String,
}

impl Topic {
    pub fn new(title: String, date_created: String, topic_id: u64) -> Topic {
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
