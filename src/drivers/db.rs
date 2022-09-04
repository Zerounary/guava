use std::env;

use lazy_static::lazy_static;
use rbatis::Rbatis;
use rbdc_pg::options::PgConnectOptions;

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
