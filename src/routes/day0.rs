use axum::{routing::get, Router};

use crate::handlers::day0_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(day0_handlers::task1_handler))
        .route("/-1/error", get(day0_handlers::task2_handler))
}