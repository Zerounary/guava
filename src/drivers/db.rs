use std::env;

use lazy_static::lazy_static;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_pg::options::PgConnectOptions;
use rbdc_sqlite::driver::SqliteDriver;
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

    pub static ref MIGRATE_PATH: String =
        env::var("MIGRATE_PATH").unwrap_or(String::from("./migrations"));
}

pub enum DB_TYPE {
    Mysql,
    Pg,
    Sqlite,
}

pub fn get_db_type() -> DB_TYPE {
    let parsed_db_url = Url::parse(&DATABASE_URL).ok();
    match parsed_db_url {
        Some(url) => match url.scheme() {
            "postgres" => DB_TYPE::Pg,
            "mysql" => DB_TYPE::Mysql,
            "sqlite" => DB_TYPE::Sqlite,
            _ => panic!("unsupport database"),
        },
        None => {
            panic!("Incorrect database url")
        }
    }
}

// 自动选择用数据库驱动
pub fn init_db() -> Rbatis {
    let db = Rbatis::new();

    match get_db_type() {
        DB_TYPE::Pg => db.init(PgDriver {}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Mysql => db.init(MysqlDriver {}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Sqlite => db.init(SqliteDriver {}, DATABASE_URL.as_str()).unwrap(),
    };

    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    db
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn migrate() {
    match get_db_type() {
        DB_TYPE::Mysql => {
            let pool = mysql::Pool::new(DATABASE_URL.as_str()).expect("Incorrect database url");
            let mut conn = pool.get_conn().expect("can't connect to mysql.");
            match embedded::migrations::runner().run(&mut conn) {
                Ok(_) => {}
                Err(e) => {
                    panic!("\nDatabase migrate Error: \n{:?}", e.kind());
                }
            }
        }
        DB_TYPE::Pg => {
            println!("Running DB migrations...");
            let (mut client, con) =
                tokio_postgres::connect(DATABASE_URL.as_str(), tokio_postgres::NoTls).await.unwrap();

            tokio::spawn(async move {
                if let Err(e) = con.await {
                    eprintln!("connection error: {}", e);
                }
            });
            let migration_report = embedded::migrations::runner()
                .run_async(&mut client)
                .await.unwrap();

            for migration in migration_report.applied_migrations() {
                println!(
                    "Migration Applied -  Name: {}, Version: {}",
                    migration.name(),
                    migration.version()
                );
            }

            println!("DB migrations finished!");
        }
        DB_TYPE::Sqlite => {
            todo!()
        }
    };
}
