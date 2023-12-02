use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use tracing::info;

pub fn routes() -> Router {
    Router::new().route("/*bits", get(handler))
}

async fn handler(Path(path): Path<String>) -> impl IntoResponse {
    info!("pat: {path}");
    path.split('/')
        .filter_map(|segment| segment.parse::<u32>().ok())
        .reduce(|acc, e| acc ^ e)
        .expect("should be valid")
        .pow(3)
        .to_string()
}
