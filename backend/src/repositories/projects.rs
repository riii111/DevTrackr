use crate::errors::repositories_error::RepositoryError;
use crate::models::projects::{ProjectCreate, ProjectFilter, ProjectInDB, ProjectUpdate};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};
use futures::TryStreamExt;
use mongodb::{
    error::Error as MongoError, options::FindOptions, results::InsertOneResult, Collection,
    Database,
};

#[async_trait]
pub trait ProjectRepository {
    async fn find_many(
        &self,
        filter: Option<ProjectFilter>,
        limit: Option<i64>,
        offset: Option<u64>,
        sort: Option<Vec<(String, i8)>>,
    ) -> Result<Vec<ProjectInDB>, RepositoryError>;

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
    async fn find_many(
        &self,
        filter: Option<ProjectFilter>,
        limit: Option<i64>,
        offset: Option<u64>,
        sort: Option<Vec<(String, i8)>>,
    ) -> Result<Vec<ProjectInDB>, RepositoryError> {
        // クエリの構築
        let mut query = Document::new();

        if let Some(filter) = filter {
            if let Some(title) = filter.title {
                query.insert("title", doc! { "$regex": title, "$options": "i" });
            }
            if let Some(status) = filter.status {
                query.insert("status", status.to_string());
            }
            if let Some(labels) = filter.skill_labels {
                query.insert("skill_labels", doc! { "$all": labels });
            }
        }

        // FindOptionsの構築
        let mut find_options = FindOptions::default();
        if let Some(limit) = limit {
            find_options.limit = Some(limit);
        }
        if let Some(offset) = offset {
            find_options.skip = Some(offset);
        }
        if let Some(sort_params) = sort {
            let sort_doc: Document = sort_params
                .into_iter()
                .map(|(k, v)| (k, bson::Bson::Int32(v as i32)))
                .collect();
            find_options.sort = Some(sort_doc);
        }

        // findメソッドの呼び出し
        let mut cursor = self
            .collection
            .find(Some(query), Some(find_options))
            .await
            .map_err(RepositoryError::DatabaseError)?;

        let mut projects = Vec::new();
        while let Some(result) = cursor
            .try_next()
            .await
            .map_err(RepositoryError::DatabaseError)?
        {
            projects.push(result);
        }

        Ok(projects)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<ProjectInDB>, RepositoryError> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn insert_one(&self, project: ProjectCreate) -> Result<ObjectId, RepositoryError> {
        let project_in_db = ProjectInDB {
            id: None, // MongoDBにID生成を任せる
            title: project.title,
            description: project.description,
            // company_id: project.company_id,  // TODO: 後で追加する
            hourly_pay: project.hourly_pay,
            status: project.status,
            total_working_time: 0,
            skill_labels: project.skill_labels,
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        let result: InsertOneResult = self.collection.insert_one(&project_in_db, None).await?;
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
            .update_one(doc! { "_id": id }, update, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        Ok(result.modified_count > 0)
    }
}
