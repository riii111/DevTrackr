use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkingTimeError {
    #[error("勤怠IDが無効です")]
    InvalidId,
    #[error("無効な勤務時間: 開始時間が終了時間と同じか後になっています")]
    InvalidTimeRange,
    #[error("更新対象の勤怠ドキュメントが見つかりません")]
    NotFound,
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    // 他にエラーがあれば追加
}
