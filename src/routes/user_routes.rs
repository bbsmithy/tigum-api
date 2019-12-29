use crate::db;
use rocket::http::Status;
use rocket::response::status;
use rocket::Route;


//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::{CreateUser, LoginUser, User, AuthUser};

// Util
use crate::util::auth::{hash_password, verify_password, encode_jwt};

// Querys
use db::querys::user_query::{create_user, get_user};

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

#[post("/user/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(
    conn: TigumPgConn,
    new_user: Json<CreateUser>,
    _auth_user: User,
) -> Result<Json<User>, status::Custom<String>> {
    if new_user.password.is_empty() {
        return Err(status::Custom(
            Status {
                code: 400,
                reason: "Bad Request",
            },
            "Password must not be empty".to_string(),
        ));
    }
    hash_password(&new_user.password)
        .map_err(|_err| {
            status::Custom(
                Status {
                    code: 500,
                    reason: "Internal server error",
                },
                "Internal server error".to_string(),
            )
        })
        .and_then(|hashed_password| create_user(&conn, new_user, hashed_password))
        .map(|user| Json(user))
}

#[post("/user/login", format = "application/json", data = "<login>")]
pub fn user_login(conn: TigumPgConn, login: Json<LoginUser>) -> String {
    // Check if email exists and return User
    let user = get_user(&conn, &login.email);
    // Check if login.password matches
    let is_correct = verify_password(&login.password, &user.password_hash);

    // Encode JWT token with user
    encode_jwt(user.id)
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup, user_login]
}
