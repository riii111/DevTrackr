use crate::api::companies::helper::create_test_company;
use crate::common::test_context::TestContext;
use actix_web::{http::StatusCode, test};
use lazy_static::lazy_static;
use serde_json::{json, Value};

const PROJECTS_ENDPOINT: &str = "/api/projects/";

lazy_static! {
    pub static ref DEFAULT_UPDATE_PAYLOAD: serde_json::Value = json!({
        "title": "更新後プロジェクト",
        "description": "更新後の説明文",
        "status": "InProgress",
        "skill_labels": ["Rust", "AWS", "Docker"],
        "hourly_pay": 4000,
        "total_working_time": 160
    });
}

/// テストプロジェクト作成時の戻り値を拡張
pub struct TestProject {
    pub id: String,
    pub company_id: String,
}

/// テスト用プロジェクトの作成
pub async fn create_test_project(context: &TestContext) -> TestProject {
    let company_id = create_test_company(context).await;

    let payload = json!({
        "title": "テストプロジェクト",
        "description": "これはテストプロジェクトです",
        "status": "Planning",
        "skill_labels": ["Rust", "MongoDB"],
        "company_id": company_id,
        "hourly_pay": 3000
    });

    let response = context
        .authenticated_request(
            test::TestRequest::post().set_json(&payload),
            PROJECTS_ENDPOINT,
        )
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    TestProject {
        id: body["id"]
            .as_str()
            .expect("Project ID not found in response")
            .to_string(),
        company_id,
    }
}

/// テスト用の複数プロジェクトを作成する
pub async fn create_test_projects(context: &TestContext) -> Vec<String> {
    let company_id = create_test_company(context).await;

    let test_projects = vec![
        json!({
            "title": "Webアプリケーション開発",
            "description": "新規Webアプリケーションの開発",
            "status": "Planning",
            "skill_labels": ["Rust", "React", "MongoDB"],
            "company_id": company_id,
            "hourly_pay": 3500
        }),
        json!({
            "title": "モバイルアプリ開発",
            "description": "iOSアプリケーションの開発",
            "status": "Planning",
            "skill_labels": ["Swift", "iOS", "Firebase"],
            "company_id": company_id,
            "hourly_pay": 4000
        }),
        json!({
            "title": "インフラ構築",
            "description": "クラウドインフラの構築と最適化",
            "status": "Completed",
            "skill_labels": ["AWS", "Terraform", "Docker"],
            "company_id": company_id,
            "hourly_pay": 4500
        }),
    ];

    let mut project_ids = Vec::new();
    for payload in test_projects {
        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
                PROJECTS_ENDPOINT,
            )
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        let body: Value = test::read_body_json(response).await;
        project_ids.push(
            body["id"]
                .as_str()
                .expect("Project ID not found in response")
                .to_string(),
        );
    }

    project_ids
}
