//! Module containing functions that are related to HTTP requests

use super::error::{Error, Result};
use crate::configuration::CONFIGURATION;
use crate::file;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::HeaderMap;
use regex::Regex;
use std::sync::LazyLock;
use uuid::Uuid;

const FALLBACK_CONTENT_TYPE: &str = "application/octet-stream";

static FILE_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("filename=\"(.*?)\"").unwrap());

/// Tries getting request Ip from given `headers`
///
/// The header name defined in [`CONFIGURATION`] will be checked for an (Ip)
/// value and then returned. If the value is missing / empty, an [`Error`] is
/// returned.
///
/// # Arguments
///
/// * `headers` - Headers to check
///
/// # Returns
///
/// * [`Ok<String>`] on success, containing the request Ip  
/// * [`Err<Error>`] on error
pub fn get_request_ip(headers: &HeaderMap) -> Result<String> {
    Ok(headers
        .get(CONFIGURATION.ip_header_name.clone())
        .ok_or(Error::IpHeaderMissing(CONFIGURATION.ip_header_name.clone()))?
        .to_str()
        .map_err(|_| Error::IpHeaderInvalid)?
        .to_string())
}

impl From<file::Metadata> for HeaderMap {
    fn from(val: file::Metadata) -> Self {
        let mut headers = HeaderMap::new();

        if let Ok(content_disposition) =
            format!("attachment; filename=\"{}\"", val.file_name).parse()
        {
            headers.append(CONTENT_DISPOSITION, content_disposition);
        }

        if let Ok(content_type) = val.mime_type.parse() {
            headers.append(CONTENT_TYPE, content_type);
        }

        headers
    }
}

impl From<HeaderMap> for file::Metadata {
    fn from(value: HeaderMap) -> Self {
        let file_name = value
            .get(CONTENT_DISPOSITION)
            .and_then(|header_value| header_value.to_str().map(String::from).ok())
            .and_then(|header_value| {
                FILE_NAME_REGEX
                    .captures(&header_value)
                    .and_then(|captures| captures.get(1))
                    .map(|capture| capture.as_str().to_string())
            });

        let mime_type = value
            .get(CONTENT_TYPE)
            .and_then(|header_value| header_value.to_str().map(String::from).ok());

        Self {
            file_name: file_name.unwrap_or(Uuid::new_v4().to_string()),
            mime_type: mime_type.unwrap_or(FALLBACK_CONTENT_TYPE.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_metadata_to_headers() {
        let metadata = file::Metadata {
            file_name: "My file.exe".into(),
            mime_type: "my/mimetype".into(),
        };

        let headers: HeaderMap = metadata.into();

        assert_eq!(2, headers.len());
        assert_eq!(
            "attachment; filename=\"My file.exe\"",
            headers
                .get("Content-Disposition")
                .unwrap()
                .to_str()
                .unwrap()
        );
        assert_eq!(
            "my/mimetype",
            headers.get("Content-Type").unwrap().to_str().unwrap()
        )
    }

    #[test]
    fn test_from_headers_to_metadata() {
        let mut headers = HeaderMap::new();
        headers.append(
            "Content-Disposition",
            "attachment;  filename=\"My file.gif\" what=ever"
                .parse()
                .unwrap(),
        );
        headers.append("Content-Type", "my/mime+type".parse().unwrap());

        let metadata: file::Metadata = headers.into();

        assert_eq!("My file.gif", metadata.file_name);
        assert_eq!("my/mime+type", metadata.mime_type);
    }
    #[test]
    fn test_with_missing_headers_to_metadata() {
        let headers = HeaderMap::new();

        let metadata: file::Metadata = headers.into();

        assert!(!metadata.file_name.is_empty());
        assert_eq!("application/octet-stream", metadata.mime_type);
    }
}
