use std::{path::PathBuf, sync::LazyLock};

use chrono::Days;

pub static CONFIGURATION: LazyLock<Configuration> = LazyLock::new(get_configuration);

pub struct Configuration {
    pub file_path: PathBuf,
    pub file_lifetime: Days,
    pub recent_uploads_timespan: Days,
    pub recent_uploads_maximum: u32,
    pub ip_header_name: String,
    pub body_max_size: usize,
}

pub fn get_configuration() -> Configuration {
    Configuration {
        file_path: PathBuf::from("../.."),
        file_lifetime: Days::new(7),
        recent_uploads_timespan: Days::new(1),
        recent_uploads_maximum: 5,
        ip_header_name: "X-Forwarded-For".into(),
        body_max_size: 10_000_000,
    }
}
