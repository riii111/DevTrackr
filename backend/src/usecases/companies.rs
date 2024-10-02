use crate::errors::app_error::AppError;
use crate::models::companies::{CompanyCreate, CompanyInDB, CompanyUpdate};
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

    pub async fn get_company_by_id(&self, id: &str) -> Result<Option<CompanyInDB>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;
        Ok(self.repository.find_by_id(&object_id).await?)
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
