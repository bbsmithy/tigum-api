use rocket_contrib::json::Json;

use crate::db::models::user::CreateUser;
use crate::db::querys::TigumPgConn;

pub fn get_user_password(conn: &TigumPgConn, email: &String) -> String {
    let user_password_hash = conn
        .query(
            "SELECT password_hash FROM users WHERE email = $1",
            &[&email],
        )
        .unwrap();
    let user_row = user_password_hash.get(0);
    let user_email = user_row.get(0);
    user_email
}

pub fn create_user(conn: &TigumPgConn, new_user: Json<CreateUser>, hashed_password: String) {
    let user_row = conn.query(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        &[&new_user.name, &new_user.email, &hashed_password],
    );
    println!("USER CREATED!: {:?}", user_row);
}
