use rocket_contrib::json::Json;
use crate::db::models::user::{CreateUser, User, AuthUser};
use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
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

pub async fn update_password(conn: &TigumPgConn, email_hash: i64, password_hash: String) -> ApiResponse {
    let user_password_update = conn.run(move |c|
        c.query(
            "UPDATE users SET password_hash = ($2) WHERE email_hash = ($1) RETURNING *",
            &[&email_hash, &password_hash]
        )
    ).await;
    if let Ok(_res) = user_password_update {
        ApiResponse {
            json: json!({ "msg": "Updated password" }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": "Failed to update password" }),
            status: Status::raw(500)
        }
    }
}

pub async fn create_user(
    conn: &TigumPgConn,
    new_user: Json<CreateUser>,
    hashed_password: String,
    hashed_email: u64,
    verify_hash: String
) -> Result<User, status::Custom<String>> {
    let hashed_email_i = hashed_email as i64;
    let user_result = conn.run(move |c|
        c.query(
            "INSERT INTO users (name, email, email_hash, password_hash, verify_hash, verified) VALUES ($1, $2, $3, $4, $5, false) RETURNING *",
            &[&new_user.name, &new_user.email_encrypted, &hashed_email_i, &hashed_password, &verify_hash],
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

pub async fn verify_user_with_hash(conn: &TigumPgConn, hash: String) -> bool {
    let copied_hash = hash.clone();
    let check_for_user = conn.run(move |c|
        c.query(
            "SELECT * FROM users WHERE verify_hash = $1",
            &[&hash]
        )
    ).await;
    if let Ok(user_rows) = check_for_user {
        if user_rows.len() > 0 {
            set_user_as_verified(conn, copied_hash).await
        } else {
            false
        }
    } else {
        false
    }
}

pub async fn set_user_as_verified(conn: &TigumPgConn, hash: String) -> bool {
    let check_for_user = conn.run(move |c|
        c.query(
            "UPDATE users SET verified = true WHERE verify_hash = ($1)",
            &[&hash]
        )
    ).await;
    if let Ok(_user) = check_for_user {
        true
    } else {
        false
    }
}
