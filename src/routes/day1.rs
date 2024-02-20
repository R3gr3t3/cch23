use axum::{routing::get, Router};

use crate::handlers::day1_handlers;

pub fn routes() -> Router {
    Router::new()
        // .route("/1/:num1/:num2", get(day1_handlers::task1_handler))
        .route("/1/*nums", get(day1_handlers::task2_handler))
}