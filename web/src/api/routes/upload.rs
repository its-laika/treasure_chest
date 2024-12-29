use crate::api::request::get_client_ip;
use crate::db::model::FileInfo;
use crate::db::pool::Db;
use axum::debug_handler;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::Request, http::StatusCode, Json};
use futures::TryStreamExt;
use log::{error, warn};
use rusty_box_base::hash::{self, Hashing};
use rusty_box_base::{
    base64,
    encryption::{Encoding, Encryption, XChaCha20Poly1305},
    file,
};
use serde::Serialize;
use sqlx::{MySql, Pool};
use std::io;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

#[debug_handler] // TODO: Remove and "macro" feat of axum
pub async fn handler(
    headers: HeaderMap,
    State(pool): State<Pool<MySql>>,
    request: Request,
) -> impl IntoResponse {
    let client_ip = match get_client_ip(&headers) {
        Some(ip) => ip,
        None => {
            warn!("Request has no client ip. Sending: NOT_ACCEPTABLE");
            return Err(StatusCode::NOT_ACCEPTABLE);
        }
    };

    // TODO: Check if upload allowed, max file size
    let mut content = vec![];

    {
        let body_stream = request
            .into_body()
            .into_data_stream()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err));

        match StreamReader::new(body_stream)
            .read_to_end(&mut content)
            .await
        {
            Ok(_) => (),
            Err(error) => {
                error!("Could not read body: {error}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    let (encryption_data, key) = match XChaCha20Poly1305::encrypt(&content) {
        Ok(data) => data,
        Err(error) => {
            error!("Could not encrypt body: {error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let hash = match hash::Argon2::hash(&key) {
        Ok(h) => h,
        Err(error) => {
            error!("Could not encrypt body: {error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let file_path = match rusty_box_base::file::get_random_file_path() {
        Ok(p) => p,
        Err(error) => {
            error!("Could not encrypt body: {error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let file_name: String = file_path.file_name().unwrap().to_str().unwrap().into();

    if let Err(error) = file::store(&file_path, &encryption_data.encode()) {
        error!("Code not store encrypted data: {error}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let file_info = FileInfo::now(file_name.clone(), hash, client_ip);

    if let Err(error) = pool.store_file_info(&file_info).await {
        error!("Code not store encrypted data: {error}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let response = Response {
        id: file_name,
        key: base64::encode(&key),
    };

    Ok(Json(response))
}

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub key: String,
}
