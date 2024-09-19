use crate::errors::repositories_error::RepositoryError;
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeInDB, WorkingTimeUpdate};
use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait WorkingTimeRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkingTimeInDB>, RepositoryError>;

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
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("working_time"),
        }
    }
}

#[async_trait]
impl WorkingTimeRepository for MongoWorkingTimeRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkingTimeInDB>, RepositoryError> {
        self.collection
            .find_one(bson::doc! { "_id": id })
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(
        &self,
        working_time: &WorkingTimeCreate,
    ) -> Result<ObjectId, RepositoryError> {
        let working_time_in_db = WorkingTimeInDB {
            id: None, // MongoDBにID生成を任せる
            start_time: working_time.start_time,
            end_time: working_time.end_time,
            created_at: Utc::now(),
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
            .ok_or(RepositoryError::InvalidId)
    }

    async fn update_one(
        &self,
        id: ObjectId,
        working_time: &WorkingTimeUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(working_time)
            .map_err(|e| RepositoryError::DatabaseError(mongodb::error::Error::from(e)))?;
        update_doc.insert("updated_at", Utc::now());
        let update = mongodb::bson::doc! {
            "$set": update_doc
        };
        let result = self
            .collection
            .update_one(mongodb::bson::doc! { "_id": id }, update)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        Ok(result.modified_count > 0)
    }
}
