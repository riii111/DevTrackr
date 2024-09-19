use thiserror::Error;

// 低レベルの階層用のエラー

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("挿入されたドキュメントのIDが無効です")]
    InvalidId,
    // 他に必要なエラーを追加
}
