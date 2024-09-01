use crate::handlers::{greet_handler::greet, root_handler::root};
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/greet", axum::routing::post(greet))
}
