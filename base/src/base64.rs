//! Facade for base64 encoding and decoding data
use base64::{prelude::BASE64_URL_SAFE, DecodeError, Engine};

/// Encodes given `data` as base64
///
/// # Arguments
///
/// * `data` - The data to encode
///
/// # Returs
///
/// Url safe base64 string representing `data`
pub fn encode(data: &[u8]) -> String {
    BASE64_URL_SAFE.encode(data)
}

/// Decodes url safe base64 encoded string
///
/// # Arguments
///
/// * `encoded` - url safe base64 encoded string
///
/// # Returns
///
/// Either data or `DecodeError` if decoding fails
pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError> {
    BASE64_URL_SAFE.decode(encoded)
}
