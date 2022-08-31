use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: Option<i64>,
    pub username: String,
    pub done: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub done: bool,
}