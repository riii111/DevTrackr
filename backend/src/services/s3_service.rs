use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;

pub struct S3Service {
    client: Arc<S3Client>,
}

impl S3Service {
    pub fn new(client: Arc<S3Client>) -> Self {
        Self { client }
    }

    pub async fn upload_avatar(&self, avatar_data: &str) -> Result<String, AppError> {
        let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAMEが設定されていません");
        let file_name = format!("avatars/{}.jpg", Uuid::new_v4());

        let avatar_bytes = STANDARD
            .decode(avatar_data)
            .map_err(|e| AppError::BadRequest(format!("無効なbase64データ: {}", e)))?;

        self.client
            .put_object()
            .bucket(&bucket_name)
            .key(&file_name)
            .body(ByteStream::from(avatar_bytes))
            .content_type("image/jpeg")
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "アバターのアップロードに失敗しました: {}",
                    e
                ))
            })?;

        let avatar_url = format!(
            "{}/{}/{}",
            env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINTが設定されていません"),
            bucket_name,
            file_name
        );

        Ok(avatar_url)
    }
}
