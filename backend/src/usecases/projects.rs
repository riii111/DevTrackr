use crate::errors::ProjectError;
use crate::models::projects::Project;
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

    pub async fn get_project_by_id(&self, id: &str) -> Result<Option<Project>, ProjectError> {
        let object_id = ObjectId::parse_str(id).map_err(|_| ProjectError::InvalidId)?;
        self.repository
            .find_by_id(&object_id)
            .await
            .map_err(ProjectError::DatabaseError)
    }
}
