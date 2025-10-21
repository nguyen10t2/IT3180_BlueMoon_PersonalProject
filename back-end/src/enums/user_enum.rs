use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    #[sqlx(rename = "Cư dân")]
    CuDan,
    #[sqlx(rename = "Kế toán")]
    KeToan,
    #[sqlx(rename = "Quản lý")]
    QuanLy,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "user_status")]
pub enum UserStatus {
    #[sqlx(rename = "Hoạt động")]
    HoatDong,
    #[sqlx(rename = "Nghỉ việc")]
    NghiViec,
}