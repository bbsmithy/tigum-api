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
    pub email_encrypted: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePassword {
    pub new_password: String,
    pub email_hash: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyUser {
    pub verify_hash: String
}


#[derive(Serialize, Deserialize, RustcEncodable, Debug)]
pub struct AuthUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_hash: i64,
    pub password_hash: String,
    pub verified: bool
}

impl AuthUser {
    pub fn to_public_user(auth_user: Self) -> User {
        let user = User {
            id: auth_user.id,
            name: auth_user.name,
            email: auth_user.email,
            email_hash: auth_user.email_hash
        };
        user
    }
}

#[derive(Serialize, Deserialize, RustcEncodable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_hash: i64
}

impl User {
    pub fn new(id: i32, name: String, email: String, email_hash: i64) -> User {
        User { id, name, email, email_hash }
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
            }
            _ => Outcome::Failure((Status::new(401, "Unauthorized"), UserIdError::BadCount)),
        }
    }
}
