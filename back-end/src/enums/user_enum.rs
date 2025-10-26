use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "user1")]
    User1,
    #[sqlx(rename = "user2")]
    User2,
    #[sqlx(rename = "user3")]
    User3,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User1
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "inactive")]
    Inactive,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}