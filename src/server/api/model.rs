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
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(into = "UserOptionBO")]
pub struct UserOptionVO {
    #[convert_field(ignore)]
    pub id: Option<i64>,
    pub username: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(from = "UserBO")]
pub struct UserVO {
    #[convert_field(to_string)]
    pub id: String,
    pub username: String,
}