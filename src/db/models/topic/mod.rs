use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, deserialize::QueryableByName};

// ORDER OF STRUCT FIELDS MUCH MATCH ORDER OF FIELDS IN TABLE

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub date_created: NaiveDateTime,
    pub notes: Vec<i32>,
    pub videos: Vec<i32>,
    pub code: Vec<i32>,
    pub article_snippets: Vec<i32>,
    pub links: Vec<i32>,
    pub excercises: Vec<i32>,
    pub user_id: i32,
    pub images: Vec<i32>,
    pub date_updated: Option<NaiveDateTime>,
    published: bool
}


#[derive(Deserialize, Debug)]
pub struct NewTopic {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TopicId {
    pub topic_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct TopicIds {
    pub ids: Vec<i32>,
}
