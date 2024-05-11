use crate::errors::{Result, ServerError};
use actix_web::{get, web::Path, HttpResponse};

#[get("/app.css")]
async fn styles() -> HttpResponse {
    let content = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(content)
}
