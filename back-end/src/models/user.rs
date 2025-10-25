use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::enums::user_enum::{UserRole, UserStatus};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub user_id         : i32,
    pub username        : String,
    #[serde(skip_serializing)]
    #[sqlx(rename = "password_hash")]
    pub password_hash   : String,
    pub fullname        : String,
    #[serde(default)]
    pub email           : Option<String>,
    #[serde(default)]
    pub phone_number    : Option<String>,
    pub role            : UserRole,
    #[serde(default)]
    pub status          : UserStatus,
    #[serde(default = "Utc::now")]
    pub created_at      : DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at      : DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateUser {
    pub username        : String,
    pub password        : String,
    pub role            : Option<UserRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub role: UserRole,
}