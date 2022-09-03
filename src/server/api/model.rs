use serde::{Deserialize, Serialize};

use crate::{
    entities,
    service::uesr_service::{CreateUserInput, UpdateUserInput},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserVO {
    pub username: String,
}
impl Into<CreateUserInput> for CreateUserVO {
    fn into(self) -> CreateUserInput {
        CreateUserInput {
            username: self.username,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserVO {
    pub id: Option<i64>,
    pub username: String,
    pub done: bool,
}

impl Into<UpdateUserInput> for UpdateUserVO {
    fn into(self) -> UpdateUserInput {
        UpdateUserInput {
            id: self.id.unwrap(),
            username: self.username,
            done: self.done,
        }
    }
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
