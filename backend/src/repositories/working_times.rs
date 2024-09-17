use crate::models::working_times::WorkingTime;
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

    }
