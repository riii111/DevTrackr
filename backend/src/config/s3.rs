use aws_sdk_s3::{
    config::{Builder, Credentials, Region},
    Client,
};
use log;
use std::env;
use std::error::Error;
use std::sync::Arc;

pub struct S3Config {
    pub client: Arc<Client>,
    pub bucket_name: String,
    pub endpoint: String,
}

pub async fn init_s3_config() -> Result<Arc<S3Config>, Box<dyn Error>> {
    let minio_endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINTが設定されていません");
    let minio_access_key =
        env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEYが設定されていません");
    let minio_secret_key =
        env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEYが設定されていません");
    let region = env::var("S3_REGION").expect("S3_REGIONが設定されていません");
    let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAMEが設定されていません");

    log::debug!("MINIO_ENDPOINT: {}", minio_endpoint);
    log::debug!("MINIO_ACCESS_KEY: {}", minio_access_key);
    log::debug!("S3_REGION: {}", region);
    log::debug!("S3_BUCKET_NAME: {}", bucket_name);

    let creds = Credentials::new(minio_access_key, minio_secret_key, None, None, "minio");

    let config = Builder::new()
        .region(Region::new(region))
        .endpoint_url(&minio_endpoint)
        .credentials_provider(creds)
        .force_path_style(true)
        .build();

    let client = Arc::new(Client::from_conf(config));

    // バケットが存在しない場合は作成
    if !bucket_exists(&client, &bucket_name).await? {
        create_bucket(&client, &bucket_name).await?;
    }

    Ok(Arc::new(S3Config {
        client,
        bucket_name,
        endpoint: minio_endpoint,
    }))
}

async fn bucket_exists(client: &Client, bucket_name: &str) -> Result<bool, Box<dyn Error>> {
    match client.head_bucket().bucket(bucket_name).send().await {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.to_string().contains("NotFound") {
                Ok(false)
            } else {
                Err(Box::new(e))
            }
        }
    }
}

async fn create_bucket(client: &Client, bucket_name: &str) -> Result<(), Box<dyn Error>> {
    client.create_bucket().bucket(bucket_name).send().await?;
    Ok(())
}
