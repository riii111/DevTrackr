use bson::Document;
use futures::TryStreamExt;
use mongodb::{Client, Collection, Database};
use std::sync::Arc;
use uuid::Uuid;

// コレクション名を定数として定義
const TEST_COLLECTIONS: &[&str] = &["auth_tokens", "users", "companies", "projects", "work_logs"];

#[derive(Clone)]
pub struct TestDb {
    pub client: Arc<Client>,
    pub db: Arc<Database>,
    pub db_name: String,
}

impl TestDb {
    pub async fn new() -> mongodb::error::Result<Self> {
        let mongodb_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

        let client = Arc::new(Client::with_uri_str(&mongodb_url).await?);
        let db_name = format!("devtrackr_test_{}", Uuid::now_v7());
        let db = Arc::new(client.database(&db_name));

        let instance = Self {
            client,
            db,
            db_name,
        };

        // セットアップ処理を実行
        instance.setup().await?;

        Ok(instance)
    }

    // 包括的なセットアップメソッド
    pub async fn setup(&self) -> mongodb::error::Result<()> {
        self.create_collections().await?;
        self.drop_existing_indexes().await?;
        self.create_indexes().await?;
        Ok(())
    }

    // コレクション作成メソッド
    async fn create_collections(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            // コレクションが存在しない場合のみ作成を試みる
            if !self.collection_exists(collection_name).await? {
                self.db.create_collection(collection_name, None).await?;
            }
        }
        Ok(())
    }

    // コレクション存在確認メソッド
    async fn collection_exists(&self, name: &str) -> mongodb::error::Result<bool> {
        let collections = self.db.list_collection_names(None).await?;
        Ok(collections.contains(&name.to_string()))
    }

    // インデックス削除メソッド
    async fn drop_existing_indexes(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let collection: Collection<Document> = self.db.collection(collection_name);

            if let Ok(mut indexes) = collection.list_indexes(None).await {
                while let Ok(Some(index)) = indexes.try_next().await {
                    if let Some(name) = index.keys.get("name").and_then(|name| name.as_str()) {
                        if name != "_id_" {
                            let _ = collection.drop_index(name, None).await;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // インデックス作成メソッド
    async fn create_indexes(&self) -> mongodb::error::Result<()> {
        devtrackr_api::config::db_index::create_indexes(&self.db).await
    }

    // コレクション取得メソッド
    pub fn get_collection<T>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let client = self.client.clone();
        let db_name = self.db_name.clone();

        tokio::spawn(async move {
            if let Err(e) = client.database(&db_name).drop(None).await {
                eprintln!("Failed to drop database {}: {}", db_name, e);
            }
        });
    }
}
