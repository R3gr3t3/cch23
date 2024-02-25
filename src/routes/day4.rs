use axum::{routing::post, Router};

use crate::handlers::day4_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/4/strength", post(day4_handlers::task1_handler))
        .route("/4/contest", post(day4_handlers::task2_handler))
}