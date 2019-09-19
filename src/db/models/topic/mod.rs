use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod note;

#[derive(Deserialize)]
pub struct NewTopic {
    pub title: String,
    pub notes: Vec<i32>,
    pub resources: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub topic_id: i32,
    pub title: String,
    pub notes: Vec<i32>,
    pub resources: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl Topic {
    pub fn new(
        topic_id: i32,
        title: String,
        date_created: NaiveDateTime,
        notes: Vec<i32>,
        resources: Vec<i32>,
    ) -> Topic {
        Topic {
            topic_id: topic_id,
            title: title,
            date_created: date_created,
            notes: notes,
            resources: resources,
        }
    }
}

#[derive(Deserialize)]
pub struct TopicId {
    pub topic_id: u64,
}

#[derive(Deserialize)]
pub struct TopicIds {
    pub ids: Vec<i32>,
}
