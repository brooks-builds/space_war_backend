use axum::{http::StatusCode, response::IntoResponse};

pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
