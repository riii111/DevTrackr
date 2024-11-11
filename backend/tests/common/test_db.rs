use devtrackr_api::models::auth::AuthTokenInDB;
use devtrackr_api::models::projects::ProjectInDB;
use devtrackr_api::models::users::UserInDB;
use devtrackr_api::models::work_logs::WorkLogsInDB;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Client, Collection, Database};
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

// コレクション名を定数として定義
const TEST_COLLECTIONS: &[&str] = &["auth_tokens", "users", "companies", "projects", "work_logs"];

pub struct TestDb {
    pub db: Database,
    collection_prefix: String,
}

impl TestDb {
    pub async fn new() -> Self {
        let mongodb_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

        let client = match timeout(Duration::from_secs(5), Client::with_uri_str(&mongodb_url)).await
        {
            Ok(Ok(client)) => client,
            Ok(Err(e)) => panic!("Failed to connect to MongoDB: {}", e),
            Err(e) => panic!("Connection timeout: {}", e),
        };

        // 接続テスト
        if let Err(e) = timeout(
            Duration::from_secs(5),
            client.list_database_names(None, None),
        )
        .await
        {
            panic!("Database connection test failed: {}", e);
        }
        // データベース名にもUUIDを付与して完全に分離する
        let db_name = format!("devtrackr_test_{}", Uuid::now_v7());
        let db = client.database(&db_name);

        let collection_prefix = format!("test_{}", Uuid::now_v7());

        let instance = Self {
            db,
            collection_prefix,
        };

        // テスト用のインデックスをセットアップ
        let _ = instance.setup_test_indexes().await;

        instance
    }

    // コレクション名にプレフィックスを付けて取得
    pub fn get_collection<T>(&self, name: &str) -> Collection<T> {
        self.db
            .collection(&format!("{}_{}", self.collection_prefix, name))
    }

    // テスト用インデックスのセットアップ
    async fn setup_test_indexes(&self) -> mongodb::error::Result<()> {
        self.drop_existing_indexes().await?;
        self.drop_collections().await?;
        self.create_collections().await?;
        self.create_indexes().await?;
        Ok(())
    }

    async fn drop_existing_indexes(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let collection = self.get_collection::<mongodb::bson::Document>(collection_name);
            let mut indexes = collection.list_indexes(None).await?;
            while let Some(index) = indexes.try_next().await? {
                if let Some(name) = index.keys.get("name").and_then(|name| name.as_str()) {
                    if name != "_id_" {
                        collection.drop_index(name, None).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn drop_collections(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let collection = self.get_collection::<mongodb::bson::Document>(collection_name);
            collection.drop(None).await?;
        }
        Ok(())
    }

    async fn create_collections(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let prefixed_name = format!("{}_{}", self.collection_prefix, collection_name);
            self.db.create_collection(&prefixed_name, None).await?;
        }
        Ok(())
    }

    async fn create_indexes(&self) -> mongodb::error::Result<()> {
        // usersコレクションのインデックス
        let users_collection = self.get_collection::<UserInDB>("users");
        let email_index = mongodb::IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("idx_email_unique".to_string())
                    .build(),
            )
            .build();
        users_collection.create_index(email_index, None).await?;

        // auth_tokensコレクションのインデックス
        let auth_collection = self.get_collection::<AuthTokenInDB>("auth_tokens");
        let access_token_index = mongodb::IndexModel::builder()
            .keys(doc! { "access_token": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("idx_access_token_unique".to_string())
                    .build(),
            )
            .build();
        auth_collection
            .create_index(access_token_index, None)
            .await?;

        let refresh_token_index = mongodb::IndexModel::builder()
            .keys(doc! { "refresh_token": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("idx_refresh_token_unique".to_string())
                    .build(),
            )
            .build();
        auth_collection
            .create_index(refresh_token_index, None)
            .await?;

        // projectsコレクションのインデックス
        let projects_collection = self.get_collection::<ProjectInDB>("projects");
        let indexes = vec![
            mongodb::IndexModel::builder()
                .keys(doc! { "company_id": 1 })
                .options(
                    IndexOptions::builder()
                        .name("idx_company_id".to_string())
                        .build(),
                )
                .build(),
            // ... 他のプロジェクト関連のインデックス
        ];
        projects_collection.create_indexes(indexes, None).await?;

        // work_logsコレクションのインデックス
        let work_logs_collection = self.get_collection::<WorkLogsInDB>("work_logs");
        let project_id_index = mongodb::IndexModel::builder()
            .keys(doc! { "project_id": 1 })
            .options(
                IndexOptions::builder()
                    .name("idx_project_id".to_string())
                    .build(),
            )
            .build();
        work_logs_collection
            .create_index(project_id_index, None)
            .await?;

        Ok(())
    }

    // データベースを明示的に削除するメソッドを追加
    pub async fn cleanup(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let prefixed_name = format!("{}_{}", self.collection_prefix, collection_name);
            let _ = self
                .db
                .collection::<mongodb::bson::Document>(&prefixed_name)
                .drop(None)
                .await;
        }
        Ok(())
    }
}
