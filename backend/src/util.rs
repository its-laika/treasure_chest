//! Module with utilites that can't be categorized otherwise

use crate::error::{Error, Result};
use crate::hash::{Hash, Hashing};
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;

/// Decodes and validates given file encryption key
///
/// Given `encoded_key` is decoded and then checked against given `hash`.
/// If the key is valid, it will be returned, otherwise error.
///
/// # Arguments
///
/// * `encoded_key` - File encryption key to decode and check
/// * `hash` - Hash to check key against
///
/// # Returns
///
/// * [`Ok<Vec<u8>>`] containing decoded and validated key  
/// * [`Err<Error>`] on error
pub fn get_validated_key(encoded_key: &str, hash: &str) -> Result<Vec<u8>> {
    let key = BASE64_URL_SAFE
        .decode(encoded_key)
        .map_err(|_| Error::KeyInvalid)?;

    match Hash::verify(&key, hash).map_err(|_| Error::KeyInvalid) {
        Ok(true) => Ok(key),
        _ => Err(Error::KeyInvalid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_key_returned() {
        let result = get_validated_key(
            "MQ==", // "1"
            "$argon2id$v=19$m=12,t=3,p=1$dzc0OGd1OWZveHMwMDAwMA$c76OJ4RDh1TlW1tdcbimWA",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [49]); // ["1"]
    }

    #[test]
    fn invalid_input_handled() {
        assert!(get_validated_key("MQ==", "xxxYYY").is_err());

        assert!(get_validated_key(
            "@@@",
            "$argon2id$v=19$m=12,t=3,p=1$dzc0OGd1OWZveHMwMDAwMA$c76OJ4RDh1TlW1tdcbimWA"
        )
        .is_err());
    }
}
