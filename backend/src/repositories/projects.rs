use crate::models::projects::{ProjectCreate, ProjectInDB};
use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{results::InsertOneResult, Collection, Database};

#[async_trait]
pub trait ProjectRepository {
    // TODO: find_oneだけに集約させるべき？
    async fn find_by_id(&self, id: &ObjectId)
        -> Result<Option<ProjectInDB>, mongodb::error::Error>;

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

    async fn insert_one(&self, project: ProjectCreate) -> Result<ObjectId, mongodb::error::Error> {
        let project_in_db = ProjectInDB {
            id: None, // MongoDBにID生成を任せる
            title: project.title,
            description: project.description,
            company_name: project.company_name,
            status: project.status,
            working_time_id: None,
            total_working_time: None,
            skill_labels: project.skill_labels,
            created_at: Utc::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self.collection.insert_one(&project_in_db).await?;
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| mongodb::error::Error::custom("挿入されたドキュメントのIDが無効です"))
    }
}
