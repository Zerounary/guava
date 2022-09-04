use std::sync::{Arc};

use moka::sync::Cache;

use crate::{repository::Repository, drivers::{db::DB, cache::{ServiceCache}}};

pub mod user_service;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB, // 为了调用事物
    cache: ServiceCache
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