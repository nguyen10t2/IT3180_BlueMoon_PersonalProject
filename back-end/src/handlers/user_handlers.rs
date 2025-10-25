use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

use sqlx::PgPool;

use crate::models::user::User;
use serde_json::json;

#[get("")]
pub async fn get_all_users(db: Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>
    ("SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}