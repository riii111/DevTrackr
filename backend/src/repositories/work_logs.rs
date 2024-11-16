use crate::errors::repositories_error::RepositoryError;
use crate::models::work_logs::{WorkLogCreate, WorkLogInDB, WorkLogUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use futures::stream::TryStreamExt;
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait WorkLogRepository {
    async fn find_all(&self) -> Result<Vec<WorkLogInDB>, RepositoryError>;

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkLogInDB>, RepositoryError>;

    async fn insert_one(&self, work_logs: &WorkLogCreate) -> Result<ObjectId, RepositoryError>;

    async fn update_one(
        &self,
        id: ObjectId,
        work_logs: &WorkLogUpdate,
    ) -> Result<bool, RepositoryError>;
}

pub struct MongoWorkLogRepository {
    collection: Collection<WorkLogInDB>,
}

impl MongoWorkLogRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("work_logs"),
        }
    }
}

#[async_trait]
impl WorkLogRepository for MongoWorkLogRepository {
    async fn find_all(&self) -> Result<Vec<WorkLogInDB>, RepositoryError> {
        let mut work_logs = Vec::new();
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
            work_logs.push(result);
        }

        Ok(work_logs)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkLogInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(&self, work_logs: &WorkLogCreate) -> Result<ObjectId, RepositoryError> {
        let work_logs_in_db = WorkLogInDB {
            id: None, // MongoDBにID生成を任せる
            project_id: work_logs.project_id,
            start_time: work_logs.start_time,
            end_time: work_logs.end_time.map(BsonDateTime::from),
            memo: work_logs.memo.clone(),
            break_time: work_logs.break_time,
            actual_work_minutes: work_logs.actual_work_minutes,
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self
            .collection
            .insert_one(&work_logs_in_db, None)
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
        work_logs: &WorkLogUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(&work_logs)
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
