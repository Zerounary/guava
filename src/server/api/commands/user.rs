use crate::{
    create, read,
    server::api::model::{CreateUserVO, UpdateUserVO, UserVO, UserOptionVO},
    service::user_service::{CreateUserInput, UpdateUserInput}, update, delete, entities::{UserBO, UserOptionBO},
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use itertools::Itertools;

use super::{AppResult, Resp, State};

read!(find_user_by_id > UserVO);
read!(find_user_by_id_no_cache > UserVO);
read!(UserOptionVO > find_user_list > Vec<UserVO>);


create!(CreateUserVO > create_user(CreateUserInput)  > UserVO);
create!(Vec<CreateUserVO> > create_user_batch(Vec<CreateUserInput>) > Vec<i64>);

update!(UpdateUserVO -> update_user(UpdateUserInput) -> UserVO);
delete!(delete_user_ids);
