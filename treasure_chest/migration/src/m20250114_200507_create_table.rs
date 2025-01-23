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
                    .col(ColumnDef::new(File::Id).uuid().not_null().primary_key())
                    .col(string(File::Hash).not_null().unique_key())
                    .col(string(File::DownloaderIp).null())
                    .col(string(File::UploaderIp).not_null())
                    .col(date_time(File::UploadedAt).not_null())
                    .col(date_time(File::DownloadUntil).not_null())
                    .col(date_time(File::DownloadedAt).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
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
    #[sea_orm(iden = "downloaded_at")]
    DownloadedAt,
    #[sea_orm(iden = "downloader_ip")]
    DownloaderIp,
}
