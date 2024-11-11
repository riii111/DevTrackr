//! テストアプリケーションの構築を担当するモジュール
//!
//! このモジュールはActix-webのテストにおけるベストプラクティスに従い、
//! 以下の理由でテスト用の独立したアプリケーションを構築する：
//!
//! 1. テスト環境の分離
//!    - テスト用DBへの接続
//!    - モックやスタブの注入
//!    - テスト固有の設定
//!
//! 2. テストの信頼性向上
//!    - 環境に依存しないテスト実行
//!    - 副作用の制御が容易
//!    - テストケース間の独立性確保
//!
//! 3. パフォーマンス
//!    - 軽量なテスト用サーバーの使用
//!    - 必要な機能のみを含む最小構成
//!    - テスト実行時間の最適化

use crate::common::test_db::TestDb;
use actix_http::StatusCode;
use actix_web::{
    test,
    web::{self, Bytes},
    App,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use devtrackr_api::{
    api,
    api::endpoints::auth::{login, logout, refresh, register},
    clients::aws_s3::S3Client,
    config::{di, s3},
    errors::app_error::json_error_handler,
    middleware::{csrf, jwt, security_headers::SecurityHeaders},
    models::users::UserCreate,
    repositories::auth::MongoAuthRepository,
    repositories::companies::MongoCompanyRepository,
    usecases::auth::AuthUseCase,
    usecases::companies::CompanyUseCase,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// フェッチ用APIレスポンスの構造体
#[derive(Debug)]
pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub body: T,
}

/// エラーレスポンス用の構造体
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: Bytes,
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub async fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }
}

#[allow(dead_code)]
pub struct TestApp {
    pub auth_usecase: Arc<AuthUseCase<MongoAuthRepository>>,
    pub company_usecase: Arc<CompanyUseCase<MongoCompanyRepository>>,
    pub db: mongodb::Database,
    pub s3_client: Arc<S3Client>,
    pub test_user: UserCreate,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl TestApp {
    pub async fn new() -> Self {
        // 環境変数のセットアップを行う
        crate::setup().await;

        // MinIOの環境変数を明示的に設定
        std::env::set_var("MINIO_ENDPOINT", "http://localhost:9000");

        // テスト用ユーザーのセットアップ（テストは並行実行されるためUUIDで一意にする）
        let uuid = Uuid::now_v7();
        let test_user = UserCreate {
            email: format!("test_{}@example.com", uuid),
            password: String::from("password123"),
            username: format!("testuser_{}", uuid),
        };

        // テスト用DBのセットアップ
        let db = TestDb::new().await;

        // 依存関係の初期化
        let s3_client = Self::init_s3_client().await;

        // ユースケースの初期化
        let auth_usecase = di::init_auth_usecase(&db.db, s3_client.clone());
        let company_usecase = di::init_company_usecase(&db.db);

        let instance = Self {
            auth_usecase,
            company_usecase,
            db: db.db.clone(),
            s3_client,
            test_user,
            access_token: None,
            refresh_token: None,
        };

        // テストユーザーの登録
        instance.register_test_user().await;

        instance
    }

    /// S3クライアントの初期化
    async fn init_s3_client() -> Arc<S3Client> {
        let s3_config = s3::init_s3_config()
            .await
            .expect("Failed to initialize S3 config");
        Arc::new(S3Client::new(s3_config))
    }

    /// テストユーザーの登録
    async fn register_test_user(&self) {
        self.auth_usecase
            .register(&web::Json(&self.test_user))
            .await
            .expect("Failed to register test user");
    }

    pub async fn build_test_app(
        &self,
    ) -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        // JWT認証のミドルウェアを設定
        let auth_usecase = self.auth_usecase.clone();
        let jwt_auth = HttpAuthentication::bearer(move |req, credentials| {
            let auth_usecase = auth_usecase.clone();
            Box::pin(
                async move { jwt::validator(req, credentials, web::Data::new(auth_usecase)).await },
            )
        });

        test::init_service(
            App::new()
                .wrap(csrf::csrf_middleware())
                .wrap(SecurityHeaders)
                .app_data(web::Data::new(self.auth_usecase.clone()))
                .app_data(web::Data::new(self.company_usecase.clone()))
                .app_data(json_error_handler())
                .service(
                    web::scope("/api")
                        .service(
                            web::scope("/auth")
                                .service(login)
                                .service(register)
                                .service(refresh)
                                .service(
                                    // logoutのみ認証ミドルウェアを適用
                                    web::scope("").wrap(jwt_auth.clone()).service(logout),
                                ),
                        )
                        // 認証が必要なAPIルート
                        .service(
                            web::scope("")
                                .wrap(jwt_auth)
                                .service(api::routes::users_scope())
                                .service(api::routes::projects_scope())
                                .service(api::routes::work_logs_scope())
                                .service(api::routes::companies_scope()),
                        ),
                ),
        )
        .await
    }

    /// ログインしてトークンを保存
    pub async fn login(&mut self) {
        let payload = json!({
            "email": self.test_user.email,
            "password": self.test_user.password
        });

        let app = self.build_test_app().await;
        let login_response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/login/")
                .set_json(&payload)
                .to_request(),
        )
        .await;

        let cookies: Vec<_> = login_response.response().cookies().collect();

        self.access_token = cookies
            .iter()
            .find(|c| c.name() == "access_token")
            .map(|c| c.value().to_string());

        self.refresh_token = cookies
            .iter()
            .find(|c| c.name() == "refresh_token")
            .map(|c| c.value().to_string());

        assert!(
            self.access_token.is_some(),
            "アクセストークンの取得に失敗しました"
        );
        assert!(
            self.refresh_token.is_some(),
            "リフレッシュトークンの取得に失敗しました"
        );
    }

    /// 認証付き共通リクエストを実行
    pub async fn request<T: serde::de::DeserializeOwned>(
        &mut self,
        method: test::TestRequest,
        endpoint: &str,
        app: &impl actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
    ) -> Result<ApiResponse<T>, ApiError> {
        if self.access_token.is_none() || self.refresh_token.is_none() {
            panic!("未ログインです。先にlogin()を実行してください。");
        }

        let request = method.uri(endpoint).insert_header((
            "Authorization",
            format!("Bearer {}", self.access_token.as_ref().unwrap()),
        ));

        let response = test::call_service(app, request.to_request()).await;

        let status = response.status();
        let body = test::read_body(response).await;

        if status.is_success() {
            let body: T =
                serde_json::from_slice(&body).expect("Failed to deserialize response body");
            Ok(ApiResponse { status, body })
        } else {
            Err(ApiError { status, body })
        }
    }
}
