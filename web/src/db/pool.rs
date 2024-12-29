use super::model::FileInfo;
use sqlx::{MySql, Pool};

pub trait Db {
    async fn store_file_info(&self, file_info: &FileInfo) -> Result<(), sqlx::Error>;
    async fn load_file_info(&self, name: &str) -> Result<Option<FileInfo>, sqlx::Error>;
}

impl Db for Pool<MySql> {
    async fn store_file_info(&self, file_info: &FileInfo) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            REPLACE INTO FileInfo
            SET
                name = ?,
                hash = ?,
                uploaded_at = ?,
                uploaded_by = ?,
                download_until = ?,
                downloaded_at = ?,
                downloaded_by = ?
            ",
        )
        .bind(file_info.name.clone())
        .bind(file_info.hash.clone())
        .bind(file_info.uploaded_at)
        .bind(file_info.uploaded_by.clone())
        .bind(file_info.download_until)
        .bind(file_info.downloaded_at)
        .bind(file_info.downloaded_by.clone())
        .execute(self)
        .await
        .map(|_| ())
    }

    async fn load_file_info(&self, name: &str) -> Result<Option<FileInfo>, sqlx::Error> {
        sqlx::query_as::<_, FileInfo>(
            "
                SELECT *
                FROM FileInfo
                WHERE name = ?
            ",
        )
        .bind(name)
        .fetch_optional(self)
        .await
    }
}
