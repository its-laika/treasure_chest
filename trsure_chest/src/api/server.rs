use axum::{routing::post, Router};
use log::info;
use sea_orm::DatabaseConnection;
use std::io::Error;
use tokio::{net::TcpListener, net::ToSocketAddrs};

use super::routes::{download, upload};

pub async fn init<'a, A: ToSocketAddrs + std::fmt::Display>(
    address: &A,
    connection: DatabaseConnection,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/files", post(upload::handler))
        .route("/files/:id", post(download::handler))
        .with_state(connection);

    let listener = TcpListener::bind(&address).await?;

    info!("Start listening on {}...", &address);

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
