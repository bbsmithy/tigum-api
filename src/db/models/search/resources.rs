use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceResult {
    pub topic_id: i32,
    pub resource_id: i32,
    pub result_type: String,
    pub title: String,
    pub misc: String,
    pub misc2: String
}