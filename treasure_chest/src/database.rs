use super::error::Error;
use crate::configuration::CONFIGURATION;
use chrono::{Days, Utc};
use entity::{self, access_log, file};
use sea_orm::sea_query::Query;
use sea_orm::{ColumnTrait, Condition, FromQueryResult};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set};
use uuid::Uuid;

#[derive(FromQueryResult)]
struct CountResult {
    count: i64,
}

pub async fn get_downloadable_file(
    database_connection: &DatabaseConnection,
    id: &Uuid,
) -> Result<Option<file::Model>, Error> {
    entity::File::find()
        .filter(file::Column::Id.eq(*id))
        .filter(file::Column::DownloadUntil.gte(Utc::now()))
        .filter(
            file::Column::Id.not_in_subquery(
                Query::select()
                    .column(access_log::Column::FileId)
                    .from(access_log::Entity)
                    .cond_where(Condition::all().add(access_log::Column::Successful.eq(1)))
                    .to_owned(),
            ),
        )
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)
}

pub async fn is_upload_limit_reached(
    database_connection: &DatabaseConnection,
    ip: &str,
) -> Result<bool, Error> {
    let min_uploaded_at = Utc::now()
        .checked_sub_days(Days::new(1))
        .ok_or(Error::DateCalculationFailed)?;

    let count = entity::File::find()
        .select_only()
        .column_as(file::Column::Id.count(), "count")
        .filter(file::Column::UploaderIp.eq(ip))
        .filter(file::Column::UploadedAt.gte(min_uploaded_at.naive_utc()))
        .into_model::<CountResult>()
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?
        .unwrap_or(CountResult { count: 0 })
        .count;

    Ok(count >= CONFIGURATION.ip_uploads_per_day.into())
}

pub async fn is_download_limit_reached(
    database_connection: &DatabaseConnection,
    id: &Uuid,
) -> Result<bool, Error> {
    let count = entity::AccessLog::find()
        .select_only()
        .column_as(access_log::Column::Id.count(), "count")
        .filter(access_log::Column::FileId.eq(*id))
        .into_model::<CountResult>()
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?
        .unwrap_or(CountResult { count: 0 })
        .count;

    Ok(count < CONFIGURATION.max_download_tries.into())
}

pub async fn store_file(
    database_connection: &DatabaseConnection,
    id: &Uuid,
    hash: &str,
    uploader_ip: &str,
) -> Result<(), Error> {
    let now = Utc::now();

    let download_until = now
        .checked_add_days(CONFIGURATION.default_lifetime)
        .ok_or(Error::DateCalculationFailed)?;

    let file = file::ActiveModel {
        id: Set((*id).into()),
        hash: Set(hash.into()),
        uploader_ip: Set(uploader_ip.into()),
        uploaded_at: Set(now.naive_utc()),
        download_until: Set(download_until.naive_utc()),
    };

    entity::File::insert(file)
        .exec(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?;

    Ok(())
}

pub async fn store_access_log(
    database_connection: &DatabaseConnection,
    ip: &str,
    file_id: &Uuid,
    successful: bool,
) -> Result<(), Error> {
    let log = access_log::ActiveModel {
        id: Set(Uuid::new_v4().into()),
        ip: Set(ip.into()),
        file_id: Set((*file_id).into()),
        date_time: Set(Utc::now().naive_utc()),
        successful: Set(if successful { 1 } else { 0 }),
    };

    entity::AccessLog::insert(log)
        .exec(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)?;

    Ok(())
}
