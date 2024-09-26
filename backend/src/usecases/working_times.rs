use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::projects::ProjectUpdate;
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::working_times::WorkingTimeRepository;
use crate::usecases::projects::ProjectUseCase;
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use std::sync::Arc;

// WorkingTimeCreate と WorkingTimeUpdate から共通のフィールドを取り出すヘルパー関数
fn calculate_working_duration(start_time: &BsonDateTime, end_time: &Option<BsonDateTime>) -> i64 {
    end_time.map_or(0, |end_time| {
        (end_time.to_chrono() - start_time.to_chrono()).num_seconds()
    })
}

pub struct WorkingTimeUseCase<R: WorkingTimeRepository> {
    repository: Arc<R>,
    project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
}

impl<R: WorkingTimeRepository> WorkingTimeUseCase<R> {
    pub fn new(
        repository: Arc<R>,
        project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
    ) -> Self {
        Self {
            repository,
            project_usecase,
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
        // 対象のproject_idを取得し、total_working_timeを更新する
        let project = match self
            .project_usecase
            .get_project_by_id(&working_time.project_id.to_string())
            .await?
        {
            Some(p) => p,
            None => {
                return Err(AppError::NotFound(
                    "プロジェクトが見つかりません".to_string(),
                ))
            }
        };
        // 最新の稼働時間を計算
        let diff_working_time =
            calculate_working_duration(&working_time.start_time, &working_time.end_time);
        let updated_total_working_time =
            project.total_working_time.unwrap_or(0) + diff_working_time;
        // 計算した稼働時間をプロジェクトに反映して更新
        let project_update = ProjectUpdate {
            total_working_time: Some(updated_total_working_time),
            // 他のフィールドは更新しない
            ..Default::default()
        };
        self.project_usecase
            .update_project(&working_time.project_id, &project_update)
            .await?;

        self.repository
            .insert_one(working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
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
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?
            .is_none()
        {
            return Err(AppError::NotFound(
                "更新対象の勤怠が見つかりません".to_string(),
            ));
        }

        // 対象のproject_idを取得し、total_working_timeを更新する
        let project = match self
            .project_usecase
            .get_project_by_id(&working_time.project_id.to_string())
            .await?
        {
            Some(p) => p,
            None => {
                return Err(AppError::NotFound(
                    "プロジェクトが見つかりません".to_string(),
                ))
            }
        };
        // 最新の稼働時間を計算
        let diff_working_time =
            calculate_working_duration(&working_time.start_time, &working_time.end_time);
        let updated_total_working_time =
            project.total_working_time.unwrap_or(0) + diff_working_time;
        // 計算した稼働時間をプロジェクトに反映して更新
        let project_update = ProjectUpdate {
            total_working_time: Some(updated_total_working_time),
            // 他のフィールドは更新しない
            ..Default::default()
        };
        self.project_usecase
            .update_project(&working_time.project_id, &project_update)
            .await?;

        self.repository
            .update_one(*id, working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }
}
