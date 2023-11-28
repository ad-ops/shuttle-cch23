mod challenges;

use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .nest("/-1", challenges::_1::routes());

    Ok(router.into())  
}