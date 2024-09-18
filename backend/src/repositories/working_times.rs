use crate::models::working_times::WorkingTime;
use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{results::InsertOneResult, Collection, Database};

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
        id: ObjectId,
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
        id: ObjectId,
        working_time: &WorkingTime,
    ) -> Result<bool, mongodb::error::Error> {
        let mut update_doc = bson::to_document(working_time)?;
        update_doc.insert("updated_at", Utc::now());
        let update = mongodb::bson::doc! {
            "$set": update_doc
        };
        self.collection
            .update_one(mongodb::bson::doc! { "_id": id }, update)
            .await
            .map(|result| result.modified_count > 0)
    }
}
