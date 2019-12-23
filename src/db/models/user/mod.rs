use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct User {
    pub user_id: u64
}