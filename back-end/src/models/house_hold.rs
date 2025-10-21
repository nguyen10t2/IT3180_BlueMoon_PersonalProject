use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::enums::room_enum::RoomType;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct HouseHold {
    pub hourse_hold_id      : i32,
    pub room_number         : String,
    pub room_type           : RoomType,
    pub hourse_hold_head    : String,
    
    #[serde(default)]
    pub members             : i32,

    pub notes               : Option<String>,
}

