use crate::error::Error;
use crate::hash::{Hash, Hashing};
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;

pub fn get_validated_key(encoded_key: &str, hash: &str) -> Result<Vec<u8>, Error> {
    let key = BASE64_URL_SAFE
        .decode(encoded_key)
        .map_err(|_| Error::KeyInvalid)?;

    match Hash::verify(&key, hash) {
        Ok(true) => Ok(key),
        Ok(false) => Err(Error::KeyInvalid),
        Err(_) => Err(Error::KeyInvalid),
    }
}
