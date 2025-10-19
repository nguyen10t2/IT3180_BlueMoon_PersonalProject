use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::enums::user_enum::*;

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct User {
    pub user_id     : i32,
    pub username    : String,
    pub password    : String,
    pub fullname    : String,

    #[serde(default)]
    pub email       : Option<String>,

    #[serde(default)]
    pub phonenumber : Option<String>,

    pub role        : UserRole,

    #[serde(default = "default_status")]
    pub status      : UserStatus,

    #[serde(default = "Utc::now")]
    pub created_at  : DateTime<Utc>,

    #[serde(default = "Utc::now")]
    pub updated_at  : DateTime<Utc>,
}

// #[derive(Debug, Deserialize)]
// pub struct CreateUser {
//     pub username: String,
//     pub password: String,
//     pub fullname: String,
//     pub email: Option<String>,
//     pub phonenumber: Option<String>,
//     pub role: UserRole,
// }

fn default_status() -> UserStatus {
    UserStatus::Stay
}
