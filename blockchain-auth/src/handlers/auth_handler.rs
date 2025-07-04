use axum::{
    extract::Json,
    response::{Json as ResponseJson, IntoResponse},
    http::StatusCode,
};
use chrono::{Duration, Utc};
use tracing::{info, error};

use crate::models::{
    AuthResponse, TonAuthRequest, SuiAuthRequest, OkxAuthRequest, 
    BlockchainType, JwtClaims, AuthError
};
use crate::ton::verify_ton_login;
use crate::sui::verify_sui_login;
use crate::okx::verify_okx_login;
use crate::utils::jwt::generate_jwt;

/// TON钱包登录
pub async fn ton_login(
    Json(payload): Json<TonAuthRequest>
) -> impl IntoResponse {
    info!("TON login attempt from address: {}", payload.address);
    
    match verify_ton_login(&payload.proof, &payload.address).await {
        Ok(()) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            match generate_jwt(&payload.address, "ton", expires_at) {
                Ok(token) => {
                    let response = AuthResponse {
                        success: true,
                        token: Some(token),
                        address: payload.address.clone(),
                        chain: BlockchainType::Ton,
                        expires_at,
                        message: "TON login successful".to_string(),
                    };
                    (StatusCode::OK, ResponseJson(response))
                }
                Err(e) => {
                    error!("JWT generation failed: {}", e);
                    let response = AuthResponse {
                        success: false,
                        token: None,
                        address: payload.address,
                        chain: BlockchainType::Ton,
                        expires_at: Utc::now(),
                        message: format!("JWT generation failed: {}", e),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(response))
                }
            }
        }
        Err(e) => {
            error!("TON verification failed: {}", e);
            let response = AuthResponse {
                success: false,
                token: None,
                address: payload.address,
                chain: BlockchainType::Ton,
                expires_at: Utc::now(),
                message: format!("TON verification failed: {}", e),
            };
            (StatusCode::UNAUTHORIZED, ResponseJson(response))
        }
    }
}

/// SUI钱包登录
pub async fn sui_login(
    Json(payload): Json<SuiAuthRequest>
) -> impl IntoResponse {
    info!("SUI login attempt from address: {}", payload.address);
    
    match verify_sui_login(&payload.signed_tx, &payload.address).await {
        Ok(()) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            match generate_jwt(&payload.address, "sui", expires_at) {
                Ok(token) => {
                    let response = AuthResponse {
                        success: true,
                        token: Some(token),
                        address: payload.address.clone(),
                        chain: BlockchainType::Sui,
                        expires_at,
                        message: "SUI login successful".to_string(),
                    };
                    (StatusCode::OK, ResponseJson(response))
                }
                Err(e) => {
                    error!("JWT generation failed: {}", e);
                    let response = AuthResponse {
                        success: false,
                        token: None,
                        address: payload.address,
                        chain: BlockchainType::Sui,
                        expires_at: Utc::now(),
                        message: format!("JWT generation failed: {}", e),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(response))
                }
            }
        }
        Err(e) => {
            error!("SUI verification failed: {}", e);
            let response = AuthResponse {
                success: false,
                token: None,
                address: payload.address,
                chain: BlockchainType::Sui,
                expires_at: Utc::now(),
                message: format!("SUI verification failed: {}", e),
            };
            (StatusCode::UNAUTHORIZED, ResponseJson(response))
        }
    }
}

/// OKX钱包登录
pub async fn okx_login(
    Json(payload): Json<OkxAuthRequest>
) -> impl IntoResponse {
    info!("OKX login attempt from address: {}", payload.address);
    
    match verify_okx_login(&payload.signature, &payload.timestamp, &payload.api_key).await {
        Ok(()) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            match generate_jwt(&payload.address, "okx", expires_at) {
                Ok(token) => {
                    let response = AuthResponse {
                        success: true,
                        token: Some(token),
                        address: payload.address.clone(),
                        chain: BlockchainType::Okx,
                        expires_at,
                        message: "OKX login successful".to_string(),
                    };
                    (StatusCode::OK, ResponseJson(response))
                }
                Err(e) => {
                    error!("JWT generation failed: {}", e);
                    let response = AuthResponse {
                        success: false,
                        token: None,
                        address: payload.address,
                        chain: BlockchainType::Okx,
                        expires_at: Utc::now(),
                        message: format!("JWT generation failed: {}", e),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(response))
                }
            }
        }
        Err(e) => {
            error!("OKX verification failed: {}", e);
            let response = AuthResponse {
                success: false,
                token: None,
                address: payload.address,
                chain: BlockchainType::Okx,
                expires_at: Utc::now(),
                message: format!("OKX verification failed: {}", e),
            };
            (StatusCode::UNAUTHORIZED, ResponseJson(response))
        }
    }
}

/// 验证JWT令牌
pub async fn verify_token(
    Json(payload): Json<serde_json::Value>
) -> impl IntoResponse {
    if let Some(token) = payload.get("token").and_then(|t| t.as_str()) {
        match crate::utils::jwt::verify_jwt(token) {
            Ok(claims) => {
                info!("Token verified for address: {}", claims.sub);
                (StatusCode::OK, ResponseJson(serde_json::json!({
                    "valid": true,
                    "address": claims.sub,
                    "chain": claims.chain,
                    "expires_at": claims.exp
                })))
            }
            Err(e) => {
                error!("Token verification failed: {}", e);
                (StatusCode::UNAUTHORIZED, ResponseJson(serde_json::json!({
                    "valid": false,
                    "error": format!("Token verification failed: {}", e)
                })))
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
            "valid": false,
            "error": "Token not provided"
        })))
    }
} 