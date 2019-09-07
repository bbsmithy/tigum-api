use serde::{Serialize, Deserialize};


pub mod topic;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct Ids {
    pub ids: Vec<i32>
}
