use actix_web::{get, post, App, HttpServer, Responder, web};

use crate::repositories::leaderboard::{Repository, InMemoryRepository};

mod entry;

pub async fn serve (url: &str, port: u16, repo: &mut dyn Repository) -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(get_entry)
            .service(post_entry)
    })
    .bind((url, port))?
    .run()
    .await
}

#[get("/entry/{id}")]
async fn get_entry(id: web::Path<(u32,)>) -> impl Responder {
    entry::get_entry(id)
}

#[post("/entry")]
async fn post_entry(req: web::Json<entry::Request>) -> impl Responder {
    entry::post_entry(req)
}