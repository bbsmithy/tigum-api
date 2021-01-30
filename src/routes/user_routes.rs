use crate::db;
use crate::util;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::Route;
use std::format;

//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::{AuthUser, CreateUser, LoginUser, User};
use db::api_response::ApiResponse;

// Util
use util::auth::{encode_jwt, hash_string, verify_password};
use util::evervault::send_evervault_verify_email;

// Querys
use db::querys::user_query::{create_user, get_user};

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

fn create_cookie<'a>(jwt_value: String) -> Result<Cookie<'a>, String> {
    let cookie_string = format!("__silly_devkeep={}; Path=/", jwt_value);
    let jwt_cookie_result = Cookie::parse(cookie_string);
    match jwt_cookie_result {
        Ok(mut jwt_cookie) => {
            jwt_cookie.make_permanent();
            Ok(jwt_cookie)
        },
        Err(_err) => {
            Err("Error making cookie".to_string())
        }
    }
    
}

fn expire_cookie<'a>() -> Cookie<'a> {
    let mut jwt_cookie = Cookie::parse("__silly_devkeep=; Path=/").unwrap();
    jwt_cookie.make_permanent();
    jwt_cookie
}

#[post("/user/checklogin", format = "application/json")]
pub fn check_login(_conn: TigumPgConn, auth_user: User) -> Json<User> {
    Json(auth_user)
}

#[post("/user/logout", format = "application/json")]
pub fn logout(mut cookies: &CookieJar<'_>, _conn: TigumPgConn) -> String {
    let expired_cookie = expire_cookie();
    cookies.remove(expired_cookie);
    "OK".to_string()
}

#[post("/user/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(
    mut cookies: &CookieJar<'_>,
    conn: TigumPgConn,
    new_user: Json<CreateUser>,
) -> ApiResponse {
    if new_user.password.is_empty() {
        return ApiResponse {
            json: json!({ "error": "Bad request password empty" }),
            status: Status::raw(400)
        }
    }
    match hash_string(&new_user.password) {
        Ok(hashed_password) => {
            match hash_string(&new_user.email) {
                Ok(hashed_email) => {
                    ApiResponse {
                        json: json!({ "data": format!("{}, {}", hashed_password, hashed_email) }),
                        status: Status::raw(200)
                    }

                    // create_user_with_ps_email(
                    //     cookies,
                    //     conn,
                    //     new_user,
                    //     hashed_password,
                    //     hashed_email
                    // )
                },
                Err (err) => {
                    println!("Error {}", err);
                    ApiResponse {
                        json: json!({ "error": "Internal server error" }),
                        status: Status::raw(500)
                    }
                }
            }
        }
        Err(err) => {
            println!("Error {}", err);
            ApiResponse {
                json: json!({ "error": "Internal server error" }),
                status: Status::raw(500)
            }
        }
    }
}

async fn create_user_with_ps_email(
    mut cookies: &CookieJar<'_>,
    conn: TigumPgConn,
    new_user: Json<CreateUser>,
    hashed_password: String,
    hashed_email: String
) -> ApiResponse {
    match create_user(&conn, new_user, hashed_email, hashed_password).await {
        Ok(user) => {
            // Encode JWT token with user
            let jwt_value = encode_jwt(&user);
            let create_cookie_result = create_cookie(jwt_value);
            match create_cookie_result {
                Ok(jwt_cookie) => {
                    cookies.add(jwt_cookie);
                    ApiResponse {
                        json: json!(user),
                        status: Status::raw(200)
                    }
                },
                Err(_err) => ApiResponse {
                    json: json!({ "error": "Interal server error" }),
                    status: Status::raw(500)
                }
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Internal server error" }),
                status: Status::raw(500)
            }
        }
    }
}

#[post("/user/login", format = "application/json", data = "<login>")]
pub async fn user_login(
    mut cookies: &CookieJar<'_>,
    conn: TigumPgConn,
    login: Json<LoginUser>,
) -> ApiResponse {
    // Check if email exists and return User
    match get_user(&conn, login.email.clone()).await {
        Ok(auth_user) => {
            match verify_password(&login.password, &auth_user.password_hash) {
                Ok(is_correct) => {
                    if is_correct {
                        let public_user = AuthUser::to_public_user(auth_user);
                        // Encode JWT token with user
                        let jwt_value = encode_jwt(&public_user);
                        let jwt_cookie_result = create_cookie(jwt_value);
                        match jwt_cookie_result {
                            Ok(jwt_cookie) => {
                                cookies.add(jwt_cookie);
                                ApiResponse {
                                    json: json!(public_user),
                                    status: Status::raw(200)
                                }
                            },
                            Err(_err) => {
                                ApiResponse {
                                    json: json!({ "error": "Internal server error" }),
                                    status: Status::raw(200)
                                }
                            }
                        }
                        
                    } else {
                        ApiResponse {
                            json: json!({"error": "Incorrect email or password"}),
                            status: Status::raw(403)
                        }
                    }
                }
                Err(_checking_err) => ApiResponse {
                    json: json!({"error": "Incorrect email or password"}),
                    status: Status::raw(403)
                },
            }
        },
        Err(_err) => ApiResponse {
            json: json!({"error": "Incorrect email or password"}),
            status: Status::raw(403)
        }
    }
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup, user_login, check_login, logout]
}
