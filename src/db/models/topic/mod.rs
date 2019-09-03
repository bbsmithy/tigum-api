use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};


pub mod note;

#[derive(Deserialize)]
pub struct NewTopic {
    pub title: String,
    pub topic_content: Vec<i32>
}


#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub topic_id: i32,
    pub title: String,
    pub topic_content: Vec<i32>,
    pub date_created: NaiveDateTime
}

impl Topic {
    pub fn new(title: String, topic_id: i32, topic_content: Vec<i32>, date_created: NaiveDateTime) -> Topic {
        Topic {
            topic_id: topic_id,
            title: title,
            topic_content: topic_content,
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
