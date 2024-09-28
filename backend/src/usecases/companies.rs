use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
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
        self.repository.find_all().await.map_err(|e| match e {
            RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
            RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
        })
    }

    pub async fn get_company_by_id(&self, id: &str) -> Result<Option<CompanyInDB>, AppError> {
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

    pub async fn create_company(&self, company: CompanyCreate) -> Result<ObjectId, AppError> {
        self.repository
            .insert_one(company)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }

    pub async fn update_company_by_id(
        &self,
        id: &ObjectId,
        company: &CompanyUpdate,
    ) -> Result<bool, AppError> {
        // 既存のドキュメントが存在するか確認
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
                "更新対象の企業が見つかりません".to_string(),
            ));
        }

        self.repository
            .update_one(*id, company)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })
    }
}
