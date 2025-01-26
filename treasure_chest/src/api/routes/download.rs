use crate::database;
use crate::file;
use crate::request;
use crate::return_logged;
use crate::util;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
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
    headers: HeaderMap,
    body: Json<RequestBody>,
) -> impl IntoResponse {
    let request_ip = match request::get_request_ip(&headers) {
        Ok(ip) => ip,
        Err(error) => return_logged!(error, StatusCode::BAD_GATEWAY),
    };

    let file = match database::get_downloadable_file(&database_connection, &id).await {
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Ok(Some(file)) => file,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let Ok(key) = util::get_validated_key(&body.key, &file.hash) else {
        if let Err(error) =
            database::store_access_log(&database_connection, &request_ip, &id, false).await
        {
            return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
        }

        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Err(error) =
        database::store_access_log(&database_connection, &request_ip, &id, true).await
    {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR)
    }

    let content = match file::load_encrypted_data(&id.to_string(), &key) {
        Ok(content) => content,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response_headers = match file::load_encrypted_metadata(&file, &key) {
        Ok(Some(metadata)) => request::build_headers(&metadata),
        _ => HeaderMap::new(),
    };

    if let Err(error) = file::delete(&id.to_string()) {
        log::error!("Could not delete used file {}: {:?}", id.to_string(), error);
    }

    Ok((response_headers, content))
}
