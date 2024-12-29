use sqlx::mysql::MySqlPoolOptions;
use tokio::spawn;

mod api;
mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    let address = "127.0.0.1:8080";

    let x = spawn(async move {
        api::init(&address, pool).await.expect("API server failed");
    });

    x.await.unwrap();

    Ok(())
}
