use serde::{Deserialize, Serialize};
use struct_convert::Convert;


use crate::{
    entities::{UserBO, UserOptionBO},
    service::user_service::{CreateUserInput, UpdateUserInput},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[derive(Convert)]
#[convert(into = "CreateUserInput")]
pub struct CreateUserVO {
    pub username: String,
}

#[derive(Convert)]
#[convert(into = "UpdateUserInput")]
#[derive(Debug, Deserialize)]
pub struct UpdateUserVO {
    #[convert_field(unwrap)]
    pub id: Option<i64>,
    pub username: String,
    pub done: bool,
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(into = "UserOptionBO")]
pub struct UserOptionVO {
    #[convert_field(ignore)]
    pub id: Option<i64>,
    pub username: Option<String>,
    pub done: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(from = "UserBO")]
pub struct UserVO {
    pub id: i64,
    pub username: String,
    pub done: bool,
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