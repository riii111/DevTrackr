use thiserror::Error;

// 低レベルの階層用のエラー

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("データベース接続エラー")]
    ConnectionError,
    #[error("データベース接続後のエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}
