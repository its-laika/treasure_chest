use chrono::Days;
use std::{path::PathBuf, sync::LazyLock};

pub static CONFIGURATION: LazyLock<Configuration> = LazyLock::new(get_configuration);

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
    Configuration {
        connection_string: "mysql://root:example@localhost/trsure_chest".into(),
        listening_address: "localhost:8081".into(),
        file_path: PathBuf::from("../../files"),
        file_lifetime: Days::new(7),
        recent_uploads_timespan: Days::new(1),
        recent_uploads_maximum: 5,
        ip_header_name: "X-Forwarded-For".into(),
        body_max_size: 10_000_000,
    }
}
