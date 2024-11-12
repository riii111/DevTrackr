use futures::{StreamExt, TryStreamExt};
use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use uuid::Uuid;

// コレクション名を定数として定義
const TEST_COLLECTIONS: &[&str] = &["auth_tokens", "users", "companies", "projects", "work_logs"];

#[derive(Clone)]
pub struct TestDb {
    pub db: Database,
    client: Client,
}

impl TestDb {
    pub async fn new() -> Self {
        let mongodb_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let client = Client::with_uri_str(&mongodb_url)
            .await
            .expect("Failed to connect to MongoDB");

        let db_name = format!("devtrackr_test_{}", Uuid::now_v7());
        let db = client.database(&db_name);

        let instance = Self {
            db,
            client: client.clone(),
        };

        // セットアップ処理を実行
        instance
            .setup()
            .await
            .expect("Failed to setup test database");
        instance
    }

    async fn setup(&self) -> mongodb::error::Result<()> {
        // 1. まずコレクションを作成
        self.create_collections().await?;

        // 2. 既存のインデックスをドロップ（必要な場合）
        self.drop_existing_indexes().await?;

        // 3. 新しいインデックスを作成
        self.create_indexes().await?;

        Ok(())
    }

    async fn create_collections(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            // コレクションが存在しない場合のみ作成を試みる
            if !self.collection_exists(collection_name).await? {
                self.db.create_collection(collection_name, None).await?;
            }
        }
        Ok(())
    }

    // コレクションの存在チェック用のヘルパーメソッド
    async fn collection_exists(&self, name: &str) -> mongodb::error::Result<bool> {
        let filter = doc! { "name": name };
        let collections = self.db.list_collections(Some(filter), None).await?;
        Ok(collections.count().await as i32 > 0)
    }

    async fn drop_existing_indexes(&self) -> mongodb::error::Result<()> {
        for collection_name in TEST_COLLECTIONS {
            let collection = self
                .db
                .collection::<mongodb::bson::Document>(collection_name);

            // インデックスが存在する場合のみドロップを試みる
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

    async fn create_indexes(&self) -> mongodb::error::Result<()> {
        devtrackr_api::config::db_index::create_indexes(&self.db).await
    }

    pub fn get_collection<T>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }

    pub async fn cleanup(&self) -> mongodb::error::Result<()> {
        // 1. まずコレクションをドロップ
        for collection_name in TEST_COLLECTIONS {
            if let Err(e) = self
                .db
                .collection::<mongodb::bson::Document>(collection_name)
                .drop(None)
                .await
            {
                eprintln!("Failed to drop collection {}: {}", collection_name, e);
            }
        }

        // 2. データベース自体をドロップ
        let db_name = self.db.name().to_string();
        if let Err(e) = self.db.drop(None).await {
            eprintln!("Failed to drop database {}: {}", db_name, e);
            return Err(e);
        }

        // 3. データベースが実際に削除されたことを確認
        let mut retries = 3;
        while retries > 0 {
            let dbs = self.client.list_database_names(None, None).await?;
            if !dbs.contains(&db_name) {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            retries -= 1;
        }

        Err(mongodb::error::Error::custom(format!(
            "Failed to confirm deletion of database {}",
            db_name
        )))
    }
}
