use crate::config::rate_limit::RateLimitConfig;
use crate::utils::redis_client::RedisClient;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorTooManyRequests,
    Error,
};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::timeout;

// レート制限を適用するためのミドルウェアの構造体
pub struct RateLimiter {
    redis_client: Rc<RedisClient>,
    config: Rc<RateLimitConfig>,
}

impl RateLimiter {
    // 新しいRateLimiterインスタンスを作成
    pub fn new(redis_client: RedisClient, config: RateLimitConfig) -> Self {
        RateLimiter {
            redis_client: Rc::new(redis_client),
            config: Rc::new(config),
        }
    }
}

// サービスを変換するために使用される
impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // 実際のミドルウェア（RateLimiterMiddleware）を生成する
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            redis_client: self.redis_client.clone(),
            config: self.config.clone(),
        }))
    }
}

// 実際のレート制限ロジックを含むミドルウェア
pub struct RateLimiterMiddleware<S> {
    service: S,
    redis_client: Rc<RedisClient>,
    config: Rc<RateLimitConfig>,
}

// HTTPリクエストを処理するためのコア機能を提供
impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    // 内部サービスの準備状態を転送
    forward_ready!(service);

    // 各リクエストに対して呼び出され、レート制限チェックを実行
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let redis_client = self.redis_client.clone();
        let config = self.config.clone();

        // クライアントのIPアドレスを取得し、Redisのキーとして使用
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        let key = format!("rate_limit:{}", ip);

        let fut = self.service.call(req);

        Box::pin(async move {
            // Redisでリクエスト数をインクリメントし、現在のカウントを取得
            // タイムアウトを5秒に設定し、Redis操作が長時間ブロックされるのを防ぐ
            let count = match timeout(
                Duration::from_secs(5),
                redis_client.increment_and_get(&key, config.duration.as_secs()),
            )
            .await
            {
                Ok(result) => result?,
                Err(_) => {
                    return Err(Error::from(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Rate limit check timed out",
                    )))
                }
            };

            // リクエスト数が制限を超えていたらエラーを返す
            if count > config.max_requests {
                return Err(ErrorTooManyRequests("Rate limit exceeded"));
            }

            // 制限内であれば、次のミドルウェアまたはハンドラに処理を渡す
            fut.await
        })
    }
}
