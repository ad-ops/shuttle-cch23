use axum::{http::StatusCode, routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(|| async { "Hello Worlds!" }))
        .route(
            "/-1/error",
            get(|| async { StatusCode::INTERNAL_SERVER_ERROR }),
        );

    Ok(router.into())
}
