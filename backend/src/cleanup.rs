use crate::{
    database,
    error::{Error, Result},
    file,
};
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::{
    sync::broadcast::{self, error::TryRecvError},
    time,
};
use uuid::Uuid;

/// The interval in seconds between each cleanup operation.
/// Currently set to 10 minutes.
const CLEANUP_INTERVAL_SECONDS: u64 = 10 * 60; /* 10 minutes */

/// Runs the cleanup process in a loop, until `shutdown` signal is received.
///
/// # Arguments
///
/// * `database_connection` - A connection to the database.
/// * `shutdown` - A broadcast receiver to listen for shutdown signals.
///
/// # Returns
///
/// * `Result<()>` - Returns an Ok result if the cleanup process runs successfully,
///   or an error if something goes wrong.
pub async fn run(
    database_connection: DatabaseConnection,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    loop {
        if !wait(&mut shutdown).await? {
            return Ok(());
        }

        log::info!("Cleaning up outdating files...");

        database::remove_undownloadable_files(&database_connection).await?;
        delete_outdated_files(&database_connection).await?;
    }
}

/// Waits for defined cleanup interval, returning early if `shutdown` is received.
///
/// # Arguments
///
/// * `shutdown` - A broadcast receiver to listen for shutdown signals.
///
/// # Returns
///
/// * `Result<bool>` - Returns an Ok result if the wait completes successfully,
///   with `true` if the wait was not interrupted by a shutdown signal, or `false` otherwise.
async fn wait(shutdown: &mut broadcast::Receiver<()>) -> Result<bool> {
    let sleep_time = Duration::from_secs(1);

    for _ in 0..CLEANUP_INTERVAL_SECONDS {
        /* Prevent locking for 10 mins after a SIGINT like a spin lock */
        match shutdown.try_recv() {
            Err(TryRecvError::Empty) => (),
            Err(_) => return Err(Error::BroadcastRecvFailed),
            Ok(()) => return Ok(false),
        };

        time::sleep(sleep_time).await;
    }

    Ok(true)
}

/// Deletes outdated files from the file system.
///
/// # Arguments
///
/// * `database_connection` - A connection to the database.
///
/// # Returns
///
/// * `Result<()>` - Returns an Ok result if the cleanup process runs successfully,
///   or an error if something goes wrong.
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
