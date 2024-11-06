use aws_sdk_s3::primitives::ByteStream;
use image::ImageFormat;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::s3::S3Config;
use crate::errors::app_error::AppError;

pub struct S3Client {
    config: Arc<S3Config>,
}

impl S3Client {
    pub fn new(config: Arc<S3Config>) -> Self {
        Self { config }
    }

    /// アバターをS3にアップロード
    pub async fn upload_avatar(&self, image_data: &[u8]) -> Result<String, AppError> {
        let file_name = format!("avatars/{}.png", Uuid::now_v7());

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
            .config
            .client
            .put_object()
            .bucket(&self.config.bucket_name)
            .key(&file_name)
            .body(ByteStream::from(png_data))
            .content_type("image/png")
            .send()
            .await;

        match result {
            Ok(_) => {
                // ファイル名のみを返す
                Ok(file_name)
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

    pub fn get_public_url(&self, object_key: &str) -> String {
        // object_keyが既にフルURLの場合はそのまま返す
        if object_key.starts_with("http://") || object_key.starts_with("https://") {
            object_key.to_string()
        } else {
            format!(
                "{}/{}/{}",
                self.config.endpoint, self.config.bucket_name, object_key
            )
        }
    }
}
