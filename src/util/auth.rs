use bcrypt::{hash, verify, hash_with_result};

pub fn hash_password(plain: &String) -> Result<String, bcrypt::BcryptError> {
    Ok(hash(plain, 10)?)
}

pub fn verify_password(plain: &String, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
   verify(plain, hash_string)
}