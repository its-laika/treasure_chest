use std::time::Duration;

use crate::{
    database,
    error::{Error, Result},
    file,
};
use sea_orm::DatabaseConnection;
use tokio::{
    sync::broadcast::{self, error::TryRecvError},
    time,
};
use uuid::Uuid;

const CLEANUP_INTERVAL_SECONDS: u64 = 10 * 60; /* 10 minutes */

pub async fn clean(
    database_connection: DatabaseConnection,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    let sleep_time = Duration::from_secs(1);
    loop {
        for _ in 0..CLEANUP_INTERVAL_SECONDS {
            match shutdown.try_recv() {
                Err(TryRecvError::Empty) => (),
                Err(_) => return Err(Error::BroadcastRecvFailed),
                Ok(_) => return Ok(()),
            };

            time::sleep(sleep_time).await;
        }

        log::info!("Cleaning up outdating files...");

        let downloadable_file_ids =
            database::get_downloadable_file_ids(&database_connection).await?;

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
    }
}
