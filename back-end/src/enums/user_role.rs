use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub enum UserRole {
    #[sqlx(rename = "Cư dân")]
    CuDan,
    #[sqlx(rename = "Kế toán")]
    KeToan,
    #[sqlx(rename = "Quản lý")]
    QuanLy,
}

