use rbs::to_value;
use itertools::Itertools;
use super::Repository;
use crate::{drivers::db::DB, entities::UserBO};

impl Repository {
    pub async fn create_user(&self, pool: &DB, user: UserBO) -> Result<i64, rbatis::Error> {
        let id: i64 = pool
            .fetch_decode(
                "
          INSERT INTO users ( username )
            VALUES ( ? )
            RETURNING id
        ",
                vec![to_value!(user.username)],
            )
            .await
            .unwrap();

        Ok(id)
    }

    pub async fn find_user(&self, pool: &DB, id: i64) -> Result<UserBO, rbatis::Error> {
        let result = pool
            .fetch_decode("SELECT * FROM users WHERE id = ?", vec![to_value!(id)])
            .await;

        match result {
            Ok(uesr_bo) => Ok(uesr_bo),
            Err(_) => Err(rbatis::Error::E("Not Found!".to_string()))
        }
    }

    pub async fn delete_user(&self, pool: &DB, id: i64) -> Result<(), rbatis::Error> {
        pool.fetch("DELETE FROM users where id = ?", vec![to_value!(id)])
            .await
            .unwrap();
        Ok(())
    }

    pub async fn delete_user_ids(&self, pool: &DB, ids: Vec<i64>) -> Result<(), rbatis::Error> {
        if ids.is_empty() {
            return Ok(())
        }
        let sql = format!("DELETE FROM users where id in ({})", ids.iter().join(","));
        pool.fetch(sql.as_str(), vec![])
            .await
            .unwrap();
        Ok(())
    }

    pub async fn update_user(&self, pool: &DB, user: UserBO) -> Result<(), rbatis::Error> {
        let id = user.id;
        let _result = pool
            .exec(
                "UPDATE users SET username = ? where id = ?",
                vec![to_value!(user.username), to_value!(id)],
            )
            .await
            .unwrap();

        if _result.rows_affected > 0 {
            Ok(())
        } else {
            Err(rbatis::Error::E("Not found".to_string()))
        }
    }
}
