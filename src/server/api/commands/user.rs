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

read!(find_user_by_id -> UserVO);
read!(find_user_by_id_no_cache -> UserVO);
read!(UserOptionVO > find_user_list {
    fn into(vo: UserOptionVO) -> UserOptionBO {
        UserOptionBO {
            username: vo.username,
            done: vo.done,
        }
    }
    fn outo(result:Vec<UserBO>) -> Vec<UserVO> {
        result.into_iter().map(|x| x.into() ).collect_vec()
    }
} > Vec<UserVO>);

// pub async fn find_user_by_done(Json(params): Json<UserOptionVO>, Extension(state): State) -> AppResult<Vec<UserVO>> {
//     fn into(vo: UserOptionVO) ->  bool {
//         let done = vo.done.unwrap_or(false);
//         done
//     }
//     let result = state.service.find_user_by_done(into(params)).await?;
//     let vos = result.iter().map(|x| UserVO::from(x)).collect_vec();
//     Resp::ok(vos)
// }

create!(CreateUserVO > create_user(CreateUserInput)  > UserVO);

create!(Vec<CreateUserVO> > create_user_batch(Vec<CreateUserInput>) > Vec<i64>);

update!(UpdateUserVO -> update_user(UpdateUserInput) -> UserVO);

delete!(delete_user_ids);
