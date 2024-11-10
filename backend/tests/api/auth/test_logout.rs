use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};

const LOGOUT_ENDPOINT: &str = "/api/auth/logout/";

#[actix_web::test]
async fn test_logout_success() {
    /*
    ログアウトが成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (login_response, logout_req) = test_app
        .login_and_create_next_request(test::TestRequest::post().uri(LOGOUT_ENDPOINT))
        .await;

    // ログイン時のCookieを確認
    let cookies_before = login_response.response().cookies().collect::<Vec<_>>();
    assert!(cookies_before.iter().any(|c| c.name() == "access_token"));
    assert!(cookies_before.iter().any(|c| c.name() == "refresh_token"));

    // ログアウトを実行
    let res = test::call_service(&app, logout_req.to_request()).await;

    assert_eq!(res.status(), StatusCode::OK);

    // ログアウト後のレスポンスでCookieが削除されていることを確認
    let cookies_after = res.response().cookies().collect::<Vec<_>>();
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
