use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use serde_json::json;
use tokio::time::timeout;

const REGISTER_ENDPOINT: &str = "/api/auth/register/";
const TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

#[actix_web::test]
async fn test_register_success() {
    /*
    ユーザー登録が成功することを確認するテスト
    */
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        let payload = json!({
            "email": "newuser@example.com",
            "password": "newpassword123",
            "username": "newuser"
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
    /*
    同じメールアドレスで登録できないことを確認するテスト
    */
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        // 最初のユーザーを登録
        test_app
            .create_test_user()
            .await
            .expect("Failed to create test user");

        // 同じメールアドレスで再度登録を試みる
        let payload = json!({
            "email": test_app.test_user.email,
            "password": "different_password",
            "username": "duplicate_user".to_string()
        });

        let req = test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    })
    .await
    .expect("Test timed out");
}
