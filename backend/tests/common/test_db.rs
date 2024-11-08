use mongodb::{Client, Collection, Database};
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

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

        let collection_prefix = format!("test_{}", Uuid::now_v7());
        let db = client.database("devtrackr_test");

        Self {
            db,
            collection_prefix,
        }
    }

    // コレクション名にプレフィックスを付けて取得
    pub fn get_collection<T>(&self, name: &str) -> Collection<T> {
        self.db
            .collection(&format!("{}_{}", self.collection_prefix, name))
    }
}
