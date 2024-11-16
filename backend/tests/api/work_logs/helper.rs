use crate::api::projects::helper::create_test_project;
use crate::common::test_context::TestContext;
use actix_web::{http::StatusCode, test};
use bson::DateTime as BsonDateTime;
use serde_json::{json, Value};

const WORK_LOGS_ENDPOINT: &str = "/api/work-logs/";

/// テスト用勤怠の作成
pub async fn create_test_work_log(context: &TestContext) -> String {
    let project = create_test_project(context).await;
    let now = BsonDateTime::now();
    let start_time = BsonDateTime::from_millis(now.timestamp_millis() - 3600000); // 1時間前
    let end_time = BsonDateTime::from_millis(now.timestamp_millis() - 1800000); // 30分前

    let payload = json!({
        "project_id": project.id,
        "start_time": start_time.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "end_time": end_time.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "break_time": 15,
        "actual_work_minutes": 45,
        "memo": "テスト用の勤怠データです"
    });

    let response = context
        .authenticated_request(
            test::TestRequest::post().set_json(&payload),
            WORK_LOGS_ENDPOINT,
        )
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    body["id"]
        .as_str()
        .expect("Work Log ID not found in response")
        .to_string()
}

/// テスト用の複数勤怠を作成
pub async fn create_test_work_logs(context: &TestContext) -> Vec<String> {
    let project = create_test_project(context).await;
    let now = BsonDateTime::now();
    let test_work_logs = vec![
        json!({
            "project_id": project.id,
            "start_time": BsonDateTime::from_millis(now.timestamp_millis() - 7200000).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true), // 2時間前
            "end_time": BsonDateTime::from_millis(now.timestamp_millis() - 3600000).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),   // 1時間前
            "break_time": 15,
            "actual_work_minutes": 105,
            "memo": "テスト勤怠データ1"
        }),
        json!({
            "project_id": project.id,
            "start_time": BsonDateTime::from_millis(now.timestamp_millis() - 3600000).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true), // 1時間前
            "end_time": BsonDateTime::from_millis(now.timestamp_millis() - 1800000).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),   // 30分前
            "break_time": 10,
            "actual_work_minutes": 50,
            "memo": "テスト勤怠データ2"
        }),
        json!({
            "project_id": project.id,
            "start_time": BsonDateTime::from_millis(now.timestamp_millis() - 1800000).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true), // 30分前
            "end_time": null,  // 終了時間なし（作業中）
            "memo": "テスト勤怠データ3"
        }),
    ];

    let mut work_log_ids = Vec::new();
    for payload in test_work_logs {
        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
                WORK_LOGS_ENDPOINT,
            )
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        let body: Value = test::read_body_json(response).await;
        work_log_ids.push(
            body["id"]
                .as_str()
                .expect("Work Log ID not found in response")
                .to_string(),
        );
    }

    work_log_ids
}

/// テスト用の勤怠更新データの生成
pub async fn generate_work_log_update_payload(context: &TestContext) -> Value {
    let project = create_test_project(context).await;
    let now = BsonDateTime::now();
    let start_time = BsonDateTime::from_millis(now.timestamp_millis() - 3600000); // 1時間前
    let end_time = BsonDateTime::from_millis(now.timestamp_millis() - 1800000); // 30分前

    let payload = json!({
        "project_id": project.id,
        "start_time": start_time.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "end_time": end_time.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "break_time": 20,
        "actual_work_minutes": 40,
        "memo": "更新された勤怠データです"
    });

    println!(
        "Update payload: {}",
        serde_json::to_string_pretty(&payload).unwrap()
    );
    payload
}
