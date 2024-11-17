use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::env;
use std::future::Future;
use std::pin::Pin;

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// トランスフォーム(リクエスト処理の前後に追加の処理を挟むための仕組み)
    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

fn get_minio_domain() -> String {
    env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "http://minio:9000".to_string())
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    /// サービスの準備状態を確認
    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    /// リクエストを処理し、レスポンスにセキュリティヘッダを追加
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let minio_domain = get_minio_domain();
        let next_public_url = env::var("NEXT_PUBLIC_MINIO_PUBLIC_URL")
            .unwrap_or_else(|_| "http://localhost:9000".to_string());

        Box::pin(async move {
            let mut res = fut.await?;

            // スクリプトやスタイルのソースを制限. XSS攻撃対策
            res.headers_mut().insert(
                HeaderName::from_static("content-security-policy"),
                HeaderValue::from_str(&format!(
                    "default-src 'self'; \
                     script-src 'self'; \
                     style-src 'self'; \
                     img-src 'self' data: {} {} /_next/image/*; \
                     font-src 'self'; \
                     object-src 'none'; \
                     base-uri 'self'; \
                     form-action 'self'; \
                     frame-ancestors 'none'; \
                     block-all-mixed-content;",
                    minio_domain, next_public_url
                ))
                .unwrap(),
            );

            // 他のサイトで、フレームとして表示されることを防ぐ. クリックジャッキング攻撃を防止
            res.headers_mut().insert(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_static("DENY"),
            );
            // nonsniff値により、ブラウザがサーバからのコンテンツタイプを無視してファイル解釈するのを防ぐ
            res.headers_mut().insert(
                HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff"),
            );
            // リファラ情報の送信を制御。プライバシー保護
            res.headers_mut().insert(
                HeaderName::from_static("referrer-policy"),
                HeaderValue::from_static("strict-origin-when-cross-origin"),
            );

            // HSTSの設定
            let enable_hsts =
                std::env::var("SECURE_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
            if enable_hsts {
                res.headers_mut().insert(
                    HeaderName::from_static("strict-transport-security"),
                    HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                );
            }

            Ok(res)
        })
    }
}
