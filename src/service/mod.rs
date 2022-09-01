use std::sync::{Arc};

use moka::sync::Cache;

use crate::{repository::Repository, drivers::db::DB, entities::UserBO};

use self::uesr_service::UserRepoError;

pub mod uesr_service;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB, // 为了调用事物
    cache: Arc<Cache<i64, Result<UserBO, UserRepoError>>> 
}

impl Service {
    pub fn new(db: DB ) -> Service {
        let repo = Repository::new();
        let cache = Arc::new(Cache::new(10_000));
        Service {
            db,
            repo,
            cache
        }
    }
}