use axum::{http::StatusCode, routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/error", get(|| async { StatusCode::INTERNAL_SERVER_ERROR }))
}
