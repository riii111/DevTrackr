use crate::errors::repositories_error::RepositoryError;
use crate::models::projects::{ProjectCreate, ProjectInDB, ProjectUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait ProjectRepository {
    // TODO: find_oneだけに集約させるべき？
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<ProjectInDB>, RepositoryError>;

    async fn insert_one(&self, project: ProjectCreate) -> Result<ObjectId, RepositoryError>;

    async fn update_one(
        &self,
        id: ObjectId,
        project: &ProjectUpdate,
    ) -> Result<bool, RepositoryError>;
}

pub struct MongoProjectRepository {
    collection: Collection<ProjectInDB>,
}

impl MongoProjectRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("projects"),
        }
    }
}

#[async_trait]
impl ProjectRepository for MongoProjectRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<ProjectInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(&self, project: ProjectCreate) -> Result<ObjectId, RepositoryError> {
        let project_in_db = ProjectInDB {
            id: None, // MongoDBにID生成を任せる
            title: project.title,
            description: project.description,
            company_name: project.company_name,
            status: project.status,
            total_working_time: None,
            skill_labels: project.skill_labels,
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self.collection.insert_one(&project_in_db).await?;
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
        project: &ProjectUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(&project)
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
