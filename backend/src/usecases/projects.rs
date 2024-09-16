use crate::models::projects::Project;
use crate::repositories::projects::ProjectRepository;
use bson::oid::ObjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("プロジェクトIDが無効です")]
    InvalidId,
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    // 他にエラーがあれば追加
}

pub struct ProjectUseCase<R: ProjectRepository> {
    repository: R,
}

impl<R: ProjectRepository> ProjectUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>, ProjectError> {
        let object_id = ObjectId::parse_str(id).map_err(|_| ProjectError::InvalidId)?;
        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(ProjectError::DatabaseError)
    }
}
