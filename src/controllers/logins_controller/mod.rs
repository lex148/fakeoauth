use crate::errors::Result;
use crate::helpers::{redirect, render};
use actix_web::{get, post, web::Form, web::Path, HttpResponse};

#[get("/logins/{name}")]
pub(crate) async fn index(name: Path<String>) -> Result<HttpResponse> {
    use crate::views::logins::index::{View, ViewArgs};
    let args = ViewArgs::new();
    render::<View, _>(args).await
}
