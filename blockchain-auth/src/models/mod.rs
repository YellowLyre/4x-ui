use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 区块链类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockchainType {
    Ton,
    Sui,
    Okx,
}

/// 统一认证请求
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub chain: BlockchainType,
    pub signed_payload: String,
    pub address: String,
    pub timestamp: u64,
}

/// TON登录请求
#[derive(Debug, Deserialize)]
pub struct TonAuthRequest {
    pub proof: TonProof,
    pub address: String,
}

/// TON证明结构
#[derive(Debug, Deserialize)]
pub struct TonProof {
    pub timestamp: u64,
    pub domain: TonDomain,
    pub signature: String,
    pub payload: String,
}

#[derive(Debug, Deserialize)]
pub struct TonDomain {
    pub value: String,
}

/// SUI登录请求
#[derive(Debug, Deserialize)]
pub struct SuiAuthRequest {
    pub signed_tx: String,
    pub address: String,
    pub signature: String,
}

/// OKX登录请求
#[derive(Debug, Deserialize)]
pub struct OkxAuthRequest {
    pub signature: String,
    pub timestamp: String,
    pub api_key: String,
    pub address: String,
}

/// 认证响应
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub address: String,
    pub chain: BlockchainType,
    pub expires_at: DateTime<Utc>,
    pub message: String,
}

/// JWT载荷
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,      // 钱包地址
    pub chain: String,    // 区块链类型
    pub exp: usize,       // 过期时间
    pub iat: usize,       // 签发时间
}

/// 错误类型
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Invalid domain")]
    InvalidDomain,
    #[error("Chain not supported: {0}")]
    UnsupportedChain(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("JWT error: {0}")]
    JwtError(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
} 