use axum::{response::Json, http::StatusCode};
use serde_json::{json, Value};

/// 健康检查端点
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "service": "blockchain-auth",
        "version": "0.1.0",
        "supported_chains": ["ton", "sui", "okx"]
    })))
} 