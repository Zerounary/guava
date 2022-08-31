use serde::{Deserialize, Serialize};

use crate::entities;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserVO {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserVO {
    pub id: Option<i64>,
    pub username: String,
    pub done: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserVO {
    pub id: i64,
    pub username: String,
    pub done: bool,
}

impl From<entities::UserBO> for UserVO {
    fn from(user: entities::UserBO) -> Self {
        UserVO {
            id: user.id,
            username: user.username,
            done: user.done,
        }
    }
}
