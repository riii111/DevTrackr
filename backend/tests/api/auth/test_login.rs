use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use serde_json::json;
use tokio::time::timeout;

const LOGIN_ENDPOINT: &str = "/api/auth/login/";
const TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

#[actix_web::test]
async fn test_login_success() {
    timeout(TEST_TIMEOUT, async {
        let test_app = TestApp::new().await;
        let app = test_app.build_test_app().await;

        let payload = json!({
            "email": test_app.test_user.email,
            "password": test_app.test_user.password
        });

        let req = test::TestRequest::post()
            .uri(LOGIN_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        // Cookieヘッダーの取得と検証
        let cookies: Vec<_> = res
            .headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .map(|v| v.to_str().unwrap())
            .collect();

        // 必要なCookieが存在することを確認
        assert!(cookies.iter().any(|c| c.starts_with("access_token=")));
        assert!(cookies.iter().any(|c| c.starts_with("refresh_token=")));

        // Cookieの属性を確認
        let access_token_cookie = cookies
            .iter()
            .find(|c| c.starts_with("access_token="))
            .unwrap();
        let refresh_token_cookie = cookies
            .iter()
            .find(|c| c.starts_with("refresh_token="))
            .unwrap();

        // アクセストークンのCookie属性を確認
        assert!(access_token_cookie.contains("Path=/"));
        assert!(!access_token_cookie.contains("HttpOnly")); // フロントエンドでJSから読み取れる必要がある

        // リフレッシュトークンのCookie属性を確認
        assert!(refresh_token_cookie.contains("Path=/"));
        assert!(refresh_token_cookie.contains("HttpOnly")); // セキュリティのためJSからアクセス不可
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
