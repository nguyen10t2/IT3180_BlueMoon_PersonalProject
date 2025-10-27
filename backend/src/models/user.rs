use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::user_enum::UserRole;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub username        : String,
    pub password        : String,
    pub fullname        : String,
    #[serde(default)]
    pub email           : Option<String>,
    #[serde(default)]
    pub role            : UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username        : String,
    pub password        : String,
}