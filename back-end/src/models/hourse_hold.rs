use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct HourseHold {
    pub hourse_hold_id: i32,
    
}

