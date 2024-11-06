mod api;
mod common;

use mongodb::Client;
use std::time::Duration;
use tokio::time::timeout;

#[cfg(test)]
async fn check_mongodb_connection() -> bool {
    if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
        println!("Attempting to connect to MongoDB at: {}", url);

        match timeout(Duration::from_secs(5), Client::with_uri_str(&url)).await {
            Ok(Ok(client)) => {
                println!("Successfully created MongoDB client");
                match timeout(
                    Duration::from_secs(5),
                    client.list_database_names(None, None),
                )
                .await
                {
                    Ok(Ok(_)) => {
                        println!("Successfully listed databases");
                        true
                    }
                    Ok(Err(e)) => {
                        println!("Failed to list databases: {}", e);
                        false
                    }
                    Err(e) => {
                        println!("Database listing timed out: {}", e);
                        false
                    }
                }
            }
            Ok(Err(e)) => {
                println!("Failed to create MongoDB client: {}", e);
                false
            }
            Err(e) => {
                println!("Connection attempt timed out: {}", e);
                false
            }
        }
    } else {
        println!("TEST_DATABASE_URL is not set");
        false
    }
}

#[cfg(test)]
pub async fn setup() {
    // カレントディレクトリを取得
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    // 親ディレクトリの.envファイルのパスを構築して読み込み
    let parent_env_path = current_dir
        .parent()
        .expect("Failed to get parent directory")
        .join(".env");
    if parent_env_path.exists() {
        println!("Loading .env from: {:?}", parent_env_path);
        dotenvy::from_path(&parent_env_path).expect("Failed to load .env");
    } else {
        println!(".env file not found at: {:?}", parent_env_path);
    }

    // .env.testの項目のみさらに上書き
    let test_env_path = current_dir.join(".env.test");
    if test_env_path.exists() {
        println!("Loading .env.test from: {:?}", test_env_path);
        dotenvy::from_path(&test_env_path).expect("Failed to load .env.test");
    } else {
        println!(".env.test file not found at: {:?}", test_env_path);
    }

    // 環境変数が設定されているか確認して出力
    let required_vars = [
        "TEST_DATABASE_URL",
        "S3_REGION",
        "MINIO_ENDPOINT",
        "JWT_SECRET",
    ];

    for var in required_vars {
        match std::env::var(var) {
            Ok(value) => println!("{} is set to: {}", var, value),
            Err(_) => panic!("Required environment variable {} is not set", var),
        }
    }

    // MongoDBの接続チェック
    if !check_mongodb_connection().await {
        panic!("Could not connect to MongoDB. Is it running on port 27018?");
    }
}
