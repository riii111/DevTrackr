use crate::adapters::async_queue::AsyncQueueAdapter;
use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
use crate::repositories::working_times::WorkingTimeRepository;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct WorkingTimeUseCase<R: WorkingTimeRepository> {
    repository: Arc<R>,
    queue_adapter: AsyncQueueAdapter,
}

impl<R: WorkingTimeRepository> WorkingTimeUseCase<R> {
    pub fn new(repository: Arc<R>, queue_adapter: AsyncQueueAdapter) -> Self {
        Self {
            repository,
            queue_adapter,
        }
    }

    pub async fn get_working_time_by_id(
        &self,
        id: &str,
    ) -> Result<Option<WorkingTimeInDB>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }

    pub async fn create_working_time(
        &self,
        working_time: &WorkingTimeCreate,
    ) -> Result<ObjectId, AppError> {
        // バリデーションチェック
        if let Some(end_time) = working_time.end_time {
            if working_time.start_time >= end_time {
                return Err(AppError::ValidationError(
                    "開始時間は終了時間より前である必要があります".to_string(),
                ));
            }
        }

        let result = self
            .repository
            .insert_one(&working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        // キューにイベントを追加
        if let Err(e) = self
            .queue_adapter
            .enqueue(working_time.project_id, calculate_duration(working_time))
            .await
        {
            log::error!("Failed to enqueue update event: {}", e);
            // ここでエラーを返すかどうかはビジネスロジック次第
        }

        Ok(result)
    }

    pub async fn update_working_time(
        &self,
        id: &ObjectId,
        working_time: &WorkingTimeUpdate,
    ) -> Result<bool, AppError> {
        // バリデーションチェック
        if let Some(end_time) = working_time.end_time {
            if working_time.start_time >= end_time {
                return Err(AppError::ValidationError(
                    "開始時間は終了時間より前である必要があります".to_string(),
                ));
            }
        }
        // 既存のドキュメントが存在するか
        if self
            .repository
            .find_by_id(id)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?
            .is_none()
        {
            return Err(AppError::NotFound(
                "更新対象の勤怠が見つかりません".to_string(),
            ));
        }

        // 対象のプロジェクトが存在するか
        if self
            .repository
            .find_by_id(&working_time.project_id)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?
            .is_none()
        {
            return Err(AppError::ValidationError(
                "更新対象のプロジェクトが存在しません".to_string(),
            ));
        }

        let result = self
            .repository
            .update_one(*id, working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        // キューにイベントを追加（更新の場合）
        if let Err(e) = self
            .queue_adapter
            .enqueue(
                working_time.project_id,
                calculate_duration_diff(working_time),
            )
            .await
        {
            log::error!("Failed to enqueue update event: {}", e);
            // ここでエラーを返すかどうかはビジネスロジック次第
        }

        Ok(result)
    }
}

fn calculate_duration(working_time: &WorkingTimeCreate) -> i64 {
    log::info!("called calculate_duration");
    // 開始時間と終了時間から稼働時間を計算するロジック
    working_time.end_time.map_or(0, |end_time| {
        (end_time.to_chrono() - working_time.start_time.to_chrono()).num_seconds()
    })
}

fn calculate_duration_diff(working_time: &WorkingTimeUpdate) -> i64 {
    log::info!("called calculate_duration_diff");
    // 更新された稼働時間の差分を計算するロジック
    working_time.end_time.map_or(0, |end_time| {
        (end_time.to_chrono() - working_time.start_time.to_chrono()).num_seconds()
    })
}
