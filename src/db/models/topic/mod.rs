use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NewTopic {
    pub title: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub date_created: NaiveDateTime,
    pub notes: Vec<i32>,
    pub videos: Vec<i32>,
    pub code: Vec<i32>,
    pub article_snippets: Vec<i32>,
    pub documents: Vec<i32>,
    pub excercises: Vec<i32>,
    pub user_id: i32
}

impl Topic {
    pub fn new(
        id: i32,
        title: String,
        date_created: NaiveDateTime,
        notes: Vec<i32>,
        videos: Vec<i32>,
        code: Vec<i32>,
        article_snippets: Vec<i32>,
        documents: Vec<i32>,
        excercises: Vec<i32>,
        user_id: i32
    ) -> Topic {
        Topic {
            id: id,
            title: title,
            date_created: date_created,
            notes: notes,
            videos: videos,
            code: code,
            article_snippets: article_snippets,
            documents: documents,
            excercises: excercises,
            user_id: user_id
        }
    }
}

#[derive(Deserialize)]
pub struct TopicId {
    pub topic_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct TopicIds {
    pub ids: Vec<i32>,
}
