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
use db::models::user::{AuthUser, CreateUser, LoginUser, User, UpdatePassword, VerifyUser};
use db::api_response::ApiResponse;

// Util
use util::auth::{
    encode_jwt,
    hash_string,
    verify_hash,
    create_known_hash_email,
    create_known_hash_string
};
use util::evervault::send_evervault_verify_email;

// Querys
use db::querys::user_query::{
    create_user,
    get_user,
    update_password,
    verify_user_with_hash
};

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
pub async fn user_signup(
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
            let hashed_email = create_known_hash_email(new_user.email.clone());
            let verify_hash = create_known_hash_string(hashed_email);
            create_user_with_ps_email(
                cookies,
                conn,
                new_user,
                hashed_password,
                hashed_email,
                verify_hash
            ).await
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
    hashed_email: u64,
    verify_hash: String
) -> ApiResponse {
    let new_user_email = new_user.email.clone();
    let new_user_verify_hash = verify_hash.clone();
    let new_user_name = new_user.name.clone();
    match create_user(&conn, new_user, hashed_password, hashed_email, verify_hash).await {
        Ok(user) => {
            println!("Created user: {:?}", user);
            // Encode JWT token with user
            let jwt_value = encode_jwt(&user);
            let create_cookie_result = create_cookie(jwt_value);
            match create_cookie_result {
                Ok(jwt_cookie) => {
                    cookies.add(jwt_cookie);
                    send_evervault_verify_email(new_user_email, new_user_verify_hash, new_user_name).await;
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
        Err(err) => {
            println!("Error creating user: {:?}", err);
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
    let hashed_email = create_known_hash_email(login.email.clone());
    match get_user(&conn, hashed_email).await {
        Ok(auth_user) => {
            match verify_hash(&login.password, &auth_user.password_hash) {
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
        Err(_err) => {
            println!("No luck");
            ApiResponse {
                json: json!({"error": "Incorrect email or password"}),
                status: Status::raw(403)
            }
        }
    } 
}


#[post("/user/update-password", format = "application/json", data = "<password>")]
pub async fn update_user_password(conn: TigumPgConn, password: Json<UpdatePassword>) -> ApiResponse {
    let email_hash = password.email_hash as u64;
    if let Ok(_user) = get_user(&conn, email_hash).await {
        if let Ok(password_hash) = hash_string(&password.new_password) {
            update_password(&conn, password.email_hash, password_hash).await
        } else {
            ApiResponse {
                json: json!({ "error": "Failed to update password" }),
                status: Status::raw(500)
            }
        }
    } else {
        ApiResponse {
            json: json!({ "error": "Failed to update password" }),
            status: Status::raw(500)
        }
    }
}

#[post("/user/verify", format = "application/json", data = "<verify>")]
pub async fn verify_user(conn: TigumPgConn, verify: Json<VerifyUser>) -> ApiResponse {
    let hash = verify.verify_hash.clone();
    // Run query
    let verified = verify_user_with_hash(&conn, hash).await;
    if verified {
        ApiResponse {
            json: json!({ "msg": "User verified" }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": "Could not verify user" }),
            status: Status::raw(500)
        }
    }
}


pub fn get_user_routes() -> Vec<Route> {
    routes![
        user_signup,
        user_login,
        check_login,
        logout,
        update_user_password,
        verify_user
    ]
}
