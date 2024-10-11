use crate::errors::repositories_error::RepositoryError;
use crate::models::companies::{
    CompanyCreate, CompanyInDB, CompanyUpdate, CompanyWithProjectsInDB,
};
use crate::models::projects::ProjectInDB;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use futures::TryStreamExt;
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait CompanyRepository {
    async fn find_all(&self) -> Result<Vec<CompanyInDB>, RepositoryError>;

    async fn find_all_with_projects(&self)
        -> Result<Vec<CompanyWithProjectsInDB>, RepositoryError>;

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
    async fn find_all(&self) -> Result<Vec<CompanyInDB>, RepositoryError> {
        let mut companies = Vec::new();
        let mut cursor = self
            .collection
            .find(doc! {}, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        while let Some(result) = cursor
            .try_next()
            .await
            .map_err(RepositoryError::DatabaseError)?
        {
            companies.push(result);
        }

        Ok(companies)
    }

    async fn find_all_with_projects(
        &self,
    ) -> Result<Vec<CompanyWithProjectsInDB>, RepositoryError> {
        let pipeline = vec![doc! {
            "$lookup": {
                "from": "projects",
                "localField": "_id",
                "foreignField": "company_id",
                "as": "projects"
            }
        }];

        let mut cursor = self
            .collection
            .aggregate(pipeline, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        let mut companies_with_projects = Vec::new();

        while let Some(result) = cursor.try_next().await.map_err(|e| {
            log::error!("Error in find_all_with_projects: {}", e);
            RepositoryError::DatabaseError(MongoError::custom(e.to_string()))
        })? {
            let company: CompanyInDB = bson::from_document(result.clone())
                .map_err(|e| RepositoryError::DatabaseError(MongoError::custom(e.to_string())))?;

            let projects: Vec<ProjectInDB> = result
                .get_array("projects")
                .map_err(|e| RepositoryError::DatabaseError(MongoError::custom(e.to_string())))?
                .iter()
                .filter_map(|p| bson::from_bson(p.clone()).ok())
                .collect();

            companies_with_projects.push(CompanyWithProjectsInDB { company, projects });
        }

        Ok(companies_with_projects)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<CompanyInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id }, None)
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
            .insert_one(&company_in_db, None)
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
            .update_one(doc! { "_id": id }, update, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        Ok(result.modified_count > 0)
    }
}
