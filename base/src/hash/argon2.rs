use super::definitions::{Error, Hashing};
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};

pub struct Argon2 {}

impl Hashing for Argon2 {
    fn hash(data: &[u8]) -> Result<String, super::definitions::Error> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(argon2::Argon2::default()
            .hash_password(data, &salt)
            .map_err(|_| Error::HashingFailure)?
            .to_string())
    }

    fn verify(data: &[u8], hash: &str) -> Result<bool, super::definitions::Error> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| Error::VerifyingFailure)?;

        Ok(argon2::Argon2::default()
            .verify_password(data, &parsed_hash)
            .is_ok())
    }
}
