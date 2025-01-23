use super::error::Error;
use crate::configuration::CONFIGURATION;
use chrono::Utc;
use entity::file;
use sea_orm::{
    ActiveValue::NotSet, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};
use sea_orm::{ColumnTrait, FromQueryResult};
use uuid::Uuid;

#[derive(FromQueryResult)]
struct CountResult {
    count: i64,
}

pub async fn get_downloadable_file(
    database_connection: &DatabaseConnection,
    id: &Uuid,
) -> Result<Option<entity::file::Model>, Error> {
    entity::prelude::File::find()
        .filter(entity::file::Column::Id.eq(*id))
        .filter(entity::file::Column::DownloadUntil.gte(Utc::now()))
        .filter(entity::file::Column::DownloadedAt.is_null())
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)
}

pub async fn mark_downloaded(
    database_connection: &DatabaseConnection,
    mut file: file::ActiveModel,
    ip: &str,
) -> Result<(), Error> {
    file.downloaded_at = Set(Some(Utc::now().naive_utc()));
    file.downloader_ip = Set(Some(ip.into()));

    entity::prelude::File::update(file)
        .exec(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?;

    Ok(())
}

pub async fn is_recent_uploads_limit_reached(
    database_connection: &DatabaseConnection,
    ip: &str,
) -> Result<bool, Error> {
    let min_uploaded_at = Utc::now()
        .checked_sub_days(CONFIGURATION.recent_uploads_timespan)
        .ok_or(Error::DateCalculationFailed)?;

    let count = entity::prelude::File::find()
        .select_only()
        .column_as(entity::file::Column::Id.count(), "count")
        .filter(entity::file::Column::UploaderIp.eq(ip))
        .filter(entity::file::Column::UploadedAt.gte(min_uploaded_at.naive_utc()))
        .into_model::<CountResult>()
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?
        .unwrap_or(CountResult { count: 0 })
        .count;

    Ok(count >= CONFIGURATION.recent_uploads_maximum.into())
}

pub async fn store(
    database_connection: &DatabaseConnection,
    id: &Uuid,
    hash: &str,
    uploader_ip: &str,
) -> Result<(), Error> {
    let now = Utc::now();

    let download_until = now
        .checked_add_days(CONFIGURATION.file_lifetime)
        .ok_or(Error::DateCalculationFailed)?;

    let file = entity::file::ActiveModel {
        id: Set((*id).into()),
        hash: Set(hash.into()),
        downloader_ip: NotSet,
        uploader_ip: Set(uploader_ip.into()),
        uploaded_at: Set(now.naive_utc()),
        download_until: Set(download_until.naive_utc()),
        downloaded_at: NotSet,
    };

    entity::prelude::File::insert(file)
        .exec(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?;

    Ok(())
}
