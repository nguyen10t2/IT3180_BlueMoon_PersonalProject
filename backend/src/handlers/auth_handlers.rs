use actix_web::{
    post, web::{Json, Data}, HttpResponse, Responder
};
use sqlx::{PgPool, Error::RowNotFound};

use crate::models::user::LoginRequest;
use crate::services::auth_services::{hash_password, verify_password, generate_jwt, LoginResponse};
use crate::models::user::User;

#[post("/register")]
pub async fn register_user(db: Data<PgPool>, user_info: Json<User>) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password.trim();
    let fullname = user_info.fullname.trim();
    let email = user_info.email.as_ref().map(|s| s.trim());
    let role = &user_info.role;

    if username.is_empty() || password.is_empty() || fullname.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Username, password, and fullname cannot be empty"
        }));
    }

    let existing_user = sqlx::query_scalar::<_, bool>
        ("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
        .bind(username)
        .fetch_one(db.get_ref())
        .await;

    match existing_user {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Username already exists"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }));
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

    match insert_result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "message": "User registered successfully"
        })),
        Err(err) => {
            eprintln!("Database error during registration: {}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to register user"
            }))
        }
    }
}

#[post("/login")]
pub async fn login_user(db: Data<PgPool>, user_info: Json<LoginRequest>) -> impl Responder {
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

            if verify_password(stored_hashed_password, password.to_string()).is_err() {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid username or password"
                }));
            }

            let secret_key = std::env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set in .env file");

            match generate_jwt(username.to_string(), &secret_key, 86400) {
                Ok(token) => {
                    println!("User '{}' logged in successfully", username);
                    HttpResponse::Ok().json(LoginResponse {
                        token,
                        message: "Login successful".to_string(),
                    })
                }
                Err(err) => {
                    eprintln!("Token generation error: {}", err);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to generate authentication token"
                    }))
                }
            }
        }
        Err(RowNotFound) => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid username or password"
            }))
        }
        Err(err) => {
            eprintln!("Database error during login: {}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}