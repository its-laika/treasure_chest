use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(uuid(File::Id).not_null().primary_key())
                    .col(string(File::Hash).not_null().unique_key())
                    .col(string(File::UploaderIp).not_null())
                    .col(date_time(File::UploadedAt).not_null())
                    .col(date_time(File::DownloadUntil).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AccessLog::Table)
                    .if_not_exists()
                    .col(uuid(AccessLog::Id).not_null().primary_key())
                    .col(string(AccessLog::Ip).not_null())
                    .col(uuid(AccessLog::FileId).not_null())
                    .col(date_time(AccessLog::DateTime).not_null())
                    .col(boolean(AccessLog::Successful).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("access_log_file")
                    .from(AccessLog::Table, AccessLog::FileId)
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().name("access_log_file").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AccessLog::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum File {
    Table,
    Id,
    Hash,
    #[sea_orm(iden = "uploaded_at")]
    UploadedAt,
    #[sea_orm(iden = "uploader_ip")]
    UploaderIp,
    #[sea_orm(iden = "download_until")]
    DownloadUntil,
}

#[derive(DeriveIden)]
enum AccessLog {
    Table,
    Id,
    Ip,
    #[sea_orm(iden = "file_id")]
    FileId,
    #[sea_orm(iden = "date_time")]
    DateTime,
    Successful,
}
