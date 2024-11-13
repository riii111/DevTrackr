use crate::api::helper::validation::assert_validation_error_with_custom_error;
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
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

pub async fn create_test_company(test_app: &mut TestApp) -> String {
    let app = test_app.build_test_app().await;

    let response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::post().set_json(json!({
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
            })),
            "/api/companies/",
            &app,
        )
        .await
        .expect("Failed to create company");

    response.body["id"].as_str().unwrap().to_string()
}

#[actix_web::test]
async fn test_update_company_success() {
    /*
    企業の更新が成功することを確認するテスト
     */
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログイン
    test_app.login().await;

    // テスト用の企業を作成
    let company_id = create_test_company(&mut test_app).await;

    // 企業情報の更新
    let update_response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::put().set_json(&*UPDATE_PAYLOAD),
            &format!("/api/companies/{}/", company_id),
            &app,
        )
        .await
        .expect("Failed to update company");

    assert_eq!(update_response.status, StatusCode::OK);

    // 更新後の企業情報を取得して検証
    let get_response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::get(),
            &format!("/api/companies/{}/", company_id),
            &app,
        )
        .await
        .expect("Failed to get updated company");

    assert_eq!(get_response.body["company_name"], "更新後企業名");
}

#[actix_web::test]
async fn test_update_company_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 認証なしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::put()
            .uri("/api/companies/123/")
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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    test_app.login().await;

    // 存在しない企業ID
    let non_existent_id = ObjectId::new();

    let response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::put().set_json(&*UPDATE_PAYLOAD),
            &format!("/api/companies/{}/", non_existent_id),
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
                    "message": "更新対象の企業が見つかりません",
                    "code": "NOT_FOUND"
                })
            );
        }
    }
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
    let mut test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログイン
    test_app.login().await;

    // テスト用の企業を作成
    let company_id = create_test_company(&mut test_app).await;

    // バリデーションエラーのテスト
    let response = test_app
        .request::<serde_json::Value>(
            test::TestRequest::put().set_json(&test_case.payload),
            &format!("/api/companies/{}/", company_id),
            &app,
        )
        .await;

    match response {
        Ok(_) => panic!("Expected validation error"),
        Err(error) => {
            assert_eq!(error.status(), StatusCode::BAD_REQUEST);
            let error_body: serde_json::Value =
                error.json().await.expect("Failed to parse error response");

            assert_validation_error_with_custom_error(
                &error_body,
                test_case.field,
                test_case.expected_message,
            );
        }
    }
}
