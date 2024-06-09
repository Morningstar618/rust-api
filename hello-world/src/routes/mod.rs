use axum::{body::Body, routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> String {
    "Hello, World!!".to_owned()
}
