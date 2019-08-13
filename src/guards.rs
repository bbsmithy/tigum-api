use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;


pub struct User {
    pub user_id: String,
}

impl User {
    pub fn new(user_id: String) -> User {
        User { user_id: user_id }
    }
}

#[derive(Debug)]
pub enum UserIdError {
    Missing,
    Invalid,
    BadCount,
}

fn is_valid(user_id: &str) -> bool {
    user_id == "test-user-id".to_string()
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserIdError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-User-ID").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, UserIdError::Missing)),
            1 if is_valid(keys[0]) => {
                let auth_user = User::new(keys[0].to_string());
                Outcome::Success(auth_user)
            }
            1 => Outcome::Failure((Status::BadRequest, UserIdError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, UserIdError::BadCount)),
        }
    }
}
