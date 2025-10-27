use actix_web::{
    post, web::{Json, Data}, HttpResponse, Responder
};
use sqlx::PgPool;

use crate::{models::user::LoginRequest, services::auth_services::hash_password};
use crate::models::user::User;

#[post("/register")]
pub async fn register_user(db: Data<PgPool>, user_info: Json<User>) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password.trim();
    let fullname = user_info.fullname.trim();
    let email = user_info.email.as_ref().map(|s| s.trim());
    let role = &user_info.role;

    if username.is_empty() || password.is_empty() || fullname.is_empty() {
        return HttpResponse::BadRequest().body("Username, password, and fullname cannot be empty");
    }

    let existing_user = sqlx::query_scalar::<_, bool>
    ("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
    .bind(username)
    .fetch_one(db.get_ref())
    .await;

    match existing_user {
        Ok(true) => {
            return HttpResponse::BadRequest().body("Username already exists");
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Database error");
        }
        _ => {}
    }

    let hashed_password = hash_password(password.to_string());

    let insert_result = sqlx::query(
        "INSERT INTO users (username, fullname, email, password_hash, role, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(username)
    .bind(fullname)
    .bind(email)
    .bind(&hashed_password)
    .bind(role)
    .bind(chrono::Utc::now().naive_utc())
    .execute(db.get_ref())
    .await;

    println!("Insert result: {:?}", insert_result);

    match insert_result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}

#[post("/login")]
pub async fn login_user(db:  Data<PgPool>, user_info: Json<LoginRequest>) -> impl Responder  {
    let username = user_info.username.trim();
    let password = user_info.password.trim();

    let get_password_result = sqlx::query_scalar::<_, String>(
        "SELECT password_hash FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_one(db.get_ref())
    .await;

    match get_password_result {
        Ok(stored_hashed_password) => {
            match crate::services::auth_services::verify_password(stored_hashed_password, password.to_string()) {
                Ok(_) => {
                    println!("User login successful");
                    return HttpResponse::Ok().body("Login successful");
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid username or password")
            }
        }
        Err(_) => {
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
    }
}