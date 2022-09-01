// use cached::proc_macro::cached;
use serde::Deserialize;

use crate::{entities::UserBO, repository::Repository, drivers::db::DB};

use super::Service;

// 业务错误
#[derive(Debug, Clone)]
pub enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserInput {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserInput {
    pub id: i64,
    pub username: String,
    pub done: bool,
}

// #[cached(
//     key = "String",
//     result = true,
//     convert = r#"{ format!("{}", id) }"#
// )]
async fn user_find(repo: &Repository, db: &DB, id: i64) -> Result<UserBO, UserRepoError> {
    let user = repo.find_user(db, id).await;
    match user {
        Ok(user) => Ok(user),
        Err(_e) => Err(UserRepoError::NotFound),
    }
}

impl Service {

    pub async fn find(&self, _user_id: i64) -> Result<UserBO, UserRepoError> {
        match self.cache.get(&_user_id)  {
            Some(cached_data) => cached_data,
            None => {
                let result = user_find(&self.repo, &self.db, _user_id).await;
                self.cache.insert(_user_id, result.clone());
                result
            }
        }
    }

    pub async fn create(&self, input: CreateUserInput) -> Result<UserBO, UserRepoError> {
        let user = UserBO {
            username: input.username,
            ..UserBO::default()
        };
        let user_id = self.repo.create_user(&self.db, user).await;

        match user_id {
            Ok(id) => self.find(id).await,
            Err(e) => {
                dbg!(e);
                Err(UserRepoError::NotFound)
            }
        }
    }

    pub async fn update(&self, input: UpdateUserInput) -> Result<UserBO, UserRepoError> {
        let user = UserBO {
            id: input.id,
            username: input.username,
            done: input.done,
        };
        let result = self.repo.update_user(&self.db, user).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&input.id);
                self.find(input.id).await
            },
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
    pub async fn delete(&self, user_id: i64) -> Result<(), UserRepoError> {
        let result = self.repo.delete_user(&self.db, user_id).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&user_id);
                Ok(())
            },
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
}
