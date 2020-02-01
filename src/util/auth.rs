use bcrypt::{hash, verify};
use crypto;
use jwt::{Claims, Header, Token};
use std::default::Default;

use crate::db::models::user::User;
use serde_json::to_string;

pub fn hash_password(plain: &String) -> Result<String, bcrypt::BcryptError> {
    hash(plain, 10)
}

pub fn verify_password(plain: &String, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
    Ok(verify(plain, hash_string)?)
}

pub fn verify_token(parsed_token: Token<Header, Claims>) -> Option<String> {
    if parsed_token.verify("secret".as_bytes(), crypto::sha2::Sha256::new()) {
        parsed_token.claims.reg.sub
    } else {
        None
    }
}

pub fn encode_jwt(user: &User) -> String {
    // Create header
    let header = Header::default();
    let mut claims: Claims = Default::default();
    let user_data = to_string(user).unwrap();
    claims.reg.sub = Some(user_data);
    let token = Token::new(header, claims);
    let key = "secret".as_bytes();
    let raw = token.signed(key, crypto::sha2::Sha256::new()).unwrap();
    raw
}

pub fn decode_jwt(jwt_token: &str) -> Option<String> {
    let parsed_token = Token::parse(jwt_token);

    match parsed_token {
        Ok(parsed_token) => verify_token(parsed_token),
        Err(parsed_token_err) => {
            println!("{:?}", parsed_token_err);
            None
        }
    }
}
