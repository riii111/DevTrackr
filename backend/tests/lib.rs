mod api;
mod common;

use mongodb::Client;
use std::time::Duration;
use tokio::time::timeout;

#[cfg(test)]
async fn check_mongodb_connection() -> bool {
    if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
        match timeout(Duration::from_secs(5), Client::with_uri_str(&url)).await {
            Ok(Ok(client)) => {
                match timeout(
                    Duration::from_secs(5),
                    client.list_database_names(None, None),
                )
                .await
                {
                    Ok(Ok(_)) => true,
                    Ok(Err(e)) => {
                        eprintln!("Failed to list databases: {}", e);
                        false
                    }
                    Err(e) => {
                        eprintln!("Database listing timed out: {}", e);
                        false
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Failed to create MongoDB client: {}", e);
                false
            }
            Err(e) => {
                eprintln!("Connection attempt timed out: {}", e);
                false
            }
        }
    } else {
        eprintln!("TEST_DATABASE_URL is not set");
        false
    }
}

#[cfg(test)]
pub async fn setup() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // 親ディレクトリの.envファイルを読み込み
    let parent_env_path = current_dir
        .parent()
        .expect("Failed to get parent directory")
        .join(".env");
    if parent_env_path.exists() {
        dotenvy::from_path(&parent_env_path).expect("Failed to load .env");
    }

    // .env.testの項目で上書き
    let test_env_path = current_dir.join(".env.test");
    if test_env_path.exists() {
        dotenvy::from_path(&test_env_path).expect("Failed to load .env.test");
    }

    // 必須環境変数のチェック
    let required_vars = [
        "TEST_DATABASE_URL",
        "S3_REGION",
        "MINIO_ENDPOINT",
        "JWT_SECRET",
    ];

    for var in required_vars {
        if std::env::var(var).is_err() {
            panic!("Required environment variable {} is not set", var);
        }
    }

    // MongoDBの接続チェック
    if !check_mongodb_connection().await {
        panic!("Could not connect to MongoDB. Is it running on port 27018?");
    }
}
