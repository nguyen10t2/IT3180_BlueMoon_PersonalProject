use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::resident_enum::{RelationShip, Sex, ResidencyStatus};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Residents {
    pub resident_id         : i32,
    pub hourse_id           : i32,
    pub fullname            : String,
    pub birth               : NaiveDate,
    pub sex                 : Sex,
    pub relationship        : RelationShip,
    pub phone_number        : String,
    pub occupation          : String,
    pub residency_status    : ResidencyStatus,
    pub residency_date      : NaiveDateTime,
}

