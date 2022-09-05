use crate::{
    create, read,
    server::api::model::{CreateUserVO, UpdateUserVO, UserVO},
    service::user_service::{CreateUserInput, UpdateUserInput}, update, delete,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use itertools::Itertools;

use super::{AppResult, Resp, State};

read!(find_user_by_id -> UserVO);
read!(find_user_by_id_no_cache -> UserVO);
// read!(find_user_by_done(Path(done): ) -> Vec<UserVO>);

pub async fn find_user_by_done(Json(params): Json<serde_json::Value>, Extension(state): State) -> AppResult<Vec<UserVO>> {
    let query = params.as_object();
    let mut done = false;

    match query {
        Some(map) => {done = map.get("done").unwrap().as_bool().unwrap_or(false)},
        _ => {}
    }

    let result = state.service.find_user_by_done(done).await?;
    let vos = result.iter().map(|x| UserVO::from(x)).collect_vec();
    Resp::ok(vos)
}

create!(CreateUserVO > create_user(CreateUserInput)  > UserVO);

create!(Vec<CreateUserVO> > create_user_batch(Vec<CreateUserInput>) > Vec<i64>);

update!(UpdateUserVO -> update_user(UpdateUserInput) -> UserVO);

delete!(delete_user_ids);
