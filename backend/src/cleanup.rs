use crate::{database, error::Result, file};
use laika::shotgun;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::{select, time};
use uuid::Uuid;

/// The interval in seconds between each cleanup operation.
/// Currently set to 10 minutes.
const CLEANUP_INTERVAL_SECONDS: u64 = 10 * 60; /* 10 minutes */

/// Runs the cleanup process in a loop, until `shutdown` signal is received.
///
/// # Arguments
///
/// * `database_connection` - A connection to the database.
/// * `shutdown` - A shotgun receiver to listen for shutdown signal.
///
/// # Returns
///
/// * [`Ok<()>`] on successful cleanup process
/// * [`Err<Error>`] on error
pub async fn run(
    database_connection: DatabaseConnection,
    shutdown: shotgun::Receiver<()>,
) -> Result<()> {
    loop {
        let shutdown = shutdown.clone();

        select! {
            _ = time::sleep(Duration::from_secs(CLEANUP_INTERVAL_SECONDS)) => (),
            _ = shutdown => return Ok(()),
        };

        log::info!("Cleaning up outdating files...");

        database::remove_undownloadable_files(&database_connection).await?;
        delete_outdated_files(&database_connection).await?;
    }
}

/// Deletes outdated files from the file system.
///
/// # Arguments
///
/// * `database_connection` - A connection to the database.
///
/// # Returns
///
/// * [`Ok<()>`] on successful cleanup
/// * [`Err<Error>`] on error
async fn delete_outdated_files(database_connection: &DatabaseConnection) -> Result<()> {
    let downloadable_file_ids = database::get_downloadable_file_ids(database_connection).await?;

    let stored_file_ids = file::get_stored_file_ids()?;

    let file_ids_to_delete = stored_file_ids
        .iter()
        .filter(|stored| {
            downloadable_file_ids
                .iter()
                .all(|downloadable| &downloadable != stored)
        })
        .collect::<Vec<&Uuid>>();

    for file_id in file_ids_to_delete {
        file::delete(file_id)?;
        log::info!("Deleted outdated file: {file_id}");
    }

    Ok(())
}
