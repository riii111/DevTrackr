use crate::errors::repositories_error::RepositoryError;
use crate::models::work_logs::{WorkLogsCreate, WorkLogsInDB, WorkLogsUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::{error::Error as MongoError, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait WorkLogsRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkLogsInDB>, RepositoryError>;

    async fn insert_one(&self, work_logs: &WorkLogsCreate) -> Result<ObjectId, RepositoryError>;

    async fn update_one(
        &self,
        id: ObjectId,
        work_logs: &WorkLogsUpdate,
    ) -> Result<bool, RepositoryError>;
}

pub struct MongoWorkLogsRepository {
    collection: Collection<WorkLogsInDB>,
}

impl MongoWorkLogsRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("work_logs"),
        }
    }
}

#[async_trait]
impl WorkLogsRepository for MongoWorkLogsRepository {
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<WorkLogsInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(&self, work_logs: &WorkLogsCreate) -> Result<ObjectId, RepositoryError> {
        let work_logs_in_db = WorkLogsInDB {
            id: None, // MongoDBにID生成を任せる
            project_id: work_logs.project_id,
            start_time: BsonDateTime::from(work_logs.start_time),
            end_time: work_logs.end_time.map(BsonDateTime::from),
            memo: work_logs.memo.clone(),
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self
            .collection
            .insert_one(&work_logs_in_db)
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
        work_logs: &WorkLogsUpdate,
    ) -> Result<bool, RepositoryError> {
        let mut update_doc = bson::to_document(&work_logs)
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
