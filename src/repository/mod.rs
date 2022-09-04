pub mod user_repo;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
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
                #[rbatis::py_sql("`update \"${table_name}\" set  `
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
                  let mut table_name = rbatis::utils::string_util::to_snake_name(stringify!($table));
                  table_name = table_name.trim_end_matches("_bo").to_string(); // remove suffix _bo
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
