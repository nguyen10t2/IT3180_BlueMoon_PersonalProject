use actix_web::{
    post, web::{self, Data}, HttpResponse, Responder
};

use sqlx::PgPool;

use crate::services::auth_services::{hash_password};
use crate::models::user::CreateUser;

#[post("/register")]
pub async fn register_user(db: Data<PgPool>, user_info: web::Json<CreateUser>) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password.trim();
    let role = &user_info.role;

    if username.is_empty() || password.is_empty() {
        return HttpResponse::BadRequest().body("Username and password cannot be empty");
    }

    let existing_user = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_one(db.get_ref())
    .await;

    match existing_user {
        Ok(count) if count > 0 => {
            return HttpResponse::BadRequest().body("Username already exists");
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {}", err));
        }
        _ => {}
    }

    let hashed_password = hash_password(password.to_string());

    let insert_result = sqlx::query(
        "INSERT INTO create_users (username, password_hash, role) VALUES ($1, $2, $3)"
    )
    .bind(username)
    .bind(&hashed_password)
    .bind(role)
    .execute(db.get_ref())
    .await;
    match insert_result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}