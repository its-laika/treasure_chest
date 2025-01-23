use crate::database::{is_recent_uploads_limit_reached, store};
use crate::error::Error;
use crate::file::store_data;
use crate::hash::{Hash, Hashing};
use crate::request::{encrypt_body, get_request_ip};
use crate::return_logged;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::Request, http::StatusCode, Json};
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub key: String,
}

pub async fn handler(
    State(database_connection): State<DatabaseConnection>,
    header_map: HeaderMap,
    request: Request,
) -> impl IntoResponse {
    let request_ip = match get_request_ip(&header_map) {
        Ok(ip) => ip,
        Err(error) => return_logged!(error, StatusCode::BAD_GATEWAY),
    };

    match is_recent_uploads_limit_reached(&database_connection, &request_ip).await {
        Ok(false) => (),
        Ok(true) => return Err(StatusCode::TOO_MANY_REQUESTS),
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    }

    let (encryption_data, key) = match encrypt_body(request.into_body()).await {
        Ok(encryption_data) => encryption_data,
        Err(Error::ReadingBodyFailed(_)) => return Err(StatusCode::BAD_REQUEST),
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let hash = match Hash::hash(&key) {
        Ok(hash) => hash,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let id = Uuid::new_v4();

    if let Err(error) = store_data(encryption_data, &id.to_string()) {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
    };

    if let Err(error) = store(&database_connection, &id, &hash, &request_ip).await {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
    };

    let response = Response {
        id: id.into(),
        key: BASE64_URL_SAFE.encode(&key),
    };

    Ok(Json(response))
}
