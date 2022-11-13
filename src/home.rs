use actix_web::{error, get, Responder};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;
#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home {}
#[get("/")]
async fn home() -> actix_web::Result<impl Responder> {
    let body = Home {}
        .render_once()
        .map_err(error::ErrorInternalServerError)?;
    Ok(Html(body))
}
