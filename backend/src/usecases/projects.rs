use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::projects::{ProjectCreate, ProjectInDB, ProjectUpdate};
use crate::repositories::projects::ProjectRepository;
use bson::oid::ObjectId;
use std::sync::Arc;
use validator::Validate;

pub struct ProjectUseCase<R: ProjectRepository> {
    repository: Arc<R>,
}

impl<R: ProjectRepository> ProjectUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_project_by_id(&self, id: &str) -> Result<Option<ProjectInDB>, AppError> {
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

    pub async fn create_project(&self, project: ProjectCreate) -> Result<ObjectId, AppError> {
        // バリデーションを実行
        project
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

        self.repository
            .insert_one(project)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }

    pub async fn update_project(
        &self,
        id: &ObjectId,
        project: &ProjectUpdate,
    ) -> Result<bool, AppError> {
        // バリデーションを実行
        project
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

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
                "更新対象のプロジェクトが見つかりません".to_string(),
            ));
        }

        self.repository
            .update_one(*id, project)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }
}
