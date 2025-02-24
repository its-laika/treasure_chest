use crate::configuration::CONFIGURATION;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

/// A struct representing the configuration response.
///
/// This struct is used to serialize the configuration settings into a JSON
/// response. The fields are renamed to match expected JSON keys.
/// This response is returned in the [`handler`] function.
#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "BodyMaxSize")]
    pub body_max_size: usize,
    #[serde(rename = "DaysFileAvailable")]
    pub default_days_lifetime: u64,
}

/// Configuration endpoint.
///
/// This function creates a `Response` struct with the current configuration
/// settings and returns it as a JSON.
pub async fn handler() -> impl IntoResponse {
    let response = Response {
        body_max_size: CONFIGURATION.body_max_size,
        default_days_lifetime: CONFIGURATION.days_file_available,
    };

    Json(response)
}
