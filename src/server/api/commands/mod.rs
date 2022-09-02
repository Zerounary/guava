pub mod hello;
pub mod user;

use crate::{AppState, server::error::AppError};
use axum::{response::Json, Extension};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

// TODO 用 serde_json::Value 来接所有不知道类型的，又要存起来的数据。 也可以看是否可以用Box

// TODO 用 type 别名来收缩复杂的类型

pub type State = Extension<Arc<AppState>>;

pub type AppResult<T> = Result<Json<Resp<T>>, AppError>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Resp<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T> Resp<T> {
    fn ok(data: T) -> AppResult<T> {
        Ok(Json(Self {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }))
    }
}

// TODO 编写 宏 来收敛重复的代码
