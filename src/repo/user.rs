use std::sync::Arc;
use axum::async_trait;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

// 业务错误
#[derive(Debug)]
pub enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}

// 业务的实体

#[derive(Debug, Serialize)]
pub struct User {
    id: Uuid,
    username: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUser {
    username: String,
}

// 业务的特性方法
#[async_trait]
pub trait UserRepo {
    /// Loop up a user by their id.
    async fn find(&self, user_id: Uuid) -> Result<User, UserRepoError>;

    /// Create a new user.
    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError>;
}

pub type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;



// 业务的特性的实现
pub struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find(&self, _user_id: Uuid) -> Result<User, UserRepoError> {
        //unimplemented!()
        Err(UserRepoError::NotFound)
    }

    async fn create(&self, _params: CreateUser) -> Result<User, UserRepoError> {
        unimplemented!()
    }
}