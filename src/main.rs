pub mod drivers;
pub mod entities;
pub mod repository;
pub mod server;
pub mod service;

use crate::{
    drivers::db::{init_DB},
    server::api::commands::{
        user::{create_user, delete_user_ids, find_user_by_id, find_user_by_id_no_cache, update_user, create_user_batch, find_user_list },
    },
    service::Service,
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use tower_http::{trace::TraceLayer};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::signal;

pub struct AppState {
    service: Service,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let db = init_DB();

    // Inject a `AppState` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let service = Arc::new(AppState {
        service: Service::new(db),
    });

    // build our application with a route
    let app = Router::new()
        // .route("/", get(hello_world))
        .route("/users/no_cache/:id", get(find_user_by_id_no_cache))
        .route("/users/cache/:id", get(find_user_by_id))
        .route("/users/list", post(find_user_list))
        .route(
            "/users/:id",
            get(find_user_by_id).delete(delete_user_ids).patch(update_user),
        )
        .route("/users", post(create_user))
        .route("/users/batch", post(create_user_batch))
        .merge(axum_extra::routing::SpaRouter::new("/assets", "dist/assets").index_file("../index.html")) // 静态页面直接复制dist目录到guava同级目录 会匹配首页
        .layer(Extension(service))
        .layer(TraceLayer::new_for_http());

    // run it
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    anyhow::Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}