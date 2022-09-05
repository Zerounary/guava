// use cached::proc_macro::cached;
use serde::Deserialize;

use crate::{entities::UserBO, drivers::{cache::ServiceResult}, cache_value, cache};

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

impl Service {

    pub async fn find_user_by_id_no_cache(&self, user_id: i64) -> Result<UserBO, UserRepoError> {
        let user = self.repo.select_user_by_id(&self.db, user_id).await;
        match user {
            Ok(user) => Ok(user),
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
    
    pub async fn find_user_by_id(&self, user_id: i64) -> Result<UserBO, UserRepoError> {
        cache!{
            self(user_id) -> Result<UserBO, UserRepoError> {
                let user = self.repo.select_user_by_id(&self.db, user_id).await;
                match user {
                    Ok(user) => Ok(user),
                    Err(_e) => Err(UserRepoError::NotFound),
                }
            }
        }
    }

    pub async fn create_user(&self, input: CreateUserInput) -> Result<UserBO, UserRepoError> {
        let user = UserBO {
            username: input.username,
            ..UserBO::default()
        };
        let user_id = self.repo.create_user(&self.db, user).await;

        match user_id {
            Ok(id) => self.find_user_by_id(id).await,
            Err(e) => {
                dbg!(e);
                Err(UserRepoError::NotFound)
            }
        }
    }

    pub async fn create_user_batch(&self, mut input: Vec<CreateUserInput>) -> Result<Vec<i64>, UserRepoError> {
        let mut users = input.iter_mut().map(|e| {
            UserBO {
                username: e.username.clone(),
                ..UserBO::default()
            }
        }).collect::<Vec<UserBO>>();
        let result = self.repo.create_user_batch(&self.db, &mut users, 100).await;

        match result {
            Ok(insert_result) => Ok(insert_result.insert_ids),
            Err(e) => {
                dbg!(e);
                Err(UserRepoError::NotFound)
            }
        }
    }

    pub async fn update_user(&self, input: UpdateUserInput) -> Result<UserBO, UserRepoError> {
        let user = UserBO {
            id: input.id,
            username: input.username,
            done: input.done,
        };
        let result = self.repo.update_user_by_id(&self.db, &user, user.id).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&input.id);
                self.find_user_by_id(input.id).await
            },
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
    pub async fn delete_user(&self, user_id: i64) -> Result<(), UserRepoError> {
        let result = self.repo.delete_user(&self.db, user_id).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&user_id);
                Ok(())
            },
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
    pub async fn delete_user_ids(&self, ids: Vec<i64>) -> Result<(), UserRepoError> {
        let result = self.repo.delete_user_ids(&self.db, ids.clone()).await;

        match result {
            Ok(_) => {
                for id in ids {
                    self.cache.invalidate(&id);
                }
                Ok(())
            },
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
}
