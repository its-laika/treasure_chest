use super::definitions::Hashing;
use crate::error::{Error, Result};
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};

/// A struct representing the Argon2 hashing algorithm.
pub struct Argon2 {}

impl Hashing for Argon2 {
    fn hash(data: &[u8]) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(argon2::Argon2::default()
            .hash_password(data, &salt)
            .map_err(|error| Error::HashingFailure(error.to_string()))?
            .to_string())
    }

    fn verify(data: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|error| Error::HashVerificationFailure(error.to_string()))?;

        Ok(argon2::Argon2::default()
            .verify_password(data, &parsed_hash)
            .is_ok())
    }
}
