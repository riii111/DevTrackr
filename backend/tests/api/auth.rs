use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use serde_json::json;
use tokio::time::timeout;

const LOGIN_ENDPOINT: &str = "/api/auth/login/";
const REGISTER_ENDPOINT: &str = "/api/auth/register/";
const TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

#[actix_web::test]
async fn test_login_success() {
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        // テストユーザーの作成
        let email = "test@example.com";
        let password = "testpassword123";
        test_app
            .create_test_user(email, password)
            .await
            .expect("Failed to create test user");

        let payload = json!({
            "email": email,
            "password": password
        });

        let req = test::TestRequest::post()
            .uri(LOGIN_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(res).await;
        assert!(body.get("access_token").is_some());
        assert!(body.get("refresh_token").is_some());
    })
    .await
    .expect("Test timed out");
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        let payload = json!({
            "email": "test@example.com",
            "password": "wrongpassword"
        });

        let req = test::TestRequest::post()
            .uri(LOGIN_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body: serde_json::Value = test::read_body_json(res).await;
        assert_eq!(body, json!({"error": "認証に失敗しました"}));
    })
    .await
    .expect("Test timed out");
}

#[actix_web::test]
async fn test_register_success() {
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        let payload = json!({
            "email": "newuser@example.com",
            "password": "newpassword123"
        });

        let req = test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let body: serde_json::Value = test::read_body_json(res).await;
        assert_eq!(body["message"], "ユーザー登録に成功しました");
    })
    .await
    .expect("Test timed out");
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        // 最初のユーザーを登録
        let email = "duplicate@example.com";
        let password = "testpass123";
        test_app
            .create_test_user(email, password)
            .await
            .expect("Failed to create test user");

        // 同じメールアドレスで再度登録を試みる
        let payload = json!({
            "email": email,
            "password": "different_password"
        });

        let req = test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CONFLICT);
    })
    .await
    .expect("Test timed out");
}
