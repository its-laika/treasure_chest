use crate::error::Error;
use base::{
    base64,
    hash::{self, Hashing},
};

pub fn get_validated_key(encoded_key: &str, hash: &str) -> Result<Vec<u8>, Error> {
    let key = base64::decode(encoded_key).map_err(|_| Error::KeyInvalid)?;

    match hash::Argon2::verify(&key, hash) {
        Ok(true) => Ok(key),
        Ok(false) => Err(Error::KeyInvalid),
        Err(_) => Err(Error::KeyInvalid),
    }
}
