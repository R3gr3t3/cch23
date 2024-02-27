use axum::{routing::post, Router};

use crate::handlers::day5_handlers;

pub fn routes() -> Router {
    Router::new()
        // .route("/5", post(day5_handlers::task1_handler))
        .route("/5", post(day5_handlers::task2_handler))
}