use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use chrono::{Duration, Utc};
use devtrackr_api::models::auth::AuthTokenInDB;
use devtrackr_api::utils::jwt;
use mongodb::bson::{doc, DateTime as BsonDateTime};
use serde_json::Value;

const REFRESH_ENDPOINT: &str = "/api/auth/refresh/";

/// テスト用ヘルパー関数. リフレッシュトークンを期限切れにする
pub async fn expire_refresh_token(test_app: &TestApp, refresh_token: &str) {
    let expired_time = Utc::now() - Duration::days(1);
    let collection = test_app.db.collection::<AuthTokenInDB>("auth_tokens");

    collection
        .update_one(
            doc! { "refresh_token": refresh_token },
            doc! { "$set": { "refresh_expires_at": BsonDateTime::from_chrono(expired_time) } },
            None,
        )
        .await
        .expect("リフレッシュトークンの期限切れ設定に失敗しました");
}

#[actix_web::test]
async fn test_refresh_success() {
    /*
    トークンのリフレッシュが成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (login_response, _) = test_app
        .login_and_create_next_request(test::TestRequest::post().uri(REFRESH_ENDPOINT))
        .await;

    // 初回ログイン時のアクセストークンを取得
    let original_access_token = login_response
        .response()
        .cookies()
        .find(|c| c.name() == "access_token")
        .expect("初回ログイン時のアクセストークンが見つかりません")
        .value()
        .to_string();

    // 少し待機して時間差を作る
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // リフレッシュを実行
    let refresh_token = login_response
        .response()
        .cookies()
        .find(|c| c.name() == "refresh_token")
        .expect("リフレッシュトークンが見つかりません");

    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REFRESH_ENDPOINT)
            .cookie(refresh_token.clone())
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);

    // Cookieの検証
    let cookies: Vec<_> = res
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .map(|v| v.to_str().unwrap())
        .collect();

    // アクセストークンのCookieが存在することを確認
    let new_access_token_cookie = cookies
        .iter()
        .find(|c| c.starts_with("access_token="))
        .expect("アクセストークンのCookieが見つかりません");

    // アクセストークンの値を抽出
    let new_access_token = new_access_token_cookie
        .split(';')
        .next()
        .unwrap()
        .trim_start_matches("access_token=");

    // デバッグ出力を先に行う
    println!("\n=== Token Comparison ===");
    println!("Original token: {}", original_access_token);
    println!("New token: {}", new_access_token);

    // トークンをデコードして中身を確認
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes();
    let decoded_original = jwt::verify_token(&original_access_token, &jwt_secret)
        .expect("Failed to decode original token");
    let decoded_new =
        jwt::verify_token(new_access_token, &jwt_secret).expect("Failed to decode new token");

    println!("\n=== Decoded Claims ===");
    println!("Original claims: {:?}", decoded_original);
    println!("New claims: {:?}", decoded_new);
    println!("=====================\n");

    // アサーションを後に行う
    assert_ne!(
        original_access_token, new_access_token,
        "アクセストークンが更新されていません"
    );

    assert!(new_access_token_cookie.contains("Path=/"));
    assert!(
        !new_access_token_cookie.contains("HttpOnly"),
        "アクセストークンのCookieはHttpOnlyであるべきではありません"
    );

    // アクセストークンが更新されていることをDBでも確認
    let collection = test_app.db.collection::<AuthTokenInDB>("auth_tokens");
    let updated_token = collection
        .find_one(doc! { "refresh_token": refresh_token.value() }, None)
        .await
        .expect("DBクエリに失敗")
        .expect("トークンが見つかりません");

    assert_eq!(
        updated_token.access_token, new_access_token,
        "DBに保存されているアクセストークンが更新されていません"
    );

    assert!(
        decoded_new.iat > decoded_original.iat,
        "新しいトークンの発行時刻が古いトークン以降になっていません"
    );
}

#[actix_web::test]
async fn test_refresh_without_token() {
    /*
    リフレッシュトークンなしでリクエストした場合は400エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let res = test::call_service(
        &app,
        test::TestRequest::post().uri(REFRESH_ENDPOINT).to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body: Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        serde_json::json!({
            "error": "不正なリクエスト",
            "message": "無効なリクエストです",
            "code": "BAD_REQUEST"
        })
    );
}

#[actix_web::test]
async fn test_refresh_with_invalid_token() {
    /*
    無効なリフレッシュトークンでリクエストした場合は400エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let invalid_cookie = actix_web::cookie::Cookie::build("refresh_token", "invalid_token")
        .path("/")
        .finish();

    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REFRESH_ENDPOINT)
            .cookie(invalid_cookie)
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body: Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        serde_json::json!({
            "error": "不正なリクエスト",
            "message": "無効なリクエストです",
            "code": "BAD_REQUEST"
        })
    );
}

#[actix_web::test]
async fn test_refresh_with_expired_token() {
    /*
    期限切れのリフレッシュトークンでリクエストした場合は400エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (login_response, _) = test_app
        .login_and_create_next_request(test::TestRequest::post().uri(REFRESH_ENDPOINT))
        .await;

    let refresh_token = login_response
        .response()
        .cookies()
        .find(|c| c.name() == "refresh_token")
        .expect("テスト用のリフレッシュトークンが見つかりません");

    // トークンを期限切れにする（DBを直接操作）
    expire_refresh_token(&test_app, refresh_token.value()).await;

    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REFRESH_ENDPOINT)
            .cookie(refresh_token)
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body: Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        serde_json::json!({
            "error": "不正なリクエスト",
            "message": "無効なリクエストです",
            "code": "BAD_REQUEST"
        })
    );
}
