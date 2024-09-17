use crate::models::working_time::WorkingTime;
use async_trait::async_trait;
use bson::oid::ObjectId;
use mongodb::{bson::Document, results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait WorkingTimeRepository {
    async fn find_by_id(&self, id: &ObjectId)
        -> Result<Option<WorkingTime>, mongodb::error::Error>;
    async fn insert_one(
        &self,
        working_time: &WorkingTime,
    ) -> Result<ObjectId, mongodb::error::Error>;
    async fn update_one(
        &self,
        filter: Document,
        working_time: &WorkingTime,
    ) -> Result<bool, mongodb::error::Error>;
}

pub struct MongoWorkingTimeRepository {
    collection: Collection<WorkingTime>,
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
    async fn find_by_id(
        &self,
        id: &ObjectId,
    ) -> Result<Option<WorkingTime>, mongodb::error::Error> {
        self.collection.find_one(bson::doc! { "_id": id }).await
    }

    async fn insert_one(
        &self,
        working_time: &WorkingTime,
    ) -> Result<ObjectId, mongodb::error::Error> {
        let result: InsertOneResult = self.collection.insert_one(working_time).await?;
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| mongodb::error::Error::custom("挿入されたドキュメントのIDが無効です"))
    }

    async fn update_one(
        &self,
        filter: Document,
        working_time: &WorkingTime,
    ) -> Result<bool, mongodb::error::Error> {
        let update = mongodb::bson::doc! {
            "$set": mongodb::bson::to_document(working_time)?
        };
        let result = self.collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }
}
