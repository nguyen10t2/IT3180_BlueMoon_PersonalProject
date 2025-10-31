use actix_web::{
    get, put, post, web::{Json, Data}, HttpResponse, Responder, HttpRequest
};
use serde_json::json;
use sqlx::PgPool;
use serde::Deserialize;

use crate::models::user::LoginRequest;
use crate::services::auth_services::{hash_password, verify_password, generate_jwt, verify_token};
use crate::models::user::CreateUser;
use crate::services::auth_services::{AuthData, create_refresh_token, refresh_access_token};

#[post("/register")]
pub async fn register_user(
    db: Data<PgPool>,
    user_info: Json<CreateUser>
) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password.trim();
    let fullname = user_info.fullname.trim();
    let email = user_info.email.trim();
    let role = &user_info.role;

    if username.is_empty() || password.is_empty() || fullname.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Username, password, and fullname cannot be empty"
        }));
    }

    let existing_user = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
    )
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

    let hashed_password = match hash_password(password.to_string()) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing error: {}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };
    let insert_result = sqlx::query(
        "INSERT INTO users (username, fullname, email, password_hash, role, created_at, resident_id)
        VALUES ($1, $2, $3, $4, $5, NOW(), $6)"
    )
        .bind(username)
        .bind(fullname)
        .bind(email)
        .bind(hashed_password)
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
pub async fn login_user(
    db: Data<PgPool>,
    secret_key: Data<String>,
    refresh_secret_key: Data<String>,
    user_info: Json<LoginRequest>
) -> impl Responder {
    let username = user_info.username.trim();
    let password = user_info.password.trim();

    let get_password_result = sqlx::query_as::<_, AuthData>(
        "SELECT user_id, password_hash FROM users WHERE username = $1 AND status = 'active'"
    )
        .bind(username)
        .fetch_optional(db.get_ref())
        .await;

    match get_password_result {
        Ok(None) => {
            HttpResponse::NotFound().json(json!({"error": "User not found"}))
        }
        Ok(Some(auth_data)) => {
            if verify_password(auth_data.password_hash, password.to_string()).is_err() {
                return HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid username or password"
                }));
            }
            let access_token = match generate_jwt(auth_data.user_id, secret_key.get_ref(), 86400) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("Token generation error: {}", err);
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to generate access token"
                    }));
                }
            };
            let new_refresh_token = match create_refresh_token(
                db.clone(),
                auth_data.user_id,
                refresh_secret_key.get_ref(),
                7 * 86400,
            )
            .await
            {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("Failed to create refresh token: {}", err);
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to create refresh token"
                    }));
                }
            };

            println!("User '{}' logged in successfully", username);
            HttpResponse::Ok().json(json!({
                "access_token": access_token,
                "refresh_token": new_refresh_token,
                "message": "Login successful"
            }))
        }
        Err(err) => {
            eprintln!("Database error during login: {}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Database error"
            }))
        }
    }
}


#[post("/logout")]
pub async fn logout_user(
    req: HttpRequest,
    secret_key: Data<String>
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|header_value| header_value.strip_prefix("Bearer "))
        .map(|s| s.to_string());
    
    match token {
        Some(token) => {
            match verify_token(&token, secret_key.get_ref()) {
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

#[get("/me")]
pub async fn get_current_user(
    req: HttpRequest,
    secret_key: Data<String>
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|header_value| header_value.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    match token {
        Some(token) => {
            match verify_token(&token, secret_key.get_ref()) {
                Ok(claims) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "username": claims.sub,
                        "exp": claims.exp,
                        "issued_at": claims.iat,
                    }))
                }
                Err(_) => {
                    HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "Invalid token"
                    }))
                }
            }
        }
        None => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Authorization token is missing"
        })),
    }
}

#[put("/change-password")]
pub async fn change_password(
    db: Data<PgPool>, 
    req: HttpRequest,
    secret_key: Data<String>,
    password_info: Json<serde_json::Value>
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|header_value| header_value.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    let new_password = password_info.get("new_password")
        .and_then(|v| v.as_str());
    if new_password.is_none() || new_password.unwrap().trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "New password cannot be empty"
        }));
    }
    match token {
        Some(token) => {
            match verify_token(&token, secret_key.get_ref()) {
                Ok(claims) => {
                    let hashed_password = match hash_password(new_password.unwrap().to_string()) {
                        Ok(hash) => Ok(hash),
                        Err(err) => Err(err),
                    };
                    match hashed_password {
                        Ok(hashed) => {
                            let update = sqlx::query(
                                "UPDATE users SET password_hash = $1 WHERE username = $2"
                            )
                                .bind(hashed)
                                .bind(claims.sub)
                                .execute(db.get_ref())
                                .await;
                            match update {
                                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                                    "message": "Password changed successfully"
                                })),
                                Err(err) => {
                                    eprintln!("Database error during password change: {}", err);
                                    HttpResponse::InternalServerError().json(serde_json::json!({
                                        "error": "Failed to change password"
                                    }))
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Password hashing error: {}", err);
                            HttpResponse::InternalServerError().json(serde_json::json!({
                                "error": "Failed to hash new password"
                            }))
                        }
                    }
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

#[post("/refresh")]
pub async fn refresh_token(
    db: Data<PgPool>,
    refresh_secret_key: Data<String>,
    body: Json<RefreshRequest>,
) -> impl Responder {
    match refresh_access_token(
        db,
        &body.refresh_token,
        refresh_secret_key.get_ref(),
        86400, // Access token mới có hạn 1 ngày
    )
    .await
    {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::Unauthorized().json(json!({
            "error": err.to_string()
        })),
    }
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}
