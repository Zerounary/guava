use std::env;

use lazy_static::lazy_static;
use rbatis::Rbatis;
use rbdc_pg::{driver::PgDriver};
use rbdc_mysql::{driver::MysqlDriver};
use rbdc_sqlite::{driver::SqliteDriver};
use rbdc_pg::options::PgConnectOptions;
use url::Url;

// alias DB pool type
pub type DB = Rbatis;
pub type DBOptions = PgConnectOptions;

lazy_static! {
        // connect database
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
    
    pub static ref MAX_CONNECTIONS: String =
        env::var("MAX_CONNECTIONS").unwrap_or(String::from("200"));
    
}

pub enum DB_TYPE {
    Mysql,
    Pg,
    Sqlite
}

pub fn get_db_type() -> DB_TYPE {
    let parsed_db_url = Url::parse(&DATABASE_URL).ok();
    match parsed_db_url {
        Some(url) => {
            match url.scheme() {
                "postgres" => DB_TYPE::Pg,
                "mysql" => DB_TYPE::Mysql,
                "sqlite" => DB_TYPE::Sqlite,
                _ => panic!("unsupport database")
            }
        },
        None => {
            panic!("uncurrent database url")
        }
    }
}

// 自动选择用数据库驱动
pub fn init_DB() -> Rbatis  {

    let db = Rbatis::new();

    match get_db_type() {
        DB_TYPE::Pg => db.init( PgDriver{}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Mysql => db.init( MysqlDriver{}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Sqlite => db.init( SqliteDriver{}, DATABASE_URL.as_str()).unwrap(),
    };

    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    db
}