use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum UserIdError {
    Missing,
    Invalid,
    BadCount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub struct AuthUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String) -> User {
        User { id, name, email }
    }
}

fn is_jwt_valid(user_id: &str) -> bool {
    // CHECK JWT TOKEN HERE
    user_id == "test-user-id".to_string()
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserIdError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Use request.cookies().get("jwt") instead to get jwt token
        let keys: Vec<_> = request.headers().get("X-User-ID").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, UserIdError::Missing)),
            1 if is_jwt_valid(keys[0]) => {
                // PARSE DATA AND CREATE USER
                // USER WILL BE ACCESSIBLE IN REQUEST GAURD auth_user
                let auth_user = User::new(123, "Brian Smith".to_string(), "bs@gm.com".to_string());
                Outcome::Success(auth_user)
            }
            1 => Outcome::Failure((Status::BadRequest, UserIdError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, UserIdError::BadCount)),
        }
    }
}
