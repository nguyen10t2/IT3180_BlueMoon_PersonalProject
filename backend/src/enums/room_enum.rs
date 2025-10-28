use serde::{Deserialize, Serialize};

use sqlx::Type;

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
#[sqlx(type_name = "room_type")]
pub enum RoomType {
    #[sqlx(rename = "Don")]
    Don,
    #[sqlx(rename = "Doi")]
    Doi,
}