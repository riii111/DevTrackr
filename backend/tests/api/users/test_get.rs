use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use serde_json::{json, Value};

const USERS_ENDPOINT: &str = "/api/users/me/";

#[actix_web::test]
async fn test_get_current_user_success() {
    /*
    現在のユーザー情報の取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(test::TestRequest::get(), USERS_ENDPOINT)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        // レスポンスの構造を確認
        assert!(body.is_object());
        assert!(body.get("id").is_some());
        assert!(body.get("email").is_some());
        assert!(body.get("username").is_some());
    })
    .await;
}

#[actix_web::test]
async fn test_get_current_user_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::get().uri(USERS_ENDPOINT).to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}
