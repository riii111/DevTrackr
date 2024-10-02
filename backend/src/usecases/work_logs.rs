use crate::dto::requests::work_logs::{WorkLogsCreateRequest, WorkLogsUpdateRequest};
use crate::errors::app_error::AppError;
use crate::models::projects::ProjectUpdate;
use crate::models::work_logs::WorkLogsInDB;
use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::work_logs::WorkLogsRepository;
use crate::usecases::projects::ProjectUseCase;
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use std::sync::Arc;
use tokio::try_join;

// WorkLogsCreate と WorkLogsUpdate から共通のフィールドを取り出すヘルパー関数
fn calculate_working_duration(start_time: &BsonDateTime, end_time: &Option<BsonDateTime>) -> i64 {
    end_time.map_or(0, |end_time| {
        (end_time.to_chrono() - start_time.to_chrono()).num_seconds()
    })
}

pub struct WorkLogsUseCase<R: WorkLogsRepository> {
    repository: Arc<R>,
    project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
}

impl<R: WorkLogsRepository> WorkLogsUseCase<R> {
    pub fn new(
        repository: Arc<R>,
        project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
    ) -> Self {
        Self {
            repository,
            project_usecase,
        }
    }

    pub async fn get_all_work_logs(&self) -> Result<Vec<WorkLogsInDB>, AppError> {
        Ok(self.repository.find_all().await?)
    }

    pub async fn get_work_logs_by_id(&self, id: &str) -> Result<Option<WorkLogsInDB>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

        Ok(self.repository.find_by_id(&object_id).await?)
    }

    pub async fn create_work_logs(
        &self,
        work_logs: &WorkLogsCreateRequest,
    ) -> Result<ObjectId, AppError> {
        let project_id_str = work_logs.project_id.to_string();
        // プロジェクトの取得と勤怠時間の作成を並行して実行
        let (project, inserted_id) = try_join!(
            self.project_usecase.get_project_by_id(&project_id_str),
            async { Ok(self.repository.insert_one(work_logs).await?) }
        )?;

        let associated_project = project.ok_or_else(|| {
            AppError::NotFound("勤怠に関連するプロジェクトが見つかりません".to_string())
        })?;

        // 最新の総稼働時間を計算
        let diff_working_time =
            calculate_working_duration(&work_logs.start_time, &work_logs.end_time);
        let updated_total_working_time = associated_project.total_working_time + diff_working_time;
        // 計算した総稼働時間をプロジェクトに反映して更新
        let project_update = ProjectUpdate {
            total_working_time: updated_total_working_time,
            ..ProjectUpdate::from(associated_project)
        };
        self.project_usecase
            .update_project(&work_logs.project_id, &project_update)
            .await?;

        Ok(inserted_id)
    }

    pub async fn update_work_logs(
        &self,
        id: &ObjectId,
        work_logs: &WorkLogsUpdateRequest,
    ) -> Result<bool, AppError> {
        let project_id_str = work_logs.project_id.to_string();

        // プロジェクトの取得と勤怠時間の更新を並行して実行
        let (project, _) = try_join!(
            self.project_usecase.get_project_by_id(&project_id_str),
            async { Ok(self.repository.update_one(*id, work_logs).await?) }
        )?;

        let associated_project = project.ok_or_else(|| {
            AppError::NotFound("勤怠に関連するプロジェクトが見つかりません".to_string())
        })?;

        // 最新の総稼働時間を計算
        let diff_working_time =
            calculate_working_duration(&work_logs.start_time, &work_logs.end_time);
        let updated_total_working_time = associated_project.total_working_time + diff_working_time;
        // 計算した総稼働時間をプロジェクトに反映して更新
        let project_update = ProjectUpdate {
            total_working_time: updated_total_working_time,
            ..ProjectUpdate::from(associated_project)
        };
        self.project_usecase
            .update_project(&work_logs.project_id, &project_update)
            .await?;

        Ok(true)
    }
}
