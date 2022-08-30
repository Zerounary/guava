use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    id: i64,
    username: String,
    done: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUser {
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: Option<i64>,
    username: String,
}

// 业务的特性方法
#[async_trait]
pub trait UserRepo {
    /// Loop up a user by their id.
    async fn find(&self, user_id: i64) -> Result<User, UserRepoError>;

    /// Delete a user by their id.
    async fn delete(&self, user_id: i64) -> Result<(), UserRepoError>;

    /// Create a new user.
    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError>;

    /// Update a user
    async fn update(&self, params: UpdateUser) -> Result<User, UserRepoError>;
}

pub type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;

// 业务的特性的实现
pub struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find(&self, _user_id: i64) -> Result<User, UserRepoError> {
        //unimplemented!()
        let user = find_user(&DB_POOL, _user_id).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => {
                Err(UserRepoError::NotFound)
            }
            _ => Err(UserRepoError::NotFound),
        }
    }

    async fn create(&self, _params: CreateUser) -> Result<User, UserRepoError> {
        let user_id = create_user(&DB_POOL, _params).await;

        match user_id {
            Ok(id) => self.find(id).await,
            Err(e) => {
                dbg!(e);
                Err(UserRepoError::NotFound)
            }
        }
    }

    async fn update(&self, user: UpdateUser) -> Result<User, UserRepoError> {
        match user.id {
            Some(user_id) => {
            let result = update_user(&DB_POOL, user).await;
            
            match result {
                Ok(_) => self.find(user_id).await,
                Err(e) => {
                    Err(UserRepoError::NotFound)
                }
            }
            },
            None =>  Err(UserRepoError::NotFound),
        }
    }
    async fn delete(&self, user_id: i64) -> Result<(), UserRepoError> {
        let result = delete_user(&DB_POOL, user_id).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(UserRepoError::NotFound)
            }
        }
    }
}

async fn create_user(pool: &PgPool, user: CreateUser) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        "
INSERT INTO users ( username )
VALUES ( $1 )
RETURNING id
        ",
        user.username
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.id)
}

async fn find_user(pool: &PgPool, id: i64) -> Result<User, sqlx::Error> {
    let mut user = sqlx::query_as!(User,"SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

async fn delete_user(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    let _result = sqlx::query!("DELETE FROM users where id = $1", id)
    .execute(pool)
    .await?;
    Ok(())
}


async fn update_user(pool: &PgPool, user: UpdateUser) -> Result<(), sqlx::Error> {
    let id = user.id.unwrap();
    let _result = sqlx::query!("UPDATE users SET username = $1 where id = $2", user.username, id)
    .execute(pool)
    .await?
    .rows_affected();

    if _result > 0 {
        Ok(())
    }else {
        Err(sqlx::Error::RowNotFound)
    }
} 