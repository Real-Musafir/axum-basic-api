use crate::models::greet_model::{GreetRequest, GreetResponse};
use axum::Json;

pub async fn greet(Json(payload): Json<GreetRequest>) -> Json<GreetResponse> {
    let message = format!("Hello, {}!", payload.name);
    Json(GreetResponse { message })
}
