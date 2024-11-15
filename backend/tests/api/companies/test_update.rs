use crate::api::companies::helper::create_test_company;
use crate::api::helper::validation::{
    assert_validation_error_with_custom_error, ValidationTestCase,
};
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

const COMPANIES_ENDPOINT: &str = "/api/companies/";
#[actix_web::test]
async fn test_update_company_success() {
    /*
    企業の更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の企業を作成
        let company_id = create_test_company(&context).await;

        // 企業情報の更新
        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&*UPDATE_PAYLOAD),
                &format!("{}{}/", COMPANIES_ENDPOINT, company_id),
            )
            .await;

        assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

        // 更新後の企業情報を取得して検証
        let get_response = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", COMPANIES_ENDPOINT, company_id),
            )
            .await;

        // 正常に取得できることを確認
        assert_eq!(get_response.status(), StatusCode::OK);
        let get_body: serde_json::Value = test::read_body_json(get_response).await;
        assert_eq!(get_body["company_name"], "更新後企業名");
    })
    .await;
}

#[actix_web::test]
async fn test_update_company_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::put()
                .uri("/api/companies/123/")
                .set_json(&*UPDATE_PAYLOAD)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_update_company_not_found() {
    /*
    存在しない企業の更新を試みた場合、404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let non_existent_id = ObjectId::new();

        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&*UPDATE_PAYLOAD),
                &format!("{}{}/", COMPANIES_ENDPOINT, non_existent_id),
            )
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let error_body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "更新対象の企業が見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
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
        expected_message: "企業名は2〜100文字である必要があります"
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
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の企業を作成
        let company_id = create_test_company(&context).await;

        // バリデーションエラーのテスト
        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&test_case.payload),
                &format!("{}{}/", COMPANIES_ENDPOINT, company_id),
            )
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let error_body: serde_json::Value = test::read_body_json(response).await;

        assert_validation_error_with_custom_error(
            &error_body,
            test_case.field,
            test_case.expected_message,
        );
    })
    .await;
}
