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
use actix_web::{dev::ServiceResponse, test, web, App};
use actix_web_httpauth::middleware::HttpAuthentication;
use devtrackr_api::{
    api,
    api::endpoints::auth::{login, logout, refresh, register},
    clients::aws_s3::S3Client,
    config::s3,
    errors::app_error::json_error_handler,
    middleware::{csrf, jwt, security_headers::SecurityHeaders},
    models::users::UserCreate,
    repositories::auth::MongoAuthRepository,
    usecases::auth::AuthUseCase,
};
use mongodb::bson::doc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestApp {
    pub auth_usecase: Arc<AuthUseCase<MongoAuthRepository>>,
    pub db: mongodb::Database,
    pub s3_client: Arc<S3Client>,
    pub test_user: UserCreate,
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

        // コレクションのセットアップ
        let users_collection = db.db.collection::<mongodb::bson::Document>("users");
        let _ = users_collection.drop(None).await;
        let _ = users_collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(Some(
                        mongodb::options::IndexOptions::builder()
                            .unique(true)
                            .build(),
                    ))
                    .build(),
                None,
            )
            .await;

        // 依存関係の初期化
        let s3_config = s3::init_s3_config()
            .await
            .expect("Failed to initialize S3 config");
        let s3_client = Arc::new(S3Client::new(s3_config));
        let auth_repository = Arc::new(MongoAuthRepository::new(&db.db));
        let jwt_secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .into_bytes();
        let auth_usecase = Arc::new(AuthUseCase::new(
            auth_repository,
            &jwt_secret,
            s3_client.clone(),
        ));

        let instance = Self {
            auth_usecase,
            db: db.db.clone(),
            s3_client,
            test_user,
        };

        // インスタンス生成時にテストユーザーを登録
        instance
            .auth_usecase
            .register(&web::Json(&instance.test_user))
            .await
            .expect("Failed to register test user");

        instance
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
                // main.rsと同じ順序でセキュリティミドルウェアを適用
                .wrap(csrf::csrf_middleware())
                .wrap(SecurityHeaders)
                .app_data(web::Data::new(self.auth_usecase.clone()))
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

    pub async fn create_new_user(
        &self,
        email: &str,
        password: &str,
        username: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let new_user = UserCreate {
            email: email.to_string(),
            password: password.to_string(),
            username: username.to_string(),
        };

        self.auth_usecase.register(&web::Json(&new_user)).await?;
        Ok(())
    }

    /// ログインを実行し、認証済みのリクエストを作成する
    pub async fn login_and_create_next_request(
        &self,
        method: test::TestRequest,
    ) -> (ServiceResponse, test::TestRequest) {
        let payload = json!({
            "email": self.test_user.email,
            "password": self.test_user.password
        });

        // ログイン実行
        let login_response = test::call_service(
            &self.build_test_app().await,
            test::TestRequest::post()
                .uri("/api/auth/login/")
                .set_json(&payload)
                .to_request(),
        )
        .await;

        // レスポンスからアクセストークンを取得
        let token = login_response
            .response()
            .cookies()
            .find(|c| c.name() == "access_token")
            .expect("アクセストークンが見つかりません")
            .value()
            .to_string();

        // 次のリクエストにアクセストークンを挿入
        let authenticated_request =
            method.insert_header(("Authorization", format!("Bearer {}", token)));
        (login_response, authenticated_request)
    }
}
