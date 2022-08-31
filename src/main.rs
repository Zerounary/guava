
pub mod error;
pub mod service;
pub mod drivers;
pub mod entities;
pub mod repository;
pub mod server;

use std::{net::SocketAddr, sync::Arc, env};
use serde_json::{json};
use axum::{response::{Json, IntoResponse}, routing::{get, post}, Router, extract::Path, Extension, http::StatusCode};
use server::api::model::{CreateUser, UpdateUser};
use service::uesr_service::{CreateUserInput, UpdateUserInput};
use crate::{error::AppError, drivers::db::{DATABASE_URL, DB, }, entities::User, service::Service};

struct AppState {
    service: Service
}

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
    // let service = Arc::new(ExampleUserRepo) as DynUserRepo;
    let service = Arc::new(AppState {
        service: Service::new(db)
    });

    // build our application with a route
    let app = Router::new()
            .route("/", get(handler))
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

async fn handler() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "data": "Hello World"
    }))
}


async fn users_show(
    Path(user_id): Path<i64>,
    Extension(state): Extension<Arc<AppState>>
) -> Result<Json<User>, AppError> {
    let user = state.service.find(user_id).await?;

    Ok(user.into())
}

/// Handler for `POST /users`.
async fn users_create(
    Json(params): Json<CreateUser>,
    Extension(state): Extension<Arc<AppState>>
) -> Result<Json<User>, AppError> {
    let service_input = CreateUserInput {
        username: params.username,
    };
    let user = state.service.create(service_input).await?;

    Ok(user.into())
}

async fn users_delete(Path(id): Path<i64>, Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    match state.service.delete(id).await {
        Ok(_) => StatusCode::OK,
        Err(_e) => StatusCode::NOT_FOUND,
    } 
}

async fn users_update(
    Path(id): Path<i64>,
    Json(mut user): Json<UpdateUser>,
    Extension(state): Extension<Arc<AppState>>
) -> Result<Json<User>, AppError> {
    user.id = Some(id);
    let service_input = UpdateUserInput {
        id,
        username: user.username,
        done: user.done,
    };
    let user = state.service.update(service_input).await?;
    Ok(Json(user))
}
