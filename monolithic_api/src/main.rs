use axum::{
    self,
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //Router
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string)) // Extracting string from request body route example
        .route("/mirror_body_json", post(mirror_body_json)) // Extracting JSON from request body route example
        .route("/path_variables/:id", get(path_variables)) // Path Variable route example
        .route("/path_variables/15", get(hard_coded_path)) // Order does not matter, as we have 15 in the path and its handled regardless
        .route("/query_param", get(query_param)) // Extracting Query Parameters from request route example
        .route("/mirror_headers/", get(mirror_headers)); // Extract headers using HeaderMap

    //Listener
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//Handler Functions
async fn hello_world() -> String {
    "Hello, World!!".to_owned()
}

async fn mirror_body_string(body: String) -> String {
    body
}

async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "Hello from Axum".to_owned(),
    })
}

async fn path_variables(Path(id): Path<i32>) -> String {
    let res = format!("Details for ID: {}", id);
    res
}

async fn hard_coded_path() -> String {
    "You hit 15!".to_owned()
}

async fn query_param(Query(query): Query<Data>) -> Json<Data> {
    Json(query)
}

async fn mirror_headers(headers: HeaderMap) -> String {
    println!("{:?}", headers);
    let x = headers.get("User-Agent").unwrap().to_str().unwrap();
    x.to_owned()
}

// Struct for handling JSON data via Serialization and Deserialization
#[derive(Serialize, Deserialize, Debug)]
struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

// Struct for fetching query parameters from the /query_param path
#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
}