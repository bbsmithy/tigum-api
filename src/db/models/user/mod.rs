use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    email: String,
    password: String
}

#[derive(Deserialize)]
pub struct User {
    pub user_id: u64
}