use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

use crate::enums::user_enum::{UserRole, UserStatus};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id         : i32,
    pub username        : String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub password_hash   : String,
    pub fullname        : String,
    pub email           : String,
    #[serde(default)]
    pub role            : UserRole,
    pub resident_id     : Option<i32>,
    pub created_at      : NaiveDateTime,
    #[serde(default)]
    pub status          : UserStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username        : String,
    pub password   : String,
    pub fullname        : String,
    pub email           : String,
    #[serde(default)]
    pub role            : UserRole,
    pub resident_id     : Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username        : String,
    pub password        : String,
}