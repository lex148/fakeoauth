use crate::errors::Result;
use crate::helpers::render;
use actix_web::{get, web::Query, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    // where to return the user to
    redirect_uri: String,
}

// common authorize endpoint
#[get("/")]
async fn index(qs: Query<Params>) -> Result<HttpResponse> {
    authorize(qs).await
}

// other common authorize endpoint
#[get("/oauth/authorize")]
async fn index2(qs: Query<Params>) -> Result<HttpResponse> {
    authorize(qs).await
}

/// page loaded when a user wants to authorize
async fn authorize(qs: Query<Params>) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(&qs.redirect_uri);
    render::<View, _>(args).await
}
