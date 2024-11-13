use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};

const LOGOUT_ENDPOINT: &str = "/api/auth/logout/";

#[actix_web::test]
async fn test_logout_success() {
    /*
    ログアウトが成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // ログアウトを実行
        let res = context
            .authenticated_request(test::TestRequest::post(), LOGOUT_ENDPOINT)
            .await;

        // レスポンスの検証
        assert_eq!(res.status(), StatusCode::OK);

        // Cookieの検証
        context.assert_auth_cookies_cleared(&res);
    })
    .await;
}

#[actix_web::test]
async fn test_logout_unauthorized() {
    /*
    認証トークンなしでリクエストした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        // 認証トークンなしでリクエスト
        let response = test::call_service(
            context.service(),
            test::TestRequest::post().uri(LOGOUT_ENDPOINT).to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}
