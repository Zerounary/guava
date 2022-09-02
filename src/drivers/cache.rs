use moka::sync::Cache;
use std::sync::Arc;

use crate::{entities::UserBO, service::uesr_service::UserRepoError};

#[derive(Debug, Clone)]
pub enum ServiceResult {
    UserBO(Result<UserBO, UserRepoError>),
}

pub type ServiceCache = Arc<Cache<i64, ServiceResult>>;

#[macro_export]
macro_rules! cache_value {
    ($name:ident as Result<$type:ident, $err:ident>) => {{
        let value: Result<$type, $err> = match $name {
            ServiceResult::$type(v) => match v {
                Ok(bo) => Ok(bo),
                Err(e) => Err(e),
            },
            _ => Err($err::NotFound),
        };
        value
    }};

    ($name:ident as $type:ident) => {{
        let value: Option<$type> = match $name {
            ServiceResult::$type(v) => Some(v.clone()),
            _ => None,
        };
        value
    }};
}

/// pub async fn find_cache(&self, _user_id: i64) -> Result<UserBO, UserRepoError> {
///     cache!{
///         self(_user_id) -> Result<UserBO, UserRepoError> {
///             let user = self.repo.find_user(&self.db, _user_id).await;
///             match user {
///                 Ok(user) => Ok(user),
///                 Err(_e) => Err(UserRepoError::NotFound),
///             }
///         }
///     }
/// }

#[macro_export]
macro_rules! cache {
    ($self:ident($key:ident) -> Result<$bo:ident, $err:ident> $block:block) => {
        match $self.cache.get(&$key) {
            Some(e) => {
                let x = cache_value!(e as Result<$bo, $err>);
                x
            }
            None => {
                let result: Result<$bo, $err> = $block;
                $self
                    .cache
                    .insert($key, ServiceResult::$bo(result.clone()));
                result
            }
        }
    };
}
