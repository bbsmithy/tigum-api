use bcrypt::{hash, hash_with_result, verify};

pub fn hash_password(plain: &String) -> Result<String, bcrypt::BcryptError> {
    Ok(hash(plain, 10)?)
}

pub fn verify_password(plain: &String, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
    Ok(verify(plain, hash_string)?)
}
