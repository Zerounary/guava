use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::service::user_service::UserRepoError;

use super::api::commands::resp_err;


macro_rules! app_error_register {
    (
        $(
            $err:ident {
                $( $err_item:ident => $expr:expr ),+
                $(,)?
            }
        ),+
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
            impl From<$err> for AppError {
                fn from(inner: $err) -> Self {
                    AppError::$err(inner)
                }
            }
        )+

        impl IntoResponse for AppError {
            fn into_response(self) -> Response {
                let (status, code, error_message) = match self { 
                    // 匹配错误类型和对应的响应
                    $(
                        $(
                            AppError::$err($err::$err_item) => {
                                $expr
                            },
                        )+
                    )+
                    _ => {
                        (StatusCode::INTERNAL_SERVER_ERROR, 99999, "Unkown Server Error")
                    }
                };
        
                let body = resp_err(code, error_message.to_string());
        
                (status, body).into_response()
            }
        }
    };
}

app_error_register!{
    // 整个应用错误响应 code 和 message 都在这里设置。
    UserRepoError {
        NotFound => (StatusCode::NOT_FOUND, 1, "User not found"),
        InvalidUsername => (StatusCode::UNPROCESSABLE_ENTITY, 2, "Invalid username"),
    }
}