use actix_web::{
    error, get,
    web, Responder,
};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "page.stpl")]
struct Page<'a> {
    id: &'a i32,
}
#[get("/page-{id:\\d+}")]
async fn pagina(params: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let body = Page { id: &params.0 }
        .render_once()
        .map_err(error::ErrorInternalServerError)?;

    Ok(Html(body))
}