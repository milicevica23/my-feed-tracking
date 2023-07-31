#![allow(unused)]
use std::vec;

use actix_cors::Cors;
use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder, Result,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};
use mft_domain::*;
use serde::Deserialize;
use sqlx::{pool, postgres::PgPoolOptions, PgPool, Pool};
struct Meal {
    id: i32,
    name: String,
    ingridients: Vec<Ingridient>,
}

#[derive(Deserialize, Debug, sqlx::Type, sqlx::FromRow)]
struct Ingridient {
    ing_name: String,
}

#[get("/ingridient")]
async fn get_ingridient(app_state: web::Data<AppState>) -> impl Responder {
    let ingridients = get_data(&app_state.pool, "ingridient").await;
    HttpResponse::Ok().body(format!("{} , {:?}", "All good", ingridients))
}

async fn get_data(pool: &PgPool, table_name: &str) -> Vec<Ingridient> {
    let result = sqlx::query_as::<_, Ingridient>("SELECT * FROM ingridient")
        .bind(table_name)
        .fetch_all(pool)
        .await
        .expect(&format!("cant fatch {}", table_name));
    result
}

#[post("/ingridient")]
async fn post_ingridient(
    ingridient: web::Json<Ingridient>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("{:?}", &ingridient);
    let result = sqlx::query("insert into ingridient (ing_name) values ($1)")
        .bind(&ingridient.ing_name)
        .execute(&data.pool)
        .await
        .unwrap();
    info!("{:?}", result);
    HttpResponse::Ok().body("Hello worldS")
}

#[get("/meal")]
async fn get_meal() -> impl Responder {
    HttpResponse::Ok().body("Hello worldS")
}

#[post("/meal")]
async fn insert_meal() -> impl Responder {
    HttpResponse::Ok().body("Hello worldS")
}

#[post("/feeling")]
async fn inser_feeling(data: web::Data<AppState>, feeling: web::Json<Feeling>) -> impl Responder {
    let result = sqlx::query("insert into mft_raw.feeling (id,name) values ($1, $2)")
        .bind(&feeling.id)
        .bind(&feeling.name)
        .execute(&data.pool)
        .await
        .unwrap();
    info!("{:?}", result);
    HttpResponse::Ok().body("Hello worldS")
}

#[get("/feeling")]
async fn get_feelings() -> Result<impl Responder> {
    let response = vec![
        Feeling {
            id: 1,
            name: "flying".to_string(),
        },
        Feeling {
            id: 2,
            name: "finger swallow".to_string(),
        },
    ];
    Ok(web::Json(response))
}

struct AppState {
    pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database Url not set");
    println!("{}", &db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Can't connect to db!");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(Logger::default())
            .service(get_ingridient)
            .service(post_ingridient)
            .service(get_feelings)
            .service(get_meal)
            .service(insert_meal)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
