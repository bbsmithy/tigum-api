
use chrono::NaiveDateTime;

#[derive(Queryable, PartialEq, Debug)]
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
    pub images: Vec<i32>,
    pub date_updated: NaiveDateTime,
}
