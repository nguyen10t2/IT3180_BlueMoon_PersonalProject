use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    #[sqlx(rename = "Nam")]
    Nam,
    #[sqlx(rename = "Nu")]
    Nu,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "operational_status")]
pub enum OperationalStatus {
    #[sqlx(rename = "active")]
    Active, 
    #[sqlx(rename = "anactive")]
    Inactive,
    #[sqlx(rename = "aemporarilyaway")]
    TemporarilyAway, 
}

impl Default for OperationalStatus {
    fn default() -> Self {
        OperationalStatus::Active
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "relationship")]
pub enum RelationShip {
    #[sqlx(rename = "chusohuu")]
    ChuSoHuu, 
    #[sqlx(rename = "nguoidaidien")]
    NguoiDaiDien,
    #[sqlx(rename = "thanhvien")]
    ThanhVien,
    #[sqlx(rename = "nguoithue")]
    NguoiThue,
}

impl Default for RelationShip {
    fn default() -> Self {
        RelationShip::ChuSoHuu
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "resident_status")]
pub enum ResidencyStatus {
    #[sqlx(rename = "ThuongTru")]
    ThuongTru,
    #[sqlx(rename = "TamTru")]
    TamTru,
    #[sqlx(rename = "TamVang")]
    TamVang,
}

impl Default for ResidencyStatus {
    fn default() -> Self {
        ResidencyStatus::ThuongTru
    }
}