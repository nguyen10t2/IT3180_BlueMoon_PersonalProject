use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::resident_enum::{RelationShip, Sex, ResidencyStauts};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Residents {
    pub resident_id         : i32,
    pub hourse_id           : i32,
    pub fullname            : String,
    pub birth               : DateTime<Utc>,
    pub sex                 : Sex,
    pub relationship        : RelationShip,
    pub phone_number        : String,
    pub occupation          : String,

    #[serde[default = "default_residency_status"]]
    pub residency_status    : ResidencyStauts,

    pub residency_date      : DateTime<Utc>,
}

fn default_residency_status() -> ResidencyStauts {
    ResidencyStauts::TamTru
}
