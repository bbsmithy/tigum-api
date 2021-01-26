use crate::util::auth::decode_jwt;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug)]
pub enum UserIdError {
    Missing,
    Invalid,
    BadCount,
}

pub struct UserId {
    pub id: u32,
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

#[derive(Serialize, Deserialize, RustcEncodable)]
pub struct AuthUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl AuthUser {
    pub fn to_public_user(auth_user: Self) -> User {
        let user = User {
            id: auth_user.id,
            name: auth_user.name,
            email: auth_user.email,
        };
        user
    }
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

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserIdError;

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let jwt_value = cookies.get("__silly_devkeep");

        match jwt_value {
            Some(jwt_token) => {
                let jwt_token = jwt_token.value();
                let user = decode_jwt(jwt_token);
                match user {
                    Some(user) => {
                        let public_user: User = from_str(&user).unwrap();
                        println!("Request from: {}", public_user.id);
                        Outcome::Success(public_user)
                    }
                    None => {
                        Outcome::Failure((Status::new(401, "Unauthorized"), UserIdError::BadCount))
                    }
                }
                // PARSE DATA AND CREATE USER
                // USER WILL BE ACCESSIBLE IN REQUEST GAURD auth_user
            }
            _ => Outcome::Failure((Status::new(401, "Unauthorized"), UserIdError::BadCount)),
        }
    }
}
