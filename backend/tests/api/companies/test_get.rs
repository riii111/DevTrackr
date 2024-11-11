use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};

const COMPANIES_ENDPOINT: &str = "/api/companies/";

pub async fn create_test_company(test_app: &mut TestApp) -> String {
    let payload = json!({
        "company_name": "テスト企業",
        "establishment_year": 2020,
        "location": "東京都渋谷区",
        "website_url": "https://example.com",
        "employee_count": 100,
        "annual_sales": {
            "amount": 100_000_000,
            "fiscal_year": 2024
        },
        "contract_type": "Contract",
        "major_clients": ["新規クライアントC", "新規クライアントD"],
        "major_services": ["新規サービスC", "新規サービスD"],
        "average_hourly_rate": 5000,
        "bonus": {
            "amount": 1_000_000,
            "frequency": 1
        },
        "status": "Cancelled",
        "affiliation_start_date": "2020-08-05",
        "affiliation_end_date": "2021-08-04"
    });

    let response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::post().set_json(&payload),
            COMPANIES_ENDPOINT,
            &test_app.build_test_app().await,
        )
        .await
        .expect("Failed to create company");

    response.body["id"]
        .as_str()
        .expect("Company ID not found in response")
        .to_string()
}

#[actix_web::test]
async fn test_get_all_companies_success() {
    /*
    企業一覧取得が成功することを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // APIリクエストの実行
    let response = test_app
        .request::<Value>(test::TestRequest::get(), COMPANIES_ENDPOINT, &app)
        .await
        .expect("Failed to get companies");

    // レスポンスの検証
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.is_array());
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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // APIリクエストの実行
    let response = test_app
        .request::<Value>(
            test::TestRequest::get(),
            COMPANIES_WITH_PROJECTS_ENDPOINT,
            &app,
        )
        .await
        .expect("Failed to get companies with projects");

    // レスポンスの検証
    assert_eq!(response.status, StatusCode::OK);

    // レスポンス構造の検証
    assert!(response.body.is_object());
    assert!(response.body.get("companies").is_some());
    assert!(response.body.get("total").is_some());

    // companiesが配列であることを確認
    let companies = response.body.get("companies").unwrap();
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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // テスト用の企業を作成
    let company_id = create_test_company(&mut test_app).await;

    // APIリクエストの実行
    let response = test_app
        .request::<Value>(
            test::TestRequest::get(),
            &format!("{}{}", COMPANIES_ENDPOINT, company_id),
            &app,
        )
        .await
        .expect("Failed to get company");

    // レスポンスの検証
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.is_object());
    assert_eq!(response.body["_id"], company_id);
}

#[actix_web::test]
async fn test_get_company_by_id_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // テスト用の企業を作成
    let company_id = create_test_company(&mut test_app).await;

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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 存在しない企業ID
    let non_existent_id = ObjectId::new();

    // ログインを実行
    test_app.login().await;

    // APIリクエストの実行
    let response = test_app
        .request::<Value>(
            test::TestRequest::get(),
            &format!("{}{}", COMPANIES_ENDPOINT, non_existent_id),
            &app,
        )
        .await;

    match response {
        Ok(_) => panic!("Expected not found error"),
        Err(error) => {
            assert_eq!(error.status(), StatusCode::NOT_FOUND);
            let error_body: serde_json::Value =
                error.json().await.expect("Failed to parse error response");

            assert_eq!(
                error_body,
                json!({
                    "error": "リソースが見つかりません",
                    "message": "企業が見つかりません",
                    "code": "NOT_FOUND"
                })
            );
        }
    }
}

#[actix_web::test]
async fn test_get_company_by_id_invalid_id() {
    /*
    無効なIDフォーマットの場合は400エラーが返ることを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行
    test_app.login().await;

    // APIリクエストの実行
    let response = test_app
        .request::<Value>(
            test::TestRequest::get(),
            &format!("{}invalid-id", COMPANIES_ENDPOINT),
            &app,
        )
        .await;

    match response {
        Ok(_) => panic!("Expected bad request error"),
        Err(error) => {
            assert_eq!(error.status(), StatusCode::BAD_REQUEST);
            let error_body: serde_json::Value =
                error.json().await.expect("Failed to parse error response");

            assert_eq!(
                error_body,
                json!({
                    "error": "不正なリクエスト",
                    "message": "無効なIDです",
                    "code": "BAD_REQUEST"
                })
            );
        }
    }
}
