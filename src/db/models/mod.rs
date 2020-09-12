use serde::{Deserialize, Serialize};

pub mod topic;
pub mod user;
pub mod resources;
pub mod search;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ids {
    pub ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: i32,
}
