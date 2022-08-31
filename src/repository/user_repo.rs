use super::Repository;
use crate::{drivers::db::DB, entities::UserBO};

impl Repository {
    pub async fn create_user(&self, pool: &DB, user: UserBO) -> Result<i64, sqlx::Error> {
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

    pub async fn find_user(&self, pool: &DB, id: i64) -> Result<UserBO, sqlx::Error> {
        let user = sqlx::query_as!(UserBO, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, pool: &DB, id: i64) -> Result<(), sqlx::Error> {
        let _result = sqlx::query!("DELETE FROM users where id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, pool: &DB, user: UserBO) -> Result<(), sqlx::Error> {
        let id = user.id;
        let _result = sqlx::query!(
            "UPDATE users SET username = $1 where id = $2",
            user.username,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();

        if _result > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
