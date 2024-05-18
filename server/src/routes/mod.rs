mod hello;

use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello::hello()))
}