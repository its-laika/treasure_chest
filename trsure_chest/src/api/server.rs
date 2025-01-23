use super::routes::{download, upload};
use crate::configuration::CONFIGURATION;
use axum::{routing::post, Router};
use log::info;
use sea_orm::DatabaseConnection;
use std::io::Error;
use tokio::net::TcpListener;

pub async fn listen(connection: DatabaseConnection) -> Result<(), Error> {
    let app = Router::new()
        .route("/files", post(upload::handler))
        .route("/files/{id}/download", post(download::handler))
        .with_state(connection);

    let listener = TcpListener::bind(&CONFIGURATION.listening_address).await?;

    info!("Start listening on {}...", &CONFIGURATION.listening_address);

    axum::serve(listener, app).await?;

    Ok(())
}

#[macro_export]
macro_rules! return_logged {
    ($error: expr, $status: expr) => {{
        log::error!("{:?}", $error);
        return Err($status);
    }};
}
