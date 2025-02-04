use configuration::CONFIGURATION;
use laika::shotgun;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{process, time::Duration};
use tokio::{signal::ctrl_c, task::JoinSet};

mod api;
mod cleanup;
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

    let database_connection = match setup_database(connection_string).await {
        Some(database_connection) => database_connection,
        None => {
            log::error!("Bye.");
            process::exit(1);
        }
    };

    let mut join_set = JoinSet::new();
    let (shotgun_tx, shotgun_rx) = shotgun::channel();

    let api_database_connection = database_connection.clone();
    let api_shutdown_rx = shotgun_rx.clone();

    join_set.spawn(async move {
        if let Err(error) = api::listen(api_database_connection, api_shutdown_rx).await {
            log::error!("API failed: {error}");
        }
    });

    let cleanup_database_connection = database_connection.clone();
    let cleanup_shutdown_rx = shotgun_rx.clone();

    join_set.spawn(async move {
        if let Err(error) = cleanup::run(cleanup_database_connection, cleanup_shutdown_rx).await {
            log::error!("Cleanup failed: {:?}", error);
        }
    });

    join_set.spawn(async move {
        use log::{error, info};

        if let Err(error) = ctrl_c().await {
            error!("Could not listen to ctrl+c (SIGINT): {error}");
            error!("Exiting process. Bye.");
            process::exit(1);
        }

        info!("Received ctrl+c (SIGINT)");

        shotgun_tx.send(());
    });

    join_set.join_all().await;

    log::info!("Closing database connection...");
    if let Err(error) = database_connection.close().await {
        log::error!("Could not close database connection: {error}");
    }

    log::info!("Bye.");
}

async fn setup_database(connection_string: &str) -> Option<DatabaseConnection> {
    let mut connect_options = ConnectOptions::new(connection_string);

    log::info!("Connecting and setting up database (connection timeout is 8 secs)...");

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
        return None;
    };

    if Migrator::up(&database_connection, None).await.is_err() {
        log::error!("Could not migrate database");
        return None;
    };

    Some(database_connection)
}
