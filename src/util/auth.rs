use bcrypt::{hash, verify};
use crypto;
use jwt::{Claims, Header, Token};
use std::default::Default;

pub fn hash_password(plain: &String) -> Result<String, bcrypt::BcryptError> {
    hash(plain, 10)
}

pub fn verify_password(plain: &String, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
    Ok(verify(plain, hash_string)?)
}

pub fn encode_jwt(user_id: i32) -> String {
    // Create header
    let header = Header::default();
    let mut claims: Claims = Default::default();
    claims.reg.sub = Some(user_id.to_string());
    let token = Token::new(header, claims);
    let key = "secret".as_bytes();
    let raw = token.signed(key, crypto::sha2::Sha256::new()).unwrap();
    raw
}
