use super::Repository;
use crate::{
    drivers::db::DB, entities::{UserBO, UserOptionBO}, impl_repo_insert, impl_repo_update, impl_repo_select_one, impl_repo_select_list, impl_repo_delete,
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

}

// impl_repo_select!(UserBO{select_user_by_id(id: i64) -> Option => "`where id = #{id}`"});
impl_repo_select_one!(UserBO{select_user_by_id});
impl_repo_select_one!(UserBO{select_user_one(code:&str) => "`where code = #{code}`"});
impl_repo_select_list!(UserBO{select_user_list(user:UserOptionBO) => 
    // 此处 可以用 py_sql 和 html_sql 对比使用
r#"
where:
  if user.done != null:
    `and done = #{user.done}`
  if user.username != null && user.username != '':
    `and username = #{user.username}`
  "#});

impl_repo_update!(UserBO{update_user_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_insert!(UserBO, create_user, create_user_batch);

impl_repo_delete!(UserBO{delete_user_ids});