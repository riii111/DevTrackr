use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::DateTime as BsonDateTime;
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

    let now = Utc::now();
    let expiration = now
        .checked_add_signed(Duration::hours(access_token_exp))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now.timestamp() as usize,
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

/// リフレッシュ時の新しいアクセストークン生成
pub fn create_refreshed_access_token(
    user_id: &str,
    secret: &[u8],
) -> Result<(String, BsonDateTime), jsonwebtoken::errors::Error> {
    let access_token_exp = env::var("ACCESS_TOKEN_EXPIRY_HOURS")
        .expect("ACCESS_TOKEN_EXPIRY_HOURSが設定されていません")
        .parse::<i64>()
        .expect("ACCESS_TOKEN_EXPIRY_HOURSは有効な整数である必要があります");

    let now = Utc::now();
    let exp = now + Duration::hours(access_token_exp);

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;

    Ok((token, BsonDateTime::from_chrono(exp)))
}

/// 認証トークンのペアを生成する関数
pub fn create_token_pair(
    user_id: &str,
    secret: &[u8],
) -> Result<(String, String, BsonDateTime, BsonDateTime), jsonwebtoken::errors::Error> {
    let access_token = create_access_token(user_id, secret)?;
    let refresh_token = create_refresh_token(user_id, secret)?;

    let access_token_exp = env::var("ACCESS_TOKEN_EXPIRY_HOURS")
        .expect("ACCESS_TOKEN_EXPIRY_HOURSが設定されていません")
        .parse::<i64>()
        .expect("ACCESS_TOKEN_EXPIRY_HOURSは有効な整数である必要があります");
    let refresh_token_exp = env::var("REFRESH_TOKEN_EXPIRY_DAYS")
        .expect("REFRESH_TOKEN_EXPIRY_DAYSが設定されていません")
        .parse::<i64>()
        .expect("REFRESH_TOKEN_EXPIRY_DAYSは有効な整数である必要があります");

    let expires_at = BsonDateTime::from_chrono(Utc::now() + Duration::hours(access_token_exp));
    let refresh_expires_at =
        BsonDateTime::from_chrono(Utc::now() + Duration::days(refresh_token_exp));

    Ok((access_token, refresh_token, expires_at, refresh_expires_at))
}

/// 認証ヘッダーからトークンを抽出する関数
pub fn extract_token(auth_header: &str) -> String {
    auth_header.trim_start_matches("Bearer ").to_string()
}
