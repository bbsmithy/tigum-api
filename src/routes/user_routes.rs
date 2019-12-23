use crate::db;
use crate::User;
use rocket::Route;

//Use Macros
use db::querys::TigumPgConn;
use rocket_contrib::json::Json;

// Models
use db::models::user::CreateUser;

// Util
use crate::util::auth::hash_password;

/////////////////////////
//// USER ROUTES ////////
/////////////////////////

#[post("/user/signup", format = "application/json", data = "<email_password>")]
pub fn user_signup(conn: TigumPgConn, email_password: Json<CreateUser>, _auth_user: User) {
    let hashed_password = hash_password(&email_password.password);
    println!("{:?}", hashed_password);
}

pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup]
}
