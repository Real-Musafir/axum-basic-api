use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build the application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/greet", post(greet));

    // Run the server on a specified address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define a handler for the root route
async fn root() -> &'static str {
    println!("Welcome to first api");
    "Welcome to the Axum API!"
}

// Define the request and response structures for the /greet route
#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

// Define a handler for the /greet route
async fn greet(Json(payload): Json<GreetRequest>) -> Json<GreetResponse> {
    let message = format!("Hello, {}!", payload.name);
    Json(GreetResponse { message })
}
