use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
use crate::repositories::working_times::WorkingTimeRepository;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct WorkingTimeUseCase<R: WorkingTimeRepository> {
    repository: Arc<R>,
}

impl<R: WorkingTimeRepository> WorkingTimeUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_working_time_by_id(
        &self,
        id: &str,
    ) -> Result<Option<WorkingTimeInDB>, AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId)?;

        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(db_err) => AppError::DatabaseError(db_err),
                RepositoryError::InvalidId => AppError::InvalidId,
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

        self.repository
            .insert_one(&working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(db_err) => AppError::DatabaseError(db_err),
                RepositoryError::InvalidId => AppError::InvalidId,
            })
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
                RepositoryError::DatabaseError(db_err) => AppError::DatabaseError(db_err),
                RepositoryError::InvalidId => AppError::InvalidId,
            })?
            .is_none()
        {
            return Err(AppError::NotFound);
        }

        self.repository
            .update_one(*id, working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(db_err) => AppError::DatabaseError(db_err),
                RepositoryError::InvalidId => AppError::InvalidId,
            })
    }
}
