use axum::debug_handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[debug_handler] // TODO: Remove and "macro" feat of axum
pub async fn handler(//   headers: HeaderMap,
    //   State(pool): State<DatabaseConnection>,
 //   Path(name): Path<String>,
 //   body: String,
) -> impl IntoResponse {
    /*   let client_ip = match get_client_ip(&headers) {
            Some(ip) => ip,
            None => {
                warn!("Request has no client ip. Sending: NOT_ACCEPTABLE");
                return Err(StatusCode::NOT_ACCEPTABLE);
            }
        };

        let mut file_info = match pool.load_file_info(&name).await {
            Ok(None) => {
                warn!("Could not find file info for name: {name}. Sending: NOT_FOUND");
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(Some(f)) => f,
            Err(error) => {
                error!(
                    "Could load file info for name: {name}: {error}. Sending: INTERNAL_SERVER_ERROR"
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if !file_info.is_downloadable() {
            info!("Tried downloading file {name} but not downloadable anymore.");
            return Err(StatusCode::GONE);
        }

        let key = match base64::decode(&body) {
            Ok(k) => k,
            Err(_) => {
                info!("Invalid base64 encoded key given. Sending: BAD_REQUEST");
                return Err(StatusCode::BAD_REQUEST);
            }
        };

        match file_info.matches_key(&key) {
            Ok(true) => (),
            Ok(false) => {
                info!("Tried downloading file {name} but not key doesn't match.");
                return Err(StatusCode::UNAUTHORIZED);
            }
            Err(error) => {
                error!("Could verify hash: {error}. Sending: INTERNAL_SERVER_ERROR");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        file_info.mark_download(client_ip.clone());
        if let Err(error) = pool.store_file_info(&file_info).await {
            error!("Could store file info: {error}. Sending: INTERNAL_SERVER_ERROR");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let file_path = match file::find(&name) {
            Ok(f) => f,
            Err(error) => {
                warn!("Could not find file: {error}. Sending: NOT_FOUND");
                return Err(StatusCode::NOT_FOUND);
            }
        };

        let file = match file::load(&file_path) {
            Ok(f) => f,
            Err(error) => {
                warn!("Could not find file: {error}. Sending: NOT_FOUND");
                return Err(StatusCode::NOT_FOUND);
            }
        };

        let encryption_data = match XChaCha20Poly1305::decode(&file) {
            Ok(d) => d,
            Err(error) => {
                error!("Could not decode file: {error}. Sending: INTERNAL_SERVER_ERROR");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let decrypted_file = match encryption_data.decrypt(&key) {
            Ok(d) => d,
            Err(error) => {
                warn!("Could not decode file: {error}. Sending: UNAUTHORIZED");
                return Err(StatusCode::UNAUTHORIZED);
            }
        };

        if let Err(error) = file::ensure_deleted(&file_path) {
            error!(
                "Could not delete file '{}': {}. Sending: INTERNAL_SERVER_ERROR",
                file_path.display(),
                error
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        info!(
            "File '{}' has been accessed by IP {} and deleted",
            client_ip,
            &file_path.display()
        );
    */
    StatusCode::OK
}
