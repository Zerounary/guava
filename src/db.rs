use lazy_static::lazy_static;
use sqlx::{SqlitePool};

lazy_static! {
        // connect database
    pub static ref DB_POOL: SqlitePool = SqlitePool::connect_lazy("sqlite:./guava.db").expect("Could not connect to database");
}
