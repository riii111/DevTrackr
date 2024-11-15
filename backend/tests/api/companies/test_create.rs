use crate::api::helper::validation::{
    assert_validation_error_with_custom_error, ValidationTestCase,
};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use lazy_static::lazy_static;
use rstest::rstest;
use serde_json::json;

const COMPANIES_ENDPOINT: &str = "/api/companies/";

lazy_static! {
    static ref TEST_PAYLOAD: serde_json::Value = json!({
        "company_name": "テスト企業",
        "establishment_year": 2020,
        "location": "東京都千代田区",
        "website_url": "https://example.com",
        "employee_count": 100,
        "annual_sales": {
            "amount": 100_000_000,
            "fiscal_year": 2024
        },
        "contract_type": "Contract",
        "major_clients": ["クライアントA", "クライアントB"],
        "major_services": ["サービスA", "サービスB"],
        "average_hourly_rate": 4000,
        "bonus": {
            "amount": 1_000_000,
            "frequency": 2
        },
        "status": "Contract",
        "affiliation_start_date": "2023-04-01",
        "affiliation_end_date": "2024-03-31"
    });
}

#[actix_web::test]
async fn test_create_company_success() {
    /*
    企業の新規作成が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // 企業作成APIの実行
        let create_response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&*TEST_PAYLOAD),
                COMPANIES_ENDPOINT,
            )
            .await;

        // 正常に作成されたことを確認
        assert_eq!(create_response.status(), StatusCode::CREATED);
        let create_body: serde_json::Value = test::read_body_json(create_response).await;
        let company_id = create_body["id"]
            .as_str()
            .expect("Company ID not found in response");

        println!("Created company ID: {}", company_id);
        println!("Request URI: {}{}", COMPANIES_ENDPOINT, company_id);

        // 企業取得APIの実行
        let get_response = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", COMPANIES_ENDPOINT, company_id),
            )
            .await;

        // 正常に取得できることを確認
        assert_eq!(get_response.status(), StatusCode::OK);
        let get_body: serde_json::Value = test::read_body_json(get_response).await;
        assert_eq!(get_body["company_name"], "テスト企業");
    })
    .await;
}

#[actix_web::test]
async fn test_create_company_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::post()
                .uri(COMPANIES_ENDPOINT)
                .set_json(&*TEST_PAYLOAD)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[rstest]
// 企業名のバリデーション
#[case::company_name_too_short(
    ValidationTestCase {
        name: "企業名が短すぎる",
        payload: json!({
            "company_name": "A",
            "establishment_year": 2020,
            "location": "東京都千代田区",
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
// 設立年のバリデーション
#[case::invalid_establishment_year(
    ValidationTestCase {
        name: "無効な設立年",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 1700,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01"
        }),
        field: "establishment_year",
        expected_message: "設立年は1800年から現在までの間である必要があります"
    }
)]
// URLのバリデーション
#[case::invalid_website_url(
    ValidationTestCase {
        name: "無効なURL",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "invalid-url",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01"
        }),
        field: "website_url",
        expected_message: "有効なURLを入力してください"
    }
)]
// 日付のバリデーション
#[case::future_start_date(
    ValidationTestCase {
        name: "未来の開始日",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2025-04-01"
        }),
        field: "dates",
        expected_message: "契約開始日は現在日付より前である必要があります"
    }
)]
#[case::end_date_before_start_date(
    ValidationTestCase {
        name: "契約終了日が契約開始日より前",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
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
// 数値バリデーション
#[case::invalid_employee_count(
    ValidationTestCase {
        name: "従業員数が0以下",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 0,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01"
        }),
        field: "employee_count",
        expected_message: "従業員数は1以上である必要があります"
    }
)]
#[case::invalid_hourly_rate(
    ValidationTestCase {
        name: "時給が範囲外",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "average_hourly_rate": 200,
            "affiliation_start_date": "2023-04-01"
        }),
        field: "average_hourly_rate",
        expected_message: "平均時給は500円から100,000円の間である必要があります"
    }
)]
// 配列の長さバリデーション
#[case::too_many_major_clients(
    ValidationTestCase {
        name: "主要顧客が多すぎる",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "major_clients": [
                "クライアント1", "クライアント2", "クライアント3", "クライアント4", "クライアント5",
                "クライアント6", "クライアント7", "クライアント8", "クライアント9", "クライアント10",
                "クライアント11"
            ],
            "affiliation_start_date": "2023-04-01"
        }),
        field: "major_clients",
        expected_message: "主要顧客は最大10件まで登録できます"
    }
)]
// デシリアライズエラー
#[case::invalid_date_format(
    ValidationTestCase {
        name: "無効な日付フォーマット",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Contract",
            "affiliation_start_date": "2023/04/01"
        }),
        field: "unknown",
        expected_message: "入力形式が正しくありません"
    }
)]
#[case::invalid_enum_value(
    ValidationTestCase {
        name: "無効な契約タイプ",
        payload: json!({
            "company_name": "テスト企業",
            "establishment_year": 2020,
            "location": "東京都千代田区",
            "website_url": "https://example.com",
            "employee_count": 100,
            "contract_type": "InvalidType",
            "status": "Contract",
            "affiliation_start_date": "2023-04-01"
        }),
        field: "unknown",
        expected_message: "入力形式が正しくありません"
    }
)]
#[actix_web::test]
async fn test_create_company_validation(#[case] test_case: ValidationTestCase) {
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&test_case.payload),
                COMPANIES_ENDPOINT,
            )
            .await;

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "バリデーションテスト失敗: {}",
            test_case.name
        );

        let error_body: serde_json::Value = test::read_body_json(response).await;

        // デシリアライズエラーの場合は特別な処理
        if test_case.name.contains("無効な日付フォーマット")
            || test_case.name.contains("無効な契約タイプ")
        {
            assert_eq!(
                error_body,
                json!({
                    "error": "入力エラー",
                    "field_errors": [{
                        "field": "unknown",
                        "message": "入力形式が正しくありません"
                    }]
                })
            );
        } else {
            // 通常のバリデーションエラー
            assert_validation_error_with_custom_error(
                &error_body,
                test_case.field,
                test_case.expected_message,
            );
        }
    })
    .await;
}
