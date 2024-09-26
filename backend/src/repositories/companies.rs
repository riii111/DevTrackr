use crate::errors::repositories_error::RepositoryError;
use crate::models::companies::{CompanyCreate, CompanyInDB, CompanyUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait CompanyRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<CompanyInDB>, RepositoryError>;

    async fn insert_one(&self, company: CompanyCreate) -> Result<ObjectId, RepositoryError>;

    async fn update_one(
        &self,
        id: ObjectId,
        company: &CompanyUpdate,
    ) -> Result<bool, RepositoryError>;
}

pub struct MongoCompanyRepository {
    collection: Collection<CompanyInDB>,
}

impl MongoCompanyRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("companies"),
        }
    }
}

#[async_trait]
impl CompanyRepository for MongoCompanyRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<CompanyInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(&self, company: CompanyCreate) -> Result<ObjectId, RepositoryError> {
        let company_in_db = CompanyInDB {
            id: None, // MongoDBにID生成を任せる
            common: company.common,
            affiliation_start_date: company.affiliation_start_date,
            affiliation_end_date: company.affiliation_end_date,
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self
            .collection
            .insert_one(&company_in_db)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        result
            .inserted_id
            .as_object_id()
            .ok_or(RepositoryError::DatabaseError(MongoError::custom(
                "挿入されたドキュメントのIDが無効です",
            )))
    }

    async fn update_one(
        &self,
        id: ObjectId,
        company: &CompanyUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(&company)
            .map_err(|e| RepositoryError::DatabaseError(MongoError::custom(e)))?;
        update_doc.insert("updated_at", BsonDateTime::now());
        let update = doc! {
            "$set": update_doc
        };
        let result = self
            .collection
            .update_one(doc! { "_id": id }, update)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        Ok(result.modified_count > 0)
    }
}
