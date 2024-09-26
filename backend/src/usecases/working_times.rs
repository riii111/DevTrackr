// use crate::adapters::async_queue_adapter::AsyncQueueAdapter;
use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
// use crate::models::projects::{MonthlyWorkingTime, ProjectInDB, ProjectUpdate};
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
// use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::working_times::WorkingTimeRepository;
// use crate::usecases::projects::ProjectUseCase;
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use std::sync::Arc;

// WorkingTimeCreate と WorkingTimeUpdate から共通のフィールドを取り出すヘルパー関数
// fn get_working_time_data(start_time: &BsonDateTime, end_time: &Option<BsonDateTime>) -> i64 {
//     end_time.map_or(0, |end_time| {
//         (end_time.to_chrono() - start_time.to_chrono()).num_seconds()
//     })
// }

pub struct WorkingTimeUseCase<R: WorkingTimeRepository> {
    repository: Arc<R>,
    // queue_adapter: Arc<AsyncQueueAdapter>,
    // project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
}

impl<R: WorkingTimeRepository> WorkingTimeUseCase<R> {
    pub fn new(
        repository: Arc<R>,
        // queue_adapter: Arc<AsyncQueueAdapter>,
        // project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
    ) -> Self {
        Self {
            repository,
            // queue_adapter,
            // project_usecase,
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

        // enqueue前にproject_idをもとにprojectデータを取得
        // let project = self
        //     .project_usecase
        //     .get_project_by_id(&working_time.project_id.to_hex())
        //     .await?
        //     .ok_or(AppError::ValidationError(
        //         "更新対象のプロジェクトが存在しません".to_string(),
        //     ))?;

        // let duration = get_working_time_data(&working_time.start_time, &working_time.end_time);

        // キューにイベントを追加
        // if let Err(e) = self.queue_adapter.enqueue(project, duration).await {
        //     log::error!("Failed to enqueue update event: {}", e);
        // }

        let result = self
            .repository
            .insert_one(working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

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

        // project_idをもとにprojectデータを取得
        // let project = self
        //     .project_usecase
        //     .get_project_by_id(&working_time.project_id.to_hex())
        //     .await?
        //     .ok_or(AppError::ValidationError(
        //         "更新対象のプロジェクトが存在しません".to_string(),
        //     ))?;

        // let duration_diff = get_working_time_data(&working_time.start_time, &working_time.end_time);

        // キューにイベントを追加
        // if let Err(e) = self.queue_adapter.enqueue(project, duration_diff).await {
        //     log::error!("Failed to enqueue update event: {}", e);
        // }

        let result = self
            .repository
            .update_one(*id, working_time)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        Ok(result)
    }

    // pub async fn update_total_working_time(&self, project: &ProjectInDB) -> Result<(), AppError> {
    //     let working_times = self
    //         .repository
    //         .find_all_by_project_id(&project.id.unwrap())
    //         .await
    //         .map_err(|e| match e {
    //             RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
    //             RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
    //         })?;

    //     if working_times.is_empty() {
    //         return Err(AppError::NotFound(
    //             "プロジェクトに関連する稼働時間が見つかりません".to_string(),
    //         ));
    //     }

    //     // 総稼働時間を計算
    //     let total_working_time: i64 = working_times
    //         .iter()
    //         .map(|wt| get_working_time_data(&wt.start_time, &wt.end_time))
    //         .sum();

    //     // プロジェクトを更新
    //     let update = ProjectUpdate {
    //         total_working_time: Some(total_working_time),
    //         // 他のフィールドは更新しない
    //         ..Default::default()
    //     };

    //     self.project_usecase
    //         .update_project(&project.id.unwrap(), &update)
    //         .await?;

    //     Ok(())
    // }
}
