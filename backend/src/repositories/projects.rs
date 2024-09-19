use crate::models::projects::{ProjectCreate, ProjectInDB};
use async_trait::async_trait;
use bson::oid::ObjectId;
// use futures::TryStreamExt;
// use mongodb::{bson::Document, Collection, Database};
use chrono::Utc;
use mongodb::{results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait ProjectRepository {
    // TODO: find_oneだけに集約させるべき？
    async fn find_by_id(&self, id: &ObjectId)
        -> Result<Option<ProjectInDB>, mongodb::error::Error>;
    // async fn find_many(
    //     &self,
    //     filter: Option<Document>,
    // ) -> Result<Vec<Project>, mongodb::error::Error>;

    async fn insert_one(&self, project: ProjectCreate) -> Result<ObjectId, mongodb::error::Error>;
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
    async fn find_by_id(
        &self,
        id: &ObjectId,
    ) -> Result<Option<ProjectInDB>, mongodb::error::Error> {
        self.collection.find_one(bson::doc! { "_id": id }).await
    }

    // async fn find_many(
    //     &self,
    //     filter: Option<Document>,
    // ) -> Result<Vec<Project>, mongodb::error::Error> {
    //     let cursor = self.collection.find(filter.unwrap_or_default()).await?;
    //     cursor.try_collect().await
    // }
}
