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

const CLEANUP_INTERVAL_SECONDS: u64 = 10 * 1; /* 10 minutes */

pub async fn run(
    database_connection: DatabaseConnection,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    loop {
        wait(&mut shutdown).await?;

        log::info!("Cleaning up outdating files...");

        database::remove_undownloadable_files(&database_connection).await?;
        delete_outdated_files(&database_connection).await?;
    }
}

async fn wait(shutdown: &mut broadcast::Receiver<()>) -> Result<()> {
    let sleep_time = Duration::from_secs(1);

    for _ in 0..CLEANUP_INTERVAL_SECONDS {
        /* Prevent locking for 10 mins after a SIGINT like a spin lock */
        match shutdown.try_recv() {
            Err(TryRecvError::Empty) => (),
            Err(_) => return Err(Error::BroadcastRecvFailed),
            Ok(_) => return Ok(()),
        };

        time::sleep(sleep_time).await;
    }

    Ok(())
}

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
