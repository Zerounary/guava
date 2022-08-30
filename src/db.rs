use std::env;

use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};

lazy_static! {
        // connect database
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
    pub static ref DB_POOL: PgPool = 
        PgPoolOptions::new()
            .max_connections(20)
            .connect_lazy(DATABASE_URL.as_str())
            .expect("Could not connect to database");
}
