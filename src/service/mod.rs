use crate::{repository::Repository, drivers::db::DB};

pub mod uesr_service;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB // 为了调用事物
}

impl Service {
    pub fn new(db: DB ) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
        }
    }
}