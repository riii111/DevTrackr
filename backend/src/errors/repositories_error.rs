use thiserror::Error;

// 低レベルの階層用のエラー

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("データベース接続エラー")]
    ConnectionError, // TODO: 削除予定（mongodb::error::Error はDB接続のエラーも含む上に、そもそもdbインスタンスを受け取った段階では発生しないため）
    #[error("データベース接続後のエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}
