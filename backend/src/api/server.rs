use super::routes;
use crate::configuration::CONFIGURATION;
use axum::{
    routing::{get, post},
    Router,
};
use laika::shotgun;
use sea_orm::DatabaseConnection;
use std::io;
use tokio::net::TcpListener;

pub async fn listen(
    connection: DatabaseConnection,
    shutdown: shotgun::Receiver<()>,
) -> io::Result<()> {
    let app = Router::new()
        .route("/api/files", post(routes::upload::handler))
        .route("/api/files/{id}/download", post(routes::download::handler))
        .route("/api/configuration", get(routes::configuration::handler))
        .with_state(connection);

    let listener = TcpListener::bind(&CONFIGURATION.listening_address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown.recv().await;
        })
        .await
}

#[macro_export]
macro_rules! return_logged {
    ($error: expr, $status: expr) => {{
        log::error!("{:?}", $error);
        return Err($status);
    }};
}
