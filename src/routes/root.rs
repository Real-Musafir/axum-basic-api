use axum::response::IntoResponse;

pub async fn root() -> impl IntoResponse {
    "Welcome to the Axum API!"
}
