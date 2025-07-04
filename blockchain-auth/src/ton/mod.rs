use crate::models::{TonProof, AuthError};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error};

/// 验证TON登录
pub async fn verify_ton_login(proof: &TonProof, address: &str) -> Result<(), AuthError> {
    debug!("Starting TON login verification for address: {}", address);
    
    // 1. 校验域名与时效性（防重放攻击）
    validate_domain(&proof.domain.value)?;
    validate_timestamp(proof.timestamp)?;
    
    // 2. 构建签名消息
    let message = format!("{}{}{}", 
        proof.timestamp, 
        proof.domain.value, 
        proof.payload
    );
    debug!("Constructed message for verification: {}", message);
    
    // 3. 验证签名
    verify_ton_signature(&message, &proof.signature, address).await
}

/// 验证域名
fn validate_domain(domain: &str) -> Result<(), AuthError> {
    // 这里可以配置允许的域名列表
    let allowed_domains = vec![
        "4x-ui.local",
        "localhost",
        "127.0.0.1",
        // 添加你的生产域名
    ];
    
    if allowed_domains.iter().any(|&d| domain.contains(d)) {
        Ok(())
    } else {
        error!("Invalid domain: {}", domain);
        Err(AuthError::InvalidDomain)
    }
}

/// 验证时间戳（防重放攻击）
fn validate_timestamp(timestamp: u64) -> Result<(), AuthError> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 允许5分钟的时间差
    let time_window = 300; // 5 minutes
    
    if timestamp > current_time + time_window || timestamp < current_time - time_window {
        error!("Invalid timestamp: {} (current: {})", timestamp, current_time);
        Err(AuthError::InvalidTimestamp)
    } else {
        Ok(())
    }
}

/// 验证TON签名
async fn verify_ton_signature(
    message: &str,
    signature: &str,
    address: &str,
) -> Result<(), AuthError> {
    debug!("Verifying TON signature for address: {}", address);
    
    // 解码签名
    let signature_bytes = hex::decode(signature)
        .map_err(|e| {
            error!("Failed to decode signature: {}", e);
            AuthError::InvalidSignature
        })?;
    
    if signature_bytes.len() != 64 {
        error!("Invalid signature length: {}", signature_bytes.len());
        return Err(AuthError::InvalidSignature);
    }
    
    // 获取TON地址对应的公钥
    let public_key = get_public_key_from_address(address).await?;
    
    // 验证Ed25519签名
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|e| {
            error!("Failed to create verifying key: {}", e);
            AuthError::InvalidSignature
        })?;
    
    let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());
    
    match verifying_key.verify(message.as_bytes(), &signature) {
        Ok(()) => {
            debug!("TON signature verification successful");
            Ok(())
        }
        Err(e) => {
            error!("TON signature verification failed: {}", e);
            Err(AuthError::InvalidSignature)
        }
    }
}

/// 从TON地址获取公钥
async fn get_public_key_from_address(address: &str) -> Result<[u8; 32], AuthError> {
    debug!("Getting public key for TON address: {}", address);
    
    // 方法1: 通过TON API获取公钥
    match get_public_key_from_ton_api(address).await {
        Ok(public_key) => return Ok(public_key),
        Err(e) => {
            debug!("Failed to get public key from TON API: {}", e);
        }
    }
    
    // 方法2: 从地址解析公钥（如果是简单钱包格式）
    parse_public_key_from_address(address)
}

/// 通过TON API获取公钥
async fn get_public_key_from_ton_api(address: &str) -> Result<[u8; 32], AuthError> {
    let client = reqwest::Client::new();
    let ton_api_url = std::env::var("TON_API_ENDPOINT")
        .unwrap_or_else(|_| "https://testnet.toncenter.com/api/v2/".to_string());
    
    let url = format!("{}getAddressInformation?address={}", ton_api_url, address);
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(AuthError::NetworkError(format!(
            "TON API request failed with status: {}", 
            response.status()
        )));
    }
    
    let response_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;
    
    // 解析响应获取公钥
    if let Some(data) = response_data.get("result") {
        if let Some(code) = data.get("code") {
            // 从智能合约代码中提取公钥
            // 这是一个简化的实现，实际情况可能需要更复杂的解析
            return parse_public_key_from_contract_code(code.as_str().unwrap_or(""));
        }
    }
    
    Err(AuthError::VerificationFailed("Cannot extract public key from TON API".to_string()))
}

/// 从地址解析公钥（简化实现）
fn parse_public_key_from_address(address: &str) -> Result<[u8; 32], AuthError> {
    // 这是一个简化的实现
    // 实际的TON地址解析需要更复杂的逻辑
    
    // 移除地址前缀和后缀
    let clean_address = address.replace("EQ", "").replace("UQ", "");
    
    // 尝试从base64解码
    let decoded = base64::decode(&clean_address)
        .map_err(|_| AuthError::VerificationFailed("Invalid address format".to_string()))?;
    
    if decoded.len() >= 32 {
        let mut public_key = [0u8; 32];
        public_key.copy_from_slice(&decoded[0..32]);
        Ok(public_key)
    } else {
        Err(AuthError::VerificationFailed("Address too short to contain public key".to_string()))
    }
}

/// 从合约代码解析公钥
fn parse_public_key_from_contract_code(code: &str) -> Result<[u8; 32], AuthError> {
    // 这是一个占位符实现
    // 实际的TON智能合约代码解析需要更复杂的逻辑
    
    if code.len() >= 64 {
        let hex_key = &code[code.len()-64..];
        let key_bytes = hex::decode(hex_key)
            .map_err(|_| AuthError::VerificationFailed("Cannot parse public key from contract code".to_string()))?;
        
        if key_bytes.len() == 32 {
            let mut public_key = [0u8; 32];
            public_key.copy_from_slice(&key_bytes);
            Ok(public_key)
        } else {
            Err(AuthError::VerificationFailed("Invalid public key length in contract code".to_string()))
        }
    } else {
        Err(AuthError::VerificationFailed("Contract code too short".to_string()))
    }
} 