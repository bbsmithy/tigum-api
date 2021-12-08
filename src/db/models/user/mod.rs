use crate::util::auth::decode_jwt;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use chrono::NaiveDateTime;
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
    pub old_password: String,
    pub new_password: String
}

#[derive(Serialize, Deserialize)]
pub struct BetaSignUp {
    pub email: String,
    pub username: String
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct BetaUser {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub setup: Option<bool>,
    pub created_at: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyUser {
    pub verify_hash: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserFeedback {
    pub feedback: String
}



#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub email_hash: i64,
    pub verify_hash: String,
    pub verified: bool
}

impl AuthUser {
    pub fn to_public_user(auth_user: Self) -> User {
        User {
            id: auth_user.id,
            name: auth_user.name,
            email: auth_user.email,
            email_hash: auth_user.email_hash
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserIdError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let jwt_value = cookies.get("__silly_devkeep");

        match jwt_value {
            Some(jwt_token) => {
                let jwt_token = jwt_token.value();
                let user = decode_jwt(jwt_token);
                match user {
                    Some(user) => {
                        let public_user: User = from_str(&user).unwrap();
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
