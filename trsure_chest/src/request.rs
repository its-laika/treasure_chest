use super::error::Error;
use crate::configuration::CONFIGURATION;
use axum::body::Body;
use axum::http::HeaderMap;
use base::encryption::{Encryption, XChaCha20Poly1305};

pub type DecryptionKey = Vec<u8>;
pub type EncryptionResult = Result<(XChaCha20Poly1305, DecryptionKey), Error>;

pub fn get_request_ip(header_map: &HeaderMap) -> Result<String, Error> {
    return Ok(header_map
        .get(CONFIGURATION.ip_header_name.clone())
        .ok_or(Error::IpHeaderMissing(CONFIGURATION.ip_header_name.clone()))?
        .to_str()
        .map_err(|_| Error::IpHeaderInvalid)?
        .to_string());
}

pub async fn encrypt_body(request_body: Body) -> EncryptionResult {
    let content = axum::body::to_bytes(request_body, CONFIGURATION.body_max_size)
        .await
        .map_err(Error::ReadingBodyFailed)?;

    let (encryption_data, key) =
        XChaCha20Poly1305::encrypt(&content).map_err(Error::EncrytpionFailed)?;

    Ok((encryption_data, key))
}
