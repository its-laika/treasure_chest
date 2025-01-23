use super::error::Error;
use crate::configuration::CONFIGURATION;
use crate::encryption::{Encryption, EncryptionData};
use axum::body::Body;
use axum::http::HeaderMap;

pub type DecryptionKey = Vec<u8>;
pub type EncryptionResult = Result<(EncryptionData, DecryptionKey), Error>;

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
        EncryptionData::encrypt(&content).map_err(Error::EncrytpionFailed)?;

    Ok((encryption_data, key))
}
