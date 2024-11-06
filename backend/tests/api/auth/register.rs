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

        // let body: serde_json::Value = test::read_body_json(res).await;
        // assert_eq!(body["message"], "ユーザー登録に成功しました");

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
        let first_login_cookie = cookies
            .iter()
            .find(|c| c.starts_with("firstLogin="))
            .unwrap();

        // アクセストークンのCookie属性を確認
        assert!(access_token_cookie.contains("Path=/"));
        assert!(!access_token_cookie.contains("HttpOnly")); // フロントエンドでJSから読み取れる必要がある

        // リフレッシュトークンのCookie属性を確認
        assert!(refresh_token_cookie.contains("Path=/"));
        assert!(refresh_token_cookie.contains("HttpOnly")); // セキュリティ対策のためJSからアクセス不可

        // 初回ログインのCookie属性を確認
        assert!(first_login_cookie.contains("Path=/"));
        assert!(!first_login_cookie.contains("HttpOnly")); // JavaScriptからアクセスできるようにする
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

        // 1人目のユーザーを登録
        test_app
            .create_new_user("duplicate@example.com", "password123", "firstuser")
            .await
            .expect("Failed to create first user");

        // 2人目のユーザーを同じメールアドレスで登録
        let payload = json!({
            "email": "duplicate@example.com",
            "password": "different_password",
            "username": "seconduser"
        });

        let req = test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body: serde_json::Value = test::read_body_json(res).await;
        assert_eq!(body["error"], "ユニーク制約違反");
    })
    .await
    .expect("Test timed out");
}
