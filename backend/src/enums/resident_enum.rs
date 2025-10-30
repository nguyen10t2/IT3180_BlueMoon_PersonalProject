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
#[sqlx(type_name = "relationship")]
pub enum RelationShip {
    #[sqlx(rename = "ChuHo")]
    ChuHo,
    #[sqlx(rename = "Vo")]
    Vo,
    #[sqlx(rename = "Chong")]
    Chong,
    #[sqlx(rename = "Con")]
    Con,
    #[sqlx(rename = "Cha")]
    Cha,
    #[sqlx(rename = "Me")]
    Me,
    #[sqlx(rename = "Anh")]
    Anh,
    #[sqlx(rename = "Chi")]
    Chi,
    #[sqlx(rename = "Em")]
    Em,
    #[sqlx(rename = "Khac")]
    Khac,
}

impl Default for RelationShip {
    fn default() -> Self {
        RelationShip::ChuHo
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