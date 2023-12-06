use axum::{
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct ElfResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf(payload: String) -> impl IntoResponse {
    let elfs = payload.matches("elf").count();
    let elfs_on_shelves = payload.matches("elf on a shelf").count();
    let shelfs = payload.matches("shelf").count();
    Json(ElfResponse {
        elf: elfs,
        elf_on_a_shelf: elfs_on_shelves,
        shelf_with_no_elf_on_it: shelfs - elfs_on_shelves,
    })
}

pub fn routes() -> Router {
    Router::new().route("/", post(elf))
}
