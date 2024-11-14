use crate::common::test_context::TestContext;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};

const PROJECTS_ENDPOINT: &str = "/api/projects/";

/// テスト用プロジェクトの作成
pub async fn create_test_project(context: &TestContext) -> String {
    let payload = json!({
        "title": "テストプロジェクト",
        "description": "これはテストプロジェクトです",
        "status": "Active",
        "start_date": "2024-01-01",
        "end_date": "2024-12-31",
        "skill_labels": ["Rust", "MongoDB"],
        "company_id": ObjectId::new().to_string(),
        "team_members": [
            {
                "name": "テストメンバー1",
                "role": "Developer"
            }
        ]
    });

    let response = context
        .authenticated_request(
            test::TestRequest::post().set_json(&payload),
            PROJECTS_ENDPOINT,
        )
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    println!("Created Project ID: {}", body["id"]);
    body["id"]
        .as_str()
        .expect("Project ID not found in response")
        .to_string()
}

/// テスト用の複数プロジェクトを作成する
pub async fn create_test_projects(context: &TestContext) -> Vec<String> {
    let test_projects = vec![
        json!({
            "title": "Webアプリケーション開発",
            "description": "新規Webアプリケーションの開発",
            "status": "Active",
            "start_date": "2024-01-01",
            "end_date": "2024-06-30",
            "skill_labels": ["Rust", "React", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "team_members": [
                {
                    "name": "開発者A",
                    "role": "Lead Developer"
                }
            ]
        }),
        json!({
            "title": "モバイルアプリ開発",
            "description": "iOSアプリケーションの開発",
            "status": "Planning",
            "start_date": "2024-03-01",
            "end_date": "2024-08-31",
            "skill_labels": ["Swift", "iOS", "Firebase"],
            "company_id": ObjectId::new().to_string(),
            "team_members": [
                {
                    "name": "開発者B",
                    "role": "iOS Developer"
                }
            ]
        }),
        json!({
            "title": "インフラ構築",
            "description": "クラウドインフラの構築と最適化",
            "status": "Completed",
            "start_date": "2023-10-01",
            "end_date": "2023-12-31",
            "skill_labels": ["AWS", "Terraform", "Docker"],
            "company_id": ObjectId::new().to_string(),
            "team_members": [
                {
                    "name": "インフラエンジニアA",
                    "role": "Infrastructure Engineer"
                }
            ]
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
