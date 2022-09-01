use std::sync::Arc;

use crate::server::error::AppError;
use crate::{
    server::api::model::{CreateUserVO, UpdateUserVO, UserVO},
    service::uesr_service::{CreateUserInput, UpdateUserInput},
    AppState,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};

pub async fn users_show_no_cache(
    Path(user_id): Path<i64>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<UserVO>, AppError> {
    let user = state.service.find_no_cache(user_id).await?;

    Ok(Json(user.into()))
}

pub async fn users_show(
    Path(user_id): Path<i64>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<UserVO>, AppError> {
    let user = state.service.find(user_id).await?;

    Ok(Json(user.into()))
}

/// Handler for `POST /users`.
pub async fn users_create(
    Json(params): Json<CreateUserVO>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<UserVO>, AppError> {
    let service_input = CreateUserInput {
        username: params.username,
    };
    let user = state.service.create(service_input).await?;

    Ok(Json(user.into()))
}

pub async fn users_delete(
    Path(id): Path<i64>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    match state.service.delete(id).await {
        Ok(_) => StatusCode::OK,
        Err(_e) => StatusCode::NOT_FOUND,
    }
}

pub async fn users_update(
    Path(id): Path<i64>,
    Json(mut user): Json<UpdateUserVO>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<UserVO>, AppError> {
    user.id = Some(id);
    let service_input = UpdateUserInput {
        id,
        username: user.username,
        done: user.done,
    };
    let user = state.service.update(service_input).await?;
    Ok(Json(user.into()))
}
