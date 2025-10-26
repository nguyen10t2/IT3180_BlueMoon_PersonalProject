mod routes;
mod models;
mod enums;
mod handlers;
mod db;
mod services;


use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use routes::user_routes;
use db::init_db;

use crate::routes::auth_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server chạy tại http://127.0.0.1:8080");

    let pool = init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes::config)
            .configure(auth_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
