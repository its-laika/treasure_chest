use crate::configuration::CONFIGURATION;
use crate::encryption::{Encoding, Encryption};
use crate::error::Error;
use crate::file;
use crate::hash::{Hash, Hashing};
use crate::request;
use crate::return_logged;
use crate::{database, encryption};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::Request, http::StatusCode, Json};
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;
use futures::{StreamExt, TryStreamExt};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::io::{Error as IoError, ErrorKind};
use tokio_util::io::StreamReader;
use uuid::Uuid;

// A struct representing the response for the upload endpoint.
///
/// This struct is used to serialize the response containing the file id and
/// the encryption key.
#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub key: String,
}

/// Handles the file upload endpoint.
///
/// This function processes the upload request, validates the request, stores
/// the file, and returns the file id and encryption key.
pub async fn handler(
    State(database_connection): State<DatabaseConnection>,
    headers: HeaderMap,
    request: Request,
) -> impl IntoResponse {
    let request_ip = match request::get_request_ip(&headers) {
        Ok(ip) => ip,
        Err(error) => return_logged!(error, StatusCode::BAD_GATEWAY),
    };

    match database::is_upload_limit_reached(&database_connection, &request_ip).await {
        Ok(false) => (),
        Ok(true) => return Err(StatusCode::TOO_MANY_REQUESTS),
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    }

    let content = match extract_body(request).await {
        Ok(content) => content,
        Err(_) => return Err(StatusCode::PAYLOAD_TOO_LARGE),
    };

    let (encryption_data, key) = match encryption::Data::encrypt(content) {
        Ok(result) => result,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let id = Uuid::new_v4();

    if let Err(error) = file::store_data(&id, encryption_data.encode()) {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
    };

    let encrypted_metadata =
        match serde_json::to_string(&std::convert::Into::<file::Metadata>::into(headers))
            .map_err(Error::JsonSerializationFailed)
            .and_then(|json| encryption::Data::encrypt_with_key(json.bytes(), &key))
            .map(encryption::definitions::Encoding::encode)
        {
            Ok(metadata) => metadata,
            Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
        };

    if encrypted_metadata.len() > 255 {
        return Err(StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE);
    }

    let hash = match Hash::hash(&key) {
        Ok(hash) => hash,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Err(error) = database::store_file(
        &database_connection,
        &id,
        hash,
        request_ip,
        encrypted_metadata,
    )
    .await
    {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(Json(Response {
        id: id.into(),
        key: BASE64_URL_SAFE.encode(&key),
    }))
}

async fn extract_body(request: Request) -> Result<Vec<u8>, IoError> {
    let mut body = vec![];

    let body_data_stream = request
        .into_body()
        .into_data_stream()
        /* Add one byte to max size for range check later. If this byte is filled,
         * we know that the body is too large. */
        .take(CONFIGURATION.body_max_size + 1)
        .map_err(|err| IoError::new(ErrorKind::Other, err));

    let body_reader = StreamReader::new(body_data_stream);

    futures::pin_mut!(body_reader);
    tokio::io::copy(&mut body_reader, &mut body).await?;

    if body.len() > CONFIGURATION.body_max_size {
        return Err(IoError::new(
            ErrorKind::StorageFull,
            "Max body size exceeded",
        ));
    }

    Ok(body)
}
