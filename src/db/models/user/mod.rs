use serde::Deserialize;


#[derive(Deserialize)]
pub struct User {
    pub user_id: u64
}