use crate::api::helper::validation::assert_validation_error_with_custom_error;
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
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::post()
                .uri(COMPANIES_ENDPOINT)
                .set_json(&*TEST_PAYLOAD),
        )
        .await;

    let res = test::call_service(&app, request.to_request()).await;

    assert_eq!(res.status(), StatusCode::CREATED);

    // レスポンスボディの検証
    let body: serde_json::Value = test::read_body_json(res).await;
    assert!(body.get("id").is_some());
    assert_eq!(body["company_name"], "テスト企業");
}

#[actix_web::test]
async fn test_create_company_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 認証なしでリクエスト
    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(COMPANIES_ENDPOINT)
            .set_json(&*TEST_PAYLOAD)
            .to_request(),
    )
    .await;

    // 認証エラーの検証
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
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
        field: "affiliation_start_date",
        expected_message: "無効な日付フォーマットです"
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
        field: "contract_type",
        expected_message: "無効な契約タイプです"
    }
)]
#[actix_web::test]
async fn test_create_company_validation(#[case] test_case: ValidationTestCase) {
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // ログインを実行し、認証済みリクエストを作成
    let (_, request) = test_app
        .login_and_create_next_request(
            test::TestRequest::post()
                .uri(COMPANIES_ENDPOINT)
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
