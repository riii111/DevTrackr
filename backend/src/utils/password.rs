use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let result = verify(password, hash).unwrap_or(false);
    log::info!("パスワード検証結果: {}", result);
    result
}
