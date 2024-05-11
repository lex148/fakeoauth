use crate::errors::{Result, ServerError::InvalidData};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/userinfo")]
pub(crate) async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let token = req.headers().get("authorization").ok_or(InvalidData)?;
    let token = token.to_str().or(Err(InvalidData))?;
    let name = get_substr_after_keyword(token, "Bearer ").ok_or(InvalidData)?;

    let json = format!(
        r##"{{
  "sub": "{name}"
}}"##
    );
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

fn get_substr_after_keyword<'a>(input: &'a str, keyword: &str) -> Option<&'a str> {
    let start = input.find(keyword)?;
    let tail = &input[(start + keyword.len())..];
    Some(tail)
}
