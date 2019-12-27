use crate::db;
use rocket::Route;

//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::{CreateUser, LoginUser, User};

// Util
use crate::util::auth::{hash_password, verify_password};

// Querys
use db::querys::user_query::{create_user, get_user_password};

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

#[post("/user/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(conn: TigumPgConn, new_user: Json<CreateUser>, _auth_user: User) {
    let hashed_password = hash_password(&new_user.password);
    match hashed_password {
        Ok(hashed_password) => create_user(&conn, new_user, hashed_password),
        _ => println!("Could not hash password"),
    }
}

#[post("/user/login", format = "application/json", data = "<login>")]
pub fn user_login(conn: TigumPgConn, login: Json<LoginUser>, _auth_user: User) {
    // Check if email exists and return User
    let password = get_user_password(&conn, &login.email);
    // Check if login.password matches
    let is_correct = verify_password(&login.password, &password);
    println!("Passord is: {:?}", is_correct);
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup, user_login]
}
