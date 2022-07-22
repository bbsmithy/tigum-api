use crate::db;
use crate::db::models::dto::ProfilePictureUrl;
use crate::util;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::Route;
use std::format;

//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::{
    AuthUser, 
    CreateUser, 
    LoginUser, 
    User, 
    UpdatePassword, 
    VerifyUser, 
    BetaSignUp,
    UserFeedback
};
use db::api_response::ApiResponse;

// Util
use util::auth::{
    encode_jwt,
    hash_string,
    verify_hash,
    create_known_hash_email,
    create_known_hash_string,
};
use util::evervault::send_evervault_verify_email;
use util::sendgrid::{send_beta_signup_email_notify, send_user_feedback};

// Querys
use db::querys::user_query::{
    create_user,
    get_user,
    update_password,
    verify_user_with_hash,
    create_betauser,
    update_subdomain,
    update_profile_pic
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

#[put("/user/subdomain/<domain>")]
pub fn set_subdomain(conn: TigumPgConn, domain: String, auth_user: User) -> ApiResponse {
    update_subdomain(&conn, domain, auth_user.id)
}

#[post("/user/profile-picture", format = "application/json", data = "<pp>")]
pub fn set_pp(conn: TigumPgConn, pp: Json<ProfilePictureUrl>, auth_user: User) -> ApiResponse {
    update_profile_pic(&conn, &pp.url, auth_user.id)
}

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
            let user = get_user(&conn, hashed_email as i64);
            if let Ok(_auth_user) = user {
                ApiResponse {
                    json: json!({ "error": "An account with that email already exists", "errorCode": "USER_EXISTS" }),
                    status: Status::raw(500)
                }
            } else {
                let verify_hash = create_known_hash_string(hashed_email);
                create_user_with_ps_email(
                    conn,
                    new_user,
                    hashed_password,
                    hashed_email,
                    verify_hash,
                    &mut cookies
                )
            }
        }
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Whoops! Something went wrong, please contact brian@tigum.io for support" }),
                status: Status::raw(500)
            }
        }
    }
}

fn create_user_with_ps_email(
    conn: TigumPgConn,
    new_user: Json<CreateUser>,
    hashed_password: String,
    hashed_email: u64,
    verify_hash: String,
    cookies: &mut Cookies
) -> ApiResponse {

    match create_user(&conn, new_user, hashed_password, hashed_email, verify_hash) {
        Ok(auth_user) => {
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
                        status: Status::raw(500)
                    }
                }
            }
            
        },
        Err(err) => {
            ApiResponse {
                json: json!({ "error": "Internal server error" }),
                status: Status::raw(500)
            }
        }
    }
}

fn attempt_login(conn: &TigumPgConn, auth_user: AuthUser, mut cookies: Cookies) -> ApiResponse {
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
                status: Status::raw(500)
            }
        }
    } 
}

pub fn verify_user_password(conn: &TigumPgConn, email_hash: i64, password: String) -> Result<AuthUser, String> {
    let hashed_email = email_hash;
    match get_user(conn, hashed_email) {
        Ok(auth_user) => {
            if auth_user.verified {
                match verify_hash(&password, &auth_user.password_hash) {
                    Ok(is_correct) => {
                        if is_correct {
                            Ok(auth_user)
                        } else {
                            Err("Incorrect password".to_string())
                        }
                    }
                    Err(_checking_err) => Err("Incorrect password".to_string())
                }
            } else {
                Err("User not verified".to_string())
            }
        },
        Err(_err) => Err("Couldn't find user".to_string()) 
    }
}


#[post("/user/login", format = "application/json", data = "<login>")]
pub fn user_login(
    mut cookies: Cookies,
    conn: TigumPgConn,
    login: Json<LoginUser>,
) -> ApiResponse {
    let email_hash = create_known_hash_email(login.email.clone()) as i64;
    match verify_user_password(&conn, email_hash, login.password.clone()) {
        Ok(auth_user) => {
            attempt_login(&conn, auth_user, cookies)
        },
        Err(err) => {
            ApiResponse {
                json: json!({ "error": "Failed to login incorrect email or password" }),
                status: Status::raw(500)
            }
        }
    }
}


#[post("/user/update-password", format = "application/json", data = "<password>")]
pub fn update_user_password(conn: TigumPgConn, password: Json<UpdatePassword>, auth_user: User) -> ApiResponse {
    let login_result = verify_user_password(&conn, auth_user.email_hash, password.old_password.clone());
    if login_result.is_ok() {
        let email_hash = auth_user.email_hash;
        if let Ok(_user) = get_user(&conn, email_hash) {
            if let Ok(password_hash) = hash_string(&password.new_password) {
                update_password(&conn, email_hash, password_hash)
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
    } else {
        ApiResponse {
            json: json!({ "error": "The old password used is incorrect" }),
            status: Status::raw(403)
        }
    }
}

#[post("/user/beta-signup", format = "application/json", data = "<beta_signup>")]
pub fn beta_user_signup(conn: TigumPgConn, beta_signup: Json<BetaSignUp>) -> ApiResponse {

    let beta_email = beta_signup.email.clone();
    let beta_username = beta_signup.username.clone();

    if create_betauser(&conn, beta_signup) {
        send_beta_signup_email_notify(beta_email, beta_username);
        ApiResponse {
            json: json!({ "msg": "beta user signed up" }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "msg": "YUP" }),
            status: Status::raw(200)
        }
    }
}

#[post("/user/feedback", format = "application/json", data = "<feedback>")]
pub fn user_feedback(feedback: Json<UserFeedback>, auth_user: User) -> ApiResponse {
    send_user_feedback(auth_user.id, auth_user.name, &feedback.feedback);
    ApiResponse {
        json: json!({ "msg": "User verified" }),
        status: Status::raw(200)
    }
}


#[post("/user/verify", format = "application/json", data = "<verify>")]
pub fn verify_user(conn: TigumPgConn, verify: Json<VerifyUser>) -> ApiResponse {
    let hash = verify.verify_hash.clone();
    // Run query
    let verified = verify_user_with_hash(&conn, hash);
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
        user_feedback,
        check_login,
        logout,
        update_user_password,
        verify_user,
        beta_user_signup,
        set_subdomain,
        set_pp
    ]
}
