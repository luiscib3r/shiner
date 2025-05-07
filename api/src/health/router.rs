use axum::{Router, routing::get};

use super::handlers::health_check;

pub fn new() -> Router {
    Router::new().route("/health", get(health_check))
}
