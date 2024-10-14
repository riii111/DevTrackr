use aws_sdk_s3::{
    config::{Builder, Credentials, Region},
    Client,
};
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

    let creds = Credentials::new(minio_access_key, minio_secret_key, None, None, "minio");

    let config = Builder::new()
        .region(Region::new("us-east-1"))
        .endpoint_url(minio_endpoint)
        .credentials_provider(creds)
        .force_path_style(true)
        .build();

    let client = Client::from_conf(config);
    Ok(Arc::new(client))
}
