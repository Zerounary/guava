use serde::{Deserialize, Serialize};

use guava_derive::AutoInto;

use crate::{
    entities::{UserBO},
    service::user_service::{CreateUserInput, UpdateUserInput},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[derive(AutoInto)]
pub struct CreateUserVO {
    pub username: String,
}

// TODO 自动生成 VO 和 BO 的相互转换


#[derive(Debug, Deserialize, AutoInto)]
pub struct UpdateUserVO {
    pub id: Option<i64>,
    pub username: String,
    pub done: bool,
}
// impl From<UpdateUserVO> for UpdateUserInput {
//     fn from(s: UpdateUserVO) -> Self {
//         UpdateUserInput {
//             username: s.username,
//             id: s.id.unwrap(),
//             done: s.done
//         }
//     }
// }

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