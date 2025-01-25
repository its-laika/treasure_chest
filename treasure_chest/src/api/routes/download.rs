use crate::database::{get_downloadable_file, store_access_log};
use crate::file::{delete_file, load_encrypted_data, load_encrypted_metadata};
use crate::request::{build_header_map, get_request_ip};
use crate::return_logged;
use crate::util::get_validated_key;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use log::error;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RequestBody {
    pub key: String,
}

pub async fn handler(
    State(database_connection): State<DatabaseConnection>,
    id: Path<Uuid>,
    header_map: HeaderMap,
    body: Json<RequestBody>,
) -> impl IntoResponse {
    let request_ip = match get_request_ip(&header_map) {
        Ok(ip) => ip,
        Err(error) => return_logged!(error, StatusCode::BAD_GATEWAY),
    };

    let file = match get_downloadable_file(&database_connection, &id).await {
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Ok(Some(file)) => file,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let Ok(key) = get_validated_key(&body.key, &file.hash) else {
        if let Err(error) = store_access_log(&database_connection, &request_ip, &id, false).await {
            return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
        }

        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Err(error) = store_access_log(&database_connection, &request_ip, &id, true).await {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR)
    }

    let content = match load_encrypted_data(&id.to_string(), &key) {
        Ok(content) => content,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let header_hap = match load_encrypted_metadata(&file, &key) {
        Ok(Some(metadata)) => build_header_map(&metadata),
        _ => HeaderMap::new(),
    };

    if let Err(error) = delete_file(&id.to_string()) {
        error!("Could not delete used file {}: {:?}", id.to_string(), error);
    }

    Ok((header_hap, content))
}
