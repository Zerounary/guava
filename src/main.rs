mod error;
mod repo;

use std::{net::SocketAddr, sync::Arc};
use repo::user::{DynUserRepo, User, CreateUser};
use serde_json::{Value, json};
use axum::{response::{Json}, routing::{get, post}, Router, extract::Path, Extension};
use uuid::Uuid;

use crate::{error::AppError, repo::user::ExampleUserRepo};


#[tokio::main]
async fn main() {

    // Inject a `UserRepo` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let user_repo = Arc::new(ExampleUserRepo) as DynUserRepo;

    // build our application with a route
    let app = Router::new()
            .route("/", get(handler))
            .route("/users/:id", get(users_show))
            .route("/users", post(users_create))
            .layer(Extension(user_repo));
    
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "data": "Hello World"
    }))
}


async fn users_show(
    Path(user_id): Path<Uuid>,
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