use rocket_contrib::json::Json;
use crate::db::models::user::{CreateUser, User, AuthUser};
use crate::db::querys::TigumPgConn;
use rocket::http::Status;
use rocket::response::status;

fn row_to_user(row: &rocket_contrib::databases::postgres::Row) -> User {
    User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        email_hash: row.get(4)
    }
}

fn row_to_auth_user(row: &rocket_contrib::databases::postgres::Row) -> AuthUser {
    AuthUser {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        password_hash: row.get(3),
        email_hash: row.get(4)
    }
}

pub async fn get_user(conn: &TigumPgConn, email_hash: u64) -> Result<AuthUser, String> {
    let user_email_hash: i64 = email_hash as i64;
    let user_password_hash = conn.run(move |c|
        c.query(
            "SELECT * FROM users WHERE email_hash = $1",
            &[&user_email_hash]
        )
    ).await;
    match user_password_hash {
       Ok(result) => {
            if let Some(user_row) = result.get(0) {
                Ok(row_to_auth_user(user_row))
            } else {
                Err("Could not find user".to_string())
            }
        },
        Err(_err) => Err("Could not find user".to_string())
    }
}

pub async fn create_user(
    conn: &TigumPgConn,
    new_user: Json<CreateUser>,
    hashed_password: String,
    hashed_email: u64
) -> Result<User, status::Custom<String>> {
    let hashed_email_i = hashed_email as i64;
    let user_result = conn.run(move |c|
        c.query(
            "INSERT INTO users (name, email, email_hash, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
            &[&new_user.name, &new_user.email_encrypted, &hashed_email_i, &hashed_password],
        )
    ).await;
    match user_result {
        Ok(user_result) => {
            if let Some(user_row) = user_result.get(0) {
                Ok(row_to_user(user_row))
            } else {
                return Err(status::Custom(
                    Status {
                        code: 500,
                        reason: "Could not return created user",
                    },
                    "Could not return created user".to_string(),
                ))
            }
        }
        Err(user_err) => {
            println!("{:?}", user_err);
            return Err(status::Custom(
                Status {
                    code: 500,
                    reason: "Could not create user",
                },
                "Could not create user".to_string(),
            ))
        }
    }
}
