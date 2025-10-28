use actix_web::{
    post, web::{Json, Data}, HttpResponse, Responder, HttpRequest
};
use sqlx::{PgPool, Error::RowNotFound};

use crate::models::user::LoginRequest;
use crate::services::auth_services::{hash_password, verify_password, generate_jwt, verify_token, LoginResponse};
use crate::models::user::CreateUser;

#[post("/register")]
pub async fn register_user(db: Data<PgPool>, user_info: Json<CreateUser>) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password_hash.trim();
    let fullname = user_info.fullname.trim();
    let email = user_info.email.trim();
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
        "INSERT INTO users (username, fullname, email, password_hash, role, created_at, resident_id)
        VALUES ($1, $2, $3, $4, $5, NOW(), $6)"
    )
    .bind(username)
    .bind(fullname)
    .bind(email)
    .bind(hashed_password.unwrap())
    .bind(role)
    .bind(user_info.resident_id)
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
    let password = user_info.password_hash.trim();

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

#[post("/logout")]
pub async fn logout_user(req: HttpRequest) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|header_value| header_value.strip_prefix("Bearer "))
        .map(|s| s.to_string());
    // token "Authorization: Bearer <token>"
    match token {
        Some(token) => {
            let secret_key = std::env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set in .env file");

            match verify_token(&token, &secret_key) {
                Ok(claims) => {
                    println!("User '{}' logged out successfully", claims.sub);
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Logout successful"
                    }))
                }
                Err(_) => {
                    HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "Invalid token"
                    }))
                }
            }
        }
        None => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Authorization token is missing"
            }))
        }
    }
}