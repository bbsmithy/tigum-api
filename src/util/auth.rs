use bcrypt::{hash, verify, hash_with_result};

pub fn hash_password(plain: &String) -> Result<String, bcrypt::BcryptError> {
        Ok(hash(plain, 10)?)
}