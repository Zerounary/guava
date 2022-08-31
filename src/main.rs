mod error;
mod repo;
mod db;

use std::{net::SocketAddr, sync::Arc, env};
use anyhow::Context;
use repo::user::{DynUserRepo, User, CreateUser, UpdateUser};
use serde_json::{Value, json};
use axum::{response::{Json, IntoResponse}, routing::{get, post}, Router, extract::Path, Extension, http::StatusCode};
use sqlx::{migrate::MigrateDatabase};
use crate::{error::AppError, repo::user::ExampleUserRepo, db::{DB_POOL, DATABASE_URL, DB}};


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv::dotenv().unwrap();

    // let db = SqlitePoolOptions::new()
    //         .max_connections(20)
    //         .connect("sqlite:./guava.db")
    //         .await
    //         .context("failed to connect to DATABASE_URL");


    let db = DB::connect(DATABASE_URL.as_str()).await?;

    sqlx::migrate!().run(&db).await?;

    // Inject a `UserRepo` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let user_repo = Arc::new(ExampleUserRepo) as DynUserRepo;

    // build our application with a route
    let app = Router::new()
            .route("/", get(handler))
            .route("/users/:id", get(users_show).delete(users_delete).patch(users_update))
            .route("/users", post(users_create))
            .layer(Extension(user_repo));
    
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

async fn handler() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "data": "Hello World"
    }))
}


async fn users_show(
    Path(user_id): Path<i64>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.find(user_id).await?;

    Ok(user.into())
}

/// Handler for `POST /users`.
async fn users_create(
    Json(params): Json<CreateUser>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.create(params).await?;

    Ok(user.into())
}

async fn users_delete(Path(id): Path<i64>, Extension(user_repo): Extension<DynUserRepo>) -> impl IntoResponse {
    match user_repo.delete(id).await {
        Ok(_) => StatusCode::OK,
        Err(e) => StatusCode::NOT_FOUND,
    } 
}

async fn users_update(
    Path(id): Path<i64>,
    Json(mut user): Json<UpdateUser>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    user.id = Some(id);
    let user = user_repo.update(user).await?;
    Ok(Json(user))
}
