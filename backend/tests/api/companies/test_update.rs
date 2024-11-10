use crate::api::helper::validation::assert_validation_error_with_custom_error;
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use lazy_static::lazy_static;
use rstest::rstest;
use serde_json::json;

lazy_static! {
    static ref UPDATE_PAYLOAD: serde_json::Value = json!({
        "company_name": "更新後企業名",
        "establishment_year": 2020,
        "location": "東京都渋谷区",
        "website_url": "https://example-updated.com",
        "employee_count": 200,
        "annual_sales": {
            "amount": 200_000_000,
            "fiscal_year": 2024
        },
        "contract_type": "Contract",
        "major_clients": ["新規クライアントA", "新規クライアントB"],
        "major_services": ["新規サービスA", "新規サービスB"],
        "average_hourly_rate": 5000,
        "bonus": {
            "amount": 2_000_000,
            "frequency": 2
        },
        "status": "Contract",
        "affiliation_start_date": "2023-04-01",
        "affiliation_end_date": "2024-03-31"
    });
}

#[actix_web::test]
async fn test_update_company_success() {
    /*
    企業の更新が成功することを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let company_id = test_app.create_test_company().await;

    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::put()
                .uri(&format!("/api/companies/{}/", company_id))
                .set_json(&*UPDATE_PAYLOAD),
        )
        .await;

    let res = test::call_service(&app, request.to_request()).await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[actix_web::test]
async fn test_update_company_unauthorized() {
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
        test::TestRequest::put()
            .uri(&format!("/api/companies/{}/", company_id))
            .set_json(&*UPDATE_PAYLOAD)
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_update_company_not_found() {
    /*
    存在しない企業の更新を試みた場合、404エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let non_existent_id = "507f1f77bcf86cd799439011";

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::put()
                .uri(&format!("/api/companies/{}/", non_existent_id))
                .set_json(&*UPDATE_PAYLOAD),
        )
        .await;

    let res = test::call_service(&app, request.to_request()).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

// バリデーションテスト用の構造体
#[derive(Debug)]
struct ValidationTestCase {
    name: &'static str,
    payload: serde_json::Value,
    field: &'static str,
    expected_message: &'static str,
}

#[rstest]
#[case::invalid_company_name(
    ValidationTestCase {
        name: "企業名が空",
        payload: json!({
            "company_name": "",
            "establishment_year": 2020,
            "location": "東京都渋谷区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01"
        }),
        field: "company_name",
        expected_message: "企業名は1文字以上である必要があります"
    }
)]
#[case::end_date_before_start_date(
    ValidationTestCase {
        name: "契約終了日が開始日より前",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都渋谷区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01",
            "affiliation_end_date": "2023-03-31"
        }),
        field: "dates",
        expected_message: "契約終了日は契約開始日より後である必要があります"
    }
)]
#[actix_web::test]
async fn test_update_company_validation(#[case] test_case: ValidationTestCase) {
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // テスト用の企業を作成
    let company_id = test_app.create_test_company().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::put()
                .uri(&format!("/api/companies/{}/", company_id))
                .set_json(&test_case.payload),
        )
        .await;

    let resp = test::call_service(&app, request.to_request()).await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "バリデーションテスト失敗: {}",
        test_case.name
    );

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_validation_error_with_custom_error(&body, test_case.field, test_case.expected_message);
}
