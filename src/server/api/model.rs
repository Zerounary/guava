use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{self, UserBO},
    service::user_service::{CreateUserInput, UpdateUserInput},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserVO {
    pub username: String,
}
impl From<CreateUserVO> for CreateUserInput {
    fn from(s: CreateUserVO) -> Self {
        CreateUserInput {
            username: s.username,
        }
    }
}

impl From<&CreateUserVO> for CreateUserInput {
    fn from(s: &CreateUserVO) -> Self {
        CreateUserInput {
            username: s.username.clone(),
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

impl From<UserBO> for UserVO {
    fn from(user: UserBO) -> Self {
        UserVO {
            id: user.id,
            username: user.username,
            done: user.done,
        }
    }
}

impl From<&UserBO> for UserVO {
    fn from(user: &UserBO) -> Self {
        UserVO {
            id: user.id,
            username: user.username.clone(),
            done: user.done,
        }
    }
}