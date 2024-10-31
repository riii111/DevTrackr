use redis::{Client, RedisError};
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

/// Redis操作を行うためのクライアントラッパー
pub struct RedisClient {
    client: Arc<Mutex<Client>>,
    timeout: Duration,
}

impl RedisClient {
    /// 新しいRedisClientインスタンスを作成
    ///
    /// - `client`: Redisクライアント
    /// - `timeout`: 操作のタイムアウト時間
    pub fn new(client: Client, timeout: Duration) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            timeout,
        }
    }

    /// 接続を取得し、指定された操作を実行する汎用関数
    ///
    /// この関数は接続の取得とタイムアウト処理を一元化する
    async fn with_connection<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(
            redis::aio::MultiplexedConnection,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
    {
        let client = self.client.lock().await;
        // 接続の取得とタイムアウト処理
        let con = timeout(self.timeout, client.get_multiplexed_async_connection())
            .await
            .map_err(|_| Error::new(ErrorKind::TimedOut, "Connection timed out"))?
            .map_err(|e| Error::new(ErrorKind::Other, format!("Redis error: {}", e)))?;

        // 指定された操作の実行とタイムアウト処理
        timeout(self.timeout, f(con))
            .await
            .map_err(|_| Error::new(ErrorKind::TimedOut, "Operation timed out"))?
    }

    /// キーをインクリメントし、レート制限をチェックする
    ///
    /// - `key`: インクリメントするキー
    /// - `max_requests`: 許可される最大リクエスト数
    /// - `expiry`: キーの有効期限（秒）
    pub async fn increment_and_check(
        &self,
        key: &str,
        max_requests: u64,
        expiry: u64,
    ) -> Result<bool> {
        let key = key.to_string();

        self.with_connection(move |mut con| {
            Box::pin(async move {
                // アトミックなパイプラインでインクリメントと有効期限の設定を行う
                let result: (u64,) = redis::pipe()
                    .atomic()
                    .incr(&key, 1) // キーをインクリメント
                    .expire(&key, expiry as i64) // キーの有効期限を設定
                    .ignore() // expireの結果を無視
                    .query_async(&mut con)
                    .await
                    .map_err(|e: RedisError| Error::new(ErrorKind::Other, e))?;

                let count = result.0;
                // カウントが最大リクエスト数以下かどうかを返す
                Ok(count <= max_requests)
            })
        })
        .await
    }

    /// Redis接続のテスト
    ///
    /// PINGコマンドを送信し、応答を確認する
    pub async fn test_connection(&self) -> Result<String> {
        self.with_connection(|mut con| {
            Box::pin(async move {
                redis::cmd("PING")
                    .query_async(&mut con)
                    .await
                    .map_err(|e: RedisError| Error::new(ErrorKind::Other, e))
            })
        })
        .await
    }
}
