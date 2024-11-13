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

use crate::common::test_context::TestContext;
use crate::common::test_db::TestDb;
use actix_web::{
    test,
    web::{self},
    App,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use devtrackr_api::{
    api::{
        self,
        endpoints::auth::{login, logout, refresh, register},
    },
    clients::{self, aws_s3::S3Client},
    config::{self, di},
    errors::app_error::json_error_handler,
    middleware::{csrf, jwt, security_headers::SecurityHeaders},
    models::users::UserCreate,
    repositories::{auth::MongoAuthRepository, companies::MongoCompanyRepository},
    usecases::{auth::AuthUseCase, companies::CompanyUseCase},
};
use serde_json::json;
use std::future::Future;
use std::sync::Arc;
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestApp {
    pub auth_usecase: Arc<AuthUseCase<MongoAuthRepository>>,
    pub company_usecase: Arc<CompanyUseCase<MongoCompanyRepository>>,
    pub test_db: TestDb,
    pub s3_client: Arc<S3Client>,
    pub test_user: UserCreate,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl TestApp {
    pub async fn new() -> Result<Self, anyhow::Error> {
        // 環境変数のセットアップ
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
        let test_db = TestDb::new().await?;
        let db = test_db.db.clone();

        // S3Clientの初期化
        let s3_config = match config::s3::init_s3_config().await {
            Ok(client) => {
                log::info!("Successfully initialized S3 (MinIO) client");
                client
            }
            Err(e) => {
                log::error!("S3 (MinIO) クライアントの初期化に失敗しました: {}", e);
                return Err(anyhow::anyhow!(
                    "S3 (MinIO) クライアントの初期化に失敗しました"
                ));
            }
        };
        let s3_client = Arc::new(clients::aws_s3::S3Client::new(s3_config.clone()));

        // ユースケースの初期化
        let auth_usecase = di::init_auth_usecase(&db, s3_client.clone());
        let company_usecase = di::init_company_usecase(&db);

        let instance = Self {
            auth_usecase,
            company_usecase,
            test_db,
            s3_client,
            test_user,
            access_token: None,
            refresh_token: None,
        };

        // テストユーザーの登録
        instance.register_test_user().await;

        Ok(instance)
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

    /// テストの実行
    pub async fn run_test<F, Fut>(f: F)
    where
        F: FnOnce(TestContext) -> Fut,
        Fut: Future<Output = ()>,
    {
        let context = TestContext::new().await;
        f(context).await;
    }

    /// 認証付きテストの実行
    pub async fn run_authenticated_test<F, Fut>(f: F)
    where
        F: FnOnce(TestContext) -> Fut,
        Fut: Future<Output = ()>,
    {
        let context = TestContext::with_auth().await;
        f(context).await;
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
}
