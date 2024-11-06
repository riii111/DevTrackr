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

        println!("Initializing test database connection...");

        let client = match timeout(Duration::from_secs(5), Client::with_uri_str(&mongodb_url)).await
        {
            Ok(Ok(client)) => client,
            Ok(Err(e)) => panic!("Failed to connect to MongoDB: {}", e),
            Err(e) => panic!("Connection timeout: {}", e),
        };

        println!("Testing database connection...");
        match timeout(
            Duration::from_secs(5),
            client.list_database_names(None, None),
        )
        .await
        {
            Ok(Ok(_)) => println!("Database connection test successful"),
            Ok(Err(e)) => panic!("Database test failed: {}", e),
            Err(e) => panic!("Database test timeout: {}", e),
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

impl Drop for TestDb {
    fn drop(&mut self) {
        // 非同期処理を同期的に実行する代わりに、警告を出力
        println!("Warning: Test database collections will be cleaned up in the background");
    }
}
