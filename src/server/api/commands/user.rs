use crate::{
    create, read,
    server::api::model::{CreateUserVO, UpdateUserVO, UserVO},
    service::uesr_service::{CreateUserInput, UpdateUserInput}, update, delete,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};

use super::{AppResult, Resp, State};

read!(find_user_by_id -> UserVO);
read!(find_user_by_id_no_cache -> UserVO);

create!(CreateUserVO -> create_user(CreateUserInput)  -> UserVO);

update!(UpdateUserVO -> update_user(UpdateUserInput) -> UserVO);

delete!(delete_user);
