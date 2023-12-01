use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/:num1/:num2", get(handler))
}

async fn handler(Path((num1, num2)): Path<(u32, u32)>) -> impl IntoResponse {
    (num1 ^ num2).pow(3).to_string()
}
