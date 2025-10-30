use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "manager")]
    Manager,
    #[sqlx(rename = "resident")]
    Resident,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Resident
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