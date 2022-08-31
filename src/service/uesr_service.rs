use serde::Deserialize;

use crate::entities::UserBO;

use super::Service;

// 业务错误
#[derive(Debug)]
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
    pub async fn find(&self, _user_id: i64) -> Result<UserBO, UserRepoError> {
        //unimplemented!()
        let user = self.repo.find_user(&self.db, _user_id).await;

        match user {
            Ok(user) => Ok(user),
            Err(_e) => Err(UserRepoError::NotFound),
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
            Ok(_) => self.find(input.id).await,
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
    pub async fn delete(&self, user_id: i64) -> Result<(), UserRepoError> {
        let result = self.repo.delete_user(&self.db, user_id).await;

        match result {
            Ok(_) => Ok(()),
            Err(_e) => Err(UserRepoError::NotFound),
        }
    }
}
