
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


// ORDER OF STRUCT FIELDS MUCH MATCH ORDER OF FIELDS IN TABLE
#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub date_created: Option<NaiveDateTime>,
    pub notes: Option<Vec<i32>>,
    pub videos: Option<Vec<i32>>,
    pub code: Option<Vec<i32>>,
    pub article_snippets: Option<Vec<i32>>,
    pub links: Option<Vec<i32>>,
    pub excercises: Option<Vec<i32>>,
    pub user_id: i32,
    pub images: Option<Vec<i32>>,
    pub date_updated: Option<NaiveDateTime>,
    published: Option<bool>
}
