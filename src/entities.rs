// 业务的实体

use serde::Serialize;

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct UserBO {
    pub id: i64,
    pub username: String,
    pub done: bool,
}