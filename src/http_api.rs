use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::database::Database;

#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
    data: Option<serde_json::Value>,
    message: Option<String>,
}

#[derive(Deserialize)]
pub struct SetRequest {
    value: String,
    ttl: Option<u64>,
}

#[derive(Deserialize)]
pub struct ListPushRequest {
    value: String,
}

#[derive(Deserialize)]
pub struct SetAddRequest {
    value: String,
}

#[derive(Deserialize)]
pub struct ZAddRequest {
    score: f64,
    member: String,
}

pub async fn create_http_server(addr: &str, db: Arc<Database>) {
    let app = Router::new()
        // String operations
        .route("/ping", get(ping))
        .route("/keys/:key", get(get_key))
        .route("/keys/:key", post(set_key))
        .route("/keys/:key", delete(delete_key))
        // List operations
        .route("/lists/:key/lpush", post(lpush))
        .route("/lists/:key/rpush", post(rpush))
        .route("/lists/:key/lpop", post(lpop))
        .route("/lists/:key/rpop", post(rpop))
        .route("/lists/:key/range/:start/:end", get(lrange))
        // Set operations
        .route("/sets/:key/add", post(sadd))
        .route("/sets/:key/remove/:value", delete(srem))
        .route("/sets/:key/members", get(smembers))
        .route("/sets/:key/ismember/:value", get(sismember))
        // Sorted Set operations
        .route("/zsets/:key/add", post(zadd))
        .route("/zsets/:key/remove/:member", delete(zrem))
        .route("/zsets/:key/range/:start/:end", get(zrange))
        .route("/zsets/:key/score/:member", get(zscore))
        .with_state(db);

    println!("HTTP API server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// PING
async fn ping() -> Json<ApiResponse> {
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!("PONG")),
        message: None,
    })
}

// GET key
async fn get_key(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    match db.get(&key).await {
        Some(value) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(value)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Key not found".to_string()),
        }),
    }
}

// SET key
async fn set_key(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
    Json(payload): Json<SetRequest>,
) -> Json<ApiResponse> {
    db.set(key, payload.value, payload.ttl).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!("OK")),
        message: None,
    })
}

// DELETE key
async fn delete_key(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    let deleted = db.delete(&key).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if deleted { 1 } else { 0 })),
        message: None,
    })
}

// LPUSH
async fn lpush(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
    Json(payload): Json<ListPushRequest>,
) -> Json<ApiResponse> {
    let len = db.lpush(key, payload.value).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(len)),
        message: None,
    })
}

// RPUSH
async fn rpush(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
    Json(payload): Json<ListPushRequest>,
) -> Json<ApiResponse> {
    let len = db.rpush(key, payload.value).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(len)),
        message: None,
    })
}

// LPOP
async fn lpop(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    match db.lpop(&key).await {
        Some(value) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(value)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: Some("List empty or not found".to_string()),
        }),
    }
}

// RPOP
async fn rpop(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    match db.rpop(&key).await {
        Some(value) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(value)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: Some("List empty or not found".to_string()),
        }),
    }
}

// LRANGE
async fn lrange(
    State(db): State<Arc<Database>>,
    Path((key, start, end)): Path<(String, i64, i64)>,
) -> Json<ApiResponse> {
    match db.lrange(&key, start, end).await {
        Some(values) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(values)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!([])),
            message: None,
        }),
    }
}

// SADD
async fn sadd(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
    Json(payload): Json<SetAddRequest>,
) -> Json<ApiResponse> {
    let added = db.sadd(key, payload.value).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if added { 1 } else { 0 })),
        message: None,
    })
}

// SREM
async fn srem(
    State(db): State<Arc<Database>>,
    Path((key, value)): Path<(String, String)>,
) -> Json<ApiResponse> {
    let removed = db.srem(&key, value).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if removed { 1 } else { 0 })),
        message: None,
    })
}

// SMEMBERS
async fn smembers(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
) -> Json<ApiResponse> {
    match db.smembers(&key).await {
        Some(members) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(members)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!([])),
            message: None,
        }),
    }
}

// SISMEMBER
async fn sismember(
    State(db): State<Arc<Database>>,
    Path((key, value)): Path<(String, String)>,
) -> Json<ApiResponse> {
    let is_member = db.sismember(&key, &value).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if is_member { 1 } else { 0 })),
        message: None,
    })
}

// ZADD
async fn zadd(
    State(db): State<Arc<Database>>,
    Path(key): Path<String>,
    Json(payload): Json<ZAddRequest>,
) -> Json<ApiResponse> {
    let added = db.zadd(key, payload.score, payload.member).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if added { 1 } else { 0 })),
        message: None,
    })
}

// ZREM
async fn zrem(
    State(db): State<Arc<Database>>,
    Path((key, member)): Path<(String, String)>,
) -> Json<ApiResponse> {
    let removed = db.zrem(&key, member).await;
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!(if removed { 1 } else { 0 })),
        message: None,
    })
}

// ZRANGE
async fn zrange(
    State(db): State<Arc<Database>>,
    Path((key, start, end)): Path<(String, usize, usize)>,
) -> Json<ApiResponse> {
    match db.zrange(&key, start, end).await {
        Some(members) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(members)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!([])),
            message: None,
        }),
    }
}

// ZSCORE
async fn zscore(
    State(db): State<Arc<Database>>,
    Path((key, member)): Path<(String, String)>,
) -> Json<ApiResponse> {
    match db.zscore(&key, &member).await {
        Some(score) => Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!(score)),
            message: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Member not found".to_string()),
        }),
    }
}
