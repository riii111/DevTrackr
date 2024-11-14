use crate::errors::app_error::AppError;
use crate::models::companies::{
    CompanyCreate, CompanyInDB, CompanyUpdate, CompanyWithProjectsInDB,
};
use crate::repositories::companies::CompanyRepository;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct CompanyUseCase<R: CompanyRepository> {
    repository: Arc<R>,
}

impl<R: CompanyRepository> CompanyUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all_companies(&self) -> Result<Vec<CompanyInDB>, AppError> {
        Ok(self.repository.find_all().await?)
    }

    pub async fn get_all_companies_with_projects(
        &self,
    ) -> Result<Vec<CompanyWithProjectsInDB>, AppError> {
        Ok(self.repository.find_all_with_projects().await?)
    }

    pub async fn get_company_by_id(&self, id: &ObjectId) -> Result<Option<CompanyInDB>, AppError> {
        Ok(self.repository.find_by_id(id).await?)
    }

    pub async fn create_company(&self, company: CompanyCreate) -> Result<ObjectId, AppError> {
        Ok(self.repository.insert_one(company).await?)
    }

    pub async fn update_company_by_id(
        &self,
        id: &ObjectId,
        company: &CompanyUpdate,
    ) -> Result<bool, AppError> {
        // 既存のドキュメントが存在するか確認
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound(
                "更新対象の企業が見つかりません".to_string(),
            ));
        }

        Ok(self.repository.update_one(*id, company).await?)
    }
}
