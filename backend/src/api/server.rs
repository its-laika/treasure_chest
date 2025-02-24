use super::routes;
use crate::configuration::CONFIGURATION;
use axum::{
    routing::{get, post},
    Router,
};
use laika::shotgun;
use sea_orm::DatabaseConnection;
use std::io::Result;
use tokio::net::TcpListener;

/// Starts the server and listens for incoming connections.
///
/// This function sets up the routes for the API, binds the server to the
/// specified address, and starts listening for incoming connections. It also
/// handles graceful shutdown when a shutdown signal is received.
///
/// # Routes
/// See [`routes`] folder for all available routes.
///
/// # Arguments
///
/// * `connection` - A `DatabaseConnection` instance used to interact with the
///   database.
/// * `shutdown` - A `shotgun::Receiver<()>` used to receive shutdown signals.
///
/// # Returns
///
/// * [`Ok<()>`] on graceful shutdown
/// * [`Err<Error>`] on error
///
/// # Example
///
/// ```rust
/// use sea_orm::DatabaseConnection;
/// use laika::shotgun;
/// use tokio::runtime::Runtime;
///
/// let connection = DatabaseConnection::new();
/// let (shutdown_sender, shutdown_receiver) = shotgun::channel();
///
/// let rt = Runtime::new().unwrap();
/// rt.block_on(async {
///     listen(connection, shutdown_receiver).await.unwrap();
/// });
/// ```
pub async fn listen(connection: DatabaseConnection, shutdown: shotgun::Receiver<()>) -> Result<()> {
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

/// Logs an error and returns a specified status.
///
/// This macro logs the provided error using the `log` crate and then returns
/// the specified status.
///
/// # Arguments
///
/// * `$error` - The error message to be logged.
/// * `$status` - The HTTP status code to be returned.
///
/// # Example
///
/// ```rust
/// return_logged!(some_error, StatusCode::INTERNAL_SERVER_ERROR);
#[macro_export]
macro_rules! return_logged {
    ($error: expr, $status: expr) => {{
        log::error!("{:?}", $error);
        return Err($status);
    }};
}
