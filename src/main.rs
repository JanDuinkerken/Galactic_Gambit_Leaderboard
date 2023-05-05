use galactic_gambit_leaderboard::schema::entries::dsl::*;
use galactic_gambit_leaderboard::*;

use diesel::prelude::*;

use axum::{
    extract::{Json, Path},
    routing::get,
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
        .route("/foo/:user_id", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Functions to handle each of the endpoints
async fn index() -> String {
    String::from("Hello World!")
}

async fn all_entries() -> Json<Vec<galactic_gambit_leaderboard::models::Entries>> {
    let mut entry_array: Vec<galactic_gambit_leaderboard::models::Entries> = Vec::new();

    let connection = &mut establish_connection();
    let results = entries
        .order_by(seconds.asc())
        .limit(10)
        .load::<galactic_gambit_leaderboard::models::Entries>(connection)
        .expect("Error loading posts");

    for entry in results {
        entry_array.push(entry);
    }

    Json(entry_array)
}

async fn get_foo(Path(user_id): Path<u32>) -> Json<Response> {
    Json(Response { id: user_id })
}

async fn post_foo(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(payload)
}

async fn foo_bar() -> String {
    String::from("Foo Bar")
}
