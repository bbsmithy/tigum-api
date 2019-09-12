use serde::{Deserialize, Serialize};

pub mod topic;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct Ids {
    pub ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Id {
    pub id: i32,
}
