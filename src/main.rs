#![allow(unused)]
use actix_web::{
    middleware::{Compress, Logger},
    web,
    web::Data,
    App, HttpServer,
};
mod services;
use actix_web_lab::__reexports::futures_util::TryFutureExt;
use dotenv::dotenv;
use sailfish::TemplateOnce;
use services::{create_user_article, fetch_user_articles, fetch_users};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres, Row};

mod home;
mod page;
mod tabela_produto;
use tabela_produto::*;

#[derive(TemplateOnce)]
#[template(path = "actix.stpl")]
struct Greet<'a> {
    name: &'a str,
}

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");
        tabela_produto::get_all_produtos(&pool).await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
         //   .service(tabela_produto::get_all_produtos)
            .service(page::pagina)
            .service(home::home)
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
