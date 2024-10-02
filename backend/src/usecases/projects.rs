use crate::errors::app_error::AppError;
use crate::models::projects::{ProjectCreate, ProjectFilter, ProjectInDB, ProjectUpdate};
use crate::repositories::companies::MongoCompanyRepository;
use crate::repositories::projects::ProjectRepository;
use crate::usecases::companies::CompanyUseCase;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct ProjectUseCase<R: ProjectRepository> {
    repository: Arc<R>,
    company_usecase: Arc<CompanyUseCase<MongoCompanyRepository>>,
}

impl<R: ProjectRepository> ProjectUseCase<R> {
    pub fn new(
        repository: Arc<R>,
        company_usecase: Arc<CompanyUseCase<MongoCompanyRepository>>,
    ) -> Self {
        Self {
            repository,
            company_usecase,
        }
    }

    /// プロジェクトを検索し、条件に一致するプロジェクトを取得する。
    /// パラメータが `None` の場合は全てのプロジェクトを取得する。
    pub async fn search_projects(
        &self,
        filter: Option<ProjectFilter>,
        limit: Option<i64>,
        offset: Option<u64>,
        sort: Option<Vec<(String, i8)>>,
    ) -> Result<Vec<ProjectInDB>, AppError> {
        Ok(self
            .repository
            .find_many(filter, limit, offset, sort)
            .await?)
    }

    pub async fn get_project_by_id(&self, id: &str) -> Result<Option<ProjectInDB>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;
        Ok(self.repository.find_by_id(&object_id).await?)
    }

    pub async fn create_project(&self, project: ProjectCreate) -> Result<ObjectId, AppError> {
        self.company_usecase
            .get_company_by_id(&project.company_id.to_string())
            .await?
            .ok_or_else(|| {
                AppError::NotFound("プロジェクトに関連する企業が見つかりません".to_string())
            })?;
        Ok(self.repository.insert_one(project).await?)
    }

    pub async fn update_project(
        &self,
        id: &ObjectId,
        project: &ProjectUpdate,
    ) -> Result<bool, AppError> {
        // 既存のドキュメントが存在するか
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound(
                "更新対象のプロジェクトが見つかりません".to_string(),
            ));
        }
        self.company_usecase
            .get_company_by_id(&project.company_id.to_string())
            .await?
            .ok_or_else(|| {
                AppError::NotFound("プロジェクトに関連する企業が見つかりません".to_string())
            })?;

        Ok(self.repository.update_one(*id, project).await?)
    }
}
