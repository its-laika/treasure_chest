use configuration::CONFIGURATION;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{process, time::Duration};

mod api;
mod configuration;
mod database;
mod encryption;
mod error;
mod file;
mod hash;
mod request;
mod util;

#[tokio::main]
async fn main() {
    env_logger::init();

    /* Init configuration */
    let connection_string = &CONFIGURATION.connection_string;

    log::info!("Connecting to database (connection timeout is 8 secs)...");

    let mut connect_options = ConnectOptions::new(connection_string);

    connect_options
        .sqlx_logging_level(log::LevelFilter::Debug)
        .max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let Ok(database_connection) = Database::connect(connect_options).await else {
        log::error!("Could not connect to database. Bye.");
        process::exit(1);
    };

    log::info!("Migrating database...");
    if let Err(error) = Migrator::up(&database_connection, None).await {
        log::error!("Could not migrate database: {error}");
    };

    log::info!("Starting API on {}...", &CONFIGURATION.listening_address);

    if let Err(error) = api::listen(database_connection.clone()).await {
        log::error!("API failed: {error}");
    }

    log::info!("API shut down. Closing database connection...");
    if let Err(error) = database_connection.close().await {
        log::error!("Could not close database connection: {error}");
    }

    log::info!("Bye.");
}
