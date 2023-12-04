use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PartialRaindeer {
    #[allow(unused)]
    name: String,
    strength: u32,
}

#[derive(Deserialize)]
struct Raindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candy_eaten_yesterday: u32,
}

#[derive(Serialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn strength(Json(payload): Json<Vec<PartialRaindeer>>) -> impl IntoResponse {
    payload
        .into_iter()
        .map(|raindeer| raindeer.strength)
        .sum::<u32>()
        .to_string()
}

async fn contest(Json(payload): Json<Vec<Raindeer>>) -> impl IntoResponse {
    let fastest = payload
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();
    let tallest = payload
        .iter()
        .max_by_key(|raindeer| raindeer.height)
        .unwrap();
    let magician = payload
        .iter()
        .max_by_key(|raindeer| raindeer.snow_magic_power)
        .unwrap();
    let consumer = payload
        .iter()
        .max_by_key(|raindeer| raindeer.candy_eaten_yesterday)
        .unwrap();

    let contest_result = ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    };
    Json(contest_result)
}

pub fn routes() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}
