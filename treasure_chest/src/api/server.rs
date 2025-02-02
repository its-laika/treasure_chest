use super::routes;
use crate::configuration::CONFIGURATION;
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use std::io;
use tokio::{net::TcpListener, signal::ctrl_c};

pub async fn listen(connection: DatabaseConnection) -> io::Result<()> {
    let app = Router::new()
        .route("/files", post(routes::upload::handler))
        .route("/files/{id}/download", post(routes::download::handler))
        .route("/configuration", get(routes::configuration::handler))
        .with_state(connection);

    let listener = TcpListener::bind(&CONFIGURATION.listening_address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            if let Err(error) = ctrl_c().await {
                log::error!("Could not wait for crtl+c: {error}");
                return;
            };

            log::info!("Received ctrl+c (SIGINT)");
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
