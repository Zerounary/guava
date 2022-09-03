use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::service::uesr_service::UserRepoError;

use super::api::commands::resp_err;

/// 业务错误类型 into 到 AppError
/// impl From<UserRepoError> for AppError {
///     fn from(inner: UserRepoError) -> Self {
///         AppError::UserRepoError(inner)
///     }
/// }
macro_rules! app_error_from {
    ($err:ident) => {
        impl From<$err> for AppError {
            fn from(inner: $err) -> Self {
                AppError::$err(inner)
            }
        }
    };
}

/// 快速创建业务错误类型
/// pub enum AppError {
///     UserRepoError(UserRepoError),
/// }
///
/// impl From<UserRepoError> for AppError {
///     fn from(inner: UserRepoError) -> Self {
///         AppError::UserRepoError(inner)
///     }
/// }
macro_rules! app_error_register {
    (
        $( $err:ident ),+
        $(,)?
    ) => {
        // 创建AppError枚举
        pub enum AppError {
            $(
                $err($err),
            )+
        }
        // 并自动实现业务错误到 AppError 的 from
        $(
            app_error_from!($err);
        )+
    };
}

/// 定义业务错误对应的响应结果
/// let (status, code, error_message) = match self {
///     AppError::UserRepoError(UserRepoError::NotFound) => {
///         (StatusCode::NOT_FOUND, 1, "User not found")
///     }
///     AppError::UserRepoError(UserRepoError::InvalidUsername) => {
///         (StatusCode::UNPROCESSABLE_ENTITY, 2, "Invalid username")
///     }
/// };
macro_rules! app_error_responeses {
    (
        $self:ident {
            $( $err:ident::$err_item:ident => $expr:expr ),+
            $(,)?
        }
    ) => {
        match $self {
        $(
            AppError::$err($err::$err_item) => {
                $expr
            },
        )+
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, 99999, "Unkown Server Error")
            }
        }
    };
}

pub enum TestError {
    TEST,
}

// 新的业务响应在此处注册
app_error_register!(UserRepoError, TestError);

// 全局默认的响应结果
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, error_message) = app_error_responeses!(
            self {
                // 整个应用错误响应 code 和 message 都在这里设置。
                UserRepoError::NotFound => (StatusCode::NOT_FOUND, 1, "User not found"),
                UserRepoError::InvalidUsername => (StatusCode::UNPROCESSABLE_ENTITY, 2, "Invalid username"),
                TestError::TEST => (StatusCode::UNPROCESSABLE_ENTITY, 3, "Invalid username"),
            }
        );

        let body = resp_err(code, error_message.to_string());

        (status, body).into_response()
    }
}
