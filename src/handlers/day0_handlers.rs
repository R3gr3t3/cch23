use axum::http::StatusCode;

pub async fn task1_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello World")
}

pub async fn task2_handler() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}