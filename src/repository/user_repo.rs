use super::Repository;
use crate::{
    drivers::db::DB, entities::UserBO, impl_repo_insert, impl_repo_select, impl_repo_update,
};
use itertools::Itertools;
use rbs::to_value;

impl Repository {
    pub async fn find_user(&self, pool: &DB, id: i64) -> Result<UserBO, rbatis::Error> {
        // let result = pool
        //     .fetch_decode("SELECT * FROM \"user\" WHERE id = ?", vec![to_value!(id)])
        //     .await;

        let result = self.select_user_by_id(&pool, id).await;

        match result {
            Ok(uesr_bos) => {
                if uesr_bos.is_empty() {
                    Err(rbatis::Error::E("Not Found!".to_string()))
                } else {
                    match uesr_bos.first() {
                        Some(user_bo) => Ok(user_bo.to_owned()),
                        None => Err(rbatis::Error::E("Not Found!".to_string())),
                    }
                }
            }
            Err(_) => Err(rbatis::Error::E("Not Found!".to_string())),
        }
    }

    pub async fn delete_user(&self, pool: &DB, id: i64) -> Result<(), rbatis::Error> {
        pool.fetch("DELETE FROM \"user\" where id = ?", vec![to_value!(id)])
            .await
            .unwrap();
        Ok(())
    }

    pub async fn delete_user_ids(&self, pool: &DB, ids: Vec<i64>) -> Result<(), rbatis::Error> {
        if ids.is_empty() {
            return Ok(());
        }
        let sql = format!(
            "DELETE FROM \"user\" where id in ({})",
            ids.iter().join(",")
        );
        pool.fetch(sql.as_str(), vec![]).await.unwrap();
        Ok(())
    }
}

impl_repo_select!(UserBO{select_user_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_update!(UserBO{update_user_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_insert!(UserBO, create_user, create_user_batch);
