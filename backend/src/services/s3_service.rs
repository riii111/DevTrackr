use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use image::ImageFormat;
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

    pub async fn upload_avatar(&self, image_data: &[u8]) -> Result<String, AppError> {
        let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAMEが設定されていません");
        let file_name = format!("avatars/{}.png", Uuid::new_v4());

        // 画像フォーマットを明示的に判別
        let format = image::guess_format(image_data).map_err(|e| {
            AppError::BadRequest(format!("画像フォーマットの判別に失敗しました: {}", e))
        })?;

        log::debug!("Detected image format: {:?}", format);

        // 画像をデコードし、PNGに変換
        let img = image::load_from_memory_with_format(image_data, format)
            .map_err(|e| AppError::BadRequest(format!("無効な画像データ: {}", e)))?;

        let mut png_data = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)
            .map_err(|e| {
                AppError::InternalServerError(format!("画像の変換に失敗しました: {}", e))
            })?;

        let result = self
            .client
            .put_object()
            .bucket(&bucket_name)
            .key(&file_name)
            .body(ByteStream::from(png_data))
            .content_type("image/png")
            .send()
            .await;

        match result {
            Ok(_) => {
                let avatar_url = format!(
                    "{}/{}/{}",
                    env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINTが設定されていません"),
                    bucket_name,
                    file_name
                );
                Ok(avatar_url)
            }
            Err(e) => {
                log::error!("S3 upload error: {:?}", e);
                Err(AppError::InternalServerError(format!(
                    "アバターのアップロードに失敗しました: {}",
                    e
                )))
            }
        }
    }
}
