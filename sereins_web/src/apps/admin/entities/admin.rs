use serde::{Serialize, Deserialize};
use sqlx::types::chrono::NaiveDateTime;
use crate::common::utils::convert::serde_datetime;

/// 管理员实体类
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Admin {
    id: u32,
    nickname: String,
    account: String,
    phone: String,
    email: String,
    sex: i8,
    state: i8,
    password: String,
    auth_group: u32,
    department: String,
    admin: i8,
    #[serde(with = "serde_datetime")]
    time_add: NaiveDateTime,
    #[serde(with = "serde_datetime")]
    time_put: NaiveDateTime,
}