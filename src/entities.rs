// 业务的实体

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserBO {
    pub id: i64,
    pub username: String,
    pub done: bool,
}
