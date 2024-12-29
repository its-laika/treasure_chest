use axum::http::HeaderMap;

const HEADER_X_FORWARDED_FOR: &str = "X-Forwarded-For";

pub fn get_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(header_value) = headers
        .get(HEADER_X_FORWARDED_FOR)
        .and_then(|v| v.to_str().ok())
    {
        if let Some(forwarded_ip) = header_value.trim().split(',').next() {
            return Some(forwarded_ip.into());
        }
    }

    Some("127.0.0.1".into()) // TODO: Remove
}
