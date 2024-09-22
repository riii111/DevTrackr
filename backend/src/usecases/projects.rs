use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::projects::{ProjectCreate, ProjectInDB, ProjectUpdate};
use crate::repositories::projects::ProjectRepository;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct ProjectUseCase<R: ProjectRepository> {
    repository: Arc<R>,
}

impl<R: ProjectRepository> ProjectUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_project_by_id(&self, id: &str) -> Result<Option<ProjectInDB>, AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId)?;
        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(e) => AppError::DatabaseError(e),
                RepositoryError::InvalidId => AppError::InvalidId,
            })
    }

    pub async fn create_project(&self, project: ProjectCreate) -> Result<ObjectId, AppError> {
        self.repository
            .insert_one(project)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(e) => AppError::DatabaseError(e),
                RepositoryError::InvalidId => AppError::InvalidId,
            })
    }

    pub async fn update_project(
        &self,
        id: &ObjectId,
        project: &ProjectUpdate,
    ) -> Result<bool, AppError> {
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
            .update_one(*id, project)
            .await
            .map_err(|e| match e {
                RepositoryError::DatabaseError(e) => AppError::DatabaseError(e),
                RepositoryError::InvalidId => AppError::InvalidId,
            })
    }
}
