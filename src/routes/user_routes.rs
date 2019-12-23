use crate::db;
use crate::User;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;
use db::querys::TigumPgConn;

// Models
use db::models::user::CreateUser;


/////////////////////////
//// USER ROUTES ////////
/////////////////////////



#[post("/user/signup", format = "application/json", data = "<email_password>")]
pub fn user_signup(conn: TigumPgConn, email_password: Json<CreateUser>, _auth_user: User) {
    println!("{:?}", email_password);
}


pub fn get_user_routes() -> Vec<Route> {
    routes![user_signup]
}