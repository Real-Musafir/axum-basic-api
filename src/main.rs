use axum::Router;
use axum_simple_api::config::AppConfig;
use axum_simple_api::routes::create_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = AppConfig::init();

    // Create the Axum router
    let app = create_routes();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
