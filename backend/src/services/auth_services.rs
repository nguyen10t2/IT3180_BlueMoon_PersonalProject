use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString, Error as PasswordHashError}, 
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};

use jsonwebtoken::{decode, EncodingKey, Header, encode, errors::Error as JwtError, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool};

use chrono::NaiveDateTime;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        
    pub exp: i64,           
    pub iat: i64,           
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
}

#[derive(sqlx::FromRow)]
pub struct AuthData {
    pub user_id: i32,
    pub password_hash: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct RefreshTokenModel {
    id: i32,
    token: String,
    user_id: i32,
    expires_at: NaiveDateTime,
    revoked_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

pub fn hash_password(password: String) -> Result<String, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).map(|hash| hash.to_string())
}

pub fn verify_password(hashed_password: String, password: String) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let hashed_password = PasswordHash::new(&hashed_password).unwrap();
    argon2.verify_password(password.as_bytes(), &hashed_password)
}

pub fn generate_jwt(user_id: i32, secret: &str, expiration: i64) -> Result<String, JwtError> {
    let current_time = chrono::Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        iat: current_time.timestamp(),
        exp: (current_time + chrono::Duration::seconds(expiration)).timestamp(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(
        token,
        &DecodingKey::from_base64_secret(secret)?,
        &validation,
    ).map(|data| data.claims)
}

pub async fn create_refresh_token(
    db: Data<PgPool>,
    user_id: i32,
    secret: &str,
    exp: i64,
) -> Result<String, Box<dyn std::error::Error>> {
    let refresh_token_string = generate_jwt(user_id, secret, exp)?;
    let expires_at = (chrono::Utc::now() + chrono::Duration::seconds(exp)).naive_utc();

    sqlx::query!(
        "INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES ($1, $2, $3)",
        refresh_token_string,
        user_id,
        expires_at
    )
        .execute(db.get_ref())
        .await?;  
    Ok(refresh_token_string)
}

pub async fn refresh_access_token(
    db: Data<PgPool>,
    refresh_token: &str,
    jwt_secret: &str,
    access_token_expiration: i64,
) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let token_record = sqlx::query_as::<_, RefreshTokenModel>(
        "SELECT * FROM refresh_tokens WHERE token = $1 AND revoked_at IS NULL AND expires_at > NOW()",
    )
        .bind(refresh_token)
        .fetch_optional(db.get_ref())
        .await?;

    let record = match token_record {
        Some(r) => r,
        None => return Err("Invalid or expired refresh token".into()),
    };

    let claims = verify_token(refresh_token, jwt_secret)?;
    let user_id: i32 = claims.sub.parse()?;
    if user_id != record.user_id {
        return Err("User ID mismatch".into());
    }

    let new_access_token = generate_jwt(user_id, jwt_secret, access_token_expiration)?;
    Ok(LoginResponse {
        token: new_access_token,
        message: "Access token refreshed successfully".to_string(),
    })
}