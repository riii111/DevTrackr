use crate::api::companies::helper::create_test_company;
use crate::api::helper::validation::{
    assert_validation_error_with_custom_error, ValidationTestCase,
};
use crate::api::projects::helper::{create_test_project, DEFAULT_UPDATE_PAYLOAD};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use rstest::rstest;
use serde_json::json;

const PROJECTS_ENDPOINT: &str = "/api/projects/";

#[actix_web::test]
async fn test_update_project_success() {
    /*
    プロジェクトの更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用のプロジェクトを作成
        let test_project = create_test_project(&context).await;

        let mut update_payload = DEFAULT_UPDATE_PAYLOAD.clone();
        // company_idを追加
        update_payload
            .as_object_mut()
            .unwrap()
            .insert("company_id".to_string(), json!(test_project.company_id));

        // プロジェクト情報の更新
        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                &format!("{}{}/", PROJECTS_ENDPOINT, test_project.id),
            )
            .await;

        assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

        // 更新後のプロジェクト情報を取得して検証
        let get_response = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", PROJECTS_ENDPOINT, test_project.id),
            )
            .await;

        assert_eq!(get_response.status(), StatusCode::OK);
        let get_body: serde_json::Value = test::read_body_json(get_response).await;
        assert_eq!(get_body["title"], "更新後プロジェクト");
        assert_eq!(get_body["hourly_pay"], 4000);
        assert_eq!(get_body["company_id"], test_project.company_id);
    })
    .await;
}

#[actix_web::test]
async fn test_update_project_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::put()
                .uri("/api/projects/123/")
                .set_json(&*DEFAULT_UPDATE_PAYLOAD)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_update_project_not_found() {
    /*
    存在しないプロジェクトの更新を試みた場合、404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用の会社を作成してcompany_idを取得
        let company_id = create_test_company(&context).await;
        let non_existent_id = ObjectId::new();

        let mut update_payload = DEFAULT_UPDATE_PAYLOAD.clone();
        update_payload
            .as_object_mut()
            .unwrap()
            .insert("company_id".to_string(), json!(company_id));

        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                &format!("{}{}/", PROJECTS_ENDPOINT, non_existent_id),
            )
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let error_body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "更新対象のプロジェクトが見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
}

#[rstest]
// タイトルのバリデーション
#[case::title_empty(
    ValidationTestCase {
        name: "タイトルが空",
        payload: json!({
            "title": "",
            "description": "プロジェクトの説明",
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS"],
            "hourly_pay": 3000,
            "total_working_time": 160
        }),
        field: "title",
        expected_message: "タイトルは1〜100文字である必要があります"
    }
)]
#[case::title_too_long(
    ValidationTestCase {
        name: "タイトルが長すぎる",
        payload: json!({
            "title": "a".repeat(101),
            "description": "プロジェクトの説明",
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000,
            "total_working_time": 160
        }),
        field: "title",
        expected_message: "タイトルは1〜100文字である必要があります"
    }
)]
// 説明のバリデーション
#[case::description_too_long(
    ValidationTestCase {
        name: "説明が長すぎる",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "a".repeat(1001),
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000,
            "total_working_time": 160
        }),
        field: "description",
        expected_message: "説明は1000文字以内である必要があります"
    }
)]
// スキルラベルのバリデーション
#[case::too_many_skill_labels(
    ValidationTestCase {
        name: "スキルラベルが多すぎる",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "プロジェクトの説明",
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS", "Docker", "Kubernetes", "React", 
                           "TypeScript", "Python", "Go", "Java", "C++", "PHP"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000,
            "total_working_time": 160
        }),
        field: "skill_labels",
        expected_message: "スキルラベルは最大10個まで登録できます"
    }
)]
// 時給のバリデーション
#[case::negative_hourly_pay(
    ValidationTestCase {
        name: "時給がマイナス",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "プロジェクトの説明",
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": -1,
            "total_working_time": 160
        }),
        field: "hourly_pay",
        expected_message: "時給は0以上である必要があります"
    }
)]
// 総作業時間のバリデーション
#[case::negative_total_working_time(
    ValidationTestCase {
        name: "総作業時間がマイナス",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "プロジェクトの説明",
            "status": "InProgress",
            "skill_labels": ["Rust", "AWS"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000,
            "total_working_time": -1
        }),
        field: "total_working_time",
        expected_message: "総作業時間は0以上である必要があります"
    }
)]
#[actix_web::test]
async fn test_update_project_validation(#[case] test_case: ValidationTestCase) {
    /*
    プロジェクト更新時のバリデーションをテストする
    各バリデーションルールに違反するデータを送信し、適切なエラーメッセージが返されることを確認する
     */
    TestApp::run_authenticated_test(|context| async move {
        // テスト用のプロジェクトを作成
        let test_project = create_test_project(&context).await;

        let mut validation_payload = test_case.payload.clone();
        validation_payload
            .as_object_mut()
            .unwrap()
            .insert("company_id".to_string(), json!(test_project.company_id));

        // バリデーションエラーのテスト
        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&validation_payload),
                &format!("{}{}/", PROJECTS_ENDPOINT, test_project.id),
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
