//! test_appインスタンスをラップするための定義
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::common::test_app::TestApp;

type ServiceFuture = Pin<Box<dyn Future<Output = Result<ServiceResponse, actix_web::Error>>>>;
type BoxedService = Pin<
    Box<
        dyn Service<
            Request,
            Response = ServiceResponse,
            Error = actix_web::Error,
            Future = ServiceFuture,
        >,
    >,
>;

pub struct TestContext {
    pub app: TestApp,
    service: TestServiceWrapper,
}

impl TestContext {
    pub async fn new() -> Self {
        let app = TestApp::new().await.expect("Failed to create TestApp");
        let service = TestServiceWrapper::new(app.build_test_app().await);

        Self { app, service }
    }

    pub async fn with_auth() -> Self {
        let mut context = Self::new().await;
        context.app.login().await;
        context
    }

    pub fn service(&self) -> &TestServiceWrapper {
        &self.service
    }

    // 認証付きリクエストのヘルパーメソッド
    pub async fn authenticated_request(
        &self,
        req: test::TestRequest,
        uri: &str,
    ) -> ServiceResponse {
        println!("Making authenticated request to: {}", uri);
        println!("Access token: {}", self.app.access_token.as_ref().unwrap());

        let request = req
            .uri(uri)
            .insert_header((
                "Authorization",
                format!("Bearer {}", self.app.access_token.as_ref().unwrap()),
            ))
            .to_request();

        let response = test::call_service(self.service(), request).await;

        // レスポンスの情報をログ出力
        println!("Response status: {}", response.status());
        println!("Response headers: {:?}", response.headers());

        // レスポンスボディを取得してログ出力
        if let Some(content_length) = response.headers().get("content-length") {
            println!("Content length: {:?}", content_length);
        }

        response
    }

    // クッキーの検証ヘルパー
    pub fn assert_auth_cookies_cleared(&self, response: &ServiceResponse) {
        let cookies: Vec<_> = response.response().cookies().collect();
        for cookie in cookies {
            if cookie.name() == "access_token" || cookie.name() == "refresh_token" {
                assert!(cookie.value().is_empty());
                assert_eq!(
                    cookie.max_age(),
                    Some(actix_web::cookie::time::Duration::ZERO)
                );
            }
        }
    }
}

// サービスラッパー
pub struct TestServiceWrapper {
    service: BoxedService,
}

impl TestServiceWrapper {
    pub fn new<S>(service: S) -> Self
    where
        S: Service<Request, Response = ServiceResponse, Error = actix_web::Error> + 'static,
        S::Future: 'static,
    {
        let service = Box::pin(ServiceWrapper::new(service));
        Self { service }
    }
}

impl Service<Request> for TestServiceWrapper {
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type Future = ServiceFuture;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.as_ref().poll_ready(cx)
    }

    fn call(&self, req: Request) -> Self::Future {
        self.service.as_ref().call(req)
    }
}

// 内部で使用するサービスラッパー
struct ServiceWrapper<S> {
    service: S,
}

impl<S> ServiceWrapper<S>
where
    S: Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    S::Future: 'static,
{
    fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> Service<Request> for ServiceWrapper<S>
where
    S: Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type Future = ServiceFuture;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: Request) -> Self::Future {
        Box::pin(self.service.call(req))
    }
}
