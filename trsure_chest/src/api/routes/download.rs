use crate::db::{get_downloadable_file, mark_downloaded};
use crate::file::{delete_file, load_encrypted_data};
use crate::request::get_request_ip;
use crate::return_logged;
use crate::util::get_validated_key;
use axum::debug_handler;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use log::error;
use sea_orm::{DatabaseConnection, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RequestBody {
    pub key: String,
}

#[debug_handler]
pub async fn handler(
    State(database_connection): State<DatabaseConnection>,
    id: Path<Uuid>,
    header_map: HeaderMap,
    body: Json<RequestBody>,
) -> impl IntoResponse {
    let Ok(request_ip) = get_request_ip(&header_map) else {
        return Err(StatusCode::BAD_GATEWAY);
    };

    // TODO: Rate limit

    let file = match get_downloadable_file(&database_connection, &id).await {
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Ok(Some(file)) => file,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    let file_id = match file.id.clone().try_into() {
        Ok(bytes) => Uuid::from_bytes(bytes),
        Err(_) => {
            error!("Could not parse stored Id into Uuid");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let Ok(key) = get_validated_key(&body.key, &file.hash) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Err(error) =
        mark_downloaded(&database_connection, file.into_active_model(), &request_ip).await
    {
        return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR);
    }

    let content = match load_encrypted_data(&file_id.to_string(), &key) {
        Ok(content) => content,
        Err(error) => return_logged!(error, StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Err(error) = delete_file(&file_id.to_string()) {
        error!("Could not delete used file {file_id}: {:?}", error);
    }

    Ok(content)
}
