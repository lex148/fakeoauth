use crate::errors::Result;
use crate::helpers::render;
use actix_web::{get, web::Query, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    // where to return the user to
    redirect_uri: String,
}

#[get("/")]
async fn index(qs: Query<Params>) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(&qs.redirect_uri);
    render::<View, _>(args).await
}
