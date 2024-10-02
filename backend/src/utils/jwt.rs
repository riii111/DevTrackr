use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ユーザーID(subject)
    pub exp: usize,  // 有効期限(expiration time)
    pub iat: usize,  // 発行時刻(issued at)
}

pub fn create_access_token(
    user_id: &str,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let access_token_exp = env::var("ACCESS_TOKEN_EXPIRY_HOURS")
        .expect("ACCESS_TOKEN_EXPIRY_HOURSが設定されていません")
        .parse::<i64>()
        .expect("ACCESS_TOKEN_EXPIRY_HOURSは有効な整数である必要があります");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(access_token_exp))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn create_refresh_token(
    user_id: &str,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let refresh_token_exp = env::var("REFRESH_TOKEN_EXPIRY_DAYS")
        .expect("REFRESH_TOKEN_EXPIRY_DAYSが設定されていません")
        .parse::<i64>()
        .expect("REFRESH_TOKEN_EXPIRY_DAYSは有効な整数である必要があります");

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(refresh_token_exp))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

// 各トークンを検証し、有効な場合はClaimsを返す関数
pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}
