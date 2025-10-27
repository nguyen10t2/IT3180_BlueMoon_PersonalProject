use argon2::{
    password_hash::{rand_core::OsRng, SaltString}, 
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};

use jsonwebtoken::{EncodingKey, Header, encode, errors::Error as JwtError};
use serde::{Serialize, Deserialize};


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

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(&password.as_bytes(), &salt).unwrap().to_string();
    hashed_password
}

pub fn verify_password(hashed_password: String, password: String) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let hashed_password = PasswordHash::new(&hashed_password).unwrap();
    argon2.verify_password(&password.as_bytes(), &hashed_password)
}

pub fn generate_jwt(username: String, secret: &str, expiration: i64) -> Result<String, JwtError> {
    let current_time = chrono::Utc::now();
    let claims = Claims {
        sub: username,
        iat: current_time.timestamp(),
        exp: (current_time + chrono::Duration::seconds(expiration)).timestamp(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}