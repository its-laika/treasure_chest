use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbErr};
use std::time::Duration;

mod api;
mod configuration;
mod db;
mod error;
mod file;
mod request;

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    env_logger::init();
    let mut opt = ConnectOptions::new("mysql://root:example@localhost/trsure_chest");
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;

    api::init(&ADDRESS, db.clone()).await.unwrap();

    db.close().await?;
    Ok(())
}
