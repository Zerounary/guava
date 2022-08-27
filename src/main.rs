use std::net::SocketAddr;
use serde_json::{Value, json};
use axum::{response::Json, routing::get, Router};


#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

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
