use galactic_gambit_leaderboard::*;
use galactic_gambit_leaderboard::{models::Entries, schema::entries::dsl::*};

use diesel::prelude::*;

use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    id: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/entries", get(all_entries))
        .route("/last_entries", get(last_entries))
        .route("/entries/:path_level_id", get(get_level_entries))
        .route("/entries", post(post_entry))
        .route("/foo/bar", get(foo_bar));

    // Get port from env
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Functions to handle each of the endpoints
async fn index() -> String {
    String::from("Hello World!")
}

async fn all_entries() -> Json<Vec<Entries>> {
    let connection = &mut establish_connection();
    let results = entries
        .order_by(seconds.asc())
        .limit(10)
        .load::<Entries>(connection)
        .expect("Error loading posts");

    Json(results)
}

async fn last_entries() -> Json<Vec<Entries>> {
    // return the most recent entries

    let connection = &mut establish_connection();
    let mut results = entries
        .load::<Entries>(connection)
        .expect("Error loading posts");

    results.reverse();
    results.truncate(10);

    Json(results)
}

async fn get_level_entries(Path(path_level_id): Path<String>) -> Json<Vec<Entries>> {
    let connection = &mut establish_connection();
    let results = entries
        .filter(level_id.eq(path_level_id))
        .order_by(seconds.asc())
        .limit(10)
        .load::<Entries>(connection)
        .expect("Error loading posts");

    Json(results)
}

#[derive(Serialize, Deserialize)]
struct TempEntry {
    level_id: String,
    username: String,
    seconds: f64,
}

async fn post_entry(Json(payload): Json<serde_json::Value>) -> Json<Entries> {
    let connection = &mut establish_connection();

    let deserialized_payload: TempEntry = serde_json::from_value(payload).unwrap();

    let l_id: String = deserialized_payload.level_id;
    let u_name: String = deserialized_payload.username;
    let scnds: f64 = deserialized_payload.seconds;

    let created_entry: Entries = create_entry(connection, &l_id, &u_name, &scnds);
    Json(created_entry)
}

async fn foo_bar() -> String {
    String::from("Foo Bar")
}
