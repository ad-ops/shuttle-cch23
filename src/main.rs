mod challenges;

use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .nest("/-1", challenges::day0::routes())
        .nest("/1", challenges::day1::routes())
        .nest("/4", challenges::day4::routes())
        .nest("/6", challenges::day6::routes());

    Ok(router.into())
}
