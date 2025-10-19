mod routes;
mod models;
mod enums;
mod handlers;
mod db;


use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use routes::user_routes;
use db::init_db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server chạy tại http://127.0.0.1:8080");

    let pool = init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
