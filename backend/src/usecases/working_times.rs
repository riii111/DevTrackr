use crate::errors::WorkingTimeError;
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
    ) -> Result<Option<WorkingTimeInDB>, WorkingTimeError> {
        let object_id = ObjectId::parse_str(id).map_err(|_| WorkingTimeError::InvalidId)?;

        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(WorkingTimeError::DatabaseError)
    }

    pub async fn create_working_time(
        &self,
        working_time: &WorkingTimeCreate,
    ) -> Result<ObjectId, WorkingTimeError> {
        // バリデーションチェック
        if working_time.start_time >= working_time.end_time {
            return Err(WorkingTimeError::InvalidTimeRange);
        }

        self.repository
            .insert_one(&working_time)
            .await
            .map_err(WorkingTimeError::DatabaseError)
    }

    pub async fn update_working_time(
        &self,
        id: &ObjectId,
        working_time: &WorkingTimeUpdate,
    ) -> Result<bool, WorkingTimeError> {
        // バリデーションチェック
        if working_time.start_time >= working_time.end_time {
            return Err(WorkingTimeError::InvalidTimeRange);
        }

        // 既存のドキュメントが存在するか
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(WorkingTimeError::NotFound);
        }

        self.repository
            .update_one(*id, working_time)
            .await
            .map_err(WorkingTimeError::DatabaseError)
    }
}
