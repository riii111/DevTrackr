use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};

const LOGOUT_ENDPOINT: &str = "/api/auth/logout/";

#[actix_web::test]
async fn test_logout_success() {
    /*
    ログアウトが成功することを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // ログアウトを実行
    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(LOGOUT_ENDPOINT)
            .insert_header((
                "Authorization",
                format!("Bearer {}", test_app.access_token.as_ref().unwrap()),
            ))
            .to_request(),
    )
    .await;

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::OK);

    // Cookieの検証
    let cookies_after: Vec<_> = res.response().cookies().collect();
    for cookie in cookies_after {
        if cookie.name() == "access_token" || cookie.name() == "refresh_token" {
            assert!(cookie.value().is_empty());
            assert_eq!(
                cookie.max_age(),
                Some(actix_web::cookie::time::Duration::ZERO)
            );
        }
    }
}

#[actix_web::test]
async fn test_logout_unauthorized() {
    /*
    認証トークンなしでリクエストした場合は401エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 認証トークンなしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::post().uri(LOGOUT_ENDPOINT).to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
