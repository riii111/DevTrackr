use crate::api::work_logs::helper::{create_test_work_log, create_test_work_logs};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};

const WORK_LOGS_ENDPOINT: &str = "/api/work-logs/";

#[actix_web::test]
async fn test_get_all_work_logs_success() {
    /*
    勤怠一覧取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の複数勤怠データを作成
        let work_log_ids = create_test_work_logs(&context).await;

        let response = context
            .authenticated_request(test::TestRequest::get(), WORK_LOGS_ENDPOINT)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        assert!(body.is_array());

        // 作成した勤怠数分のデータが取得できていることを確認
        let work_logs = body.as_array().unwrap();
        assert!(work_logs.len() >= work_log_ids.len());

        // 作成した勤怠のIDが含まれていることを確認
        let response_ids: Vec<String> = work_logs
            .iter()
            .map(|work_log| work_log["id"].as_str().unwrap().to_string())
            .collect();
        for id in work_log_ids {
            assert!(response_ids.contains(&id));
        }
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_work_logs_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(WORK_LOGS_ENDPOINT)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_get_work_log_by_id_success() {
    /*
    勤怠IDによる取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let work_log_id = create_test_work_log(&context).await;
        let url = format!("{}{}/", WORK_LOGS_ENDPOINT, work_log_id);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        assert!(body.is_object());
        assert_eq!(body["id"], work_log_id);
    })
    .await;
}

#[actix_web::test]
async fn test_get_work_log_by_id_not_found() {
    /*
    存在しない勤怠IDの場合は404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let non_existent_id = ObjectId::new();
        let url = format!("{}{}/", WORK_LOGS_ENDPOINT, non_existent_id);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        let error_body: Value = test::read_body_json(response).await;

        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "勤怠が見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
}

#[actix_web::test]
async fn test_get_work_log_by_id_invalid_id() {
    /*
    無効なIDフォーマットの場合は400エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let url = format!("{}{}/", WORK_LOGS_ENDPOINT, "invalid-id");

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let error_body: Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "不正なリクエスト",
                "message": "無効なIDです",
                "code": "BAD_REQUEST"
            })
        );
    })
    .await;
}
