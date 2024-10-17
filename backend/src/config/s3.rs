use aws_sdk_s3::{
    config::{Builder, Credentials, Region},
    Client,
};
use log;
use std::error::Error;
use std::sync::Arc;

// TODO: ローカルならMinIO、本番ならS3を使うようにする
pub async fn init_s3_client() -> Result<Arc<Client>, Box<dyn Error>> {
    let minio_endpoint =
        std::env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINTが設定されていません");
    let minio_access_key =
        std::env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEYが設定されていません");
    let minio_secret_key =
        std::env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEYが設定されていません");
    let region = std::env::var("S3_REGION").expect("S3_REGIONが設定されていません");

    log::debug!("MINIO_ENDPOINT: {}", minio_endpoint);
    log::debug!("MINIO_ACCESS_KEY: {}", minio_access_key);
    log::debug!("S3_REGION: {}", region);

    let creds = Credentials::new(minio_access_key, minio_secret_key, None, None, "minio");

    let config = Builder::new()
        .region(Region::new(region))
        .endpoint_url(minio_endpoint)
        .credentials_provider(creds)
        .force_path_style(true)
        .build();

    let client = Client::from_conf(config);
    Ok(Arc::new(client))
}
