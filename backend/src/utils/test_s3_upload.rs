use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::env;

use crate::errors::app_error::AppError;

pub async fn test_upload(client: &Client) -> Result<(), AppError> {
    let bucket_name = env::var("MINIO_BUCKET_NAME").expect("MINIO_BUCKET_NAMEが設定されていません");
    log::debug!("Using bucket name: {}", bucket_name);

    let file_name = "test.txt";
    let content = "Hello, MinIO!";

    let result = client
        .put_object()
        .bucket(&bucket_name)
        .key(file_name)
        .body(ByteStream::from(content.as_bytes().to_vec()))
        .content_type("text/plain")
        .send()
        .await;

    match result {
        Ok(_) => {
            log::info!("Test upload successful");
            Ok(())
        }
        Err(e) => {
            log::error!("Test upload failed: {:?}", e);
            Err(AppError::InternalServerError(format!(
                "テストアップロードに失敗しました: {}",
                e
            )))
        }
    }
}
