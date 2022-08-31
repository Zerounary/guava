
pub mod error;
pub mod service;
pub mod drivers;
pub mod entities;
pub mod repository;
pub mod server;

use std::{net::SocketAddr, sync::Arc, env};
use axum::{routing::{get, post}, Router, Extension};
use crate::{drivers::db::{DATABASE_URL, DBOptions, }, service::Service, server::api::commands::{user::{users_update, users_delete, users_show, users_create}, hello::hello_world}};

pub struct AppState {
    service: Service
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv::dotenv().unwrap();

    let db = DBOptions::new()
                .max_connections(20)
                .connect(DATABASE_URL.as_str()).await?;


    sqlx::migrate!().run(&db).await?;

    // Inject a `AppState` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let service = Arc::new(AppState {
        service: Service::new(db)
    });

    // build our application with a route
    let app = Router::new()
            .route("/", get(hello_world))
            .route("/users/:id", get(users_show).delete(users_delete).patch(users_update))
            .route("/users", post(users_create))
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