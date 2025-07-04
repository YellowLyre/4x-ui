use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber;

mod auth;
mod handlers;
mod models;
mod middleware;
mod ton;
mod sui;
mod okx;
mod utils;

use handlers::{auth_handler, health_handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // 加载环境变量
    dotenv::dotenv().ok();

    // 构建路由
    let app = Router::new()
        .route("/health", get(health_handler::health_check))
        .route("/auth/ton", post(auth_handler::ton_login))
        .route("/auth/sui", post(auth_handler::sui_login))
        .route("/auth/okx", post(auth_handler::okx_login))
        .route("/auth/verify", post(auth_handler::verify_token))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        );

    // 启动服务器
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid port number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("🚀 Blockchain Auth Service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
