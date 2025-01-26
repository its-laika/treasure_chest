use super::error::{Error, Result};
use crate::configuration::CONFIGURATION;
use chrono::{Days, Utc};
use migration::ExprTrait;
use sea_orm::sea_query::Query;
use sea_orm::{ColumnTrait, Condition, FromQueryResult};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set};
use uuid::Uuid;

/// Wrapper for `COUNT(*)` queries
#[derive(FromQueryResult)]
struct CountResult {
    count: i64,
}

/// Gets file from database for id that can currently be downloaded
///
/// Checks if file has already been downloaded and if it's still in time range.
///
/// # Arguments
///
/// * `database_connection` - [`DatabaseConnection`] to use
/// * `id` - Id of the file entry
///
/// # Returns
///
/// [`Ok`]\([`Some`]\([`entity::file::Model`])): file exists and may be downloaded,  
/// [`Ok`]\([`None`]): file doesn't exist or isn't downloadable anymore  
/// [`Err`]\([`Error::DatabaseOperationFailed`]) on error
pub async fn get_downloadable_file(
    database_connection: &DatabaseConnection,
    id: &Uuid,
) -> Result<Option<entity::file::Model>> {
    entity::File::find()
        .filter(entity::file::Column::Id.eq(*id))
        .filter(entity::file::Column::DownloadUntil.gte(Utc::now()))
        .filter(
            entity::file::Column::Id.not_in_subquery(
                Query::select()
                    .column(entity::access_log::Column::FileId)
                    .from(entity::access_log::Entity)
                    .cond_where(Condition::all().add(entity::access_log::Column::Successful.eq(1)))
                    .to_owned(),
            ),
        )
        .filter(
            entity::file::Column::Id.not_in_subquery(
                Query::select()
                    .column(entity::access_log::Column::FileId)
                    .from(entity::access_log::Entity)
                    .group_by_col(entity::access_log::Column::FileId)
                    .cond_having(
                        Condition::all().add(
                            entity::access_log::Column::FileId
                                .count()
                                .gte(CONFIGURATION.max_download_tries),
                        ),
                    )
                    .to_owned(),
            ),
        )
        .one(database_connection)
        .await
        .map_err(Error::DatabaseOperationFailed)
}

/// Returns whether given `ip` may currently upload a file
///
/// # Arguments
///
/// * `database_connection` - [`DatabaseConnection`] to use
/// * `ip` - Ip to check
///
/// # Returns
///
/// [`Ok`]\(`true`): Client may upload a file  
/// [`Ok`]\(`false`): Client must not upload a file at this time  
/// [`Err`]\([`Error::DatabaseOperationFailed`]) on error
pub async fn is_upload_limit_reached(
    database_connection: &DatabaseConnection,
    ip: &str,
) -> Result<bool> {
    let min_uploaded_at = Utc::now()
        .checked_sub_days(Days::new(1))
        .ok_or(Error::DateCalculationFailed)?;

    let count = entity::File::find()
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

    Ok(count >= CONFIGURATION.ip_uploads_per_day.into())
}

/// Store new file entry to database
///
/// # Arguments
///
/// * `database_connection` - [`DatabaseConnection`] to use
/// * `id` - Id of new file
/// * `hash` - Encryption key hash
/// * `uploader_ip` - Ip of client uploading this file
/// * `encrypted_metadata` - File metadata in encrypted form
///
/// # Returns
///
/// * [`Ok`]\(`()`) - file stored
/// * [`Error`]\([`Error::DateCalculationFailed`]) - Could not calculate `download_until`
/// * [`Error`]\([`Error::DatabaseOperationFailed`]) - Saving to database failed
pub async fn store_file(
    database_connection: &DatabaseConnection,
    id: &Uuid,
    hash: String,
    uploader_ip: String,
    encrypted_metadata: Vec<u8>,
) -> Result<()> {
    let now = Utc::now();

    let download_until = now
        .checked_add_days(CONFIGURATION.file_lifetime)
        .ok_or(Error::DateCalculationFailed)?;

    let file = entity::file::ActiveModel {
        id: Set((*id).into()),
        hash: Set(hash),
        uploader_ip: Set(uploader_ip),
        uploaded_at: Set(now.naive_utc()),
        download_until: Set(download_until.naive_utc()),
        encrypted_metadata: Set(encrypted_metadata),
    };

    entity::File::insert(file)
        .exec(database_connection)
        .await
        .map(|_| ())
        .map_err(Error::DatabaseOperationFailed)
}

/// Store new access log entry to database
///
/// # Arguments
///
/// * `database_connection` - [`DatabaseConnection`] to use
/// * `ip` - Ip of the client accessing the file
/// * `file_id` - Id of the file being accessed
/// * `successful` - Whether validation was successful or not
///
/// # Returns
///
/// * [`Ok`]\(`()`) - access log entry stored
/// * [`Error`]\([`Error::DatabaseOperationFailed`]) - Saving to database failed
pub async fn store_access_log(
    database_connection: &DatabaseConnection,
    ip: &str,
    file_id: &Uuid,
    successful: bool,
) -> Result<()> {
    let log = entity::access_log::ActiveModel {
        id: Set(Uuid::new_v4().into()),
        ip: Set(ip.into()),
        file_id: Set((*file_id).into()),
        date_time: Set(Utc::now().naive_utc()),
        successful: Set(i8::from(successful)),
    };

    entity::AccessLog::insert(log)
        .exec(database_connection)
        .await
        .map(|_| ())
        .map_err(Error::DatabaseOperationFailed)
}
