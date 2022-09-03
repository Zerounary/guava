use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::service::uesr_service::UserRepoError;

use super::api::commands::resp_err;

pub enum AppError {
    UserRepo(UserRepoError),
}

/// This makes it possible to use `?` to automatically convert a `UserRepoError`
/// into an `AppError`.
impl From<UserRepoError> for AppError {
    fn from(inner: UserRepoError) -> Self {
        AppError::UserRepo(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, error_message) = match self {
            AppError::UserRepo(UserRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, 1, "User not found")
            }
            AppError::UserRepo(UserRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, 2, "Invalid username")
            }
        };

        let body = resp_err(code, error_message.to_string());

        (status, body).into_response()
    }
}
