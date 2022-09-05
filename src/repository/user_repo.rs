use super::Repository;
use crate::{
    drivers::db::DB, entities::{UserBO, UserOptionBO}, impl_repo_insert, impl_repo_update, impl_repo_select_one, impl_repo_select_list,
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
impl_repo_select_one!(UserBO{select_user_one(code:&str) => "`where code = #{code}`"});
impl_repo_select_list!(UserBO{select_user_list(user:UserOptionBO) => 
    // TODO 此处 py_sql 没有 html_sql 方便，组合条件容易报错，得改
r#"
if user.done != null:
  `where done = #{user.done}`
if user.username != null && user.username != '':
  `where username = #{user.username}`
  "#});

impl_repo_update!(UserBO{update_user_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_insert!(UserBO, create_user, create_user_batch);
