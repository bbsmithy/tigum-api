use serde::{Deserialize, Serialize};
use diesel::{Queryable, QueryableByName};
use diesel::sql_types::{Integer, Text, BigInt, Nullable, Timestamp, Timestamptz, Bool};
use diesel::SqlType;
use chrono::{NaiveDateTime, NaiveDate};
use diesel::types::FromSql;
use diesel::backend::Backend;
use diesel::deserialize;



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


#[derive(Queryable, QueryableByName, Serialize, Deserialize, Debug)]
pub struct TopicResult {
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
    pub misc2: String,
    #[sql_type="Text"]
    pub updated: String,
    #[sql_type="Bool"]
    pub published: bool,

}



