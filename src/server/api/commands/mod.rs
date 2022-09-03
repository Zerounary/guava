pub mod hello;
pub mod user;

use crate::{server::error::AppError, AppState};
use axum::{response::Json, Extension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Empty;

impl<T> Resp<T> {
    pub fn ok(data: T) -> AppResult<T> {
        Ok(Json(Self {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }))
    }
}

pub fn resp_err(code: i32, msg: String) -> Json<Resp<Empty>> {
    Json(Resp {
        code,
        msg,
        data: None,
    })
}

// TODO 编写 宏 来收敛重复的代码
#[macro_export]
macro_rules! read {
    ($service_fn:ident -> $vo:ty) => {
        pub async fn $service_fn(
            Path(id): Path<i64>,
            Extension(state): State,
        ) -> AppResult<$vo> {
            let res = state.service.$service_fn(id).await?;

            Resp::ok(res.into())
        }
    };
}

#[macro_export]
macro_rules! create {
    ($req_vo:ident -> $service_fn:ident ( $service_input:ident)  -> $res_vo:ident) => {
        pub async fn $service_fn(
            Json(params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            let service_input: $service_input = params.into() ;
            let user = state.service.$service_fn(service_input).await?;

            Resp::ok(user.into())
        }
    };
}

#[macro_export]
macro_rules! update {
    ($req_vo:ident -> $service_fn:ident ( $service_input:ident)  -> $res_vo:ident) => {
        pub async fn $service_fn(
            Path(id): Path<i64>,
            Json(mut params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            params.id = Some(id);
            let service_input: $service_input = params.into();
            let user = state.service.update_user(service_input).await?;
            Resp::ok(user.into())
        }
    };
}


#[macro_export]
macro_rules! delete {
    ( $service_fn:ident ) => {
        pub async fn $service_fn(
            Path(id): Path<i64>,
            Extension(state): State
        ) -> impl IntoResponse {
            match state.service.$service_fn(id).await {
                Ok(_) => StatusCode::OK,
                Err(_e) => StatusCode::NOT_FOUND,
            }
        }
    };
}