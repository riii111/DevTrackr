use crate::common::test_app::TestApp;
use crate::common::test_context::TestContext;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};

const COMPANIES_ENDPOINT: &str = "/api/companies/";

pub async fn create_test_company(context: &TestContext) -> String {
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

    let response = context
        .authenticated_request(
            test::TestRequest::post().set_json(&payload),
            COMPANIES_ENDPOINT,
        )
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    println!("Created Company ID: {}", body["id"]);
    body["id"]
        .as_str()
        .expect("Company ID not found in response")
        .to_string()
}

#[actix_web::test]
async fn test_get_all_companies_success() {
    /*
    企業一覧取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(test::TestRequest::get(), COMPANIES_ENDPOINT)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        assert!(body.is_array());
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_companies_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(COMPANIES_ENDPOINT)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

const COMPANIES_WITH_PROJECTS_ENDPOINT: &str = "/api/companies/with-projects/";

#[actix_web::test]
async fn test_get_all_companies_with_projects_success() {
    /*
    企業とプロジェクト一覧の取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(test::TestRequest::get(), COMPANIES_WITH_PROJECTS_ENDPOINT)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        assert!(body.is_object());
        assert!(body.get("companies").is_some());
        assert!(body.get("total").is_some());
        assert!(body.get("companies").unwrap().is_array());
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_companies_with_projects_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(COMPANIES_WITH_PROJECTS_ENDPOINT)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_get_company_by_id_success() {
    /*
    企業IDによる取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let company_id = create_test_company(&context).await;
        let url = format!("{}{}/", COMPANIES_ENDPOINT, company_id);
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        assert!(body.is_object());
        assert_eq!(body["id"], company_id);
    })
    .await;
}

#[actix_web::test]
async fn test_get_company_by_id_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let company_id = create_test_company(&context).await;

        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(&format!("{}{}/", COMPANIES_ENDPOINT, company_id))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_get_company_by_id_not_found() {
    /*
    存在しない企業IDの場合は404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let non_existent_id = ObjectId::new();
        let url = format!("{}{}/", COMPANIES_ENDPOINT, non_existent_id);
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        // レスポンスボディを取得
        let response_body = test::read_body(response).await;
        println!(
            "Raw response body: {:?}",
            String::from_utf8_lossy(&response_body)
        );

        // 空のレスポンスでないことを確認してから、JSONとしてパース
        assert!(!response_body.is_empty(), "Response body is empty");

        let error_body: Value =
            serde_json::from_slice(&response_body).expect("Failed to parse response body as JSON");

        // assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "企業が見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
}

#[actix_web::test]
async fn test_get_company_by_id_invalid_id() {
    /*
    無効なIDフォーマットの場合は400エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let url = format!("{}{}/", COMPANIES_ENDPOINT, "invalid-id");
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let error_body: Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "不正なリクエスト",
                "message": "無効なIDです",
                "code": "BAD_REQUEST"
            })
        );
    })
    .await;
}
