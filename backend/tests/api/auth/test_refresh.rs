use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use chrono::{Duration, Utc};
use devtrackr_api::models::auth::AuthTokenInDB;
use devtrackr_api::utils::jwt;
use mongodb::bson::{doc, DateTime as BsonDateTime};
use serde_json::{json, Value};

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

// Cookie検証用の構造体
struct CookieCheck<'a> {
    name: &'a str,
    should_be_http_only: bool,
}

const COOKIE_CHECKS: &[CookieCheck<'static>] = &[
    CookieCheck {
        name: "access_token",
        should_be_http_only: false,
    },
    CookieCheck {
        name: "refresh_token",
        should_be_http_only: true,
    },
];

#[actix_web::test]
async fn test_refresh_success() {
    /*
    トークンのリフレッシュが成功することを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // 初回ログイン時のアクセストークンを取得
    let initial_token = test_app.access_token.clone().unwrap();
    let refresh_token = test_app.refresh_token.clone().unwrap();

    // 少し待機して時間差を作る
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // リフレッシュを実行
    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REFRESH_ENDPOINT)
            .cookie(
                actix_web::cookie::Cookie::build("refresh_token", &refresh_token)
                    .path("/")
                    .finish(),
            )
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
    println!("Initial token: {}", initial_token);
    println!("New token: {}", new_access_token);

    // トークンをデコードして中身を確認
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes();
    let decoded_original =
        jwt::verify_token(&initial_token, &jwt_secret).expect("Failed to decode original token");
    let decoded_new =
        jwt::verify_token(new_access_token, &jwt_secret).expect("Failed to decode new token");

    println!("\n=== Decoded Claims ===");
    println!("Original claims: {:?}", decoded_original);
    println!("New claims: {:?}", decoded_new);
    println!("=====================\n");

    assert_ne!(
        initial_token, new_access_token,
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
        .find_one(doc! { "refresh_token": refresh_token }, None)
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

// 最初のトークンを取得
// let initial_token = test_app.access_token.clone().unwrap();
// let refresh_token = test_app.refresh_token.clone().unwrap();

// let jwt_secret = std::env::var("JWT_SECRET")
//     .expect("JWT_SECRET must be set")
//     .into_bytes();
// let initial_decoded =
//     jwt::verify_token(&initial_token, &jwt_secret).expect("Failed to decode initial token");

// // リフレッシュを実行
// let response = test::call_service(
//     &app,
//     test::TestRequest::post()
//         .uri(REFRESH_ENDPOINT)
//         .cookie(
//             actix_web::cookie::Cookie::build("refresh_token", &refresh_token)
//                 .path("/")
//                 .http_only(true)
//                 .finish(),
//         )
//         .to_request(),
// )
// .await;

// assert_eq!(response.status(), StatusCode::OK);

// // Cookieの検証
// let cookies: Vec<_> = response
//     .headers()
//     .get_all(actix_web::http::header::SET_COOKIE)
//     .map(|v| v.to_str().unwrap())
//     .collect();

// // 必要なCookieが存在することを確認
// for check in COOKIE_CHECKS {
//     let cookie = cookies
//         .iter()
//         .find(|c| c.starts_with(&format!("{}=", check.name)))
//         .unwrap_or_else(|| panic!("{} cookie not found", check.name));

//     assert!(cookie.contains("Path=/"));
//     assert_eq!(
//         cookie.contains("HttpOnly"),
//         check.should_be_http_only,
//         "Unexpected HttpOnly flag for {} cookie",
//         check.name
//     );
// }

// // 新しいアクセストークンを取得
// let new_access_token = cookies
//     .iter()
//     .find(|c| c.starts_with("access_token="))
//     .expect("アクセストークンが見つかりません");

// // トークン値を抽出
// let token_value = new_access_token
//     .split(';')
//     .next()
//     .unwrap()
//     .trim_start_matches("access_token=");

// // トークンの検証
// let decoded_new =
//     jwt::verify_token(token_value, &jwt_secret).expect("Failed to decode new token");

// // 新しいトークンが古いトークンと異なることを確認
// assert!(
//     decoded_new.iat > initial_decoded.iat,
//     "新しいトークンの発行時刻が古いトークンより後になっていません"
// );

// // DBでの検証
// let collection = test_app.db.collection::<AuthTokenInDB>("auth_tokens");
// let token = collection
//     .find_one(doc! { "access_token": token_value }, None)
//     .await
//     .expect("DBクエリに失敗")
//     .expect("トークンが見つかりません");

// assert!(token.refresh_expires_at > BsonDateTime::now());
// }

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
        json!({
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
        json!({
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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    let access_token = test_app.access_token.clone().unwrap();
    let refresh_token = test_app.refresh_token.clone().unwrap();

    // トークンを期限切れにする
    expire_refresh_token(&test_app, &refresh_token).await;

    // 期限切れトークンでリフレッシュを試行
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REFRESH_ENDPOINT)
            .cookie(
                actix_web::cookie::Cookie::build("access_token", &access_token)
                    .path("/")
                    .finish(),
            )
            .cookie(
                actix_web::cookie::Cookie::build("refresh_token", &refresh_token)
                    .path("/")
                    .finish(),
            )
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(
        body,
        json!({
            "error": "不正なリクエスト",
            "message": "無効なリクエストです",
            "code": "BAD_REQUEST"
        })
    );
}
