use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::enums::user_role::UserRole;

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct User {
    pub user_id     : u32,
    pub username    : String,
    pub password    : String,
    pub fullname    : String,
    pub email       : Option<String>,
    pub phonenumber : Option<String>,
    pub role        : UserRole,
    pub status      : String,
    pub created_at  : DateTime<Utc>,
    pub updated_at  : DateTime<Utc>,
}