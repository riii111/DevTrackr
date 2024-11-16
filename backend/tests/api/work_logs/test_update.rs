use crate::api::helper::validation::{
    assert_validation_error_with_custom_error, ValidationTestCase,
};
use crate::api::projects::helper::create_test_project;
use crate::api::work_logs::helper::{create_test_work_log, generate_work_log_update_payload};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use rstest::rstest;
use serde_json::json;

const WORK_LOGS_ENDPOINT: &str = "/api/work-logs/";

#[actix_web::test]
async fn test_update_work_logs_success() {
    /*
    勤怠の更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の勤怠を作成
        let work_log_id = create_test_work_log(&context).await;

        // 更新前のデータを取得
        let get_response_before = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, work_log_id),
            )
            .await;
        let body_before: serde_json::Value = test::read_body_json(get_response_before).await;
        println!(
            "Before update: {}",
            serde_json::to_string_pretty(&body_before).unwrap()
        );

        // 更新用のペイロードを生成
        let update_payload = generate_work_log_update_payload(&context).await;
        println!(
            "Update payload: {}",
            serde_json::to_string_pretty(&update_payload).unwrap()
        );

        // 勤怠更新APIの実行
        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, work_log_id),
            )
            .await;

        // エラーの場合はレスポンスボディを出力
        if update_response.status() != StatusCode::NO_CONTENT {
            let error_body: serde_json::Value = test::read_body_json(update_response).await;
            println!(
                "Error response: {}",
                serde_json::to_string_pretty(&error_body).unwrap()
            );
        }

        // 更新後のデータを取得して確認
        let get_response = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, work_log_id),
            )
            .await;

        assert_eq!(get_response.status(), StatusCode::OK);
        let get_body: serde_json::Value = test::read_body_json(get_response).await;
        println!(
            "After update: {}",
            serde_json::to_string_pretty(&get_body).unwrap()
        );

        // 更新されたフィールドを確認
        assert_eq!(
            get_body["memo"], update_payload["memo"],
            "メモが正しく更新されていません"
        );
        assert_eq!(
            get_body["project_id"], update_payload["project_id"],
            "プロジェクトIDが正しく更新されていません"
        );
        assert_eq!(
            get_body["break_time"], update_payload["break_time"],
            "休憩時間が正しく更新されていません"
        );
        assert_eq!(
            get_body["actual_work_minutes"], update_payload["actual_work_minutes"],
            "実労働時間が正しく更新されていません"
        );
    })
    .await;
}

#[actix_web::test]
async fn test_update_work_logs_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::put()
                .uri(&format!("{}{}/", WORK_LOGS_ENDPOINT, ObjectId::new()))
                .set_json(json!({
                    "project_id": ObjectId::new().to_string(),
                    "start_time": BsonDateTime::now().to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                    "memo": "テストメモ"
                }))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_update_work_logs_not_found() {
    /*
    存在しない勤怠の更新を試みた場合、404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let non_existent_id = ObjectId::new().to_string();
        let update_payload = generate_work_log_update_payload(&context).await;

        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, non_existent_id),
            )
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    })
    .await;
}

#[actix_web::test]
async fn test_update_work_logs_invalid_id() {
    /*
    無効なIDで更新を試みた場合、400エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let invalid_id = "invalid-id";
        let update_payload = generate_work_log_update_payload(&context).await;

        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, invalid_id),
            )
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    })
    .await;
}

// バリデーションテストケース
#[rstest]
// プロジェクトIDのバリデーション
#[case::invalid_project_id(
    ValidationTestCase {
        name: "無効なプロジェクトID",
        payload: json!({
            "project_id": "invalid_id_format",  // 無効なIDを明示的に指定
            "start_time": BsonDateTime::now().to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "end_time": null,
            "memo": "テストメモ"
        }),
        field: "project_id",  // フィールド名を修正
        expected_message: "入力形式が正しくありません"
    }
)]
// 時間のバリデーション
#[case::future_start_time(
    ValidationTestCase {
        name: "未来の開始時間",
        payload: {
            let future_time = BsonDateTime::from_millis(BsonDateTime::now().timestamp_millis() + 3600000);
            json!({
                "start_time": future_time.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": null,
                "memo": "テストメモ"
            })
        },
        field: "time",
        expected_message: "開始時間は現在時刻より前である必要があります"
    }
)]
#[case::end_time_before_start_time(
    ValidationTestCase {
        name: "終了時間が開始時間より前",
        payload: {
            let now = BsonDateTime::now();
            let start_time = now.timestamp_millis() - 1800000; // 30分前
            let end_time = now.timestamp_millis() - 3600000;   // 1時間前
            json!({
                "start_time": BsonDateTime::from_millis(start_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": BsonDateTime::from_millis(end_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "memo": "テストメモ"
            })
        },
        field: "time",
        expected_message: "終了時間は開始時間より後である必要があります"
    }
)]
#[case::future_end_time(
    ValidationTestCase {
        name: "未来の終了時間",
        payload: {
            let now = BsonDateTime::now();
            let start_time = now.timestamp_millis() - 3600000; // 1時間前
            let future_time = now.timestamp_millis() + 3600000; // 1時間後
            json!({
                "start_time": BsonDateTime::from_millis(start_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": BsonDateTime::from_millis(future_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "memo": "テストメモ"
            })
        },
        field: "time",
        expected_message: "終了時間は現在時刻より前である必要があります"
    }
)]
// 休憩時間のバリデーション
#[case::negative_break_time(
    ValidationTestCase {
        name: "休憩時間がマイナス",
        payload: {
            let now = BsonDateTime::now();
            let start_time = now.timestamp_millis() - 3600000; // 1時間前
            json!({
                "start_time": BsonDateTime::from_millis(start_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": now.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "break_time": -1,
                "memo": "テストメモ"
            })
        },
        field: "break_time",
        expected_message: "休憩時間は0以上である必要があります"
    }
)]
// 実労働時間のバリデーション
#[case::negative_actual_work_minutes(
    ValidationTestCase {
        name: "実労働時間がマイナス",
        payload: {
            let now = BsonDateTime::now();
            let start_time = now.timestamp_millis() - 3600000; // 1時間前
            json!({
                "start_time": BsonDateTime::from_millis(start_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": now.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "actual_work_minutes": -1,
                "memo": "テストメモ"
            })
        },
        field: "actual_work_minutes",
        expected_message: "実労働時間は0以上である必要があります"
    }
)]
// メモのバリデーション
#[case::memo_too_long(
    ValidationTestCase {
        name: "メモが長すぎる",
        payload: {
            let now = BsonDateTime::now();
            let start_time = now.timestamp_millis() - 3600000; // 1時間前
            json!({
                "start_time": BsonDateTime::from_millis(start_time).to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "end_time": now.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                "memo": "a".repeat(1001)
            })
        },
        field: "memo",
        expected_message: "メモは0〜1000文字である必要があります"
    }
)]
#[actix_web::test]
async fn test_update_work_logs_validation(#[case] test_case: ValidationTestCase) {
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の勤怠を作成
        let work_log_id = create_test_work_log(&context).await;
        let mut payload = test_case.payload.clone();

        // 無効なプロジェクトIDのテスト以外は、正しいプロジェクトIDを設定する
        if !test_case.name.contains("無効なプロジェクトID") {
            let project = create_test_project(&context).await;
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("project_id".to_string(), json!(project.id));
            }
        }

        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&payload),
                &format!("{}{}/", WORK_LOGS_ENDPOINT, work_log_id),
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
        if test_case.name.contains("無効なプロジェクトID") {
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
