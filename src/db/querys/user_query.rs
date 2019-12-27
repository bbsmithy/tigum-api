use rocket_contrib::json::Json;

use crate::db::models::user::{CreateUser, User};
use crate::db::querys::TigumPgConn;
use crate::util::auth::hash_password;

pub fn get_user_password(conn: &TigumPgConn, id: i32) {
    let user_rows = conn.query("SELECT * FROM users WHERE id = $1", &[&id]);
    println!("{:?}", user_rows);
}

pub fn create_user(conn: &TigumPgConn, new_user: Json<CreateUser>) {
    let name = "Brian Smith".to_string();
    let email = "bean.smith77@gmail.com".to_string();
    let salt = "salty".to_string();
    let ps_hash = "ps_hash".to_string();
    let user_row = conn.query(
        "INSERT INTO users (name, email, salt, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
        &[&name, &email, &salt, &ps_hash],
    );
    println!("USER CREATED!: {:?}", user_row);
}
