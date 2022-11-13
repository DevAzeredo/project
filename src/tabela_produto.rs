use crate::AppState;
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{self, Error, FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Pessoas {
    id: i64,
    name: String,
}

pub async fn get_all_produtos(pool: &Pool<Postgres>) -> Vec<Pessoas> {
    let tst = sqlx::query_as::<_, Pessoas>("SELECT id, name FROM pessoa")
        .fetch_all(pool)
        .await.unwrap();
        log::info!(" pegar todas as pessoas e retornar em um vetor: {:?}", tst);
        for (e, pessoa) in tst.iter().enumerate() {
            println!("{}: id:{:?}, nome {:?}", e, pessoa.id, pessoa.name);
        }
        tst
}
