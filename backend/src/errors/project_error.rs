use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("プロジェクトIDが無効です")]
    InvalidId,
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    // 他にエラーがあれば追加
}
