use rocket_contrib::json::Json;

use crate::db::models::user::{CreateUser, User, AuthUser};
use crate::db::querys::TigumPgConn;
use rocket::http::Status;
use rocket::response::status;

fn row_to_user(row: rocket_contrib::databases::postgres::rows::Row) -> User {
    User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    }
}

fn row_to_auth_user(row: rocket_contrib::databases::postgres::rows::Row) -> AuthUser {
    AuthUser {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        password_hash: row.get(3)
    }
}

pub fn get_user(conn: &TigumPgConn, email: &String) -> AuthUser {
    let user_password_hash = conn
        .query(
            "SELECT * FROM users WHERE email = $1",
            &[&email],
        )
        .unwrap();
    let user_row = user_password_hash.get(0);
    row_to_auth_user(user_row)
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
