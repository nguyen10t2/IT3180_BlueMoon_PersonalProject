use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::resident_enum::{RelationShip, Gender, ResidencyStatus};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Resident {
    pub resident_id         : i32,
    pub house_id           : i32,
    pub fullname            : String,
    pub birth               : NaiveDate,
    pub gender              : Gender,
    #[serde(default)]
    pub relationship        : RelationShip,
    pub phone_number        : Option<String>,
    pub occupation          : Option<String>,
    #[serde(default)]
    pub residency_status    : ResidencyStatus,
    pub created_at          : NaiveDateTime,
}

