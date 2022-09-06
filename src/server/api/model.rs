use serde::{Deserialize, Serialize};

use crate::{
    entities::{UserBO},
    service::user_service::{CreateUserInput, UpdateUserInput},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserVO {
    pub username: String,
}

// TODO 自动生成 VO 和 BO 的相互转换

impl From<CreateUserVO> for CreateUserInput {
    fn from(s: CreateUserVO) -> Self {
        CreateUserInput {
            username: s.username,
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
pub struct UserOptionVO {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub done: Option<bool>,
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


#[cfg(test)]
mod test {
    #[derive(Debug, Default)]
    struct E {
        f: i32,
    }
    #[derive(Default, Debug)]
    struct A {
        a: i32,
        b: Option<i32>,
        e: E
    }
    fn test() {
     let a = A::default();
    }
}