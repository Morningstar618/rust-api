use axum::{
    self,
    extract::{Path, Query, Request, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::{from_fn, Next},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    // Instantiating SharedData struct
    let shared_data = SharedData {
        message: "Hello, from shared data middleware".to_owned(),
    };

    //Router
    let app = Router::new()
        .route(
            // Route that will receive the custom header set by the `set_custom_middleware_header`
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .layer(from_fn(set_custom_middleware_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string)) // Extracting string from request body route example
        .route("/mirror_body_json", post(mirror_body_json)) // Extracting JSON from request body route example
        .route("/path_variables/:id", get(path_variables)) // Path Variable route example
        .route("/path_variables/15", get(hard_coded_path)) // Order does not matter, as we have 15 in the path and its handled regardless
        .route("/query_param", get(query_param)) // Extracting Query Parameters from request route example
        .route("/mirror_headers/", get(mirror_headers)) // Extract headers using HeaderMap
        .route("/middleware_message", get(middleware_message))
        .layer(cors) // Adds the CORS middleware. It should always be added in the end as doing so effects every route above it from it
        .layer(Extension(shared_data)); // Allows sharing of common data with the routes above this layer

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

async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

// Function setting custom middleware
async fn set_custom_middleware_header(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned(); // Why did adding to_owned() here fix the error ? You can see the error by removing the to_owned method

    let extensions = req.extensions_mut();

    extensions.insert(HeaderMessage(message.to_owned()));

    let response = next.run(req).await;
    Ok(response)
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

// Struct for sharing data across routes via middleware
#[derive(Clone)]
struct SharedData {
    message: String,
}

// Struct for HeaderMessage custom middleware
#[derive(Clone)]
struct HeaderMessage(String);
