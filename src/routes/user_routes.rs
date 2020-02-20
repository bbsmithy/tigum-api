use crate::db;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::response::status;
use rocket::Route;
use std::format;

//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::{AuthUser, CreateUser, LoginUser, User};

// Util
use crate::util::auth::{encode_jwt, hash_password, verify_password};

// Querys
use db::querys::user_query::{create_user, get_user};

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

fn create_cookie<'a>(jwt_value: String) -> Cookie<'a> {
    let cookie_string = format!("__silly_devkeep={}; Path=/; SameSite=None", jwt_value);
    let mut jwt_cookie = Cookie::parse(cookie_string).unwrap();
    jwt_cookie.make_permanent();
    jwt_cookie.set_secure(true);
    jwt_cookie
}

fn expire_cookie<'a>() -> Cookie<'a> {
    let mut jwt_cookie = Cookie::parse("__silly_devkeep=; Path=/; SameSite=None").unwrap();
    jwt_cookie.make_permanent();
    jwt_cookie.set_secure(true);
    jwt_cookie
}

/**
 * DK {
 *  name: check_login
 *  desc: "Check login takes in a request and looks for a jwt token in the cookie, 
 *  then trys to verify the cookie"
 * }
 */
#[post("/user/checklogin", format = "application/json")]
pub fn check_login(_conn: TigumPgConn, auth_user: User) -> Json<User> {
    Json(auth_user)
}

#[post("/user/logout", format = "application/json")]
pub fn logout(mut cookies: Cookies, _conn: TigumPgConn) -> String {
    let expired_cookie = expire_cookie();
    cookies.remove(expired_cookie);
    "OK".to_string()
}

#[post("/user/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(
    mut cookies: Cookies,
    conn: TigumPgConn,
    new_user: Json<CreateUser>,
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
    //TODO: Handle user exists
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
        .map(|user| {
            // Encode JWT token with user
            let jwt_value = encode_jwt(&user);
            let jwt_cookie = create_cookie(jwt_value);
            cookies.add(jwt_cookie);
            Json(user)
        })
}

#[post("/user/login", format = "application/json", data = "<login>")]
pub fn user_login(
    mut cookies: Cookies,
    conn: TigumPgConn,
    login: Json<LoginUser>,
) -> Result<Json<User>, status::Custom<String>> {
    // Check if email exists and return User
    let auth_user = get_user(&conn, &login.email);
    // Check if login.password matches
    let is_correct = verify_password(&login.password, &auth_user.password_hash);
    match is_correct {
        Ok(is_correct) => {
            if is_correct {
                let public_user = AuthUser::to_public_user(auth_user);
                // Encode JWT token with user
                let jwt_value = encode_jwt(&public_user);
                let jwt_cookie = create_cookie(jwt_value);
                cookies.add(jwt_cookie);
                Ok(Json(public_user))
            } else {
                Err(status::Custom(
                    Status {
                        code: 400,
                        reason: "Incorrect email or password",
                    },
                    "Incorrect email or password".to_string(),
                ))
            }
        }
        Err(_is_correct) => Err(status::Custom(
            Status {
                code: 400,
                reason: "Incorrect email or password",
            },
            "Incorrect email or password".to_string(),
        )),
    }
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup, user_login, check_login, logout]
}
