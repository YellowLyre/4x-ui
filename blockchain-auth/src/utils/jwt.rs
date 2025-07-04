use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{DateTime, Utc};
use crate::models::{JwtClaims, AuthError};

/// 生成JWT令牌
pub fn generate_jwt(
    address: &str,
    chain: &str,
    expires_at: DateTime<Utc>
) -> Result<String, AuthError> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-change-in-production".to_string());
    
    let now = Utc::now();
    let claims = JwtClaims {
        sub: address.to_string(),
        chain: chain.to_string(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    )
    .map_err(|e| AuthError::JwtError(e.to_string()))
}

/// 验证JWT令牌
pub fn verify_jwt(token: &str) -> Result<JwtClaims, AuthError> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-change-in-production".to_string());
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| AuthError::JwtError(e.to_string()))
} 