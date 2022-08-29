use std::{sync::Arc, fmt::Error};
use axum::async_trait;
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};

use crate::db::DB_POOL;

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
#[derive(sqlx::FromRow)]
pub struct User {
    id: i64,
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
    async fn find(&self, user_id: i64) -> Result<User, UserRepoError>;

    /// Create a new user.
    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError>;
}

pub type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;



// 业务的特性的实现
pub struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find(&self, _user_id: i64) -> Result<User, UserRepoError> {
        //unimplemented!()
      let user =  find_user(&DB_POOL, _user_id).await;

      match user {
          Ok(user) => Ok(user),
          Err(e) => {
            dbg!(e);
            Err(UserRepoError::NotFound)
          },
          _ => Err(UserRepoError::NotFound)
      }
    }

    async fn create(&self, _params: CreateUser) -> Result<User, UserRepoError> {
      !unimplemented!()
    }
}

async fn create_user(pool: &SqlitePool, user: CreateUser) -> anyhow::Result<i64> {
    Ok(1 as i64)
}

async fn find_user(pool: &SqlitePool, id: i64) -> Result<User, sqlx::Error> {
    let mut user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}