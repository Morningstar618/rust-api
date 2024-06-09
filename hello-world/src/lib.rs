mod routes;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run() {
    let app = Router::new().route("/", get(hello_world));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    "Hello, World!!".to_owned()
}
