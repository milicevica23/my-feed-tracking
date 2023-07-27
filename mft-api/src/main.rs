use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


struct Meal{
    id: i32,
    name: String,
    ingridients: Vec<Ingridient>
}
#[derive(Deserialize)]
struct Ingridient {
    name: String
}

#[get("/ingridients")]
async fn get_ingridients(ingridient: web::Json<Ingridient>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}",ingridient.name))
}

#[post("/ingridients")]
async fn post_ingridients() -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_meal)
            .service(insert_meal)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}