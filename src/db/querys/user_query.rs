use rocket_contrib::json::Json;

use crate::db::models::user::{CreateUser, User};
use crate::db::querys::TigumPgConn;
use rocket::http::Status;
use rocket::response::status;

fn row_to_user(row: rocket_contrib::databases::postgres::rows::Row) -> User {
    let user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    };
    user
}

pub fn get_user_password(conn: &TigumPgConn, email: &String) -> String {
    let user_password_hash = conn
        .query(
            "SELECT password_hash FROM users WHERE email = $1",
            &[&email],
        )
        .unwrap();
    let user_ps_hash_row = user_password_hash.get(0);
    let user_email = user_ps_hash_row.get(0);
    user_email
}

pub fn create_user(
    conn: &TigumPgConn,
    new_user: Json<CreateUser>,
    hashed_password: String,
) -> Result<User, status::Custom<String>> {
    let user_result = conn.query(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        &[&new_user.name, &new_user.email, &hashed_password],
    );

    match user_result {
        Ok(user_result) => {
            let user_row = user_result.get(0);
            return Ok(row_to_user(user_row));
        }
        Err(_user_result) => {
            return Err(status::Custom(
                Status {
                    code: 500,
                    reason: "Could not create user",
                },
                "Hello".to_string(),
            ))
        }
    }
}
