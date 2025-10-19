mod routes;
mod models;

use actix_web::{App, HttpServer};
use routes::auth::login;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server chạy tại http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
