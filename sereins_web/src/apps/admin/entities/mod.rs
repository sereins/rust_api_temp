use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,sqlx::FromRow)]
pub struct Admin{
    pub id:u32,
    pub name:String,
    pub icon:String,
    pub path:String,
    pub url:String,
    pub state:i8,
    pub p_id:i32
}