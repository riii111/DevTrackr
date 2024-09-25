use crate::errors::repositories_error::RepositoryError;
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use futures::stream::TryStreamExt;
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};
use std::sync::Arc;

#[async_trait]
pub trait WorkingTimeRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkingTimeInDB>, RepositoryError>;

    async fn find_all_by_project_id(
        &self,
        project_id: &ObjectId,
    ) -> Result<Vec<WorkingTimeInDB>, RepositoryError>;

    async fn insert_one(
        &self,
        working_time: &WorkingTimeCreate,
    ) -> Result<ObjectId, RepositoryError>;

    async fn update_one(
        &self,
        id: ObjectId,
        working_time: &WorkingTimeUpdate,
    ) -> Result<bool, RepositoryError>;
}

pub struct MongoWorkingTimeRepository {
    collection: Collection<WorkingTimeInDB>,
}

impl MongoWorkingTimeRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            collection: db.collection("working_time"),
        }
    }
}

#[async_trait]
impl WorkingTimeRepository for MongoWorkingTimeRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkingTimeInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn find_all_by_project_id(
        &self,
        project_id: &ObjectId,
    ) -> Result<Vec<WorkingTimeInDB>, RepositoryError> {
        let cursor = self
            .collection
            .find(doc! { "project_id": project_id })
            .await
            .map_err(RepositoryError::DatabaseError)?;

        let results: Vec<WorkingTimeInDB> = cursor
            .try_collect()
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(results)
    }

    async fn insert_one(
        &self,
        working_time: &WorkingTimeCreate,
    ) -> Result<ObjectId, RepositoryError> {
        let working_time_in_db = WorkingTimeInDB {
            id: None, // MongoDBにID生成を任せる
            project_id: working_time.project_id,
            start_time: BsonDateTime::from(working_time.start_time),
            end_time: working_time.end_time.map(BsonDateTime::from),
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self
            .collection
            .insert_one(&working_time_in_db)
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
        working_time: &WorkingTimeUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(&working_time)
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
