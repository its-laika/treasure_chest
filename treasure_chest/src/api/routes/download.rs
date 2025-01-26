use crate::database;
use crate::encryption;
use crate::encryption::Encoding;
use crate::encryption::Encryption;
use crate::error::Error;
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

    let content = match file::load_data(&id)
        .and_then(encryption::Data::decode)
        .and_then(|data| data.decrypt(&key))
    {
        Ok(content) => content,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response_headers = match encryption::Data::decode(file.encrypted_metadata)
        .and_then(|data| data.decrypt(&key))
        .and_then(|data| String::from_utf8(data).map_err(|_| Error::DecryptionFailed))
        .and_then(|json| {
            serde_json::from_str::<file::Metadata>(&json).map_err(Error::JsonSerializationFailed)
        }) {
        Ok(metadata) => metadata.into(),
        _ => HeaderMap::new(),
    };

    if let Err(error) = file::delete(&id) {
        log::error!("Could not delete used file {}: {error:?}", id.to_string());
    }

    Ok((response_headers, content))
}
