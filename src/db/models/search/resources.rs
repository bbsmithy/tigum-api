use serde::{Deserialize, Serialize};
use diesel::{Queryable, QueryableByName};
use diesel::sql_types::{Integer, Text};

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Debug)]
pub struct ResourceResult {
    #[sql_type="Integer"]
    pub topic_id: i32,
    #[sql_type="Integer"]
    pub resource_id: i32,
    #[sql_type="Text"]
    pub result_type: String,
    #[sql_type="Text"]
    pub title: String,
    #[sql_type="Text"]
    pub misc: String,
    #[sql_type="Text"]
    pub misc2: String
}