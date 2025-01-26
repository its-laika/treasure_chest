use chrono::Days;
use config::{Environment, File, FileFormat};
use serde::Deserialize;
use std::{path::PathBuf, process::exit, sync::LazyLock};

pub const CONFIG_FILE_NAME: &str = "config.json";
pub const CONFIG_ENV_PREFIX: &str = "TREASURE_CHEST";

pub static CONFIGURATION: LazyLock<Configuration> = LazyLock::new(build);

/// Configuration that can be automatically read from Json / env,
/// containing only base types. See [`Configuration`]
#[derive(Deserialize)]
struct RawConfiguration {
    #[serde(rename = "ConnectionString")]
    pub connection_string: String,
    #[serde(rename = "BindTo")]
    pub listening_address: String,
    #[serde(rename = "FilePath")]
    pub file_path: PathBuf,
    #[serde(rename = "DaysFileAvailable")]
    pub default_days_lifetime: u64,
    #[serde(rename = "UserUploadsPerDay")]
    pub user_uploads_per_day: u32,
    #[serde(rename = "MaxDownloadTries")]
    pub max_download_tries: u32,
    #[serde(rename = "IpHeaderName")]
    pub ip_header_name: String,
    #[serde(rename = "BodyMaxSize")]
    pub body_max_size: usize,
}

/// Configuration of program
pub struct Configuration {
    /// Database connection string
    pub connection_string: String,
    /// Address to listen to (e.g. "_localhost:8080_")
    pub listening_address: String,
    /// Path of encrypted files
    pub file_path: PathBuf,
    /// Lifetime of uploaded files until deletion
    pub file_lifetime: Days,
    /// Number of max uploads by a single IP (rate limiting)
    pub ip_uploads_per_day: u32,
    /// Number of max tries to access a file (in case of wrong keys etc)
    pub max_download_tries: u32,
    /// Name of IP header, set by proxy server
    pub ip_header_name: String,
    /// Max size of request body (in bytes)
    pub body_max_size: usize,
}

/// Builds [`Configuration`] by configuration file and env vars
///
/// # Returns
///
/// Instance of [`Configuration`]
///
/// # Note
///
/// If configuration is not buildable, it exits the program.
pub fn build() -> Configuration {
    let Ok(raw) = config::Config::builder()
        .add_source(File::new(CONFIG_FILE_NAME, FileFormat::Json).required(false))
        .add_source(Environment::with_prefix(CONFIG_ENV_PREFIX))
        .build()
        .expect("Configuration is not buildable")
        .try_deserialize::<RawConfiguration>()
    else {
        log::error!("Could not build configuration. Bye.");
        exit(1);
    };

    Configuration {
        connection_string: raw.connection_string,
        listening_address: raw.listening_address,
        file_path: raw.file_path,
        file_lifetime: Days::new(raw.default_days_lifetime),
        max_download_tries: raw.max_download_tries,
        ip_uploads_per_day: raw.user_uploads_per_day,
        ip_header_name: raw.ip_header_name,
        body_max_size: raw.body_max_size,
    }
}
