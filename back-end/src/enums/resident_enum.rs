use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub enum Sex {
    #[sqlx(rename = "Nam")]
    Nam,
    #[sqlx(rename = "Nữ")]
    Nu,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub enum RelationShip {
    #[sqlx(rename = "Chủ hộ")]
    ChuHo,
    #[sqlx(rename = "Vợ")]
    Vo,
    #[sqlx(rename = "Chồng")]
    Chong,
    #[sqlx(rename = "Con")]
    Con,
    #[sqlx(rename = "Cha")]
    Cha,
    #[sqlx(rename = "Mẹ")]
    Me,
    #[sqlx(rename = "Anh")]
    Anh,
    #[sqlx(rename = "Chị")]
    Chi,
    #[sqlx(rename = "Em")]
    Em,
    #[sqlx(rename = "Khac")]
    Khac,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub enum ResidencyStauts {
    #[sqlx(rename = "Thường trú")]
    ThuongTru,
    #[sqlx(rename = "Tạm trú")]
    TamTru,
    #[sqlx(rename = "Tạm vắng")]
    TamVang,
}