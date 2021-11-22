use bcrypt::{hash, verify};
use crypto;
use jwt::{Claims, Header, Token};
use std::default::Default;
use crate::db::models::user::User;
use serde_json::to_string;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Sha512, Digest};

const HASH_KEY_STRING: &str = "9KlI12SBL";
const HASH_KEY_NUMBER: &str = "5612hsgdLK";

#[derive(Hash)]
pub struct EmailHash {
    hash_key: String,
    email: String
}

pub fn create_known_hash_email(email: String) -> u64 {
    let email_hash = EmailHash {
        hash_key: HASH_KEY_NUMBER.to_string(),
        email: email
    };
    let mut s = DefaultHasher::new();
    email_hash.hash(&mut s);
    s.finish()
}

pub fn create_known_hash_string(val: u64) -> String {

    let email_hash_string = format!("{}", val);

    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(format!("{}{}", email_hash_string, HASH_KEY_STRING));

    // read hash digest and consume hasher
    let byte_array_result = hasher.finalize();
    let hash_string_result: String = format!("{:x}", byte_array_result);

    hash_string_result

}

pub fn hash_string(plain: &String) -> Result<String, bcrypt::BcryptError> {
    hash(plain, 10)
}

pub fn verify_hash(plain: &String, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(plain, hash_string)
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
            None
        }
    }
}
