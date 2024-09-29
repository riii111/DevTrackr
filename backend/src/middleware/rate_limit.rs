use crate::config::rate_limit::RateLimitConfig;
use crate::utils::redis_client::RedisClient;
use actix_limitation::Limiter;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use log::{info, warn};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// レート制限を適用するためのミドルウェア構造体
pub struct RateLimiterMiddleware {
    redis_client: Arc<RedisClient>,
    max_requests: u64,
    duration: Duration,
}

impl RateLimiterMiddleware {
    // 新しいRateLimiterMiddlewareインスタンスを作成
    pub fn new(redis_client: Arc<RedisClient>, rate_limit_config: RateLimitConfig) -> Self {
        RateLimiterMiddleware {
            redis_client,
            max_requests: rate_limit_config.max_requests,
            duration: rate_limit_config.duration,
        }
    }
}

// Transformトレイトの実装
impl<S, B> Transform<S, ServiceRequest> for RateLimiterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // 新しいトランスフォームを作成
    fn new_transform(&self, service: S) -> Self::Future {
        let redis_url = self.redis_client.get_url();
        let limiter = Limiter::builder(redis_url)
            .limit(self.max_requests as usize)
            .period(self.duration)
            .build()
            .expect("Failed to create RateLimiter");
        ready(Ok(RateLimiterMiddlewareService { service, limiter }))
    }
}

// 実際のレート制限ロジックを含むミドルウェアサービス
pub struct RateLimiterMiddlewareService<S> {
    service: S,
    limiter: Limiter,
}

// Serviceトレイトの実装
impl<S, B> Service<ServiceRequest> for RateLimiterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    // 内部サービスの準備状態を転送
    actix_web::dev::forward_ready!(service);

    // 各リクエストに対して呼び出され、レート制限チェックを実行
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // クライアントのIPアドレスを取得し、レート制限のキーとして使用
        let key = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let fut = self.service.call(req);
        let limiter = self.limiter.clone();

        Box::pin(async move {
            // レート制限のカウントを取得
            match limiter.count(key.clone()).await {
                // レート制限が許可された場合
                Ok(status) => {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize;
                    let reset_in = status.reset_epoch_utc().saturating_sub(now);
                    info!(
                        "Rate limit status for {}: {}/{} requests, reset in {} seconds",
                        key,
                        status.remaining(),
                        status.limit(),
                        reset_in
                    );
                    fut.await
                }
                // レート制限が超過した場合
                Err(actix_limitation::Error::LimitExceeded(status)) => {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize;
                    let reset_in = status.reset_epoch_utc().saturating_sub(now);
                    warn!(
                        "Rate limit exceeded for {}: {}/{} requests, reset in {} seconds",
                        key,
                        status.remaining(),
                        status.limit(),
                        reset_in
                    );
                    Err(actix_web::error::ErrorTooManyRequests("Rate limit exceeded").into())
                }
                // それ以外のエラー
                Err(e) => {
                    warn!("Rate limiter error for {}: {}", key, e);
                    Err(actix_web::error::ErrorInternalServerError(e).into())
                }
            }
        })
    }
}
