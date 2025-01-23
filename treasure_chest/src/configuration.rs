use chrono::Days;
use config::{Environment, File, FileFormat};
use log::error;
use serde::Deserialize;
use std::{path::PathBuf, process::exit, sync::LazyLock};

pub const CONFIG_FILE_NAME: &str = "config.json";
pub const CONFIG_ENV_PREFIX: &str = "TREASURE_CHEST";

pub static CONFIGURATION: LazyLock<Configuration> = LazyLock::new(get_configuration);

#[derive(Deserialize)]
struct RawConfiguration {
    #[serde(rename = "ConnectionString")]
    pub connection_string: String,
    #[serde(rename = "BindTo")]
    pub listening_address: String,
    #[serde(rename = "FilePath")]
    pub file_path: PathBuf,
    #[serde(rename = "FileLifetime")]
    pub file_lifetime: u64,
    #[serde(rename = "RecentUploadsTimespan")]
    pub recent_uploads_timespan: u64,
    #[serde(rename = "RecentUploadsMaximum")]
    pub recent_uploads_maximum: u32,
    #[serde(rename = "IPHeaderName")]
    pub ip_header_name: String,
    #[serde(rename = "BodyMaxSize")]
    pub body_max_size: usize,
}

pub struct Configuration {
    pub connection_string: String,
    pub listening_address: String,
    pub file_path: PathBuf,
    pub file_lifetime: Days,
    pub recent_uploads_timespan: Days,
    pub recent_uploads_maximum: u32,
    pub ip_header_name: String,
    pub body_max_size: usize,
}

pub fn get_configuration() -> Configuration {
    let Ok(raw) = config::Config::builder()
        .add_source(File::new(CONFIG_FILE_NAME, FileFormat::Json).required(false))
        .add_source(Environment::with_prefix(CONFIG_ENV_PREFIX))
        .build()
        .expect("Configuration is not buildable")
        .try_deserialize::<RawConfiguration>()
    else {
        error!("Could not build configuration. Bye.");
        exit(1);
    };

    Configuration {
        connection_string: raw.connection_string,
        listening_address: raw.listening_address,
        file_path: raw.file_path,
        file_lifetime: Days::new(raw.file_lifetime),
        recent_uploads_timespan: Days::new(raw.recent_uploads_timespan),
        recent_uploads_maximum: raw.recent_uploads_maximum,
        ip_header_name: raw.ip_header_name,
        body_max_size: raw.body_max_size,
    }
}
