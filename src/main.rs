pub mod drivers;
pub mod entities;
pub mod repository;
pub mod server;
pub mod service;

use crate::{
    drivers::db::{DATABASE_URL},
    server::api::commands::{
        hello::hello_world,
        user::{create_user, delete_user_ids, find_user_by_id, find_user_by_id_no_cache, update_user, create_user_batch, find_user_list },
    },
    service::Service,
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use rbatis::Rbatis;
use rbdc_pg::{driver::PgDriver};
use std::{env, net::SocketAddr, sync::Arc};

pub struct AppState {
    service: Service,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let db = Rbatis::new();

    db.init(PgDriver {}, DATABASE_URL.as_str()).unwrap();
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");


    // Inject a `AppState` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let service = Arc::new(AppState {
        service: Service::new(db),
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users/no_cache/:id", get(find_user_by_id_no_cache))
        .route("/users/cache/:id", get(find_user_by_id))
        .route("/users/list", post(find_user_list))
        .route(
            "/users/:id",
            get(find_user_by_id).delete(delete_user_ids).patch(update_user),
        )
        .route("/users", post(create_user))
        .route("/users/batch", post(create_user_batch))
        .layer(Extension(service));

    // run it
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    anyhow::Ok(())
}
