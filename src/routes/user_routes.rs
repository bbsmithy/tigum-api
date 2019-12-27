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
use db::querys::user_query::create_user;

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

// JWT

#[post("/user/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(conn: TigumPgConn, new_user: Json<CreateUser>, _auth_user: User) {
    let hashed_password = hash_password(&new_user.password);
    create_user(&conn, new_user);
    println!("{:?}", hashed_password);
}

#[post("/user/login", format = "application/json", data = "<login>")]
pub fn user_login(conn: TigumPgConn, login: Json<LoginUser>, _auth_user: User) {
    let is_correct = verify_password(&login.password, &login.hash);
    println!("Passord is: {:?}", is_correct);
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup, user_login]
}
