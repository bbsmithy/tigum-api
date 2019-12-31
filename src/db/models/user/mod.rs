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

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserIdError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Use request.cookies().get("jwt") instead to get jwt token
        let cookies = request.cookies();
        let jwt_value = cookies.get("jwt");

        match jwt_value {
            Some(jwt_token) => {
                let jwt_token = jwt_token.value();
                println!("{}", jwt_token);
                // PARSE DATA AND CREATE USER
                // USER WILL BE ACCESSIBLE IN REQUEST GAURD auth_user
                let auth_user = User::new(123, "Brian Smith".to_string(), "bs@gm.com".to_string());
                Outcome::Success(auth_user)
            }
            _ => Outcome::Failure((Status::BadRequest, UserIdError::BadCount)),
        }
    }
}
