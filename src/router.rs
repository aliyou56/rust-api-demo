use axum::{routing::*, Router};
use crate::handlers;

pub fn new() -> Router {
    Router::new()
        .route("/hello", get(handlers::hello_world))
}
