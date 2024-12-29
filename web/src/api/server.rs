use axum::{routing::post, Router};
use log::info;
use sqlx::{MySql, Pool};
use std::io::Error;
use tokio::{net::TcpListener, net::ToSocketAddrs};

use super::routes::{download, upload};

pub async fn init<'a, A: ToSocketAddrs + std::fmt::Display>(
    address: &A,
    pool: Pool<MySql>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/files", post(upload::handler))
        .route("/files/:id", post(download::handler))
        .with_state(pool);

    let listener = TcpListener::bind(&address).await?;

    info!("Start listening on {}...", &address);

    axum::serve(listener, app).await?;

    Ok(())
}
