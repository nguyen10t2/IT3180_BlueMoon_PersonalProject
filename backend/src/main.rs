mod routes;
mod models;
mod enums;
mod handlers;
mod db;
mod services;


use actix_web::{web::Data, App, HttpServer};
use actix_cors::Cors;

use routes::*;
use db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file");
    println!("Server chạy tại http://localhost:8080");

    let pool = init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(secret_key.clone()))
            .configure(user_routes::config)
            .configure(auth_routes::config)
            .configure(resident_routes::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
