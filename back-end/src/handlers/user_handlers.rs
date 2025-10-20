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

#[get("/{id}")]
pub async fn get_user_by_id(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, User>
    ("SELECT * FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(db.get_ref())
        .await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}
