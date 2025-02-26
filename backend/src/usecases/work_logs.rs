use crate::errors::app_error::AppError;
use crate::models::projects::ProjectUpdate;
use crate::models::work_logs::{WorkLogCreate, WorkLogInDB, WorkLogUpdate};
use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::work_logs::WorkLogRepository;
use crate::usecases::projects::ProjectUseCase;
use bson::oid::ObjectId;
use std::sync::Arc;
use tokio::try_join;

pub struct WorkLogUseCase<R: WorkLogRepository> {
    repository: Arc<R>,
    project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
}

impl<R: WorkLogRepository> WorkLogUseCase<R> {
    pub fn new(
        repository: Arc<R>,
        project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
    ) -> Self {
        Self {
            repository,
            project_usecase,
        }
    }

    pub async fn get_all_work_logs(&self) -> Result<Vec<WorkLogInDB>, AppError> {
        Ok(self.repository.find_all().await?)
    }

    pub async fn get_work_logs_by_id(
        &self,
        id: &ObjectId,
    ) -> Result<Option<WorkLogInDB>, AppError> {
        Ok(self.repository.find_by_id(id).await?)
    }

    pub async fn create_work_logs(&self, work_logs: &WorkLogCreate) -> Result<ObjectId, AppError> {
        // プロジェクトの取得と勤怠時間の作成を並行して実行
        let (project, inserted_id) = try_join!(
            self.project_usecase
                .get_project_by_id(&work_logs.project_id),
            async { Ok(self.repository.insert_one(work_logs).await?) }
        )?;

        let associated_project = project.ok_or_else(|| {
            AppError::NotFound("勤怠に関連するプロジェクトが見つかりません".to_string())
        })?;

        // 実作業時間を使用して総稼働時間を更新
        let actual_work_minutes = work_logs.actual_work_minutes.unwrap_or(0);
        let updated_total_working_time =
            associated_project.total_working_time + actual_work_minutes as i64 * 60; // 分を秒に変換

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
        work_logs: &WorkLogUpdate,
    ) -> Result<bool, AppError> {
        // 既存の勤怠ドキュメントが存在するか確認
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound(
                "更新対象の勤怠が見つかりません".to_string(),
            ));
        }

        // プロジェクトの取得と勤怠時間の更新を並行して実行
        let (project, _) = try_join!(self.project_usecase.get_project_by_id(id), async {
            Ok(self.repository.update_one(*id, work_logs).await?)
        })?;

        let associated_project = project.ok_or_else(|| {
            AppError::NotFound("勤怠に関連するプロジェクトが見つかりません".to_string())
        })?;

        // 実作業時間を使用して総稼働時間を更新
        let actual_work_minutes = work_logs.actual_work_minutes.unwrap_or(0);
        let updated_total_working_time =
            associated_project.total_working_time + actual_work_minutes as i64 * 60; // 分を秒に変換

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
