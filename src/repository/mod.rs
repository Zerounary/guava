pub mod user_repo;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}

pub fn to_sql_table_name(table_name: &str) -> String {
    let name = rbatis::utils::string_util::to_snake_name(&table_name).trim_end_matches("_bo").to_string();
    format!(r#""{}""#, name)
}

#[macro_export]
macro_rules! impl_repo_update {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}) => {
        impl Repository {
            pub async fn $fn_name(
                &self,
                mut rb: &DB,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> Result<(), rbatis::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err(rbatis::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[rbatis::py_sql("`update ${table_name} set  `
                                 trim ',':
                                   for k,v in table:
                                     if k == column || v== null:
                                        continue:
                                     `${k}=#{v},`
                                 ` `",$sql_where)]
                  async fn $fn_name(
                      rb: &mut dyn rbatis::executor::Executor,
                      table_name: String,
                      table: &rbs::Value,
                      $($param_key:$param_type,)*
                  ) -> Result<rbatis::rbdc::db::ExecResult, rbatis::rbdc::Error> {
                      impled!()
                  }
                  let mut table_name = crate::repository::to_sql_table_name(stringify!($table));
                  let table = rbs::to_value!(table);
                  let result = $fn_name(&mut rb, table_name, &table, $($param_key,)*).await;
                match result {
                    Ok(_result) => {
                        if _result.rows_affected > 0 {
                            Ok(())
                        } else {
                            Err(rbatis::Error::E("Not found".to_string()))
                        }
                    },
                    Err(e) => {
                        dbg!(e);
                        Err(rbatis::Error::E("Not found".to_string()))
                    }
                }
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct InsertBatchResult {
    pub rows_affected: u64,
    pub insert_ids: Vec<i64>,
}

#[macro_export]
macro_rules! impl_repo_insert {
    ($table:ty, $insert_fn:ident, $insert_batch_fn:ident) => {
        impl_repo_insert!(
            $table,
            $insert_fn,
            $insert_batch_fn,
            crate::repository::to_sql_table_name(stringify!($table))
        );
    };
    ($table:ty, $insert_fn:ident, $insert_batch_fn:ident, $table_name:expr) => {
        impl Repository {
            pub async fn $insert_batch_fn(
                &self,
                mut rb: &DB,
                tables: &mut [$table],
                batch_size: u64,
            ) -> Result<crate::repository::InsertBatchResult, rbatis::rbdc::Error> {
                #[rbatis::py_sql(
                    "`insert into ${table_name} (`
             trim ',':
               for k,v in tables[0]:
                  if k == 'id' && v== null:
                    continue:
                 ${k},
             `) VALUES `
             trim ',':
              for _,table in tables:
               (
               trim ',':
                for k,v in table:
                  if k == 'id' && v== null:
                     continue:
                  #{v},
               ),
             "
                )]
                async fn insert_batch(
                    rb: &mut dyn rbatis::executor::Executor,
                    tables: &[$table],
                    table_name: &str,
                ) -> Result<rbatis::rbdc::db::ExecResult, rbatis::rbdc::Error> {
                    impled!()
                }
                if tables.is_empty() {
                    return Err(rbatis::rbdc::Error::from(
                        "insert can not insert empty array tables!",
                    ));
                }
                let mut insert_ids = Vec::new();
                for table in tables.iter_mut() {
                    let id = rbatis::plugin::snowflake::new_snowflake_id();
                    table.id = id;
                    insert_ids.push(id);
                }
                let table_name = $table_name.to_string();
                let mut result = rbatis::rbdc::db::ExecResult {
                    rows_affected: 0,
                    last_insert_id: rbs::Value::Null,
                };
                let ranges = rbatis::sql::Page::<()>::into_ranges(tables.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    let exec_result = insert_batch(
                        &mut rb,
                        &tables[offset as usize..limit as usize],
                        table_name.as_str(),
                    )
                    .await?;
                    result.rows_affected += exec_result.rows_affected;
                    result.last_insert_id = exec_result.last_insert_id;
                }

                Ok(crate::repository::InsertBatchResult {
                    rows_affected: result.rows_affected,
                    insert_ids,
                })
            }
        }

        impl Repository {
            pub async fn $insert_fn(
                &self,
                mut rb: &DB,
                mut table: $table,
            ) -> Result<i64, rbatis::Error> {
                let result = self.$insert_batch_fn(rb, &mut [table.clone()], 1).await;
                match result {
                    Ok(insert_result) => {
                        let id = insert_result.insert_ids[0];
                        Ok(id)
                    }
                    Err(_e) => Err(rbatis::Error::E("Not found".to_string())),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_repo_select {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $container:tt => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, mut rb: &DB, $($param_key:$param_type,)*)->Result<$container<$table>, rbatis::rbdc::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<$container<$table>,rbatis::rbdc::Error> {impled!()}
                let mut table_column = "*".to_string();
                let mut table_name = crate::repository::to_sql_table_name(stringify!($table));
                table_name = table_name.trim_end_matches("_bo").to_string(); // remove suffix _bo
                $fn_name(&mut rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
}

#[macro_export]
macro_rules! impl_repo_select_one {
    ($table:ty{$fn_name:ident}) => {
        impl Repository{
            pub async fn $fn_name(&self, mut rb: &DB, id: i64)->Result<$table, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} `", "`where id = #{id}`")]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str, id: i64) -> Result<Option<$table>,rbatis::rbdc::Error> {impled!()}
                let mut table_column = "*".to_string();
                let mut table_name = crate::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(&mut rb,&table_column,&table_name, id).await;
                match result {
                    Ok(bo_option) => {
                        match bo_option {
                            Some(bo) => Ok(bo.to_owned()),
                            None => Err(rbatis::Error::E("Not Found!".to_string())),
                        }
                    }
                    Err(_) => Err(rbatis::Error::E("Not Found!".to_string())),
                }
            }
        }
    };
}
