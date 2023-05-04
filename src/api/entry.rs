use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response {
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    user: String,
    level: u32,
    seconds: u64,
}

pub fn get_entry(id: web::Path<(u32,)>) -> web::Json<Response> {
    web::Json(Response {
        id: id.into_inner().0,
    })
}

pub fn post_entry(req: web::Json<Request>) -> web::Json<Request> {
    req
}
