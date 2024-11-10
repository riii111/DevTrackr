use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};

const COMPANIES_ENDPOINT: &str = "/api/companies/";

#[actix_web::test]
async fn test_get_all_companies_success() {
    /*
    企業一覧取得が成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(test::TestRequest::get().uri(COMPANIES_ENDPOINT))
        .await;

    // APIリクエストの実行
    let res = test::call_service(&app, request.to_request()).await;
    println!("{:?}", res);

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::OK);

    // レスポンスボディの検証
    let body: Value = test::read_body_json(res).await;
    assert!(body.is_array());
}

#[actix_web::test]
async fn test_get_all_companies_unauthorized() {
    // テストアプリケーションの作成
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 認証なしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(COMPANIES_ENDPOINT)
            .to_request(),
    )
    .await;

    // 認証エラーの検証
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

const COMPANIES_WITH_PROJECTS_ENDPOINT: &str = "/api/companies/with-projects/";

#[actix_web::test]
async fn test_get_all_companies_with_projects_success() {
    /*
    企業とプロジェクト一覧の取得が成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::get().uri(COMPANIES_WITH_PROJECTS_ENDPOINT),
        )
        .await;

    // APIリクエストの実行
    let res = test::call_service(&app, request.to_request()).await;

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::OK);

    // レスポンスボディの検証
    let body: Value = test::read_body_json(res).await;

    // レスポンス構造の検証
    assert!(body.is_object());
    assert!(body.get("companies").is_some());
    assert!(body.get("total").is_some());

    // companiesが配列であることを確認
    let companies = body.get("companies").unwrap();
    assert!(companies.is_array());
}

#[actix_web::test]
async fn test_get_all_companies_with_projects_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 認証なしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(COMPANIES_WITH_PROJECTS_ENDPOINT)
            .to_request(),
    )
    .await;

    // 認証エラーの検証
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_get_company_by_id_success() {
    /*
    企業IDによる取得が成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // テスト用の企業を作成
    let company_id = test_app.create_test_company().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::get().uri(&format!("/api/companies/{}/", company_id)),
        )
        .await;

    // APIリクエストの実行
    let res = test::call_service(&app, request.to_request()).await;

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::OK);

    // レスポンスボディの検証
    let body: Value = test::read_body_json(res).await;
    assert!(body.is_object());
    assert_eq!(body["_id"], company_id.to_string());
}

#[actix_web::test]
async fn test_get_company_by_id_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // テスト用の企業を作成
    let company_id = test_app.create_test_company().await;

    // 認証なしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/companies/{}/", company_id))
            .to_request(),
    )
    .await;

    // 認証エラーの検証
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_get_company_by_id_not_found() {
    /*
    存在しない企業IDの場合は404エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 存在しない企業ID
    let non_existent_id = ObjectId::new();

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::get().uri(&format!("/api/companies/{}/", non_existent_id)),
        )
        .await;

    // APIリクエストの実行
    let res = test::call_service(&app, request.to_request()).await;

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body: Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        json!({
            "error": "リソースが見つかりません",
            "message": "企業が見つかりません",
            "code": "NOT_FOUND"
        })
    );
}

#[actix_web::test]
async fn test_get_company_by_id_invalid_id() {
    /*
    無効なIDフォーマットの場合は400エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(test::TestRequest::get().uri("/api/companies/invalid-id/"))
        .await;

    // APIリクエストの実行
    let res = test::call_service(&app, request.to_request()).await;

    // レスポンスの検証
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body: Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        json!({
            "error": "不正なリクエスト",
            "message": "無効なIDです",
            "code": "BAD_REQUEST"
        })
    );
}
