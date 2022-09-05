use super::Repository;
use crate::{
    drivers::db::DB, entities::UserBO, impl_repo_insert, impl_repo_update, impl_repo_select_one,
};
use itertools::Itertools;
use rbs::to_value;

impl Repository {

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

// impl_repo_select!(UserBO{select_user_by_id(id: i64) -> Option => "`where id = #{id}`"});
impl_repo_select_one!(UserBO{select_user_by_id});

impl_repo_update!(UserBO{update_user_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_insert!(UserBO, create_user, create_user_batch);
