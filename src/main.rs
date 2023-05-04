use actix_web;
use repositories::leaderboard::{Repository, InMemoryRepository};

mod domain;
mod repositories;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = InMemoryRepository::new();
    api::serve("localhost", 8080, &repo).await
}