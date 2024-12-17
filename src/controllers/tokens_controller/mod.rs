use crate::errors::Result;
use actix_web::{post, web::Form, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    code: String,
}

#[post("/token")]
pub(crate) async fn index(form: Form<Params>) -> Result<HttpResponse> {
    token_exchange(form).await
}

#[post("/oauth/token")]
pub(crate) async fn index2(form: Form<Params>) -> Result<HttpResponse> {
    token_exchange(form).await
}

async fn token_exchange(form: Form<Params>) -> Result<HttpResponse> {
    let code = form.code.to_string();
    let json = format!(
        r##"
{{
  "access_token": "{code}",
  "token_type": "Bearer",
  "expires_in": 3600,
  "refresh_token": "{code}",
  "scope": "read write"
}}
"##
    );

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
