use axum::Json;
use serde_json::json;

pub async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "data": "Hello World"
    }))
}
