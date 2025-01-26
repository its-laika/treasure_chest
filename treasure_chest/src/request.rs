use super::error::Error;
use crate::configuration::CONFIGURATION;
use crate::encryption::{self, Encryption};
use crate::file;
use axum::body::Body;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::HeaderMap;
use regex::Regex;
use std::sync::LazyLock;
use uuid::Uuid;

const FALLBACK_CONTENT_TYPE: &str = "application/octet-stream";

static FILE_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("filename=\"(.*?)\"").unwrap());

pub type DecryptionKey = Vec<u8>;
pub type EncryptionResult = Result<(encryption::Data, DecryptionKey), Error>;

pub fn get_request_ip(headers: &HeaderMap) -> Result<String, Error> {
    return Ok(headers
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
        encryption::Data::encrypt(&content).map_err(Error::EncryptionFailed)?;

    Ok((encryption_data, key))
}

pub fn get_metadata(headers: &HeaderMap) -> file::Metadata {
    let file_name = headers
        .get(CONTENT_DISPOSITION)
        .and_then(|header_value| header_value.to_str().map(String::from).ok())
        .and_then(|header_value| {
            FILE_NAME_REGEX
                .captures(&header_value)
                .and_then(|captures| captures.get(1))
                .map(|capture| capture.as_str().to_string())
        });

    let mime_type = headers
        .get(CONTENT_TYPE)
        .and_then(|header_value| header_value.to_str().map(String::from).ok());

    file::Metadata {
        file_name: file_name.unwrap_or(Uuid::new_v4().to_string()),
        mime_type: mime_type.unwrap_or(FALLBACK_CONTENT_TYPE.into()),
    }
}

pub fn build_headers(metadata: &file::Metadata) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Ok(content_disposition) =
        format!("attachment; filename=\"{}\"", metadata.file_name).parse()
    {
        headers.append(CONTENT_DISPOSITION, content_disposition);
    }

    if let Ok(content_type) = metadata.mime_type.parse() {
        headers.append(CONTENT_TYPE, content_type);
    }

    headers
}
