use crate::common::test_context::TestContext;
use actix_web::{http::StatusCode, test};
use serde_json::{json, Value};

const COMPANIES_ENDPOINT: &str = "/api/companies/";

/// テスト用企業の作成
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
        "major_clients": ["クライアントA", "クライアントB"],
        "major_services": ["サービスA", "サービスB"],
        "average_hourly_rate": 4000,
        "bonus": {
            "amount": 1_000_000,
            "frequency": 2
        },
        "status": "Contract",
        "affiliation_start_date": "2023-04-01"
    });

    let response = context
        .authenticated_request(
            test::TestRequest::post().set_json(&payload),
            COMPANIES_ENDPOINT,
        )
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    body["id"]
        .as_str()
        .expect("Company ID not found in response")
        .to_string()
}

/// テスト用の複数企業を作成
pub async fn create_test_companies(context: &TestContext) -> Vec<String> {
    let test_companies = vec![
        json!({
            "company_name": "株式会社A",
            "establishment_year": 2020,
            "location": "東京都渋谷区",
            "website_url": "https://example-a.com",
            "employee_count": 100,
            "contract_type": "Contract",
            "status": "Active"
        }),
        json!({
            "company_name": "株式会社B",
            "establishment_year": 2019,
            "location": "東京都新宿区",
            "website_url": "https://example-b.com",
            "employee_count": 200,
            "contract_type": "Contract",
            "status": "Active"
        }),
        json!({
            "company_name": "株式会社C",
            "establishment_year": 2018,
            "location": "東京都千代田区",
            "website_url": "https://example-c.com",
            "employee_count": 300,
            "contract_type": "Contract",
            "status": "Active"
        }),
    ];

    let mut company_ids = Vec::new();
    for payload in test_companies {
        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
                COMPANIES_ENDPOINT,
            )
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        let body: Value = test::read_body_json(response).await;
        company_ids.push(
            body["id"]
                .as_str()
                .expect("Company ID not found in response")
                .to_string(),
        );
    }

    company_ids
}
